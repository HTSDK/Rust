use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct MethodResponse {
    pub auth_type: Vec<Value>,
    pub mode: String,
    pub status: String,
    pub time: i64,
}

pub struct SynoSecureSignInMethod {
    api: Arc<API>,
}

impl SynoSecureSignInMethod {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<MethodResponse> {
        self.api
            .request(Request::new("SYNO.SecureSignIn.Method", "get", 1))
            .await
    }
}
