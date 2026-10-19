use crate::api::API;
use crate::request::{Output, Request};
use serde_json::Value;
use std::sync::Arc;

pub struct SynoSecurityAdvisorConfLocation {
    api: Arc<API>,
}

impl SynoSecurityAdvisorConfLocation {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<Value> {
        self.api
            .request(Request::new("SYNO.SecurityAdvisor.Conf.Location", "get", 1))
            .await
    }
}
