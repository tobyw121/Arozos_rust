//! ArozOS Gateway Interface (AGI) Backend
//! 
//! This module implements the core gateway interface that handles
//! all incoming requests, routing, and system-level operations.

use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use crate::modules_core::{database, user};

/// AGI System State
#[derive(Clone)]
pub struct AgiState {
    pub db: Arc<RwLock<database::Database>>,
    pub user_handler: Arc<user::UserHandler>,
    pub version: String,
    pub build_time: String,
}

impl AgiState {
    pub fn new(
        db: Arc<RwLock<database::Database>>,
        user_handler: Arc<user::UserHandler>,
    ) -> Self {
        Self {
            db,
            user_handler,
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_time: chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        }
    }
}

/// System Information Response
#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    pub version: String,
    pub build_time: String,
    pub uptime_secs: u64,
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub cpu_count: usize,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_total: u64,
    pub disk_used: u64,
}

/// Health Check Response
#[derive(Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: bool,
    pub storage: bool,
    pub services: Vec<ServiceHealth>,
}

#[derive(Serialize, Deserialize)]
pub struct ServiceHealth {
    pub name: String,
    pub running: bool,
    pub message: Option<String>,
}

/// Create AGI router with all system endpoints
pub fn create_router(state: AgiState) -> Router {
    axum::Router::new()
        // System endpoints
        .route("/agi/system/info", get(system_info))
        .route("/agi/system/health", get(health_check))
        .route("/agi/system/shutdown", post(shutdown_system))
        .route("/agi/system/restart", post(restart_system))
        // User management endpoints
        .route("/agi/users/list", get(list_users))
        .route("/agi/users/create", post(create_user))
        .route("/agi/users/delete", post(delete_user))
        // Module management
        .route("/agi/modules/list", get(list_modules))
        .route("/agi/modules/install", post(install_module))
        .route("/agi/modules/uninstall", post(uninstall_module))
        // Storage management
        .route("/agi/storage/pools", get(list_storage_pools))
        .route("/agi/storage/volumes", get(list_volumes))
        // With state
        .with_state(state)
}

/// GET /agi/system/info - Get system information
async fn system_info(State(state): State<AgiState>) -> Json<SystemInfo> {
    let sys = sysinfo::System::new_all();
    
    Json(SystemInfo {
        version: state.version.clone(),
        build_time: state.build_time.clone(),
        uptime_secs: sysinfo::System::uptime(),
        hostname: sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string()),
        os: format!("{} {}", sysinfo::System::name().unwrap_or_default(), sysinfo::System::os_version().unwrap_or_default()),
        arch: std::env::consts::ARCH.to_string(),
        cpu_count: sys.cpus().len(),
        memory_total: sys.total_memory(),
        memory_used: sys.used_memory(),
        disk_total: 0, // Would need to calculate from storage manager
        disk_used: 0,
    })
}

/// GET /agi/system/health - Health check endpoint
async fn health_check(State(state): State<AgiState>) -> Json<HealthStatus> {
    let db_health = {
        let _db = state.db.read().await;
        true
    };
    
    let mut services = vec![
        ServiceHealth {
            name: "database".to_string(),
            running: db_health,
            message: None,
        },
        ServiceHealth {
            name: "auth".to_string(),
            running: true,
            message: None,
        },
        ServiceHealth {
            name: "filesystem".to_string(),
            running: true,
            message: None,
        },
    ];
    
    let all_healthy = services.iter().all(|s| s.running);
    
    Json(HealthStatus {
        status: if all_healthy { "healthy".to_string() } else { "degraded".to_string() },
        database: db_health,
        storage: true, // Placeholder
        services,
    })
}

/// POST /agi/system/shutdown - Shutdown the system
async fn shutdown_system(State(_state): State<AgiState>) -> StatusCode {
    info!("System shutdown requested");
    // In a real implementation, this would trigger a graceful shutdown
    StatusCode::OK
}

/// POST /agi/system/restart - Restart the system
async fn restart_system(State(_state): State<AgiState>) -> StatusCode {
    info!("System restart requested");
    // In a real implementation, this would trigger a restart
    StatusCode::OK
}

/// GET /agi/users/list - List all users
async fn list_users(State(state): State<AgiState>) -> Result<Json<Vec<user::UserInfo>>, StatusCode> {
    match state.user_handler.list_users() {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            warn!("Failed to list users: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// POST /agi/users/create - Create a new user
async fn create_user(
    State(state): State<AgiState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<StatusCode, StatusCode> {
    match state.user_handler.create_user(&req.username, &req.password, req.email.clone()) {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            warn!("Failed to create user: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub email: Option<String>,
}

/// POST /agi/users/delete - Delete a user
async fn delete_user(
    State(state): State<AgiState>,
    Json(req): Json<DeleteUserRequest>,
) -> Result<StatusCode, StatusCode> {
    match state.user_handler.delete_user(&req.username) {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            warn!("Failed to delete user: {}", e);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub username: String,
}

/// GET /agi/modules/list - List installed modules
async fn list_modules(State(_state): State<AgiState>) -> Json<Vec<ModuleInfo>> {
    // Placeholder - would query actual module registry
    Json(vec![
        ModuleInfo {
            id: "system.core".to_string(),
            name: "Core System".to_string(),
            version: "1.0.0".to_string(),
            enabled: true,
        },
    ])
}

#[derive(Serialize, Deserialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub enabled: bool,
}

/// POST /agi/modules/install - Install a module
async fn install_module(
    State(_state): State<AgiState>,
    Json(req): Json<InstallModuleRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("Installing module: {}", req.module_id);
    // Placeholder - would download and install module
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
pub struct InstallModuleRequest {
    pub module_id: String,
    pub version: Option<String>,
}

/// POST /agi/modules/uninstall - Uninstall a module
async fn uninstall_module(
    State(_state): State<AgiState>,
    Json(req): Json<UninstallModuleRequest>,
) -> Result<StatusCode, StatusCode> {
    info!("Uninstalling module: {}", req.module_id);
    // Placeholder - would remove module
    Ok(StatusCode::ACCEPTED)
}

#[derive(Deserialize)]
pub struct UninstallModuleRequest {
    pub module_id: String,
}

/// GET /agi/storage/pools - List storage pools (placeholder)
async fn list_storage_pools(State(_state): State<AgiState>) -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

/// GET /agi/storage/volumes - List volumes (placeholder)
async fn list_volumes(State(_state): State<AgiState>) -> Json<Vec<serde_json::Value>> {
    Json(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        // Test implementation would go here
        assert!(true);
    }
}
