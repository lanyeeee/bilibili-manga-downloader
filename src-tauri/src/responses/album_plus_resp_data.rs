use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlusRespData {
    pub list: Vec<AlbumPlusDetailRespData>,
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    #[serde(rename = "comic_title")]
    pub comic_title: String,
    #[serde(rename = "server_time")]
    pub server_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlusDetailRespData {
    pub is_lock: bool,
    pub cost: i64,
    pub reward: i64,
    pub item: AlbumPlusItemRespData,
    #[serde(rename = "unlocked_item_ids")]
    pub unlocked_item_ids: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumPlusItemRespData {
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
    pub video: Option<VideoRespData>,
    #[serde(rename = "item_infos")]
    pub item_infos: Vec<ItemInfoRespData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRespData {
    pub id: i64,
    pub url: String,
    pub cover: String,
    pub duration: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfoRespData {
    pub id: i64,
    pub title: String,
}
