use reqwest::{Client, Error};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Resource {
    base_url: String,
    token: Option<String>,
    http: Client,
}

impl Resource {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            token: None,
            http: Client::new(),
        }
    }

    pub(crate) async fn get<Type, Params>(
        &self,
        path: impl Into<String>,
        params: Option<Params>,
    ) -> Result<Type, Error>
    where
        Type: DeserializeOwned,
        Params: Serialize,
    {
        let url = format!("{}{}", self.base_url, path.into());

        let mut request = self.http.get(&url);

        if let Some(token) = &self.token {
            request = request.bearer_auth(token);
        }

        let response = request
            .send()
            .await?
            .error_for_status()?
            .json::<Type>()
            .await?;

        Ok(response)
    }
}
