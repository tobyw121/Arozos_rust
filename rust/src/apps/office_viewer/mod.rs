//! App Module
use axum::{Router, routing::get, response::Html, Json};
use serde::Serialize;

pub const APP_NAME: &str = "App";
pub const APP_VERSION: &str = "1.0.0";
pub const APP_DESCRIPTION: &str = "ArozOS Application";

#[derive(Debug, Serialize)]
pub struct AppInfo { name: String, version: String, description: String }

pub async fn index() -> Html<String> {
    Html(format!(r#"<!DOCTYPE html><html><head><title>{} - arozOS</title></head><body><h1>{}</h1><p>{}</p></body></html>"#, APP_NAME, APP_NAME, APP_DESCRIPTION))
}

pub async fn api_info() -> Json<AppInfo> {
    Json(AppInfo { name: APP_NAME.to_string(), version: APP_VERSION.to_string(), description: APP_DESCRIPTION.to_string() })
}

pub fn routes() -> Router<crate::AppState> {
    Router::new().route("/", get(index)).route("/api/info", get(api_info))
}
