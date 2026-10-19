mod auth;
mod encryption;
mod info;

use crate::api::API;
use crate::apis::syno::api::auth::SynoApiAuth;
use crate::apis::syno::api::encryption::SynoApiEncryption;
use crate::apis::syno::api::info::SynoApiInfo;
use std::sync::Arc;

pub struct SynoApi {
    api: Arc<API>,
}

impl SynoApi {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn auth(&self) -> SynoApiAuth {
        SynoApiAuth::new(self.api.clone())
    }

    pub fn encryption(&self) -> SynoApiEncryption {
        SynoApiEncryption::new(self.api.clone())
    }

    pub fn info(&self) -> SynoApiInfo {
        SynoApiInfo::new(self.api.clone())
    }
}
