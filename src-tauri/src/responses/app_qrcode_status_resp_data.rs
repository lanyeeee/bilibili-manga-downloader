use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct AppQrcodeStatusRespData {
    #[serde(rename = "is_new")]
    pub is_new: bool,
    pub mid: i64,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
    #[serde(rename = "token_info")]
    pub token_info: TokenInfoRespData,
    #[serde(rename = "cookie_info")]
    pub cookie_info: CookieInfoRespData,
    pub sso: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfoRespData {
    pub mid: i64,
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "refresh_token")]
    pub refresh_token: String,
    #[serde(rename = "expires_in")]
    pub expires_in: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CookieInfoRespData {
    pub cookies: Vec<CookieRespData>,
    pub domains: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CookieRespData {
    pub name: String,
    pub value: String,
    #[serde(rename = "http_only")]
    pub http_only: i64,
    pub expires: i64,
    pub secure: i64,
}
