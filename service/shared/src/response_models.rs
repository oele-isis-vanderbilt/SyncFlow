use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: u16,
    pub message: String,
}

// Map to HttpResponse
impl Into<HttpResponse> for Response {
    fn into(self) -> HttpResponse {
        HttpResponse::build(actix_web::http::StatusCode::from_u16(self.status as u16).unwrap())
            .json(&self)
    }
}
