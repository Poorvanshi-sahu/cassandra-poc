use actix_web::HttpResponse;
use serde::Serialize;
use crate::models::ApiResponse;

pub fn success_response<T: Serialize>(message: &str, data: Option<T>) -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse {
        message: message.to_string(),
        success: true,
        data,
    })
}

pub fn error_response(message: &str, status_code: u16) -> HttpResponse {
    HttpResponse::build(actix_web::http::StatusCode::from_u16(status_code).unwrap()).json(ApiResponse::<()> {
        message: message.to_string(),
        success: false,
        data: None,
    })
}