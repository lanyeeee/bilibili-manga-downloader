mod album_plus_resp_data;
mod app_qrcode_status_resp_data;
mod comic_resp_data;
mod confirm_app_qrcode_resp_data;
mod generate_app_qrcode_resp_data;
mod generate_web_qrcode_resp_data;
mod image_index_resp_data;
mod image_token_resp_data;
mod search_resp_data;
mod user_profile_resp_data;
mod web_qrcode_status_resp_data;

pub use album_plus_resp_data::*;
pub use app_qrcode_status_resp_data::*;
pub use comic_resp_data::*;
pub use confirm_app_qrcode_resp_data::*;
pub use generate_app_qrcode_resp_data::*;
pub use generate_web_qrcode_resp_data::*;
pub use image_index_resp_data::*;
pub use image_token_resp_data::*;
pub use search_resp_data::*;
pub use user_profile_resp_data::*;
pub use web_qrcode_status_resp_data::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliResp {
    pub code: i64,
    #[serde(default, alias = "message")]
    pub msg: String,
    pub data: Option<serde_json::Value>,
}
