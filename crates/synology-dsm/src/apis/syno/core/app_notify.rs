use crate::api::API;
use crate::request::Request;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize)]
pub struct ViewAppNotifyParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
}

pub struct SynoCoreAppNotify {
    api: Arc<API>,
}

impl SynoCoreAppNotify {
    pub fn new(api: Arc<API>) -> Self {
        Self { api }
    }

    pub async fn view(&self, params: Option<ViewAppNotifyParams>) {
        let request = Request::new("SYNO.Core.AppNotify", "view", 1);
        self.api
            .request(match params {
                Some(params) => request.params(params),
                None => request,
            })
            .await
            .unwrap()
    }
}
