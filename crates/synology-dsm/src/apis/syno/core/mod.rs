mod app_notify;
mod app_portal;

use crate::api::API;
use crate::apis::syno::core::app_notify::SynoCoreAppNotify;
use crate::apis::syno::core::app_portal::SynoCoreAppPortal;
use std::sync::Arc;

pub struct SynoCore {
    api: Arc<API>,
}

impl SynoCore {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn app_notify(&self) -> SynoCoreAppNotify {
        SynoCoreAppNotify::new(self.api.clone())
    }

    pub fn app_portal(&self) -> SynoCoreAppPortal {
        SynoCoreAppPortal::new(self.api.clone())
    }
}
