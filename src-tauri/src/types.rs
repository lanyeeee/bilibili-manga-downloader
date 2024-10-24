use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize, Type)]
pub struct QrcodeData {
    pub base64: String,
    #[serde(rename = "qrcodeKey")]
    pub qrcode_key: String,
}
