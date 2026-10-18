use std::sync::Arc;
use crate::api::API;

mod connection;

pub struct SYNOSynologyDriveShareSync {
    api: Arc<API>
}

impl SYNOSynologyDriveShareSync {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }
}