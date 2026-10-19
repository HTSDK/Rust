mod reverse_proxy;
mod config;
mod access_control;

use crate::api::API;
use crate::apis::syno::core::app_portal::reverse_proxy::SynoCoreAppPortalReverseProxy;
use crate::request::{Output, Request};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::apis::syno::core::app_portal::config::SynoCoreAppPortalConfig;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AppPortalListParamsAdditional {
    DefaultSetting,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppPortalListParams {
    additional: Option<Vec<AppPortalListParamsAdditional>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPortalListResponse {
    pub portal: Vec<AppPortalListResponsePortal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPortalListResponsePortal {
    pub additional: Option<AppPortalListResponsePortalAdditional>,
    pub display_name: String,
    pub enable_redirect: bool,
    pub id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPortalListResponsePortalAdditional {
    pub default_setting: Option<AppPortalListResponsePortalAdditionalDefaultSetting>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AppPortalListResponsePortalAdditionalDefaultSetting {
    pub alias: String,
    pub fqdn: String,
    pub hsts: bool,
    pub http_port: u32,
    pub https_port: u32,
}

pub struct SynoCoreAppPortal {
    api: Arc<API>,
}

impl SynoCoreAppPortal {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn config(&self) -> SynoCoreAppPortalConfig {
        SynoCoreAppPortalConfig::new(self.api.clone())
    }

    pub fn reverse_proxy(&self) -> SynoCoreAppPortalReverseProxy {
        SynoCoreAppPortalReverseProxy::new(self.api.clone())
    }

    pub async fn list(&self, params: Option<AppPortalListParams>) -> Output<AppPortalListResponse> {
        let request = Request::new("SYNO.Core.AppPortal", "list", 2);
        self.api
            .request(match params {
                Some(params) => request.params(params),
                None => request,
            })
            .await
    }
}
