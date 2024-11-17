use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WebQrcodeStatusRespData {
    pub url: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    pub timestamp: i64,
    pub code: i64,
    pub message: String,
}
