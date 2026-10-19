mod conf;

use crate::api::API;
use crate::apis::syno::security_advisor::conf::SynoSecurityAdvisorConf;
use std::sync::Arc;

pub struct SynoSecurityAdvisor {
    api: Arc<API>,
}

impl SynoSecurityAdvisor {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn conf(&self) -> SynoSecurityAdvisorConf {
        SynoSecurityAdvisorConf::new(self.api.clone())
    }
}
