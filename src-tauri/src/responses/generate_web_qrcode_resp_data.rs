use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GenerateWebQrcodeRespData {
    pub url: String,
    #[serde(rename = "qrcode_key")]
    pub qrcode_key: String,
}
