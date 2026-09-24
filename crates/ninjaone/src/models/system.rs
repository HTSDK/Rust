use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: u32,
    pub organization_id: u32,
    pub uid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub job_title: String
}