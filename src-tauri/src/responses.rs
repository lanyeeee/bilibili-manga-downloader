use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliResp {
    pub code: i64,
    #[serde(default, alias = "message")]
    pub msg: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GenerateQrcodeData {
    pub url: String,
    #[serde(rename = "qrcode_key")]
    pub qrcode_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct QrcodeStatusData {
    pub url: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    pub timestamp: i64,
    pub code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Buvid3Data {
    pub buvid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchMangaData {
    pub list: Vec<SearchDetail>,
    #[serde(rename = "total_page")]
    pub total_page: i32,
    #[serde(rename = "total_num")]
    pub total_num: i32,
    pub recommends: Vec<Recommend>,
    pub similar: String,
    #[serde(rename = "se_id")]
    pub se_id: String,
    pub banner: Banner,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchDetail {
    pub id: i32,
    pub title: String,
    #[serde(rename = "org_title")]
    pub org_title: String,
    #[serde(rename = "horizontal_cover")]
    pub horizontal_cover: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "is_finish")]
    pub is_finish: i32,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "discount_type")]
    pub discount_type: i32,
    #[serde(rename = "type")]
    pub type_field: i32,
    pub wiki: Wiki,
    pub numbers: i32,
    #[serde(rename = "jump_value")]
    pub jump_value: String,
    #[serde(rename = "real_title")]
    pub real_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Wiki {
    pub id: i32,
    pub title: String,
    #[serde(rename = "origin_title")]
    pub origin_title: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    pub producer: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    #[serde(rename = "publish_time")]
    pub publish_time: String,
    pub frequency: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Recommend {
    pub id: i32,
    pub title: String,
    #[serde(rename = "horizontal_cover")]
    pub horizontal_cover: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "last_short_title")]
    pub last_short_title: String,
    pub recommendation: String,
    #[serde(rename = "is_finish")]
    pub is_finish: i32,
    pub total: i32,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "discount_type")]
    pub discount_type: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Banner {
    pub icon: String,
    pub title: String,
    pub url: String,
}
