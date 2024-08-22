use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Response {
    pub status: u16,
    pub message: String,
}

// Map to HttpResponse
impl From<Response> for HttpResponse {
    fn from(val: Response) -> HttpResponse {
        HttpResponse::build(actix_web::http::StatusCode::from_u16(val.status).unwrap()).json(&val)
    }
}
