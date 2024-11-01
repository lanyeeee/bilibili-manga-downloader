use crate::errors::CommandResult;
use crate::responses::{BiliResp, GenerateQrcodeRespData, QrcodeStatusRespData};
use crate::types::{QrcodeData, QrcodeStatus};
use anyhow::{anyhow, Context};
use base64::engine::general_purpose;
use base64::Engine;
use image::Rgb;
use qrcode::QrCode;
use reqwest::{Client, ClientBuilder, StatusCode};
use std::collections::BTreeMap;
use std::io::Cursor;
use std::time::Duration;
use tauri::AppHandle;
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
        let mut form = BTreeMap::new();
        form.insert("ts".to_string(), "0".to_string());
        form.insert("local_id".to_string(), "0".to_string());
        let signed_form = app_sign(form);
        // 发送生成二维码请求
        let http_resp = Self::client()
            .post("https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/auth_code")
            .query(&signed_form)
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
        let mut form = BTreeMap::new();
        form.insert("auth_code".to_string(), auth_code);
        form.insert("ts".to_string(), "0".to_string());
        form.insert("local_id".to_string(), "0".to_string());
        let signed_form = app_sign(form);
        // 发送获取二维码状态请求
        let http_res = Self::client()
            .post("https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/poll")
            .query(&signed_form)
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
