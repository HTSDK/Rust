mod _type;
mod ui_config;

use crate::api::API;
use crate::apis::syno::api::auth::_type::SynoApiAuthType;
use crate::request::{Output, Request};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::sync::Arc;
use crate::apis::syno::api::auth::ui_config::SynoApiAuthUiConfig;

#[derive(Debug, Clone, Serialize)]
pub struct LoginParams {
    pub account: String,
    pub passwd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otp_code: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub account: String,
    pub device_id: String,
    pub sid: String,
    pub is_portal_port: bool,
    pub synotoken: String,
}

pub struct SynoApiAuth {
    api: Arc<API>,
}

impl SynoApiAuth {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn _type(&self) -> SynoApiAuthType {
        SynoApiAuthType::new(self.api.clone())
    }

    pub fn ui_config(&self) -> SynoApiAuthUiConfig {
        SynoApiAuthUiConfig::new(self.api.clone())
    }

    pub async fn login(&mut self, params: LoginParams) -> Output<LoginResponse> {
        let response = self
            .api
            .request::<LoginResponse>(
                Request::new("SYNO.API.Auth", "login", 7)
                    .param("format", json!("cookie"))
                    .param("enable_syno_token", json!("yes"))
                    .params(params),
            )
            .await?;
        Arc::get_mut(&mut self.api)
            .unwrap()
            .set_session(Some(response.sid.clone()), Some(response.synotoken.clone()));
        Ok(response)
    }

    pub async fn logout(&mut self) {
        self.api
            .request::<Option<Value>>(Request::new("SYNO.API.Auth", "logout", 1))
            .await
            .unwrap();
        Arc::get_mut(&mut self.api).unwrap().set_session(None, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut auth = SynoApiAuth::new(Arc::new(API::new("http://localhost:5000")));
        let _ = auth.login(LoginParams {
            account: "test".to_string(),
            passwd: "test".to_string(),
            otp_code: None,
        });
    }
}
