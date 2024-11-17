use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri_specta::Event;

pub mod prelude {
    pub use crate::events::{
        DownloadEndEvent, DownloadImageErrorEvent, DownloadImageSuccessEvent, DownloadPendingEvent,
        DownloadSpeedEvent, DownloadStartEvent, RemoveWatermarkEndEvent, RemoveWatermarkErrorEvent,
        RemoveWatermarkStartEvent, RemoveWatermarkSuccessEvent,
    };
}

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoveWatermarkStartEventPayload {
    pub dir_path: PathBuf,
    pub total: u32,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct RemoveWatermarkStartEvent(pub RemoveWatermarkStartEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoveWatermarkSuccessEventPayload {
    pub dir_path: PathBuf,
    pub img_path: PathBuf,
    pub current: u32,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct RemoveWatermarkSuccessEvent(pub RemoveWatermarkSuccessEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoveWatermarkErrorEventPayload {
    pub dir_path: PathBuf,
    pub img_path: PathBuf,
    pub err_msg: String,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct RemoveWatermarkErrorEvent(pub RemoveWatermarkErrorEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct RemoveWatermarkEndEventPayload {
    pub dir_path: PathBuf,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct RemoveWatermarkEndEvent(pub RemoveWatermarkEndEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadPendingEventPayload {
    pub id: i64,
    pub title: String,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadPendingEvent(pub DownloadPendingEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadStartEventPayload {
    pub id: i64,
    pub title: String,
    pub total: u32,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadStartEvent(pub DownloadStartEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadImageSuccessEventPayload {
    pub id: i64,
    pub url: String,
    pub current: u32,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadImageSuccessEvent(pub DownloadImageSuccessEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadImageErrorEventPayload {
    pub id: i64,
    pub url: String,
    pub err_msg: String,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadImageErrorEvent(pub DownloadImageErrorEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadEndEventPayload {
    pub id: i64,
    pub err_msg: Option<String>,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadEndEvent(pub DownloadEndEventPayload);

#[derive(Serialize, Deserialize, Clone, Type)]
#[serde(rename_all = "camelCase")]
pub struct DownloadSpeedEventPayload {
    pub speed: String,
}
#[derive(Serialize, Deserialize, Clone, Type, Event)]
pub struct DownloadSpeedEvent(pub DownloadSpeedEventPayload);
