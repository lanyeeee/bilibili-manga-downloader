use std::fs::File;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crate::bili_client::BiliClient;
use crate::config::Config;
use crate::events;
use crate::events::{DownloadSpeedEvent, DownloadSpeedEventPayload};
use crate::extensions::AnyhowErrorToStringChain;
use crate::types::{AlbumPlusItem, ArchiveFormat, EpisodeInfo};

use anyhow::{anyhow, Context};
use bytes::Bytes;
use parking_lot::RwLock;
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;
use tokio::sync::mpsc::Receiver;
use tokio::sync::{mpsc, Semaphore};
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

// TODO: EpisodeInfo与AlbumPlusItem的内存差距过大，应该用Box包裹EpisodeInfo
enum DownloadPayload {
    Episode(EpisodeInfo),
    AlbumPlus(AlbumPlusItem),
}

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
    http_client: ClientWithMiddleware,
    sender: Arc<mpsc::Sender<DownloadPayload>>,
    ep_sem: Arc<Semaphore>,
    byte_per_sec: Arc<AtomicU64>,
}

impl DownloadManager {
    pub fn new(app: &AppHandle) -> Self {
        let (sender, receiver) = mpsc::channel::<DownloadPayload>(32);

        let manager = DownloadManager {
            app: app.clone(),
            http_client: create_http_client(),
            sender: Arc::new(sender),
            ep_sem: Arc::new(Semaphore::new(5)),
            byte_per_sec: Arc::new(AtomicU64::new(0)),
        };

        tauri::async_runtime::spawn(Self::log_download_speed(app.clone()));
        tauri::async_runtime::spawn(Self::receiver_loop(app.clone(), receiver));

        manager
    }

    pub async fn submit_episode(&self, ep_info: EpisodeInfo) -> anyhow::Result<()> {
        let value = DownloadPayload::Episode(ep_info);
        self.sender.send(value).await?;
        Ok(())
    }

    pub async fn submit_album_plus(&self, item: AlbumPlusItem) -> anyhow::Result<()> {
        let value = DownloadPayload::AlbumPlus(item);
        self.sender.send(value).await?;
        Ok(())
    }

    #[allow(clippy::cast_precision_loss)]
    // TODO: 换个函数名，如emit_download_speed_loop
    async fn log_download_speed(app: AppHandle) {
        let mut interval = tokio::time::interval(Duration::from_secs(1));

        loop {
            interval.tick().await;
            let manager = app.state::<DownloadManager>();
            let byte_per_sec = manager.byte_per_sec.swap(0, Ordering::Relaxed);
            let mega_byte_per_sec = byte_per_sec as f64 / 1024.0 / 1024.0;
            let speed = format!("{mega_byte_per_sec:.2} MB/s");
            emit_download_speed_event(&app, speed);
        }
    }

    async fn receiver_loop(app: AppHandle, mut receiver: Receiver<DownloadPayload>) {
        while let Some(payload) = receiver.recv().await {
            let manager = app.state::<DownloadManager>().inner().clone();
            match payload {
                DownloadPayload::Episode(ep_info) => {
                    tauri::async_runtime::spawn(manager.process_episode(ep_info));
                }
                DownloadPayload::AlbumPlus(item) => {
                    tauri::async_runtime::spawn(manager.process_album_plus(item));
                }
            }
        }
    }

    // TODO: 这里不应该返回错误，否则会被忽略
    #[allow(clippy::cast_possible_truncation)]
    async fn process_episode(self, ep_info: EpisodeInfo) -> anyhow::Result<()> {
        emit_pending_event(&self.app, ep_info.episode_id, ep_info.episode_title.clone());
        // 限制同时下载的章节数量
        let permit = self.ep_sem.acquire().await?;

        let bili_client = self.bili_client();
        let image_index_resp_data = bili_client.get_image_index(ep_info.episode_id).await?;
        let urls: Vec<String> = image_index_resp_data
            .images
            .iter()
            .map(|img| img.path.clone())
            .collect();
        let image_token_data_data = bili_client.get_image_token(&urls).await?;
        // 构造图片下载链接
        let urls: Vec<String> = image_token_data_data
            .into_iter()
            .map(|data| data.complete_url)
            .collect();
        let total = urls.len() as u32;
        let mut current = 0;
        emit_start_event(
            &self.app,
            ep_info.episode_id,
            ep_info.episode_title.clone(),
            total,
        );
        // 下载前先创建临时下载目录
        let temp_download_dir = get_ep_temp_download_dir(&self.app, &ep_info);
        std::fs::create_dir_all(&temp_download_dir)
            .context(format!("创建目录 {temp_download_dir:?} 失败"))?;
        for (i, url) in urls.iter().enumerate() {
            let save_path = temp_download_dir.join(format!("{:03}.jpg", i + 1));
            // 下载图片
            if let Err(err) = self.download_image(url, &save_path).await {
                emit_error_event(
                    &self.app,
                    ep_info.episode_id,
                    url.clone(),
                    err.to_string_chain(),
                );
                // 如果下载失败，则不再下载剩余的图片，直接跳出循环
                break;
            }
            // 下载完成后，更新章节下载进度
            current += 1;
            emit_success_event(
                &self.app,
                ep_info.episode_id,
                save_path.to_string_lossy().to_string(), // TODO: 把save_path.to_string_lossy().to_string()保存到一个变量里，像current一样
                current,
            );
            // 每下载完一张图片，都休息1秒，避免风控
            // tokio::time::sleep(Duration::from_secs(1)).await;
        }
        // 该章节的图片下载完成，释放permit，允许其他章节下载
        drop(permit);
        // 检查此章节的图片是否全部下载成功
        // 此章节的图片未全部下载成功
        if current != total {
            let err_msg = Some(format!("总共有 {total} 张图片，但只下载了 {current} 张"));
            emit_end_event(&self.app, ep_info.episode_id, err_msg);
            return Ok(());
        }
        // 此章节的图片全部下载成功，保存图片
        let err_msg = match self.save_archive(&ep_info, &temp_download_dir) {
            Ok(_) => None,
            Err(err) => Some(err.to_string_chain()),
        };
        emit_end_event(&self.app, ep_info.episode_id, err_msg);
        Ok(())
    }

    fn save_archive(
        &self,
        ep_info: &EpisodeInfo,
        temp_download_dir: &PathBuf,
    ) -> anyhow::Result<()> {
        let archive_format = self
            .app
            .state::<RwLock<Config>>()
            .read()
            .archive_format
            .clone();

        let Some(parent) = temp_download_dir.parent() else {
            let err_msg = Some(format!("无法获取 {temp_download_dir:?} 的父目录"));
            emit_end_event(&self.app, ep_info.episode_id, err_msg);
            return Ok(());
        };

        let download_dir = parent.join(&ep_info.episode_title);
        // TODO: 把每种格式的保存操作提取到一个函数里
        match archive_format {
            ArchiveFormat::Image => {
                if download_dir.exists() {
                    std::fs::remove_dir_all(&download_dir)
                        .context(format!("删除 {download_dir:?} 失败"))?;
                }

                std::fs::rename(temp_download_dir, &download_dir).context(format!(
                    "将 {temp_download_dir:?} 重命名为 {download_dir:?} 失败"
                ))?;
            }
            ArchiveFormat::Cbz | ArchiveFormat::Zip => {
                let comic_info_path = temp_download_dir.join("ComicInfo.xml");
                let comic_info_xml = yaserde::ser::to_string(&ep_info.comic_info)
                    .map_err(|err_msg| anyhow!("序列化 {comic_info_path:?} 失败: {err_msg}"))?;
                std::fs::write(&comic_info_path, comic_info_xml)
                    .context(format!("创建 {comic_info_path:?} 失败"))?;

                let zip_path = download_dir.with_extension(archive_format.extension());
                let zip_file =
                    File::create(&zip_path).context(format!("创建 {zip_path:?} 失败"))?;

                let mut zip_writer = ZipWriter::new(zip_file);

                for entry in std::fs::read_dir(temp_download_dir)?.filter_map(Result::ok) {
                    let path = entry.path();
                    if !path.is_file() {
                        continue;
                    }

                    let filename = match path.file_name() {
                        Some(name) => name.to_string_lossy(),
                        None => continue,
                    };

                    zip_writer
                        .start_file(&filename, SimpleFileOptions::default())
                        .context(format!("在 {zip_path:?} 创建 {filename:?} 失败"))?;

                    let mut file = File::open(&path).context(format!("打开 {path:?} 失败"))?;

                    std::io::copy(&mut file, &mut zip_writer)
                        .context(format!("将 {path:?} 写入 {zip_path:?} 失败"))?;
                }

                zip_writer
                    .finish()
                    .context(format!("关闭 {zip_path:?} 失败"))?;

                std::fs::remove_dir_all(temp_download_dir)
                    .context(format!("删除 {temp_download_dir:?} 失败"))?;
            }
        }
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    // TODO: 这里不应该返回错误，否则会被忽略
    async fn process_album_plus(self, album_plus_item: AlbumPlusItem) -> anyhow::Result<()> {
        emit_pending_event(&self.app, album_plus_item.id, album_plus_item.title.clone());
        // 限制同时下载的章节数量
        let permit = self.ep_sem.acquire().await?;

        let bili_client = self.bili_client();
        let image_token_data_data = bili_client.get_image_token(&album_plus_item.pic).await?;
        // 构造图片下载链接
        let urls: Vec<String> = image_token_data_data
            .into_iter()
            .map(|data| data.complete_url)
            .collect();
        let total = urls.len() as u32;
        let mut current = 0;
        emit_start_event(
            &self.app,
            album_plus_item.id,
            album_plus_item.title.clone(),
            total,
        );
        // 下载前先创建临时下载目录
        let temp_download_dir = get_album_plus_temp_download_dir(&self.app, &album_plus_item);
        std::fs::create_dir_all(&temp_download_dir)
            .context(format!("创建目录 {temp_download_dir:?} 失败"))?;
        for (i, url) in urls.iter().enumerate() {
            let url = url.clone();
            let save_path = temp_download_dir.join(format!("{:03}.jpg", i + 1));
            // 下载图片
            if let Err(err) = self.download_image(&url, &save_path).await {
                emit_error_event(
                    &self.app,
                    album_plus_item.id,
                    url.clone(),
                    err.to_string_chain(),
                );
                // 如果下载失败，则不再下载剩余的图片，直接跳出循环
                break;
            }
            // 下载完成后，更新章节下载进度
            current += 1;
            emit_success_event(
                &self.app,
                album_plus_item.id,
                save_path.to_string_lossy().to_string(),
                current,
            );
            // 每下载完一张图片，都休息1秒，避免风控
            // tokio::time::sleep(Duration::from_secs(1)).await;
        }
        // 该章节的图片下载完成，释放permit，允许其他章节下载
        drop(permit);
        // 检查此章节的图片是否全部下载成功
        // TODO: 重构下面的代码
        if current == total {
            // 下载成功，则把临时目录重命名为正式目录
            if let Some(parent) = temp_download_dir.parent() {
                let download_dir = parent.join(&album_plus_item.title);
                std::fs::rename(&temp_download_dir, &download_dir).context(format!(
                    "将 {temp_download_dir:?} 重命名为 {download_dir:?} 失败"
                ))?;
            }
            emit_end_event(&self.app, album_plus_item.id, None);
        } else {
            let err_msg = Some(format!("总共有 {total} 张图片，但只下载了 {current} 张"));
            emit_end_event(&self.app, album_plus_item.id, err_msg);
        };
        Ok(())
    }

    async fn download_image(&self, url: &str, save_path: &Path) -> anyhow::Result<()> {
        let image_data = self
            .get_image_bytes(url)
            .await
            .context(format!("下载图片 {url} 失败"))?;
        // 保存图片
        std::fs::write(save_path, &image_data).context(format!("保存图片 {save_path:?} 失败"))?;
        // 记录下载字节数
        self.byte_per_sec
            .fetch_add(image_data.len() as u64, Ordering::Relaxed);
        Ok(())
    }

    async fn get_image_bytes(&self, url: &str) -> anyhow::Result<Bytes> {
        // 发送下载图片请求
        let http_resp = self.http_client.get(url).send().await?;
        // 检查http响应状态码
        let status = http_resp.status();
        if status != StatusCode::OK {
            let body = http_resp.text().await?;
            return Err(anyhow!("下载图片 {url} 失败，预料之外的状态码: {body}"));
        }
        // 读取图片数据
        let image_data = http_resp.bytes().await?;

        Ok(image_data)
    }

    fn bili_client(&self) -> BiliClient {
        self.app.state::<BiliClient>().inner().clone()
    }
}

fn get_ep_temp_download_dir(app: &AppHandle, ep_info: &EpisodeInfo) -> PathBuf {
    app.state::<RwLock<Config>>()
        .read()
        .download_dir
        .join(&ep_info.comic_title)
        .join(format!(".下载中-{}", ep_info.episode_title)) // 以 `.下载中-` 开头，表示是临时目录
}

fn get_album_plus_temp_download_dir(app: &AppHandle, album_plus_item: &AlbumPlusItem) -> PathBuf {
    app.state::<RwLock<Config>>()
        .read()
        .download_dir
        .join(&album_plus_item.comic_title)
        .join("特典")
        .join(format!(".下载中-{}", album_plus_item.title)) // 以 `.下载中-` 开头，表示是临时目录
}

fn emit_start_event(app: &AppHandle, id: i64, title: String, total: u32) {
    let payload = events::DownloadStartEventPayload { id, title, total };
    let event = events::DownloadStartEvent(payload);
    let _ = event.emit(app);
}

fn emit_pending_event(app: &AppHandle, id: i64, title: String) {
    let payload = events::DownloadPendingEventPayload { id, title };
    let event = events::DownloadPendingEvent(payload);
    let _ = event.emit(app);
}

fn emit_success_event(app: &AppHandle, id: i64, url: String, current: u32) {
    let payload = events::DownloadImageSuccessEventPayload { id, url, current };
    let event = events::DownloadImageSuccessEvent(payload);
    let _ = event.emit(app);
}

fn emit_error_event(app: &AppHandle, id: i64, url: String, err_msg: String) {
    let payload = events::DownloadImageErrorEventPayload { id, url, err_msg };
    let event = events::DownloadImageErrorEvent(payload);
    let _ = event.emit(app);
}

fn emit_end_event(app: &AppHandle, id: i64, err_msg: Option<String>) {
    let payload = events::DownloadEndEventPayload { id, err_msg };
    let event = events::DownloadEndEvent(payload);
    let _ = event.emit(app);
}

fn emit_download_speed_event(app: &AppHandle, speed: String) {
    let payload = DownloadSpeedEventPayload { speed };
    let event = DownloadSpeedEvent(payload);
    let _ = event.emit(app);
}

fn create_http_client() -> ClientWithMiddleware {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(2);

    reqwest_middleware::ClientBuilder::new(reqwest::Client::new())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}
