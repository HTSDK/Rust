use crate::api::API;
use crate::request::{Output, Request};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
pub struct EncryptionInfo {
    #[serde(rename = "cipherkey")]
    pub cipher_key: String,
    #[serde(rename = "ciphertoken")]
    pub cipher_token: String,
    pub public_key: String,
    pub server_time: u32,
}

pub struct SynoApiEncryption {
    api: Arc<API>,
}

impl SynoApiEncryption {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn get_info(&self) -> Output<EncryptionInfo> {
        self.api
            .request(Request::new("SYNO.API.Encryption", "getinfo", 1))
            .await
    }
}
