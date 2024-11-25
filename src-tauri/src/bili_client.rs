use crate::config::Config;
use crate::events::{SetProxyErrorEvent, SetProxyErrorEventPayload};
use crate::extensions::AnyhowErrorToStringChain;
use crate::responses::{
    BiliResp, ComicRespData, GenerateWebQrcodeRespData, ImageIndexRespData, ImageTokenRespData,
    SearchRespData, UserProfileRespData, WebQrcodeStatusRespData,
};
use crate::types::{AsyncRwLock, Comic, ProxyMode, WebQrcodeData};
use anyhow::{anyhow, Context};
use base64::engine::general_purpose;
use base64::Engine;
use bytes::Bytes;
use image::Rgb;
use parking_lot::RwLock;
use qrcode::QrCode;
use reqwest::StatusCode;
use reqwest_middleware::ClientWithMiddleware;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use serde_json::json;
use std::io::Cursor;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tauri_specta::Event;

#[allow(clippy::unreadable_literal)]
#[derive(Clone)]
pub struct BiliClient {
    app: AppHandle,
    http_client: Arc<AsyncRwLock<ClientWithMiddleware>>,
}

impl BiliClient {
    pub fn new(app: AppHandle) -> Self {
        let http_client = create_http_client(&app);
        let http_client = Arc::new(AsyncRwLock::new(http_client));
        Self { app, http_client }
    }

    pub async fn recreate_http_client(&self) {
        let http_client = create_http_client(&self.app);
        *self.http_client.write().await = http_client;
    }

    pub async fn generate_web_qrcode(&self) -> anyhow::Result<WebQrcodeData> {
        // 发送生成二维码请求
        let http_resp = self
            .http_client
            .read()
            .await
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow::anyhow!(
                "生成Web二维码失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("生成Web二维码失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("生成Web二维码失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为GenerateWebQrcodeRespData
        let data_str = data.to_string();
        let generate_qrcode_resp_data =
            serde_json::from_str::<GenerateWebQrcodeRespData>(&data_str).context(format!(
                "生成Web二维码失败，将data解析为GenerateQrcodeRespData失败: {data_str}"
            ))?;
        // 生成二维码
        let qr_code = QrCode::new(generate_qrcode_resp_data.url)
            .context("生成Web二维码失败，从url创建QrCode失败")?;
        let img = qr_code.render::<Rgb<u8>>().build();
        let mut img_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Jpeg)
            .context("生成Web二维码失败，将QrCode写入img_data失败")?;
        let base64 = general_purpose::STANDARD.encode(img_data);
        let web_qrcode_data = WebQrcodeData {
            base64,
            qrcode_key: generate_qrcode_resp_data.qrcode_key,
        };

        Ok(web_qrcode_data)
    }

    pub async fn get_web_qrcode_status(
        &self,
        qrcode_key: &str,
    ) -> anyhow::Result<WebQrcodeStatusRespData> {
        let params = json!({
            "qrcode_key": qrcode_key,
        });
        // 发送获取二维码状态请求
        let http_resp = self
            .http_client
            .read()
            .await
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
            .query(&params)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取Web二维码状态失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
            "获取Web二维码状态失败，将body解析为BiliResp失败: {body}"
        ))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!(
                "获取Web二维码状态失败，预料之外的code: {bili_resp:?}"
            ));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!(
                "获取Web二维码状态失败，data字段不存在: {bili_resp:?}"
            ));
        };
        // 尝试将data解析为WebQrcodeStatusRespData
        let data_str = data.to_string();
        let web_qrcode_status_resp_data =
            serde_json::from_str::<WebQrcodeStatusRespData>(&data_str).context(format!(
                "获取二维码状态失败，将data解析为QrcodeStatusRespData失败: {data_str}"
            ))?;

        Ok(web_qrcode_status_resp_data)
    }

    pub async fn get_user_profile(&self) -> anyhow::Result<UserProfileRespData> {
        let cookie = self.cookie();
        // 发送获取用户信息请求
        let http_resp = self
            .http_client
            .read()
            .await
            .get("https://api.bilibili.com/x/web-interface/nav")
            .header("cookie", cookie)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取用户信息失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("获取用户信息失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("获取用户信息失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为UserProfileRespData
        let data_str = data.to_string();
        let user_profile_resp_data = serde_json::from_str::<UserProfileRespData>(&data_str)
            .context(format!(
                "获取用户信息失败，将data解析为UserProfileRespData失败: {data_str}"
            ))?;

        Ok(user_profile_resp_data)
    }

    pub async fn search(&self, keyword: &str, page_num: i64) -> anyhow::Result<SearchRespData> {
        let payload = json!({
            "keyword": keyword,
            "pageNum": page_num,
            "pageSize": 20,
        });
        // 发送搜索漫画请求
        let http_resp = self
            .http_client
            .read()
            .await
            .post("https://manga.bilibili.com/twirp/search.v1.Search/SearchKeyword")
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!("搜索漫画失败，预料之外的状态码({status}): {body}"));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("搜索漫画失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("搜索漫画失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为SearchRespData
        let data_str = data.to_string();
        let search_resp_data = serde_json::from_str::<SearchRespData>(&data_str).context(
            format!("搜索漫画失败，将data解析为SearchRespData失败: {data_str}"),
        )?;

        Ok(search_resp_data)
    }

    pub async fn get_comic(&self, comic_id: i64) -> anyhow::Result<Comic> {
        let cookie = self.cookie();
        let referer = format!("https://manga.bilibili.com/detail/mc{comic_id}?from=manga_person");
        let params = json!({
            "device": "pc",
            "platform": "web",
        });
        let payload = json!({"comic_id": comic_id});
        // 发送获取漫画详情请求
        let http_resp = self
            .http_client
            .read()
            .await
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ComicDetail")
            .query(&params)
            .header("accept", "application/json, text/plain, */*")
            .header("accept-encoding", "gzip, deflate, br, zstd")
            .header("accept-language", "zh-CN,zh;q=0.9")
            .header("content-type", "application/json;charset=UTF-8")
            .header("cookie", cookie)
            .header("origin", "https://manga.bilibili.com")
            .header("priority", "u=1, i")
            .header("referer", referer)
            .header("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-mobile", r#""Windows""#)
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取漫画详情失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
            "获取漫画详情失败，将body解析为BiliResp失败: {body}"
        ))?;
        // 检查BiliResp的code字段
        if bili_resp.code == 99 {
            return Err(anyhow!("获取漫画详情失败，Cookie不完整，请返回浏览器刷新页面后重新获取完整的Cookie: {bili_resp:?}"));
        }
        if bili_resp.code != 0 {
            return Err(anyhow!("获取漫画详情失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("获取漫画详情失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为ComicRespData
        let data_str = data.to_string();
        let comic_resp_data = serde_json::from_str::<ComicRespData>(&data_str).context(format!(
            "获取漫画详情失败，将data解析为ComicRespData失败: {data_str}"
        ))?;
        let comic = Comic::from(&self.app, comic_resp_data);

        Ok(comic)
    }

    pub async fn get_image_index(
        &self,
        comic_id: i64,
        episode_id: i64,
    ) -> anyhow::Result<ImageIndexRespData> {
        let cookie = self.cookie();
        let referer = format!("https://manga.bilibili.com/mc{comic_id}/{episode_id}");
        let params = json!({
            "device": "pc",
            "platform": "web",
        });
        let payload = json!({"ep_id": episode_id});
        // 发送获取ImageIndex的请求
        let http_resp = self.http_client.read().await
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex")
            .query(&params)
            .header("accept", "application/json, text/plain, */*")
            .header("accept-encoding", "gzip, deflate, br, zstd")
            .header("accept-language", "zh-CN,zh;q=0.9")
            .header("content-type", "application/json;charset=UTF-8")
            .header("cookie", cookie)
            .header("origin", "https://manga.bilibili.com")
            .header("priority", "u=1, i")
            .header("referer", referer)
            .header("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-mobile", r#""Windows""#)
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取章节 `{episode_id}` 的ImageIndex失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
            "获取章节 `{episode_id}` 的ImageIndex失败，将body解析为BiliResp失败: {body}"
        ))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!(
                "获取章节 `{episode_id}` 的ImageIndex失败，预料之外的code: {bili_resp:?}"
            ));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!(
                "获取章节 `{episode_id}` 的ImageIndex失败，data字段不存在: {bili_resp:?}"
            ));
        };
        // 尝试将data解析为ImageIndexRespData
        let data_str = data.to_string();
        let image_index_data = serde_json::from_str::<ImageIndexRespData>(&data_str).context(format!(
            "获取章节 `{episode_id}` 的ImageIndex失败，将data解析为ImageIndexRespData失败: {data_str}"
        ))?;

        Ok(image_index_data)
    }

    pub async fn get_image_token(
        &self,
        comic_id: i64,
        episode_id: i64,
        urls: &Vec<String>,
    ) -> anyhow::Result<ImageTokenRespData> {
        let cookie = self.cookie();
        let referer = format!("https://manga.bilibili.com/mc{comic_id}/{episode_id}");
        let params = json!({
            "device": "pc",
            "platform": "web",
        });
        let urls_str = serde_json::to_string(urls)?;
        let payload = json!({"urls": urls_str});
        // 发送获取ImageToken的请求
        let http_resp = self.http_client.read().await
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken")
            .query(&params)
            .header("accept", "application/json, text/plain, */*")
            .header("accept-encoding", "gzip, deflate, br, zstd")
            .header("accept-language", "zh-CN,zh;q=0.9")
            .header("content-type", "application/json;charset=UTF-8")
            .header("cookie", cookie)
            .header("origin", "https://manga.bilibili.com")
            .header("priority", "u=1, i")
            .header("referer", referer)
            .header("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-mobile", r#""Windows""#)
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "same-origin")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取ImageToken失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
            "获取ImageToken失败，将body解析为BiliResp失败: {body}"
        ))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("获取ImageToken失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("获取ImageToken失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为ImageTokenRespData
        let data_str = data.to_string();
        let image_token_data = serde_json::from_str::<ImageTokenRespData>(&data_str).context(
            format!("获取ImageToken失败，将data解析为ImageTokenRespData失败: {data_str}"),
        )?;

        Ok(image_token_data)
    }

    pub async fn get_image_bytes(&self, url: &str) -> anyhow::Result<Bytes> {
        // 发送下载图片请求
        let http_resp = self.http_client.read().await.get(url)
            .header("accept", "*/*")
            .header("accept-encoding", "gzip, deflate, br, zstd")
            .header("accept-language", "zh-CN,zh;q=0.9")
            .header("origin", "https://manga.bilibili.com")
            .header("referer", "https://manga.bilibili.com/")
            .header("sec-ch-ua", r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#)
            .header("sec-ch-ua-mobile", "?0")
            .header("sec-ch-ua-mobile", r#""Windows""#)
            .header("sec-fetch-dest", "empty")
            .header("sec-fetch-mode", "cors")
            .header("sec-fetch-site", "cross-site")
            .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36")
            .send()
            .await?;
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

    fn cookie(&self) -> String {
        self.app.state::<RwLock<Config>>().read().cookie.clone()
    }
}

fn create_http_client(app: &AppHandle) -> ClientWithMiddleware {
    let builder = reqwest::ClientBuilder::new();

    let proxy_mode = app.state::<RwLock<Config>>().read().proxy_mode.clone();
    let builder = match proxy_mode {
        ProxyMode::NoProxy => builder.no_proxy(),
        ProxyMode::System => builder,
        ProxyMode::Custom => {
            let config = app.state::<RwLock<Config>>();
            let config = config.read();
            let proxy_host = &config.proxy_host;
            let proxy_port = &config.proxy_port;
            let proxy_url = format!("http://{proxy_host}:{proxy_port}");

            match reqwest::Proxy::all(&proxy_url).map_err(anyhow::Error::from) {
                Ok(proxy) => builder.proxy(proxy),
                Err(err) => {
                    let err = err.context(format!("BiliClient设置代理 {proxy_url} 失败"));
                    emit_set_proxy_error_event(app, err.to_string_chain());
                    builder
                }
            }
        }
    };

    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);

    reqwest_middleware::ClientBuilder::new(builder.build().unwrap())
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
}

fn emit_set_proxy_error_event(app: &AppHandle, err_msg: String) {
    let payload = SetProxyErrorEventPayload { err_msg };
    let event = SetProxyErrorEvent(payload);
    let _ = event.emit(app);
}
