use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct AmfaPolicyResponse {
    pub _type: String,
    pub group_list: String,
    pub user_list: String,
}

pub struct SynoSecureSignInAmfaPolicy {
    api: Arc<API>,
}

impl SynoSecureSignInAmfaPolicy {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<AmfaPolicyResponse> {
        self.api
            .request(Request::new("SYNO.SecureSignIn.AMFA.Policy", "get", 1))
            .await
    }
}
