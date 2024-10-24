use crate::config::Config;
use crate::errors::CommandResult;
use crate::responses::{BiliResp, GenerateQrcodeData, QrcodeStatusData};
use crate::types::QrcodeData;
use anyhow::{anyhow, Context};
use base64::engine::general_purpose;
use base64::Engine;
use image::Rgb;
use qrcode::QrCode;
use reqwest::StatusCode;
use std::io::Cursor;

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn get_config(config: tauri::State<std::sync::RwLock<Config>>) -> Config {
    config.read().unwrap().clone()
}

#[tauri::command(async)]
#[specta::specta]
pub async fn generate_qrcode() -> CommandResult<QrcodeData> {
    // 发送生成二维码请求
    let http_resp = reqwest::Client::new()
        .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36")
        .header("origin", "https://manga.bilibili.com")
        .send()
        .await?;
    // 检查http响应状态码
    let status = http_resp.status();
    let body = http_resp.text().await?;
    if status != StatusCode::OK {
        return Err(anyhow::anyhow!("生成二维码失败，预料之外的状态码({status}): {body}").into());
    }
    // 尝试将body解析为BiliResp
    let bili_resp = serde_json::from_str::<BiliResp>(&body)
        .context(format!("将body解析为BiliResp失败: {body}"))?;
    // 检查BiliResp的code字段
    if bili_resp.code != 0 {
        return Err(anyhow!("生成二维码失败，预料之外的code: {bili_resp:?}").into());
    }
    // 检查BiliResp的data是否存在
    let Some(data) = bili_resp.data else {
        return Err(anyhow!("生成二维码失败，data字段不存在: {bili_resp:?}").into());
    };
    // 尝试将data解析为GenerateQrcodeData
    let generate_qrcode_data: GenerateQrcodeData = serde_json::from_value(data)?;
    // 生成二维码
    let qr_code = QrCode::new(generate_qrcode_data.url)?;
    let img = qr_code.render::<Rgb<u8>>().build();
    let mut img_data: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut img_data), image::ImageFormat::Jpeg)?;
    let base64 = general_purpose::STANDARD.encode(img_data);
    let qrcode_data = QrcodeData {
        base64,
        qrcode_key: generate_qrcode_data.qrcode_key,
    };

    Ok(qrcode_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_qrcode_status_data(qrcode_key: &str) -> CommandResult<QrcodeStatusData> {
    // 发送获取二维码状态请求
    let http_res = reqwest::Client::new()
        .get("https://passport.bilibili.com/x/passport-login/web/qrcode/poll")
        .query(&[("qrcode_key", qrcode_key)])
        .send()
        .await?;
    // 检查http响应状态码
    let status = http_res.status();
    let body = http_res.text().await?;
    if status != StatusCode::OK {
        return Err(anyhow!("获取二维码状态失败，预料之外的状态码({status}): {body}").into());
    }
    // 尝试将body解析为BiliResp
    let bili_resp = serde_json::from_str::<BiliResp>(&body)
        .context(format!("将body解析为BiliResp失败: {body}"))?;
    // 检查BiliResp的code字段
    if bili_resp.code != 0 {
        return Err(anyhow!("获取二维码状态失败，预料之外的code: {bili_resp:?}").into());
    }
    // 检查BiliResp的data是否存在
    let Some(data) = bili_resp.data else {
        return Err(anyhow!("获取二维码状态失败，data字段不存在: {bili_resp:?}").into());
    };
    // 尝试将data解析为QrcodeStatusData
    let data_str = data.to_string();
    let qrcode_status_data = serde_json::from_str::<QrcodeStatusData>(&data_str).context(
        format!("获取二维码状态失败，将data解析为QrcodeStatusData失败: {data_str}"),
    )?;

    Ok(qrcode_status_data)
}
