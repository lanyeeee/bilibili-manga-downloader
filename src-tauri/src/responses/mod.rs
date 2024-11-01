mod buvid3_resp_data;
mod generate_qrcode_resp_data;
mod image_index_resp_data;
mod image_token_resp_data;
mod manga_resp_data;
mod qrcode_status_resp_data;
mod search_resp_data;
mod user_profile_resp_data;

pub use buvid3_resp_data::*;
pub use generate_qrcode_resp_data::*;
pub use image_index_resp_data::*;
pub use image_token_resp_data::*;
pub use manga_resp_data::*;
pub use qrcode_status_resp_data::*;
pub use search_resp_data::*;
pub use user_profile_resp_data::*;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliResp {
    pub code: i64,
    #[serde(default, alias = "message")]
    pub msg: String,
    pub data: Option<serde_json::Value>,
}
