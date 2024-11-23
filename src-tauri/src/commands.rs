use std::path::PathBuf;
use std::vec;

use anyhow::anyhow;
use parking_lot::RwLock;
use path_slash::PathBufExt;
use reqwest::StatusCode;
use tauri::{AppHandle, State};

use crate::bili_client::BiliClient;
use crate::config::Config;
use crate::download_manager::DownloadManager;
use crate::errors::CommandResult;
use crate::responses::{
    ConfirmAppQrcodeRespData, GithubReleasesResp, SearchRespData, UserProfileRespData,
    WebQrcodeStatusRespData,
};
use crate::types::{
    AlbumPlus, AlbumPlusItem, AppQrcodeData, AppQrcodeStatus, CheckUpdateResult, Comic,
    EpisodeInfo, WebQrcodeData,
};

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn get_config(config: State<RwLock<Config>>) -> Config {
    config.read().clone()
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub async fn save_config(
    app: AppHandle,
    bili_client: State<'_, BiliClient>,
    config_state: State<'_, RwLock<Config>>,
    config: Config,
) -> CommandResult<()> {
    let need_recreate = {
        let config_state = config_state.read();
        config_state.proxy_mode != config.proxy_mode
            || config_state.proxy_host != config.proxy_host
            || config_state.proxy_port != config.proxy_port
    };

    *config_state.write() = config;
    config_state.write().save(&app)?;

    if need_recreate {
        bili_client.recreate_http_client().await;
    }

    Ok(())
}

#[tauri::command(async)]
#[specta::specta]
pub async fn generate_app_qrcode(
    bili_client: State<'_, BiliClient>,
) -> CommandResult<AppQrcodeData> {
    let app_qrcode_data = bili_client.generate_app_qrcode().await?;
    Ok(app_qrcode_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_app_qrcode_status(
    bili_client: State<'_, BiliClient>,
    auth_code: String,
) -> CommandResult<AppQrcodeStatus> {
    let app_qrcode_status = bili_client.get_app_qrcode_status(auth_code).await?;
    Ok(app_qrcode_status)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn generate_web_qrcode(
    bili_client: State<'_, BiliClient>,
) -> CommandResult<WebQrcodeData> {
    let web_qrcode_data = bili_client.generate_web_qrcode().await?;
    Ok(web_qrcode_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_web_qrcode_status(
    bili_client: State<'_, BiliClient>,
    qrcode_key: String,
) -> CommandResult<WebQrcodeStatusRespData> {
    let web_qrcode_status = bili_client.get_web_qrcode_status(&qrcode_key).await?;
    Ok(web_qrcode_status)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn confirm_app_qrcode(
    bili_client: State<'_, BiliClient>,
    auth_code: String,
    sessdata: String,
    csrf: String,
) -> CommandResult<ConfirmAppQrcodeRespData> {
    let confirm_app_qrcode_resp_data = bili_client
        .confirm_app_qrcode(&auth_code, &sessdata, &csrf)
        .await?;
    Ok(confirm_app_qrcode_resp_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_user_profile(
    bili_client: State<'_, BiliClient>,
) -> CommandResult<UserProfileRespData> {
    let user_profile_resp_data = bili_client.get_user_profile().await?;
    Ok(user_profile_resp_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn search(
    bili_client: State<'_, BiliClient>,
    keyword: &str,
    page_num: i64,
) -> CommandResult<SearchRespData> {
    let search_resp_data = bili_client.search(keyword, page_num).await?;
    Ok(search_resp_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_comic(bili_client: State<'_, BiliClient>, comic_id: i64) -> CommandResult<Comic> {
    let comic = bili_client.get_comic(comic_id).await?;
    Ok(comic)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_album_plus(
    bili_client: State<'_, BiliClient>,
    comic_id: i64,
) -> CommandResult<AlbumPlus> {
    let album_plus = bili_client.get_album_plus(comic_id).await?;
    Ok(album_plus)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn download_episodes(
    download_manager: State<'_, DownloadManager>,
    episodes: Vec<EpisodeInfo>,
) -> CommandResult<()> {
    for ep in episodes {
        download_manager.submit_episode(ep).await?;
    }
    Ok(())
}

#[tauri::command(async)]
#[specta::specta]
pub async fn download_album_plus_items(
    download_manager: State<'_, DownloadManager>,
    items: Vec<AlbumPlusItem>,
) -> CommandResult<()> {
    for item in items {
        download_manager.submit_album_plus(item).await?;
    }
    Ok(())
}

#[tauri::command(async)]
#[specta::specta]
pub fn show_path_in_file_manager(path: &str) -> CommandResult<()> {
    let path = PathBuf::from_slash(path);
    if !path.exists() {
        return Err(anyhow!("路径`{path:?}`不存在").into());
    }
    showfile::show_path_in_file_manager(path);
    Ok(())
}

#[tauri::command(async)]
#[specta::specta]
pub async fn check_update(app: AppHandle) -> CommandResult<CheckUpdateResult> {
    let http_client = reqwest::ClientBuilder::new().build()?;
    let http_resp = http_client
        .get("https://api.github.com/repos/lanyeeee/bilibili-manga-downloader/releases")
        .header("user-agent", "lanyeeee/bilibili-manga-downloader")
        .send()
        .await?;
    let status = http_resp.status();
    let body = http_resp.text().await?;
    if status != StatusCode::OK {
        return Err(anyhow!("获取更新信息失败，预料之外的状态码({status}: {body})").into());
    }
    // current_version 格式为 0.0.0 的版本号
    let current_version = app.package_info().version.to_string();
    // 滤出 tag_name 为 v0.0.0 格式且大于当前版本的 release
    let releases = serde_json::from_str::<GithubReleasesResp>(&body)?
        .into_iter()
        .filter_map(|release| {
            // 滤出 tag_name 为 v0.0.0 格式的 release
            let tag_name = &release.tag_name;
            if !tag_name.starts_with('v') {
                return None;
            }
            if tag_name[1..].split('.').count() != 3 {
                return None;
            }
            // 滤出大于当前版本的 release
            let Ok(current_version) = semver::Version::parse(&current_version) else {
                return None;
            };
            let Ok(release_version) = semver::Version::parse(&tag_name[1..]) else {
                return None;
            };
            if release_version <= current_version {
                return None;
            }
            Some(release)
        });
    // 将 release 的 tag_name 提取出来
    let mut normal_releases = vec![];
    let mut important_releases = vec![];
    for release in releases {
        if release.name.contains("重要重要重要") {
            important_releases.push(release.tag_name);
        } else {
            normal_releases.push(release.tag_name);
        }
    }
    // 返回检查更新结果
    let check_update_result = CheckUpdateResult {
        normal_versions: normal_releases,
        important_versions: important_releases,
    };

    Ok(check_update_result)
}
