use crate::config::Config;
use crate::errors::CommandResult;
use crate::extensions::IgnoreRwLockPoison;
use crate::responses::{
    BiliResp, ComicRespData, GenerateQrcodeRespData, QrcodeStatusRespData, SearchRespData,
    UserProfileRespData,
};
use crate::types::{Comic, QrcodeData, QrcodeStatus};
use anyhow::{anyhow, Context};
use base64::engine::general_purpose;
use base64::Engine;
use image::Rgb;
use qrcode::QrCode;
use reqwest::{Client, ClientBuilder, StatusCode};
use serde_json::json;
use std::collections::BTreeMap;
use std::io::Cursor;
use std::sync::RwLock;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use url::form_urlencoded;

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

    pub async fn generate_qrcode(&self) -> anyhow::Result<QrcodeData> {
        let mut params = BTreeMap::new();
        params.insert("ts".to_string(), "0".to_string());
        params.insert("local_id".to_string(), "0".to_string());
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
        // 尝试将data解析为GenerateQrcodeRespData
        let data_str = data.to_string();
        let generate_qrcode_resp_data = serde_json::from_str::<GenerateQrcodeRespData>(&data_str)
            .context(format!(
            "生成二维码失败，将data解析为GenerateQrcodeRespData失败: {data_str}"
        ))?;
        // 生成二维码
        let qr_code = QrCode::new(generate_qrcode_resp_data.url)
            .context("生成二维码失败，从url创建QrCode失败")?;
        let img = qr_code.render::<Rgb<u8>>().build();
        let mut img_data: Vec<u8> = Vec::new();
        img.write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Jpeg)
            .context("生成二维码失败，将QrCode写入img_data失败")?;
        let base64 = general_purpose::STANDARD.encode(img_data);
        let qrcode_data = QrcodeData {
            base64,
            auth_code: generate_qrcode_resp_data.auth_code,
        };

        Ok(qrcode_data)
    }

    pub async fn get_qrcode_status(&self, auth_code: String) -> anyhow::Result<QrcodeStatus> {
        let mut params = BTreeMap::new();
        params.insert("auth_code".to_string(), auth_code);
        params.insert("ts".to_string(), "0".to_string());
        params.insert("local_id".to_string(), "0".to_string());
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
            return Ok(QrcodeStatus::from(
                bili_resp,
                QrcodeStatusRespData::default(),
            ));
        };
        // 尝试将data解析为QrcodeStatusRespData
        let data_str = data.to_string();
        let qrcode_status_resp_data = serde_json::from_str::<QrcodeStatusRespData>(&data_str)
            .context(format!(
                "获取二维码状态失败，将data解析为QrcodeStatusRespData失败: {data_str}"
            ))?;
        let qrcode_status = QrcodeStatus::from(bili_resp, qrcode_status_resp_data);

        Ok(qrcode_status)
    }

    pub async fn get_user_profile(&self) -> anyhow::Result<UserProfileRespData> {
        let access_token = self.access_token();
        let mut params = BTreeMap::new();
        params.insert("access_key".to_string(), access_token);
        params.insert("ts".to_string(), "0".to_string());
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
        let payload = json!({"comic_id": comic_id});
        let params = json!({
            "device": "android",
            "access_key": access_token,
        });
        // 发送获取漫画详情请求
        let http_res = Self::client()
            .post("https://manga.bilibili.com/twirp/comic.v1.Comic/ComicDetail")
            .json(&payload)
            .query(&params)
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
        let comic = Comic::from_comic_resp_data(&self.app, comic_resp_data);

        Ok(comic)
    }

    fn access_token(&self) -> String {
        self.app
            .state::<RwLock<Config>>()
            .read_or_panic()
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
