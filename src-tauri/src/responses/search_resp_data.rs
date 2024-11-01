use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchRespData {
    #[serde(rename = "comic_data")]
    pub comic_data: SearchComicRespData,
    #[serde(rename = "novel_data")]
    pub novel_data: SearchNovelRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchComicRespData {
    pub list: Vec<ComicInSearchRespData>,
    #[serde(rename = "total_page")]
    pub total_page: i64,
    #[serde(rename = "total_num")]
    pub total_num: i64,
    pub similar: String,
    #[serde(rename = "se_id")]
    pub se_id: String,
    pub banner: BannerRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ComicInSearchRespData {
    pub id: i64,
    pub title: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "is_finish")]
    pub is_finish: i64,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub wiki: WikiRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WikiRespData {
    pub id: i64,
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
pub struct BannerRespData {
    pub icon: String,
    pub title: String,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SearchNovelRespData {
    pub total: i64,
    pub list: Vec<NovelInSearchRespData>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NovelInSearchRespData {
    #[serde(rename = "novel_id")]
    pub novel_id: i64,
    pub title: String,
    #[serde(rename = "v_cover")]
    pub v_cover: String,
    #[serde(rename = "finish_status")]
    pub finish_status: i64,
    pub status: i64,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    pub numbers: i64,
    pub style: StyleRespData,
    pub evaluate: String,
    pub author: String,
    pub tag: TagRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StyleRespData {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TagRespData {
    pub id: i64,
    pub name: String,
}
