use crate::api::API;
use crate::request::{Output, Request};
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestParamsMode {
    Sequential,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestParamsCompound {
    pub api: String,
    pub method: String,
    pub version: u8,
    #[serde(flatten)]
    pub data: HashMap<String, Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestParams {
    pub stop_when_error: bool,
    pub mode: RequestParamsMode,
    pub compound: Vec<RequestParamsCompound>,
}

pub struct SynoEntryRequest {
    api: Arc<API>,
}

impl SynoEntryRequest {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn request(&self, params: RequestParams) -> Output<Value> {
        self.api
            .request(Request::new("SYNO.Entry.Request", "request", 1).params(params))
            .await
    }
}
