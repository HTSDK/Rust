mod request;

use crate::api::API;
use crate::apis::syno::entry::request::SynoEntryRequest;
use std::sync::Arc;

pub struct SynoEntry {
    api: Arc<API>,
}

impl SynoEntry {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn request(&self) -> SynoEntryRequest {
        SynoEntryRequest::new(self.api.clone())
    }
}
