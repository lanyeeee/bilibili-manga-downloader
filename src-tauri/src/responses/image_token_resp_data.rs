use serde::{Deserialize, Serialize};

pub type ImageTokenRespData = Vec<ImageTokenItemRespData>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageTokenItemRespData {
    #[serde(rename = "complete_url")]
    pub complete_url: String,
    #[serde(rename = "hit_encrpyt")]
    pub hit_encrpyt: bool,
    pub url: String,
    pub token: String,
}
