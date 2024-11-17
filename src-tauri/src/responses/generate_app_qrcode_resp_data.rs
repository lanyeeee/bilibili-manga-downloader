use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GenerateAppQrcodeRespData {
    pub url: String,
    #[serde(rename = "auth_code")]
    pub auth_code: String,
}
