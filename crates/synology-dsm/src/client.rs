use crate::api::API;
use crate::apis::syno::Syno;
use std::sync::Arc;

pub struct Client {
    api: Arc<API>,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            api: Arc::new(API::new(base_url.into())),
        }
    }

    pub fn set_session(&mut self, sid: Option<String>, syno_token: Option<String>) {
        Arc::get_mut(&mut self.api)
            .unwrap()
            .set_session(sid, syno_token)
    }

    pub fn has_session(&self) -> bool {
        self.api.has_session()
    }

    pub fn syno(&self) -> Syno {
        Syno::new(self.api.clone())
    }
}
