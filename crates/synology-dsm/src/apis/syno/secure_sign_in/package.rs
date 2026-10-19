use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct PackageResponse {
    pub account_status: bool,
    pub enabled: bool,
}

pub struct SynoSecureSignInPackage {
    api: Arc<API>,
}

impl SynoSecureSignInPackage {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get(&self) -> Output<PackageResponse> {
        self.api
            .request(Request::new("SYNO.SecureSignIn.Package", "get", 1))
            .await
    }
}
