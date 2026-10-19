use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct UIConfigResponse {
    pub css_path: Option<String>,
    pub js_path: String,
    pub texts: HashMap<String, Texts>,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Texts {
    pub app: TextsApp,
    pub authenticator: TextsAuthenticator,
    pub common: TextsCommon,
    pub error: TextsError
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextsApp {
    pub description: String,
    #[serde(rename = "displayname")]
    pub display_name: String,
    pub warning_remove_2fa_device: String,
    pub warning_remove_2step_device: String,
    pub warning_remove_the_last_2fa_device: String,
    pub warning_remove_the_last_2step_device: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextsAuthenticator {
    #[serde(rename = "2fa_authenticator_desc")]
    pub _2fa_authenticator_desc: String,
    #[serde(rename = "2fa_desc")]
    pub _2fa_desc: String,
    #[serde(rename = "2fa_login_flow")]
    pub _2fa_login_flow: String,
    #[serde(rename = "2fa_note_scan_qr_code")]
    pub _2fa_note_scan_qr_code: String,
    #[serde(rename = "2fa_setup_finish_desc")]
    pub _2fa_setup_finish_desc: String,
    #[serde(rename = "2fa_with_authenticator")]
    pub _2fa_with_authenticator: String,
    pub app_version: String,
    pub approve_sign_in: String,
    pub authenticator_desc: String,
    pub enforce_2fa_authenticator_desc: String,
    pub install_authenticator: String,
    pub install_authenticator_desc: String,
    pub login_desc: String,
    pub login_flow: String,
    pub login_page_desc: String,
    pub login_request_resend: String,
    pub login_request_resend_button: String,
    pub management_empty_text: String,
    pub mobile_wait_too_long_info: String,
    pub model: String,
    pub note_scan_qr_code: String,
    pub os_version: String,
    pub private_ip_error_msg_admin: String,
    pub private_ip_error_msg_admin_with_params: String,
    pub private_ip_error_msg_user: String,
    pub receive_sign_in_request: String,
    pub recv_sign_in_requests: String,
    pub register_unavailable_title: String,
    pub request_denied: String,
    pub request_denied_desc: String,
    pub request_sent_desc: String,
    pub request_sent_num_desc: String,
    pub request_timeout: String,
    pub request_timeout_desc: String,
    pub scan_qr_code: String,
    pub setup_authenticator: String,
    pub setup_authenticator_desc: String,
    pub sign_in_method: String,
    pub sign_in_with_authenticator: String,
    pub tap_to_continue: String,
    pub try_again: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextsCommon {
    #[serde(rename = "2fa_off")]
    pub _2fa_off: String,
    #[serde(rename = "2fa_on")]
    pub _2fa_on: String,
    #[serde(rename = "2fa_package_broken_msg")]
    pub _2fa_package_broken_msg: String,
    pub add_time: String,
    pub _continue: String,
    pub last_used_time: String,
    pub package_broken_msg: String,
    pub passwordless_off: String,
    pub passwordless_on: String,
    pub passwordless_signin: String,
    pub setup_success: String,
    pub turn_off: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextsError {
    pub invalid_device_name: String,
    pub invalid_domain: String,
    pub invalid_domain_user: String,
    pub other_ways_desc: String,
    pub register_again: String,
    pub something_wrong: String
}

// TODO: Expand

pub struct SynoApiAuthUiConfig {
    api: Arc<API>,
}

impl SynoApiAuthUiConfig {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<UIConfigResponse> {
        self.api
            .request(Request::new("SYNO.API.Auth.UIConfig", "get", 1))
            .await
    }
}
