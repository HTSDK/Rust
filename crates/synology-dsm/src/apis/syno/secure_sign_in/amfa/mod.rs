mod policy;

use crate::api::API;
use crate::apis::syno::secure_sign_in::amfa::policy::SynoSecureSignInAmfaPolicy;
use std::sync::Arc;

pub struct SynoSecureSignInAmfa {
    api: Arc<API>,
}

impl SynoSecureSignInAmfa {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn policy(&self) -> SynoSecureSignInAmfaPolicy {
        SynoSecureSignInAmfaPolicy::new(self.api.clone())
    }
}
