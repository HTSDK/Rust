use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub struct Request {
    pub api: String,
    pub method: String,
    pub version: u8,
    pub path: String,
    pub params: Option<HashMap<String, Value>>,
}

impl Request {
    pub fn new(api: impl Into<String>, method: impl Into<String>, version: u8) -> Self {
        Self {
            api: api.into(),
            method: method.into(),
            version,
            path: "entry.cgi".to_string(),
            params: None,
        }
    }

    pub fn params(mut self, key: impl Into<String>, value: Value) -> Self {
        let mut params = self.params.unwrap_or_else(|| HashMap::new());
        params.insert(key.into(), value);
        self.params = Some(params);
        self
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct RequestOutput<O> {
    pub success: bool,
    pub data: Option<O>,
    pub error: Option<Value>
}

pub type Output<O> = Result<O, std::io::Error>;
