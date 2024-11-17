use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
pub struct ConfirmAppQrcodeRespData {
    pub code: i64,
    #[serde(default, alias = "message")]
    pub msg: String,
}
