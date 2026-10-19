mod location;

use crate::api::API;
use crate::apis::syno::security_advisor::conf::location::SynoSecurityAdvisorConfLocation;
use std::sync::Arc;

pub struct SynoSecurityAdvisorConf {
    api: Arc<API>,
}

impl SynoSecurityAdvisorConf {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn location(&self) -> SynoSecurityAdvisorConfLocation {
        SynoSecurityAdvisorConfLocation::new(self.api.clone())
    }
}
