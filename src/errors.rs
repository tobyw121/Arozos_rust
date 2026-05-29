use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not found")]
    NotFound,
    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Serialize)]
struct ErrorBody<'a> { error: &'a str, message: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::NotImplemented(_) => StatusCode::NOT_IMPLEMENTED,
            AppError::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let body = Json(ErrorBody { error: status.canonical_reason().unwrap_or("error"), message: self.to_string() });
        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
