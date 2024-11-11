use crate::config::Config;
use crate::responses::AlbumPlusRespData;
use crate::utils::filename_filter;

use parking_lot::RwLock;
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
                let title = filename_filter(&detail.item.title);
                let is_downloaded = Self::get_is_downloaded(app, &title, &comic_title);
                let item = AlbumPlusItem {
                    id: detail.item.id,
                    title,
                    comic_title: comic_title.clone(),
                    pic: detail.item.pic,
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
            comic_title,
            server_time: resp_data.server_time,
        }
    }

    fn get_is_downloaded(app: &AppHandle, album_plus_title: &str, comic_title: &str) -> bool {
        app.state::<RwLock<Config>>()
            .read()
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
    pub comic_title: String,
    pub pic: Vec<String>,
}
