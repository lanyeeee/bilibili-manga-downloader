mod bili_client;
mod commands;
mod config;
mod download_manager;
mod errors;
mod events;
mod extensions;
mod responses;
mod types;
mod utils;

use crate::commands::*;
use crate::config::Config;
use crate::download_manager::DownloadManager;
use crate::events::prelude::*;
use anyhow::Context;
use parking_lot::RwLock;
use tauri::{Manager, Wry};

fn generate_context() -> tauri::Context<Wry> {
    tauri::generate_context!()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri_specta::Builder::<Wry>::new()
        .commands(tauri_specta::collect_commands![
            greet,
            get_config,
            save_config,
            generate_app_qrcode,
            get_app_qrcode_status,
            generate_web_qrcode,
            get_web_qrcode_status,
            confirm_app_qrcode,
            search,
            get_comic,
            get_album_plus,
            download_episodes,
            download_album_plus_items,
            show_path_in_file_manager,
            get_user_profile,
        ])
        .events(tauri_specta::collect_events![
            RemoveWatermarkStartEvent,
            RemoveWatermarkSuccessEvent,
            RemoveWatermarkErrorEvent,
            RemoveWatermarkEndEvent,
            DownloadPendingEvent,
            DownloadStartEvent,
            DownloadImageSuccessEvent,
            DownloadImageErrorEvent,
            DownloadEndEvent,
            DownloadSpeedEvent,
        ]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                .bigint(specta_typescript::BigIntExportBehavior::Number)
                .formatter(specta_typescript::formatter::prettier)
                .header("// @ts-nocheck"), // 跳过检查
            "../src/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);

            let app_data_dir = app
                .path()
                .app_data_dir()
                .context("failed to get app data dir")?;

            std::fs::create_dir_all(&app_data_dir)
                .context(format!("failed to create app data dir: {app_data_dir:?}"))?;
            println!("app data dir: {app_data_dir:?}");

            let config = RwLock::new(Config::new(app.handle())?);
            app.manage(config);

            let download_manager = DownloadManager::new(app.handle());
            app.manage(download_manager);

            let bili_client = bili_client::BiliClient::new(app.handle().clone());
            app.manage(bili_client);

            Ok(())
        })
        .run(generate_context())
        .expect("error while running tauri application");
}
