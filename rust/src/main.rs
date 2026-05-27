//! Haupt-Einstiegspunkt für Rust ArOZ
//! 
//! Entspricht der main.go im Originalprojekt.

use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use clap::Parser;

mod startup;
mod router;

use crate::error::Result;

/// Startup-Flags und Kommandozeilen-Argumente
#[derive(Parser, Debug)]
#[command(name = "arozos")]
#[command(author = "tobychui (original), Rust Port Contributors")]
#[command(version = "1.0.0")]
#[command(about = "AlpNAS Desktop - Web-based Operating System", long_about = None)]
struct Args {
    /// Port für HTTP-Server
    #[arg(short = 'p', long, default_value_t = 8080)]
    port: u16,

    /// Port für HTTPS-Server
    #[arg(long, default_value_t = 8443)]
    tls_port: u16,

    /// Host für den Server
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// TLS aktivieren
    #[arg(long)]
    use_tls: bool,

    /// TLS-Zertifikat-Pfad
    #[arg(long, default_value = "./system/tls/cert.pem")]
    tls_cert: String,

    /// TLS-Key-Pfad
    #[arg(long, default_value = "./system/tls/key.pem")]
    tls_key: String,

    /// Nur HTTPS verwenden (HTTP deaktivieren)
    #[arg(long)]
    disable_http: bool,

    /// Temporäres Verzeichnis
    #[arg(long, default_value = "/tmp/arozos")]
    tmp_directory: String,

    /// Maximale Upload-Größe in MB
    #[arg(long, default_value_t = 500)]
    max_upload: u64,

    /// Version anzeigen
    #[arg(short = 'v', long)]
    version: bool,

    /// Console aktivieren
    #[arg(long)]
    enable_console: bool,

    /// Subservices deaktivieren
    #[arg(long)]
    disable_subservices: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialisiere Logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("rust_arozos=info".parse().unwrap()),
        )
        .init();

    // Parse Kommandozeilen-Argumente
    let args = Args::parse();

    if args.version {
        println!("AlpNAS Desktop 1.0.0");
        println!("Developed by tobychui and other co-developers");
        return Ok(());
    }

    info!("Starting AlpNAS Desktop...");
    info!("Configuration:");
    info!("  Host: {}", args.host);
    info!("  HTTP Port: {}", args.port);
    info!("  HTTPS Port: {}", args.tls_port);
    info!("  TLS enabled: {}", args.use_tls);
    info!("  Temp directory: {}", args.tmp_directory);

    // Initialisiere Systemkomponenten
    let app_state = startup::initialize_system(&args).await?;

    // Erstelle HTTP Router
    let router = router::create_router(app_state.clone());

    // Setup Close Handler für Graceful Shutdown
    let shutdown_handle = tokio::spawn(async move {
        match signal::ctrl_c().await {
            Ok(()) => {
                info!("Shutdown signal received");
                shutdown_sequence(app_state.clone()).await;
            }
            Err(err) => {
                error!("Unable to listen for shutdown signal: {}", err);
            }
        }
    });

    // Starte HTTP/HTTPS Server
    let server_handle = if args.use_tls {
        if !args.disable_http {
            // Starte HTTP Server im Hintergrund
            let http_addr = format!("{}:{}", args.host, args.port).parse().unwrap();
            info!("Standard (HTTP) Web server listening at {}", http_addr);
            
            let http_listener = tokio::net::TcpListener::bind(http_addr).await?;
            tokio::spawn(async move {
                axum::serve(http_listener, router.clone()).await.unwrap();
            });
        }

        // Starte HTTPS Server
        let https_addr = format!("{}:{}", args.host, args.tls_port).parse().unwrap();
        info!("Secure (HTTPS) Web server listening at {}", https_addr);
        
        // TODO: TLS Configuration hier einfügen
        // Für jetzt nur HTTP als Placeholder
        let https_listener = tokio::net::TcpListener::bind(https_addr).await?;
        axum::serve(https_listener, router).await
    } else {
        let addr = format!("{}:{}", args.host, args.port).parse().unwrap();
        info!("Web server listening at {}", addr);
        
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, router).await
    };

    // Warte auf Shutdown
    shutdown_handle.await?;
    
    match server_handle {
        Ok(_) => info!("Server stopped gracefully"),
        Err(e) => error!("Server error: {}", e),
    }

    Ok(())
}

/// Führt die Shutdown-Sequenz aus
async fn shutdown_sequence(state: Arc<startup::AppState>) {
    info!("Shutting down auth gateway...");
    // auth_agent.close().await;

    info!("Shutting down storage pools...");
    // close_all_storage_pools().await;

    info!("Shutting down logger...");
    // system_wide_logger.close().await;

    info!("Shutting down database...");
    // sysdb.close().await;

    info!("Shutting down network services...");
    // stop_network_services().await;

    info!("Cleaning up tmp folder...");
    // cleanup_tmp_folder(&state.config.tmp_directory).await;

    info!("Shutdown complete");
}
