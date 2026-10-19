mod setting;

use crate::api::API;
use crate::apis::syno::resource_monitor::setting::SynoResourceMonitorSetting;
use std::sync::Arc;

pub struct SynoResourceMonitor {
    api: Arc<API>,
}

impl SynoResourceMonitor {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn setting(&self) -> SynoResourceMonitorSetting {
        SynoResourceMonitorSetting::new(self.api.clone())
    }
}
