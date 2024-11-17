use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct WebQrcodeData {
    pub base64: String,
    #[serde(rename = "qrcodeKey")]
    pub qrcode_key: String,
}
