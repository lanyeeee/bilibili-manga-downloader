use std::collections::BTreeMap;
use std::io::Cursor;
use std::time::Duration;

use anyhow::{anyhow, Context};
use base64::engine::general_purpose;
use base64::Engine;
use image::Rgb;
use parking_lot::RwLock;
use qrcode::QrCode;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde_json::json;
use tauri::{AppHandle, Manager};
use url::form_urlencoded;

use crate::config::Config;
use crate::responses::{
    AlbumPlusRespData, AppQrcodeStatusRespData, BiliResp, ComicRespData, ConfirmAppQrcodeRespData,
    GenerateAppQrcodeRespData, GenerateWebQrcodeRespData, ImageIndexRespData, ImageTokenRespData,
    SearchRespData, UserProfileRespData, WebQrcodeStatusRespData,
};
use crate::types::{AlbumPlus, AppQrcodeData, AppQrcodeStatus, Comic, WebQrcodeData};

const APP_KEY: &str = "cc8617fd6961e070";
const APP_SEC: &str = "3131924b941aac971e45189f265262be";

#[derive(Clone)]
pub struct BiliClient {
    app: AppHandle,
}

impl BiliClient {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn client() -> Client {
        // TODO: 添加重试机制
        ClientBuilder::new()
            .timeout(Duration::from_secs(2)) // 每个请求超过2秒就超时
            .build()
            .unwrap()
    }

    pub async fn generate_app_qrcode(&self) -> anyhow::Result<AppQrcodeData> {
        let params = BTreeMap::from([
            ("ts".to_string(), "0".to_string()),
            ("local_id".to_string(), "0".to_string()),
        ]);
        let signed_params = app_sign(params);
        // 发送生成二维码请求
        let http_resp = Self::client()
            .post("https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/auth_code")
            .query(&signed_params)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "生成App二维码失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("生成App二维码失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("生成App二维码失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为GenerateAppQrcodeRespData
        let data_str = data.to_string();
        let generate_app_qrcode_resp_data =
            serde_json::from_str::<GenerateAppQrcodeRespData>(&data_str).context(format!(
                "生成App二维码失败，将data解析为GenerateAppQrcodeRespData失败: {data_str}"
            ))?;
        // 生成二维码
        let qr_code = QrCode::new(generate_app_qrcode_resp_data.url)
            .context("生成App二维码失败，从url创建QrCode失败")?;
        let img = qr_code.render::<Rgb<u8>>().build();
        let mut img_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Jpeg)
            .context("生成App二维码失败，将QrCode写入img_data失败")?;
        let base64 = general_purpose::STANDARD.encode(img_data);
        let app_qrcode_data = AppQrcodeData {
            base64,
            auth_code: generate_app_qrcode_resp_data.auth_code,
        };

        Ok(app_qrcode_data)
    }

    pub async fn get_app_qrcode_status(
        &self,
        auth_code: String,
    ) -> anyhow::Result<AppQrcodeStatus> {
        let params = BTreeMap::from([
            ("auth_code".to_string(), auth_code),
            ("ts".to_string(), "0".to_string()),
            ("local_id".to_string(), "0".to_string()),
        ]);
        let signed_params = app_sign(params);
        // 发送获取二维码状态请求
        let http_res = Self::client()
            .post("https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/poll")
            .query(&signed_params)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_res.status();
        let body = http_res.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "获取App二维码状态失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body).context(format!(
            "获取App二维码状态失败，将body解析为BiliResp失败: {body}"
        ))?;
        // 检查BiliResp的code字段
        if !matches!(bili_resp.code, 0 | 86038 | 86039 | 86090) {
            return Err(anyhow!(
                "获取App二维码状态失败，预料之外的code: {bili_resp:?}"
            ));
        }
        // 检查BiliResp的data是否存在
        let Some(ref data) = bili_resp.data else {
            return Ok(AppQrcodeStatus::from(
                bili_resp,
                AppQrcodeStatusRespData::default(),
            ));
        };
        // 尝试将data解析为AppQrcodeStatusRespData
        let data_str = data.to_string();
        let app_qrcode_status_resp_data =
            serde_json::from_str::<AppQrcodeStatusRespData>(&data_str).context(format!(
                "获取App二维码状态失败，将data解析为AppQrcodeStatusRespData失败: {data_str}"
            ))?;
        let app_qrcode_status = AppQrcodeStatus::from(bili_resp, app_qrcode_status_resp_data);

        Ok(app_qrcode_status)
    }

    pub async fn generate_web_qrcode(&self) -> anyhow::Result<WebQrcodeData> {
        // 发送生成二维码请求
        let http_resp = Self::client()
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
        let http_resp = Self::client()
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

    #[allow(clippy::unreadable_literal)]
    pub async fn confirm_app_qrcode(
        &self,
        auth_code: &str,
        sessdata: &str,
        csrf: &str,
    ) -> anyhow::Result<ConfirmAppQrcodeRespData> {
        let cookie = format!("SESSDATA={sessdata}");
        let form = json!({
            "auth_code": auth_code,
            "build": 708200,
            "csrf": csrf,
        });
        // 发送确认App二维码请求
        let http_resp = Self::client()
            .post("https://passport.bilibili.com/x/passport-tv-login/h5/qrcode/confirm")
            .header("cookie", cookie)
            .form(&form)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_resp.status();
        let body = http_resp.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!(
                "确认App二维码失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为ConfirmAppQrcodeRespData
        let confirm_app_qrcode_resp_data = serde_json::from_str::<ConfirmAppQrcodeRespData>(&body)
            .context(format!(
                "确认App二维码失败，将body解析为ConfirmAppQrcodeRespData失败: {body}"
            ))?;

        Ok(confirm_app_qrcode_resp_data)
    }

    pub async fn get_user_profile(&self) -> anyhow::Result<UserProfileRespData> {
        let access_token = self.access_token();
        let params = BTreeMap::from([
            ("access_key".to_string(), access_token),
            ("ts".to_string(), "0".to_string()),
        ]);
        let signed_params = app_sign(params);
        // 发送获取用户信息请求
        let http_resp = Self::client()
            .get("https://app.bilibili.com/x/v2/account/myinfo")
            .query(&signed_params)
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
        let http_resp = Self::client()
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
        let access_token = self.access_token();
        let params = json!({
            "device": "android",
            "access_key": access_token,
        });
        let payload = json!({"comic_id": comic_id});
        // 发送获取漫画详情请求
        let http_res = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ComicDetail")
            .query(&params)
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_res.status();
        let body = http_res.text().await?;
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
        // TODO: 获取comic_resp_data与album_plus可以并行
        let album_plus = self.get_album_plus(comic_id).await?;
        let comic = Comic::from(&self.app, comic_resp_data, album_plus);

        Ok(comic)
    }

    pub async fn get_album_plus(&self, comic_id: i64) -> anyhow::Result<AlbumPlus> {
        let access_token = self.access_token();
        let params = json!({
            "version": "6.5.0",
            "access_key": access_token,
        });
        let payload = json!({"comic_id": comic_id});
        // 发送获取特典请求
        let http_res = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetComicAlbumPlus")
            .query(&params)
            .json(&payload)
            .send()
            .await?;
        // 检查http响应状态码
        let status = http_res.status();
        let body = http_res.text().await?;
        if status != StatusCode::OK {
            return Err(anyhow!("获取特典失败，预料之外的状态码({status}): {body}"));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("获取特典失败，将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("获取特典失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("获取特典失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为AlbumPlusRespData
        let data_str = data.to_string();
        let comic_album_plus_resp_data = serde_json::from_str::<AlbumPlusRespData>(&data_str)
            .context(format!(
                "获取特典失败，将data解析为AlbumPlusRespData失败: {data_str}"
            ))?;
        let comic_album_plus = AlbumPlus::from(&self.app, comic_album_plus_resp_data);

        Ok(comic_album_plus)
    }

    pub async fn get_image_index(&self, episode_id: i64) -> anyhow::Result<ImageIndexRespData> {
        let access_token = self.access_token();
        let params = json!({
            "mobi_app": "android_comic",
            "version": "6.5.0",
            "device": "android",
            "access_key": access_token,
        });
        let payload = json!({"ep_id": episode_id});
        // 发送获取ImageIndex的请求
        let http_resp = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex")
            .query(&params)
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

    pub async fn get_image_token(&self, urls: &Vec<String>) -> anyhow::Result<ImageTokenRespData> {
        let access_token = self.access_token();
        let params = json!({
            "mobi_app": "android_comic",
            "version": "6.5.0",
            "access_key": access_token,
        });
        let urls_str = serde_json::to_string(urls)?;
        let payload = json!({"urls": urls_str});
        // 发送获取ImageToken的请求
        let http_resp = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken")
            .query(&params)
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

    fn access_token(&self) -> String {
        self.app
            .state::<RwLock<Config>>()
            .read()
            .access_token
            .clone()
    }
}

fn app_sign(mut params: BTreeMap<String, String>) -> BTreeMap<String, String> {
    params.insert("appkey".to_string(), APP_KEY.to_string());
    let query = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(params.iter())
        .finish();
    let sign = format!("{:x}", md5::compute(query + APP_SEC));
    params.insert("sign".to_string(), sign);
    params
}
