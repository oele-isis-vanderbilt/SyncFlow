use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceRegisterRequest {
    pub name: String,
    pub group: String,
    pub comments: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceResponse {
    pub id: String,
    pub name: String,
    pub group: String,
    pub comments: Option<String>,
    pub registered_at: usize,
    pub registered_by: i32,
    pub project_id: String,
}
