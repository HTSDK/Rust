use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct ReverseProxyListResponse {
    entries: Vec<Value>,
}

pub struct SynoCoreAppPortalReverseProxy {
    api: Arc<API>,
}

impl SynoCoreAppPortalReverseProxy {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn list(&self) -> Output<ReverseProxyListResponse> {
        self.api
            .request(Request::new("SYNO.Core.AppPortal.ReverseProxy", "list", 1))
            .await
    }
}
