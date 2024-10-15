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
    pub session_notification_exchange_name: Option<String>,
    pub session_notification_binding_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewSessionMessage {
    pub session_id: String,
    pub session_name: String,
}
