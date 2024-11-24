use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct UserProfileRespData {
    pub mid: u64,
    pub face: String,
    #[serde(alias = "uname")]
    pub name: String,
}
