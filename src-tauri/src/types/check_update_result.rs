use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CheckUpdateResult {
    pub normal_versions: Vec<String>,
    pub important_versions: Vec<String>,
}
