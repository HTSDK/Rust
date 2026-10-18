mod info;

use std::sync::Arc;
use crate::api::API;
use crate::apis::syno::api::info::SynoApiInfo;

pub struct SynoApi {
    api: Arc<API>
}

impl SynoApi {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn info(&self) -> SynoApiInfo {
        SynoApiInfo::new(self.api.clone())
    }
}