use crate::api::API;
use crate::apis::syno::api::SynoApi;
use crate::apis::syno::core::SynoCore;
use crate::apis::syno::entry::SynoEntry;
use crate::apis::syno::secure_sign_in::SynoSecureSignIn;
use crate::apis::syno::security_advisor::SynoSecurityAdvisor;
use crate::apis::syno::synology_drive_share_sync::SYNOSynologyDriveShareSync;
use std::sync::Arc;

mod api;
mod core;
mod entry;
mod secure_sign_in;
mod security_advisor;
mod synology_drive_share_sync;
mod resource_monitor;

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

    pub fn secure_sign_in(&self) -> SynoSecureSignIn {
        SynoSecureSignIn::new(self.api.clone())
    }

    pub fn security_advisor(&self) -> SynoSecurityAdvisor {
        SynoSecurityAdvisor::new(self.api.clone())
    }

    pub fn synology_drive_share_sync(&self) -> SYNOSynologyDriveShareSync {
        SYNOSynologyDriveShareSync::new(self.api.clone())
    }
}
