use std::path::PathBuf;
use std::sync::RwLock;

use anyhow::anyhow;
use path_slash::PathBufExt;
use tauri::{AppHandle, State};

use crate::bili_client::BiliClient;
use crate::config::Config;
use crate::download_manager::DownloadManager;
use crate::errors::CommandResult;
use crate::extensions::IgnoreRwLockPoison;
use crate::responses::{SearchRespData, UserProfileRespData};
use crate::types::{Comic, EpisodeInfo, QrcodeData, QrcodeStatus};

#[tauri::command]
#[specta::specta]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn get_config(config: State<RwLock<Config>>) -> Config {
    config.read().unwrap().clone()
}

#[tauri::command(async)]
#[specta::specta]
#[allow(clippy::needless_pass_by_value)]
pub fn save_config(
    app: AppHandle,
    config_state: State<RwLock<Config>>,
    config: Config,
) -> CommandResult<()> {
    let mut config_state = config_state.write_or_panic();
    *config_state = config;
    config_state.save(&app)?;
    Ok(())
}

#[tauri::command(async)]
#[specta::specta]
pub async fn generate_qrcode(bili_client: State<'_, BiliClient>) -> CommandResult<QrcodeData> {
    let qrcode_data = bili_client.generate_qrcode().await?;
    Ok(qrcode_data)
}

#[tauri::command(async)]
#[specta::specta]
pub async fn get_qrcode_status(
    bili_client: State<'_, BiliClient>,
    auth_code: String,
) -> CommandResult<QrcodeStatus> {
    let qrcode_status = bili_client.get_qrcode_status(auth_code).await?;
    Ok(qrcode_status)
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
pub fn show_path_in_file_manager(path: &str) -> CommandResult<()> {
    let path = PathBuf::from_slash(path);
    if !path.exists() {
        return Err(anyhow!("路径`{path:?}`不存在").into());
    }
    showfile::show_path_in_file_manager(path);
    Ok(())
}
