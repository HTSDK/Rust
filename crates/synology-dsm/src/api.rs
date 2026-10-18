use crate::request::{Output, Request, RequestOutput};
use reqwest::Url;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use std::io::Error;

pub struct API {
    base_url: String,
    http: reqwest::Client,
    sid: Option<String>,
    syno_token: Option<String>,
}

impl API {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            http: reqwest::Client::new(),
            sid: None,
            syno_token: None,
        }
    }

    pub fn set_session(&mut self, sid: Option<String>, syno_token: Option<String>) {
        self.sid = sid;
        self.syno_token = syno_token;
    }

    pub fn has_session(&self) -> bool {
        self.sid.is_some() && self.syno_token.is_some()
    }

    pub async fn request<O>(&self, request: Request) -> Output<O>
    where
        O: DeserializeOwned,
    {
        let url = Url::parse(&format!("{}/webapi/{}", self.base_url, request.path))
            .map_err(|e| Error::other(e.to_string()))?;

        let mut body: HashMap<String, Value> = HashMap::new();

        body.insert("api".to_string(), Value::String(request.api));
        body.insert("method".to_string(), Value::String(request.method));
        body.insert("version".to_string(), Value::Number(request.version.into()));

        if let Some(params) = request.params {
            if let Value::Object(map) = serde_json::to_value(params)? {
                for (key, value) in map {
                    match value {
                        Value::Null => {}
                        Value::String(string) => {
                            body.insert(key, Value::String(string));
                        }
                        other => {
                            body.insert(key, other);
                        }
                    }
                }
            }
        }

        if let Some(sid) = &self.sid {
            body.insert("_sid".to_string(), Value::String(sid.clone()));
        }

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        headers.insert(
            "Referer",
            HeaderValue::from_str(&self.base_url.as_str()).unwrap(),
        );

        if let Some(syno_token) = &self.syno_token {
            headers.insert("X-SYNO-TOKEN", HeaderValue::from_str(syno_token).unwrap());
        }

        let output = self
            .http
            .post(url)
            .form(&body)
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap()
            .json::<RequestOutput<O>>()
            .await
            .unwrap();

        if !output.success {
            return Err(Error::other(format!(
                "Request failed: {:?}",
                output.error.unwrap_or(Value::Null)
            )));
        }

        Ok(output.data.unwrap())
    }
}
