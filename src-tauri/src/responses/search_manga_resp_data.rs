use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchMangaRespData {
    pub list: Vec<MangaInSearchRespData>,
    #[serde(rename = "total_page")]
    pub total_page: i32,
    #[serde(rename = "total_num")]
    pub total_num: i32,
    pub recommends: Vec<RecommendRespData>,
    pub similar: String,
    #[serde(rename = "se_id")]
    pub se_id: String,
    pub banner: BannerRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct MangaInSearchRespData {
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
    pub wiki: WikiRespData,
    pub numbers: i32,
    #[serde(rename = "jump_value")]
    pub jump_value: String,
    #[serde(rename = "real_title")]
    pub real_title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiRespData {
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
pub struct RecommendRespData {
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
pub struct BannerRespData {
    pub icon: String,
    pub title: String,
    pub url: String,
}
