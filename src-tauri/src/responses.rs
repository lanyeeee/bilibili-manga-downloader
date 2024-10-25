// TODO: 把responses.rs拆成多个文件
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
pub struct GenerateQrcodeRespData {
    pub url: String,
    #[serde(rename = "qrcode_key")]
    pub qrcode_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct QrcodeStatusRespData {
    pub url: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    pub timestamp: i64,
    pub code: i64,
    pub message: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Buvid3RespData {
    pub buvid: String,
}

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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_excessive_bools)]
#[allow(clippy::struct_field_names)]
pub struct MangaRespData {
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
    pub last_ord: i64,
    #[serde(rename = "is_finish")]
    pub is_finish: i64,
    pub status: i64,
    pub fav: i64,
    #[serde(rename = "read_order")]
    pub read_order: i64,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageIndexRespData {
    pub host: String,
    pub images: Vec<ImageRespData>,
    #[serde(rename = "last_modified")]
    pub last_modified: String,
    pub path: String,
    pub video: VideoRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageRespData {
    pub path: String,
    #[serde(rename = "video_path")]
    pub video_path: String,
    #[serde(rename = "video_size")]
    pub video_size: String,
    pub x: i64,
    pub y: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRespData {
    #[serde(rename = "bin_url")]
    pub bin_url: String,
    pub filename: String,
    #[serde(rename = "img_urls")]
    pub img_urls: Vec<serde_json::Value>,
    #[serde(rename = "img_x_len")]
    pub img_x_len: i64,
    #[serde(rename = "img_x_size")]
    pub img_x_size: i64,
    #[serde(rename = "img_y_len")]
    pub img_y_len: i64,
    #[serde(rename = "img_y_size")]
    pub img_y_size: i64,
    #[serde(rename = "raw_height")]
    pub raw_height: String,
    #[serde(rename = "raw_rotate")]
    pub raw_rotate: String,
    #[serde(rename = "raw_width")]
    pub raw_width: String,
    pub resource: Vec<serde_json::Value>,
    pub route: String,
    pub svid: String,
}

pub type ImageTokenRespData = Vec<UrlTokenRespData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlTokenRespData {
    pub url: String,
    pub token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileRespData {
    pub is_login: bool,
    #[serde(rename = "email_verified")]
    pub email_verified: i64,
    pub face: String,
    #[serde(rename = "face_nft")]
    pub face_nft: i64,
    #[serde(rename = "face_nft_type")]
    pub face_nft_type: i64,
    #[serde(rename = "level_info")]
    pub level_info: LevelInfoRespData,
    pub mid: i64,
    #[serde(rename = "mobile_verified")]
    pub mobile_verified: i64,
    pub money: i64,
    pub moral: i64,
    pub official: OfficialRespData,
    pub official_verify: OfficialVerifyRespData,
    pub pendant: PendantRespData,
    pub scores: i64,
    pub uname: String,
    pub vip_due_date: i64,
    pub vip_status: i64,
    pub vip_type: i64,
    #[serde(rename = "vip_pay_type")]
    pub vip_pay_type: i64,
    #[serde(rename = "vip_theme_type")]
    pub vip_theme_type: i64,
    #[serde(rename = "vip_label")]
    pub vip_label: LabelRespData,
    #[serde(rename = "vip_avatar_subscript")]
    pub vip_avatar_subscript: i64,
    #[serde(rename = "vip_nickname_color")]
    pub vip_nickname_color: String,
    pub vip: VipRespData,
    pub wallet: WalletRespData,
    #[serde(rename = "has_shop")]
    pub has_shop: bool,
    #[serde(rename = "shop_url")]
    pub shop_url: String,
    #[serde(rename = "answer_status")]
    pub answer_status: i64,
    #[serde(rename = "is_senior_member")]
    pub is_senior_member: i64,
    #[serde(rename = "wbi_img")]
    pub wbi_img: WbiImgRespData,
    #[serde(rename = "is_jury")]
    pub is_jury: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct LevelInfoRespData {
    #[serde(rename = "current_level")]
    pub current_level: i64,
    #[serde(rename = "current_min")]
    pub current_min: i64,
    #[serde(rename = "current_exp")]
    pub current_exp: i64,
    #[serde(rename = "next_exp")]
    pub next_exp: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OfficialRespData {
    pub role: i64,
    pub title: String,
    pub desc: String,
    #[serde(rename = "type")]
    pub type_field: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct OfficialVerifyRespData {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub desc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct PendantRespData {
    pub pid: i64,
    pub name: String,
    pub image: String,
    pub expire: i64,
    #[serde(rename = "image_enhance")]
    pub image_enhance: String,
    #[serde(rename = "image_enhance_frame")]
    pub image_enhance_frame: String,
    #[serde(rename = "n_pid")]
    pub n_pid: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct VipRespData {
    #[serde(rename = "type")]
    pub type_field: i64,
    pub status: i64,
    #[serde(rename = "due_date")]
    pub due_date: i64,
    #[serde(rename = "vip_pay_type")]
    pub vip_pay_type: i64,
    #[serde(rename = "theme_type")]
    pub theme_type: i64,
    pub label: LabelRespData,
    #[serde(rename = "avatar_subscript")]
    pub avatar_subscript: i64,
    #[serde(rename = "nickname_color")]
    pub nickname_color: String,
    pub role: i64,
    #[serde(rename = "avatar_subscript_url")]
    pub avatar_subscript_url: String,
    #[serde(rename = "tv_vip_status")]
    pub tv_vip_status: i64,
    #[serde(rename = "tv_vip_pay_type")]
    pub tv_vip_pay_type: i64,
    #[serde(rename = "tv_due_date")]
    pub tv_due_date: i64,
    #[serde(rename = "avatar_icon")]
    pub avatar_icon: AvatarIconRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct LabelRespData {
    pub path: String,
    pub text: String,
    #[serde(rename = "label_theme")]
    pub label_theme: String,
    #[serde(rename = "text_color")]
    pub text_color: String,
    #[serde(rename = "bg_style")]
    pub bg_style: i64,
    #[serde(rename = "bg_color")]
    pub bg_color: String,
    #[serde(rename = "border_color")]
    pub border_color: String,
    #[serde(rename = "use_img_label")]
    pub use_img_label: bool,
    #[serde(rename = "img_label_uri_hans")]
    pub img_label_uri_hans: String,
    #[serde(rename = "img_label_uri_hant")]
    pub img_label_uri_hant: String,
    #[serde(rename = "img_label_uri_hans_static")]
    pub img_label_uri_hans_static: String,
    #[serde(rename = "img_label_uri_hant_static")]
    pub img_label_uri_hant_static: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AvatarIconRespData {
    #[serde(rename = "icon_resource")]
    pub icon_resource: IconResourceRespData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct IconResourceRespData {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WalletRespData {
    pub mid: i64,
    #[serde(rename = "bcoin_balance")]
    pub bcoin_balance: i64,
    #[serde(rename = "coupon_balance")]
    pub coupon_balance: i64,
    #[serde(rename = "coupon_due_time")]
    pub coupon_due_time: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WbiImgRespData {
    #[serde(rename = "img_url")]
    pub img_url: String,
    #[serde(rename = "sub_url")]
    pub sub_url: String,
}
