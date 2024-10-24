use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BiliResp {
    pub code: i64,
    #[serde(alias = "message")]
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
