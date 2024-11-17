use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppQrcodeData {
    pub base64: String,
    #[serde(rename = "auth_code")]
    pub auth_code: String,
}
