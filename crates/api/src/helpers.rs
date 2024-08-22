use actix_web::HttpResponse;
use shared::response_models::Response;

pub fn json_ok_response<T: serde::Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}

pub fn error_response(e: impl Into<Response>) -> HttpResponse {
    e.into().into()
}
