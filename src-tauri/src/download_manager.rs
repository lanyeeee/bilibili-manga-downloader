use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use anyhow::{anyhow, Context};
use bytes::Bytes;
use reqwest::StatusCode;
use serde_json::json;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, Semaphore};
use tokio::task::JoinSet;

use crate::config::Config;
use crate::events;
use crate::events::{DownloadSpeedEvent, DownloadSpeedEventPayload};
use crate::extensions::{AnyhowErrorToStringChain, IgnoreRwLockPoison};
use crate::responses::{BiliResp, ImageIndexRespData, ImageTokenRespData};
use crate::types::EpisodeInfo;

/// 用于管理下载任务
///
/// 克隆 `DownloadManager` 的开销极小，性能开销几乎可以忽略不计。
/// 可以放心地在多个线程中传递和使用它的克隆副本。
///
/// 具体来说：
/// - `app` 是 `AppHandle` 类型，根据 `Tauri` 文档，它的克隆开销是极小的。
/// - 其他字段都被 `Arc` 包裹，这些字段的克隆操作仅仅是增加引用计数。
#[derive(Clone)]
pub struct DownloadManager {
    app: AppHandle,
    sender: Arc<mpsc::Sender<EpisodeInfo>>,
    ep_sem: Arc<Semaphore>,
    img_sem: Arc<Semaphore>,
    byte_per_sec: Arc<AtomicU64>,
    downloaded_image_count: Arc<AtomicU32>,
    total_image_count: Arc<AtomicU32>,
}

impl DownloadManager {
    pub fn new(app: AppHandle) -> Self {
        let (sender, receiver) = mpsc::channel::<EpisodeInfo>(32);
        let ep_sem = Arc::new(Semaphore::new(16));
        let img_sem = Arc::new(Semaphore::new(50));
        let manager = DownloadManager {
            app,
            sender: Arc::new(sender),
            ep_sem,
            img_sem,
            byte_per_sec: Arc::new(AtomicU64::new(0)),
            downloaded_image_count: Arc::new(AtomicU32::new(0)),
            total_image_count: Arc::new(AtomicU32::new(0)),
        };

        tauri::async_runtime::spawn(manager.clone().log_download_speed());
        tauri::async_runtime::spawn(manager.clone().receiver_loop(receiver));

        manager
    }

    pub async fn submit_episode(&self, ep_info: EpisodeInfo) -> anyhow::Result<()> {
        Ok(self.sender.send(ep_info).await?)
    }

    #[allow(clippy::cast_precision_loss)]
    async fn log_download_speed(self) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            let byte_per_sec = self.byte_per_sec.swap(0, Ordering::Relaxed);
            let mega_byte_per_sec = byte_per_sec as f64 / 1024.0 / 1024.0;
            let speed = format!("{mega_byte_per_sec:.2} MB/s");
            emit_download_speed_event(&self.app, speed);
        }
    }

    async fn receiver_loop(self, mut receiver: Receiver<EpisodeInfo>) {
        while let Some(ep) = receiver.recv().await {
            let manager = self.clone();
            tauri::async_runtime::spawn(manager.process_episode(ep));
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn process_episode(self, ep_info: EpisodeInfo) -> anyhow::Result<()> {
        emit_pending_event(&self.app, ep_info.episode_id, ep_info.episode_title.clone());

        let config = self.app.state::<RwLock<Config>>();
        let cookie = config.read_or_panic().get_cookie();

        let image_index_data = get_image_index_data(ep_info.episode_id, &cookie).await?;
        let image_token_data = get_image_token_data(&image_index_data, &cookie).await?;

        let temp_download_dir = get_temp_download_dir(&self.app, &ep_info)?;
        std::fs::create_dir_all(&temp_download_dir)
            .context(format!("创建目录 {temp_download_dir:?} 失败"))?;
        // 构造图片下载链接
        let urls: Vec<String> = image_token_data
            .into_iter()
            .map(|data| (data.url, data.token))
            .map(|(url, token)| format!("{url}?token={token}"))
            .collect();
        let total = urls.len() as u32;
        // 记录总共需要下载的图片数量
        self.total_image_count.fetch_add(total, Ordering::Relaxed);
        let current = Arc::new(AtomicU32::new(0));
        let mut join_set = JoinSet::new();
        // 限制同时下载的章节数量
        let permit = self.ep_sem.acquire().await?;
        emit_start_event(
            &self.app,
            ep_info.episode_id,
            ep_info.episode_title.clone(),
            total,
        );
        for (i, url) in urls.iter().enumerate() {
            let manager = self.clone();
            let ep_id = ep_info.episode_id;
            let save_path = temp_download_dir.join(format!("{i:03}.jpg"));
            let url = url.clone();
            let current = current.clone();
            // 创建下载任务
            join_set.spawn(manager.download_image(url, save_path, ep_id, current));
        }
        // 逐一处理完成的下载任务
        while let Some(completed_task) = join_set.join_next().await {
            completed_task?;
            self.downloaded_image_count.fetch_add(1, Ordering::Relaxed);
            let downloaded_image_count = self.downloaded_image_count.load(Ordering::Relaxed);
            let total_image_count = self.total_image_count.load(Ordering::Relaxed);
            // 更新下载进度
            emit_update_overall_progress_event(
                &self.app,
                downloaded_image_count,
                total_image_count,
            );
        }
        drop(permit);
        // 如果DownloadManager所有图片全部都已下载(无论成功或失败)，则清空下载进度
        let downloaded_image_count = self.downloaded_image_count.load(Ordering::Relaxed);
        let total_image_count = self.total_image_count.load(Ordering::Relaxed);
        if downloaded_image_count == total_image_count {
            self.downloaded_image_count.store(0, Ordering::Relaxed);
            self.total_image_count.store(0, Ordering::Relaxed);
        }
        // 检查此章节的图片是否全部下载成功
        let current = current.load(Ordering::Relaxed);
        if current == total {
            // 下载成功，则把临时目录重命名为正式目录
            if let Some(parent) = temp_download_dir.parent() {
                let download_dir = parent.join(&ep_info.episode_title);
                std::fs::rename(&temp_download_dir, &download_dir).context(format!(
                    "将 {temp_download_dir:?} 重命名为 {download_dir:?} 失败"
                ))?;
            }
            emit_end_event(&self.app, ep_info.episode_id, None);
        } else {
            let err_msg = Some(format!("总共有 {total} 张图片，但只下载了 {current} 张"));
            emit_end_event(&self.app, ep_info.episode_id, err_msg);
        };
        Ok(())
    }

    // TODO: 把current变量名改成downloaded_count比较合适
    async fn download_image(
        self,
        url: String,
        save_path: PathBuf,
        ep_id: i64,
        current: Arc<AtomicU32>,
    ) {
        // 下载图片
        let permit = match self.img_sem.acquire().await.map_err(anyhow::Error::from) {
            Ok(permit) => permit,
            Err(err) => {
                let err = err.context("获取下载图片的semaphore失败");
                emit_error_event(&self.app, ep_id, url, err.to_string_chain());
                return;
            }
        };
        let image_data = match get_image_bytes(&url).await {
            Ok(data) => data,
            Err(err) => {
                let err = err.context(format!("下载图片 {url} 失败"));
                emit_error_event(&self.app, ep_id, url, err.to_string_chain());
                return;
            }
        };
        drop(permit);
        // 保存图片
        if let Err(err) = std::fs::write(&save_path, &image_data).map_err(anyhow::Error::from) {
            let err = err.context(format!("保存图片 {save_path:?} 失败"));
            emit_error_event(&self.app, ep_id, url, err.to_string_chain());
            return;
        }
        // 记录下载字节数
        self.byte_per_sec
            .fetch_add(image_data.len() as u64, Ordering::Relaxed);
        // 更新章节下载进度
        let current = current.fetch_add(1, Ordering::Relaxed) + 1;
        emit_success_event(
            &self.app,
            ep_id,
            save_path.to_string_lossy().to_string(), // TODO: 把save_path.to_string_lossy().to_string()保存到一个变量里，像current一样
            current,
        );
    }
}

fn get_temp_download_dir(app: &AppHandle, ep_info: &EpisodeInfo) -> anyhow::Result<PathBuf> {
    let download_dir = app
        .path()
        .app_data_dir()?
        .join("漫画下载")
        .join(&ep_info.manga_title)
        .join(format!(".下载中-{}", ep_info.episode_title)); // 以 `.下载中-` 开头，表示是临时目录
    Ok(download_dir)
}

async fn get_image_bytes(url: &str) -> anyhow::Result<Bytes> {
    // TODO: 添加重试规则
    let http_res = reqwest::get(url).await?;

    let status = http_res.status();
    if status != StatusCode::OK {
        let text = http_res.text().await?;
        let err = anyhow!("下载图片 {url} 失败，预料之外的状态码: {text}");
        return Err(err);
    }

    let image_data = http_res.bytes().await?;

    Ok(image_data)
}

async fn get_image_index_data(episode_id: i64, cookie: &str) -> anyhow::Result<ImageIndexRespData> {
    let payload = json!({"ep_id": episode_id});
    // 发送获取ImageIndexRespData的请求
    let http_resp = reqwest::Client::new()
        .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex")
        .query(&[("device", "pc"), ("platform", "web")])
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("cookie", cookie)
        .json(&payload)
        .send()
        .await?;
    // 检查http响应状态码
    let status = http_resp.status();
    let body = http_resp.text().await?;
    if status != StatusCode::OK {
        return Err(anyhow!(
            "获取章节 `{episode_id}` 的ImageIndexRespData失败，预料之外的状态码({status}): {body}"
        ));
    }
    // 尝试将body解析为BiliResp
    let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
        "获取章节 `{episode_id}` 的ImageIndexRespData失败，将body解析为BiliResp失败: {body}"
    ))?;
    // 检查BiliResp的code字段
    if bili_resp.code != 0 {
        return Err(anyhow!(
            "获取章节 `{episode_id}` 的ImageIndexRespData失败，预料之外的code: {bili_resp:?}"
        ));
    }
    // 检查BiliResp的data是否存在
    let Some(data) = bili_resp.data else {
        return Err(anyhow!(
            "获取章节 `{episode_id}` 的ImageIndexRespData失败，data字段不存在: {bili_resp:?}"
        ));
    };
    // 尝试将data解析为ImageIndexRespData
    let data_str = data.to_string();
    let image_index_data = serde_json::from_str::<ImageIndexRespData>(&data_str).context(format!(
        "获取章节 `{episode_id}` 的ImageIndexRespData失败，将data解析为ImageIndexRespData失败: {data_str}"
    ))?;

    Ok(image_index_data)
}

async fn get_image_token_data(
    image_index_data: &ImageIndexRespData,
    cookie: &str,
) -> anyhow::Result<ImageTokenRespData> {
    let urls: Vec<String> = image_index_data
        .images
        .iter()
        .map(|img| img.path.clone())
        .collect();
    let urls_str = serde_json::to_string(&urls)?;
    let payload = json!({"urls": urls_str});
    // 发送获取ImageTokenData的请求
    let http_resp = reqwest::Client::new()
        .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken")
        .query(&[("device", "pc"), ("platform", "web")])
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("cookie", cookie)
        .json(&payload)
        .send()
        .await?;
    // 检查http响应状态码
    let status = http_resp.status();
    let body = http_resp.text().await?;
    if status != StatusCode::OK {
        return Err(anyhow!(
            "获取ImageTokenData失败，预料之外的状态码({status}): {body}"
        ));
    }
    // 尝试将body解析为BiliResp
    let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
        "获取ImageTokenData失败，将body解析为BiliResp失败: {body}"
    ))?;
    // 检查BiliResp的code字段
    if bili_resp.code != 0 {
        let err = anyhow!("获取ImageTokenData失败，预料之外的code: {bili_resp:?}");
        return Err(anyhow!(err));
    }
    // 检查BiliResp的data是否存在
    let Some(data) = bili_resp.data else {
        let err = anyhow!("获取ImageTokenData失败，data字段不存在: {bili_resp:?}");
        return Err(anyhow!(err));
    };
    // 尝试将data解析为ImageTokenRespData
    let data_str = data.to_string();
    let image_token_data = serde_json::from_str::<ImageTokenRespData>(&data_str).context(
        format!("获取ImageTokenData失败，将data解析为ImageTokenRespData失败: {data_str}"),
    )?;

    Ok(image_token_data)
}

fn emit_start_event(app: &AppHandle, ep_id: i64, title: String, total: u32) {
    let payload = events::DownloadEpisodeStartEventPayload {
        ep_id,
        title,
        total,
    };
    let event = events::DownloadEpisodeStartEvent(payload);
    let _ = event.emit(app);
}

fn emit_pending_event(app: &AppHandle, ep_id: i64, title: String) {
    let payload = events::DownloadEpisodePendingEventPayload { ep_id, title };
    let event = events::DownloadEpisodePendingEvent(payload);
    let _ = event.emit(app);
}

fn emit_success_event(app: &AppHandle, ep_id: i64, url: String, current: u32) {
    let payload = events::DownloadImageSuccessEventPayload {
        ep_id,
        url,
        current,
    };
    let event = events::DownloadImageSuccessEvent(payload);
    let _ = event.emit(app);
}

fn emit_error_event(app: &AppHandle, ep_id: i64, url: String, err_msg: String) {
    let payload = events::DownloadImageErrorEventPayload {
        ep_id,
        url,
        err_msg,
    };
    let event = events::DownloadImageErrorEvent(payload);
    let _ = event.emit(app);
}

fn emit_end_event(app: &AppHandle, ep_id: i64, err_msg: Option<String>) {
    let payload = events::DownloadEpisodeEndEventPayload { ep_id, err_msg };
    let event = events::DownloadEpisodeEndEvent(payload);
    let _ = event.emit(app);
}

#[allow(clippy::cast_lossless)]
fn emit_update_overall_progress_event(
    app: &AppHandle,
    downloaded_image_count: u32,
    total_image_count: u32,
) {
    let percentage: f64 = downloaded_image_count as f64 / total_image_count as f64 * 100.0;
    let payload = events::UpdateOverallDownloadProgressEventPayload {
        downloaded_image_count,
        total_image_count,
        percentage,
    };
    let event = events::UpdateOverallDownloadProgressEvent(payload);
    let _ = event.emit(app);
}

fn emit_download_speed_event(app: &AppHandle, speed: String) {
    let payload = DownloadSpeedEventPayload { speed };
    let event = DownloadSpeedEvent(payload);
    let _ = event.emit(app);
}
