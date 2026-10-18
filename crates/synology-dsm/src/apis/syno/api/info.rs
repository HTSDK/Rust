use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum RequestFormat {
    Json,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultItem {
    pub min_version: u8,
    pub max_version: u8,
    pub path: String,
    pub request_format: Option<RequestFormat>,
}

pub type Result = HashMap<String, ResultItem>;

pub struct SynoApiInfo {
    api: Arc<API>,
}

impl SynoApiInfo {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn query(&self, apis: Option<&[&str]>) -> Output<Result> {
        let query = apis
            .map(|a| a.join(","))
            .unwrap_or_else(|| "all".to_string());
        self.api
            .request(
                Request::new("SYNO.API.Info", "query", 1).params("query", Value::String(query)),
            )
            .await
    }
}
