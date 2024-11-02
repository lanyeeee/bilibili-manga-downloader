use std::sync::RwLock;

use crate::config::Config;
use crate::extensions::IgnoreRwLockPoison;
use crate::responses::AlbumPlusRespData;
use crate::utils::filename_filter;

use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlus {
    pub list: Vec<AlbumPlusDetail>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "comic_title")]
    pub comic_title: String,
    #[serde(rename = "server_time")]
    pub server_time: String,
}
impl AlbumPlus {
    pub fn from(app: &AppHandle, resp_data: AlbumPlusRespData) -> Self {
        let comic_title = filename_filter(&resp_data.comic_title);
        let list = resp_data
            .list
            .into_iter()
            .map(|detail| {
                let video = detail.item.video.map(|v| Video {
                    id: v.id,
                    url: v.url,
                    cover: v.cover,
                    duration: v.duration,
                });

                let item_infos = detail
                    .item
                    .item_infos
                    .into_iter()
                    .map(|info| ItemInfo {
                        id: info.id,
                        title: info.title,
                    })
                    .collect();

                let title = filename_filter(&detail.item.title);
                let is_downloaded = Self::get_is_downloaded(app, &title, &comic_title);
                let item = AlbumPlusItem {
                    id: detail.item.id,
                    title,
                    cover: detail.item.cover,
                    pic: detail.item.pic,
                    rank: detail.item.rank,
                    detail: detail.item.detail,
                    limits: detail.item.limits,
                    pic_type: detail.item.pic_type,
                    pic_num: detail.item.pic_num,
                    online_time: detail.item.online_time,
                    offline_time: detail.item.offline_time,
                    num: detail.item.num,
                    type_field: detail.item.type_field,
                    icon: detail.item.icon,
                    activity_url: detail.item.activity_url,
                    activity_name: detail.item.activity_name,
                    item_ids: detail.item.item_ids,
                    no_local: detail.item.no_local,
                    video,
                    item_infos,
                };

                AlbumPlusDetail {
                    is_lock: detail.is_lock,
                    is_downloaded,
                    cost: detail.cost,
                    reward: detail.reward,
                    item,
                    unlocked_item_ids: detail.unlocked_item_ids,
                }
            })
            .collect();

        Self {
            list,
            icon_url: resp_data.icon_url,
            comic_title: resp_data.comic_title,
            server_time: resp_data.server_time,
        }
    }
    fn get_is_downloaded(app: &AppHandle, album_plus_title: &str, comic_title: &str) -> bool {
        app.state::<RwLock<Config>>()
            .read_or_panic()
            .download_dir
            .join(comic_title)
            .join("特典")
            .join(album_plus_title)
            .exists()
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlusDetail {
    pub is_lock: bool,
    pub is_downloaded: bool,
    pub cost: i64,
    pub reward: i64,
    pub item: AlbumPlusItem,
    #[serde(rename = "unlocked_item_ids")]
    pub unlocked_item_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlusItem {
    pub id: i64,
    pub title: String,
    pub cover: String,
    pub pic: Vec<String>,
    pub rank: i64,
    pub detail: String,
    pub limits: i64,
    #[serde(rename = "pic_type")]
    pub pic_type: i64,
    #[serde(rename = "pic_num")]
    pub pic_num: i64,
    #[serde(rename = "online_time")]
    pub online_time: String,
    #[serde(rename = "offline_time")]
    pub offline_time: String,
    pub num: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub icon: String,
    #[serde(rename = "activity_url")]
    pub activity_url: String,
    #[serde(rename = "activity_name")]
    pub activity_name: String,
    #[serde(rename = "item_ids")]
    pub item_ids: Vec<i64>,
    #[serde(rename = "no_local")]
    pub no_local: bool,
    pub video: Option<Video>,
    #[serde(rename = "item_infos")]
    pub item_infos: Vec<ItemInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: i64,
    pub url: String,
    pub cover: String,
    pub duration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    pub id: i64,
    pub title: String,
}
