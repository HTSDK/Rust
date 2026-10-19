use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct AccessControlListResponse {
    entries: Vec<Value>,
}

pub struct SynoCoreAppPortalAccessControl {
    api: Arc<API>,
}

impl SynoCoreAppPortalAccessControl {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn list(&self) -> Output<AccessControlListResponse> {
        self.api
            .request(Request::new("SYNO.Core.AppPortal.AccessControl", "list", 1))
            .await
    }
}
