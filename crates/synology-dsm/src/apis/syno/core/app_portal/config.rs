use crate::api::API;
use crate::request::{Output, Request};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct AppPortalConfigResponse {
    pub show_titlebar: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppPortalConfigSetParams {
    pub show_titlebar: Option<bool>,
}

pub struct SynoCoreAppPortalConfig {
    api: Arc<API>,
}

impl SynoCoreAppPortalConfig {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<AppPortalConfigResponse> {
        self.api
            .request(Request::new("SYNO.Core.AppPortal.Config", "get", 1))
            .await
    }

    pub async fn set(&self, params: Option<AppPortalConfigSetParams>) {
        let request = Request::new("SYNO.Core.AppPortal.Config", "set", 1);
        self.api
            .request(match params {
                Some(params) => request.params(params),
                None => request,
            })
            .await
            .unwrap()
    }
}
