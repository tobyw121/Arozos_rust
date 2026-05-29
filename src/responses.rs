use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub ok: bool,
    pub data: Option<T>,
    pub message: Option<String>,
}

pub fn ok<T: Serialize>(data: T) -> impl IntoResponse {
    Json(ApiResponse { ok: true, data: Some(data), message: None })
}

pub fn message(status: StatusCode, text: impl Into<String>) -> impl IntoResponse {
    (status, Json(ApiResponse::<()> { ok: status.is_success(), data: None, message: Some(text.into()) }))
}
