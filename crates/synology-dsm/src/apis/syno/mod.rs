use crate::api::API;
use crate::apis::syno::api::SynoApi;
use crate::apis::syno::entry::SynoEntry;
use crate::apis::syno::synology_drive_share_sync::SYNOSynologyDriveShareSync;
use std::sync::Arc;
use crate::apis::syno::core::SynoCore;

mod api;
mod entry;
mod synology_drive_share_sync;
mod core;

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

    pub fn core(&self) -> SynoCore {
        SynoCore::new(self.api.clone())
    }

    pub fn entry(&self) -> SynoEntry {
        SynoEntry::new(self.api.clone())
    }

    pub fn synology_drive_share_sync(&self) -> SYNOSynologyDriveShareSync {
        SYNOSynologyDriveShareSync::new(self.api.clone())
    }
}
