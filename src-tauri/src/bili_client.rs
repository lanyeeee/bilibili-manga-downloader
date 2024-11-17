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
    AlbumPlusRespData, AppQrcodeStatusRespData, BiliResp, ComicRespData, GenerateAppQrcodeRespData,
    ImageIndexRespData, ImageTokenRespData, SearchRespData, UserProfileRespData,
};
use crate::types::{AlbumPlus, AppQrcodeData, AppQrcodeStatus, Comic};

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
                "生成二维码失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if bili_resp.code != 0 {
            return Err(anyhow!("生成二维码失败，预料之外的code: {bili_resp:?}"));
        }
        // 检查BiliResp的data是否存在
        let Some(data) = bili_resp.data else {
            return Err(anyhow!("生成二维码失败，data字段不存在: {bili_resp:?}"));
        };
        // 尝试将data解析为GenerateAppQrcodeRespData
        let data_str = data.to_string();
        let generate_app_qrcode_resp_data =
            serde_json::from_str::<GenerateAppQrcodeRespData>(&data_str).context(format!(
                "生成二维码失败，将data解析为GenerateAppQrcodeRespData失败: {data_str}"
            ))?;
        // 生成二维码
        let qr_code = QrCode::new(generate_app_qrcode_resp_data.url)
            .context("生成二维码失败，从url创建QrCode失败")?;
        let img = qr_code.render::<Rgb<u8>>().build();
        let mut img_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Jpeg)
            .context("生成二维码失败，将QrCode写入img_data失败")?;
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
                "获取二维码状态失败，预料之外的状态码({status}): {body}"
            ));
        }
        // 尝试将body解析为BiliResp
        let bili_resp = serde_json::from_str::<BiliResp>(&body)
            .context(format!("将body解析为BiliResp失败: {body}"))?;
        // 检查BiliResp的code字段
        if !matches!(bili_resp.code, 0 | 86038 | 86039 | 86090) {
            return Err(anyhow!("获取二维码状态失败，预料之外的code: {bili_resp:?}"));
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
                "获取二维码状态失败，将data解析为AppQrcodeStatusRespData失败: {data_str}"
            ))?;
        let app_qrcode_status = AppQrcodeStatus::from(bili_resp, app_qrcode_status_resp_data);

        Ok(app_qrcode_status)
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
        let cookie = self.cookie();
        let params = json!({
            "platform": "web",
            "device": "pc",
        });
        let payload = json!({"ep_id": episode_id});
        // 发送获取ImageIndex的请求
        let http_resp = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/GetImageIndex")
            .query(&params)
            .header("cookie", cookie)
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
        urls: &Vec<String>,
        from_web_api: bool,
    ) -> anyhow::Result<ImageTokenRespData> {
        let url = "https://manga.bilibili.com/twirp/comic.v1.Comic/ImageToken";
        let urls_str = serde_json::to_string(urls)?;
        let payload = json!({"urls": urls_str});
        // 构造获取ImageToken的请求
        let http_req = if from_web_api {
            let cookie = self.cookie();
            let params = json!({
                "platform": "web",
                "device": "pc",
            });
            Self::client()
                .post(url)
                .query(&params)
                .header("cookie", cookie)
                .json(&payload)
        } else {
            let access_token = self.access_token();
            let params = json!({
                "mobi_app": "android_comic",
                "version": "6.5.0",
                "access_key": access_token,
            });
            Self::client().post(url).query(&params).json(&payload)
        };
        // 发送获取ImageToken的请求
        let http_resp = http_req.send().await?;
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

    fn cookie(&self) -> String {
        self.app.state::<RwLock<Config>>().read().get_cookie()
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
