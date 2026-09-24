use crate::resource::Resource;
use crate::resources::system::SystemResource;
use std::sync::Arc;

pub struct Client {
    pub system: SystemResource,
}

impl Client {
    pub fn new(base_url: impl Into<String>) -> Self {
        let resource = Arc::new(Resource::new(base_url.into()));

        Self {
            system: SystemResource::new(Arc::clone(&resource)),
        }
    }
}
