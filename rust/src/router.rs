//! Router-Konfiguration für Rust ArOZ
//! 
//! Entspricht der main.router.go im Originalprojekt.

use axum::{
    Router,
    routing::get,
    middleware,
};
use tower_http::services::ServeDir;
use std::sync::Arc;

use crate::startup::AppState;

/// Erstellt den Haupt-Router für die Anwendung
pub fn create_router(state: Arc<AppState>) -> Router {
    // Statische Dateien aus dem web/ Verzeichnis
    let static_files = ServeDir::new("./web");

    Router::new()
        // API Endpoints
        .route("/api/system/info", get(system_info_handler))
        .route("/api/modules/list", get(modules_list_handler))
        
        // Module Router
        .nest("/system/modules", create_module_router(state.clone()))
        
        // Auth Router
        .nest("/auth", create_auth_router(state.clone()))
        
        // Storage Router  
        .nest("/storage", create_storage_router(state.clone()))
        
        // Fallback auf statische Dateien
        .fallback_service(static_files)
        .with_state(state)
}

/// Erstellt den Router für Modul-Endpoints
fn create_module_router(state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/list", get(modules_list_handler))
        .route("/getDefault", get(module_get_default))
        .route("/getLaunchPara", get(module_get_launch_parameter))
        .route("/reload", post(module_reload))
        .route("/installViaZip", post(module_install_via_zip))
}

/// Erstellt den Router für Auth-Endpoints
fn create_auth_router(_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/register", post(register_handler))
}

/// Erstellt den Router für Storage-Endpoints
fn create_storage_router(_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/pools", get(storage_pools_handler))
        .route("/mount", post(storage_mount_handler))
        .route("/unmount", post(storage_unmount_handler))
}

// Handler-Funktionen

async fn system_info_handler() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    axum::Json(json!({
        "version": "1.0.0",
        "name": "AlpNAS Desktop",
        "vendor": "Rust Port"
    }))
}

async fn modules_list_handler(
    axum::extract::State(state): axum::extract::State<Arc<AppState>>,
) -> axum::Json<serde_json::Value> {
    use serde_json::json;
    
    let modules = state.module_handler.list_loaded_modules();
    axum::Json(json!({
        "modules": modules
    }))
}

async fn module_get_default() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    axum::Json(json!({
        "default": {}
    }))
}

async fn module_get_launch_parameter() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    axum::Json(json!({
        "parameters": {}
    }))
}

async fn module_reload() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn module_install_via_zip() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn login_handler() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    axum::Json(json!({
        "status": "ok"
    }))
}

async fn logout_handler() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn register_handler() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn storage_pools_handler() -> axum::Json<serde_json::Value> {
    use serde_json::json;
    axum::Json(json!({
        "pools": []
    }))
}

async fn storage_mount_handler() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

async fn storage_unmount_handler() -> axum::http::StatusCode {
    axum::http::StatusCode::OK
}

use axum::routing::post;
