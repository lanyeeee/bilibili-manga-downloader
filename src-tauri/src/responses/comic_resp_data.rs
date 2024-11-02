use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::struct_field_names)]
pub struct ComicRespData {
    pub id: i64,
    pub title: String,
    #[serde(rename = "comic_type")]
    pub comic_type: i64,
    #[serde(rename = "page_default")]
    pub page_default: i64,
    #[serde(rename = "page_allow")]
    pub page_allow: i64,
    #[serde(rename = "horizontal_cover")]
    pub horizontal_cover: String,
    #[serde(rename = "square_cover")]
    pub square_cover: String,
    #[serde(rename = "vertical_cover")]
    pub vertical_cover: String,
    #[serde(rename = "author_name")]
    pub author_name: Vec<String>,
    pub styles: Vec<String>,
    #[serde(rename = "last_ord")]
    pub last_ord: f64,
    #[serde(rename = "is_finish")]
    pub is_finish: i64,
    pub status: i64,
    pub fav: i64,
    #[serde(rename = "read_order")]
    pub read_order: f64,
    pub evaluate: String,
    pub total: i64,
    #[serde(rename = "ep_list")]
    pub ep_list: Vec<EpisodeRespData>,
    #[serde(rename = "release_time")]
    pub release_time: String,
    #[serde(rename = "is_limit")]
    pub is_limit: i64,
    #[serde(rename = "read_epid")]
    pub read_epid: i64,
    #[serde(rename = "last_read_time")]
    pub last_read_time: String,
    #[serde(rename = "is_download")]
    pub is_download: i64,
    #[serde(rename = "read_short_title")]
    pub read_short_title: String,
    pub styles2: Vec<Styles2RespData>,
    #[serde(rename = "renewal_time")]
    pub renewal_time: String,
    #[serde(rename = "last_short_title")]
    pub last_short_title: String,
    #[serde(rename = "discount_type")]
    pub discount_type: i64,
    pub discount: i64,
    #[serde(rename = "discount_end")]
    pub discount_end: String,
    #[serde(rename = "no_reward")]
    pub no_reward: bool,
    #[serde(rename = "batch_discount_type")]
    pub batch_discount_type: i64,
    #[serde(rename = "ep_discount_type")]
    pub ep_discount_type: i64,
    #[serde(rename = "has_fav_activity")]
    pub has_fav_activity: bool,
    #[serde(rename = "fav_free_amount")]
    pub fav_free_amount: i64,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    #[serde(rename = "wait_hour")]
    pub wait_hour: i64,
    #[serde(rename = "wait_free_at")]
    pub wait_free_at: String,
    #[serde(rename = "no_danmaku")]
    pub no_danmaku: i64,
    #[serde(rename = "auto_pay_status")]
    pub auto_pay_status: i64,
    #[serde(rename = "no_month_ticket")]
    pub no_month_ticket: bool,
    pub immersive: bool,
    #[serde(rename = "no_discount")]
    pub no_discount: bool,
    #[serde(rename = "show_type")]
    pub show_type: i64,
    #[serde(rename = "pay_mode")]
    pub pay_mode: i64,
    #[serde(rename = "classic_lines")]
    pub classic_lines: String,
    #[serde(rename = "pay_for_new")]
    pub pay_for_new: i64,
    #[serde(rename = "fav_comic_info")]
    pub fav_comic_info: FavComicInfoRespData,
    #[serde(rename = "serial_status")]
    pub serial_status: i64,
    #[serde(rename = "album_count")]
    pub album_count: i64,
    #[serde(rename = "wiki_id")]
    pub wiki_id: i64,
    #[serde(rename = "disable_coupon_amount")]
    pub disable_coupon_amount: i64,
    #[serde(rename = "japan_comic")]
    pub japan_comic: bool,
    #[serde(rename = "interact_value")]
    pub interact_value: String,
    #[serde(rename = "temporary_finish_time")]
    pub temporary_finish_time: String,
    pub introduction: String,
    #[serde(rename = "comment_status")]
    pub comment_status: i64,
    #[serde(rename = "no_screenshot")]
    pub no_screenshot: bool,
    #[serde(rename = "type")]
    pub type_field: i64,
    #[serde(rename = "no_rank")]
    pub no_rank: bool,
    #[serde(rename = "presale_text")]
    pub presale_text: String,
    #[serde(rename = "presale_discount")]
    pub presale_discount: i64,
    #[serde(rename = "no_leaderboard")]
    pub no_leaderboard: bool,
    #[serde(rename = "auto_pay_info")]
    pub auto_pay_info: AutoPayInfoRespData,
    pub orientation: i64,
    #[serde(rename = "story_elems")]
    pub story_elems: Vec<StoryElemRespData>,
    pub tags: Vec<TagRespData>,
    #[serde(rename = "is_star_hall")]
    pub is_star_hall: i64,
    #[serde(rename = "hall_icon_text")]
    pub hall_icon_text: String,
    #[serde(rename = "rookie_fav_tip")]
    pub rookie_fav_tip: RookieFavTipRespData,
    pub authors: Vec<AuthorRespData>,
    #[serde(rename = "comic_alias")]
    pub comic_alias: Vec<String>,
    #[serde(rename = "horizontal_covers")]
    pub horizontal_covers: Vec<String>,
    #[serde(rename = "data_info")]
    pub data_info: DataInfoRespData,
    #[serde(rename = "last_short_title_msg")]
    pub last_short_title_msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct EpisodeRespData {
    pub id: i64,
    pub ord: f64,
    pub read: i64,
    #[serde(rename = "pay_mode")]
    pub pay_mode: i64,
    #[serde(rename = "is_locked")]
    pub is_locked: bool,
    #[serde(rename = "pay_gold")]
    pub pay_gold: i64,
    pub size: i64,
    #[serde(rename = "short_title")]
    pub short_title: String,
    #[serde(rename = "is_in_free")]
    pub is_in_free: bool,
    pub title: String,
    pub cover: String,
    #[serde(rename = "pub_time")]
    pub pub_time: String,
    pub comments: i64,
    #[serde(rename = "unlock_expire_at")]
    pub unlock_expire_at: String,
    #[serde(rename = "unlock_type")]
    pub unlock_type: i64,
    #[serde(rename = "allow_wait_free")]
    pub allow_wait_free: bool,
    pub progress: String,
    #[serde(rename = "like_count")]
    pub like_count: i64,
    #[serde(rename = "chapter_id")]
    pub chapter_id: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub extra: i64,
    #[serde(rename = "image_count")]
    pub image_count: i64,
    #[serde(rename = "index_last_modified")]
    pub index_last_modified: String,
    #[serde(rename = "jump_url")]
    pub jump_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Styles2RespData {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct FavComicInfoRespData {
    #[serde(rename = "has_fav_activity")]
    pub has_fav_activity: bool,
    #[serde(rename = "fav_free_amount")]
    pub fav_free_amount: i64,
    #[serde(rename = "fav_coupon_type")]
    pub fav_coupon_type: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AutoPayInfoRespData {
    #[serde(rename = "auto_pay_orders")]
    pub auto_pay_orders: Vec<AutoPayOrderRespData>,
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AutoPayOrderRespData {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct StoryElemRespData {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TagRespData {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct RookieFavTipRespData {
    #[serde(rename = "is_show")]
    pub is_show: bool,
    pub used: i64,
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AuthorRespData {
    pub id: i64,
    pub name: String,
    pub cname: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct DataInfoRespData {
    #[serde(rename = "read_score")]
    pub read_score: ReadScoreRespData,
    #[serde(rename = "interactive_value")]
    pub interactive_value: InteractiveValueRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names)]
pub struct ReadScoreRespData {
    #[serde(rename = "read_score")]
    pub read_score: String,
    #[serde(rename = "is_jump")]
    pub is_jump: bool,
    pub increase: IncreaseRespData,
    pub percentile: f64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct InteractiveValueRespData {
    #[serde(rename = "interact_value")]
    pub interact_value: String,
    #[serde(rename = "is_jump")]
    pub is_jump: bool,
    pub increase: IncreaseRespData,
    pub percentile: f64,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct IncreaseRespData {
    pub days: i64,
    #[serde(rename = "increase_percent")]
    pub increase_percent: i64,
}
