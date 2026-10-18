use crate::api::API;
use crate::apis::syno::api::SynoApi;
use crate::apis::syno::synology_drive_share_sync::SYNOSynologyDriveShareSync;
use std::sync::Arc;

mod api;
mod synology_drive_share_sync;

pub struct Syno {
    api: Arc<API>,
}

impl Syno {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn api(&self) -> SynoApi {
        SynoApi::new(self.api.clone())
    }

    pub fn synology_drive_share_sync(&self) -> SYNOSynologyDriveShareSync {
        SYNOSynologyDriveShareSync::new(self.api.clone())
    }
}
