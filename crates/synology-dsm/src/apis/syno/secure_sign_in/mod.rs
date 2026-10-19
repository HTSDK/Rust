mod amfa;
mod method;
mod package;

use crate::api::API;
use crate::apis::syno::secure_sign_in::amfa::SynoSecureSignInAmfa;
use crate::apis::syno::secure_sign_in::method::SynoSecureSignInMethod;
use std::sync::Arc;

pub struct SynoSecureSignIn {
    api: Arc<API>,
}

impl SynoSecureSignIn {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub fn amfa(&self) -> SynoSecureSignInAmfa {
        SynoSecureSignInAmfa::new(self.api.clone())
    }

    pub fn method(&self) -> SynoSecureSignInMethod {
        SynoSecureSignInMethod::new(self.api.clone())
    }
}
