use crate::api::API;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMonitorSettings {
    pub enable_history: bool,
}

pub struct SynoResourceMonitorSetting {
    api: Arc<API>,
}

impl SynoResourceMonitorSetting {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }
}
