//! Startup-Initialisierung für Rust ArOZ
//! 
//! Entspricht den startup.go und register.go Dateien im Originalprojekt.

use std::sync::Arc;
use tokio::fs;
use tracing::{info, error};

use crate::error::{Result, ArozError};
use crate::main::Args;

/// Application State - hält alle globalen Komponenten
#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub database: Arc<crate::mod::database::Database>,
    pub user_handler: Arc<crate::mod::user::UserHandler>,
    pub module_handler: Arc<crate::mod::modules::ModuleHandler>,
    // pub auth_agent: Arc<AuthAgent>,
    // pub storage_manager: Arc<StorageManager>,
    // pub logger: Arc<SystemLogger>,
}

/// Konfiguration für die Anwendung
#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub tls_port: u16,
    pub use_tls: bool,
    pub tls_cert: String,
    pub tls_key: String,
    pub disable_http: bool,
    pub tmp_directory: String,
    pub max_upload_size: u64,
    pub enable_console: bool,
    pub disable_subservices: bool,
}

/// Initialisiert das gesamte System
pub async fn initialize_system(args: &Args) -> Result<Arc<AppState>> {
    info!("Initializing system components...");

    // Erstelle Konfiguration
    let config = AppConfig {
        host: args.host.clone(),
        port: args.port,
        tls_port: args.tls_port,
        use_tls: args.use_tls,
        tls_cert: args.tls_cert.clone(),
        tls_key: args.tls_key.clone(),
        disable_http: args.disable_http,
        tmp_directory: args.tmp_directory.clone(),
        max_upload_size: args.max_upload * 1024 * 1024, // MB zu Bytes
        enable_console: args.enable_console,
        disable_subservices: args.disable_subservices,
    };

    // Bereinige temporäres Verzeichnis
    cleanup_tmp_directory(&config.tmp_directory).await?;

    // Initialisiere Datenbank
    info!("Initializing database...");
    let database = Arc::new(crate::mod::database::Database::new("./system/sysdb").await?);

    // Initialisiere User Handler
    info!("Initializing user handler...");
    let user_handler = Arc::new(
        crate::mod::user::UserHandler::new(database.clone(), &config.tmp_directory).await?
    );

    // Initialisiere Module Handler
    info!("Initializing module handler...");
    let module_handler = Arc::new(
        crate::mod::modules::ModuleHandler::new(user_handler.clone(), &config.tmp_directory)
    );

    // Erstelle AppState
    let state = AppState {
        config,
        database,
        user_handler,
        module_handler,
    };

    // Führe Modul-Initialisierung durch
    info!("Running module initialization...");
    run_module_initialization(&state).await?;

    info!("System initialization complete!");
    Ok(Arc::new(state))
}

/// Bereinigt das temporäre Verzeichnis
async fn cleanup_tmp_directory(tmp_dir: &str) -> Result<()> {
    info!("Cleaning up tmp directory: {}", tmp_dir);
    
    match fs::remove_dir_all(tmp_dir).await {
        Ok(_) => {},
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                error!("Failed to remove tmp directory: {}", e);
            }
        }
    }
    
    fs::create_dir_all(tmp_dir).await.map_err(|e| {
        ArozError::FileSystem(format!("Failed to create tmp directory: {}", e))
    })?;
    
    Ok(())
}

/// Führt die Initialisierung aller Module durch
async fn run_module_initialization(state: &AppState) -> Result<()> {
    // Initialisiere Module-Service
    crate::mod::modules::module_service_init(state)?;
    
    // Initialisiere weitere Module hier
    // crate::mod::auth::auth_init(state)?;
    // crate::mod::storage::storage_init(state)?;
    // crate::mod::network::network_init(state)?;
    
    Ok(())
}

// TODO: Weitere Startup-Funktionen aus startup.go portieren:
// - RunStartup()
// - InitFTP()
// - InitNetworkServices()
// - RegisterSetting()
