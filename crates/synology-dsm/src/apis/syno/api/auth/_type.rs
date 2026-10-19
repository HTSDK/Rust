use crate::api::API;
use crate::request::{Output, Request};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct AuthTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthTypeResponse {
    #[serde(rename = "type")]
    _type: String,
}

pub struct SynoApiAuthType {
    api: Arc<API>,
}

impl SynoApiAuthType {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self, params: Option<AuthTypeParams>) -> Output<Vec<AuthTypeResponse>> {
        let request = Request::new("SYNO.API.Auth.Type", "get", 1);
        self.api
            .request(match params {
                Some(params) => request.params(params),
                None => request,
            })
            .await
    }
}
