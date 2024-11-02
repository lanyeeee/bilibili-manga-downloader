use crate::responses::{BiliResp, CookieInfoRespData, QrcodeStatusRespData, TokenInfoRespData};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct QrcodeStatus {
    pub code: i64,
    pub message: String,
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
impl QrcodeStatus {
    pub fn from(bili_resp: BiliResp, qrcode_status_resp_data: QrcodeStatusRespData) -> Self {
        Self {
            code: bili_resp.code,
            message: bili_resp.msg,
            is_new: qrcode_status_resp_data.is_new,
            mid: qrcode_status_resp_data.mid,
            access_token: qrcode_status_resp_data.access_token,
            refresh_token: qrcode_status_resp_data.refresh_token,
            expires_in: qrcode_status_resp_data.expires_in,
            token_info: qrcode_status_resp_data.token_info,
            cookie_info: qrcode_status_resp_data.cookie_info,
            sso: qrcode_status_resp_data.sso,
        }
    }
}
