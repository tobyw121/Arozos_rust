#![allow(dead_code, unused_imports)]

mod app_api;
mod auth;
mod config;
mod errors;
mod file_api;
mod http_api;
mod https_native;
mod legacy_api;
mod ported;
mod protocol_services;
mod responses;
mod service_adapter;
mod state;
mod system_info;
mod utils;
mod webdav_server;

use anyhow::Context;
use axum_server::tls_rustls::RustlsConfig;
use clap::Parser;
use config::Config;
use state::AppState;
use std::{net::SocketAddr, sync::Arc};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "arozos_rs=info,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::parse();
    if config.show_version {
        println!("AlpNAS Desktop {} Revision {} (Rust migration)", env!("CARGO_PKG_VERSION"), "0.2.025");
        return Ok(());
    }

    let state = Arc::new(AppState::new(config.clone()).await?);
    protocol_services::start_enabled_services(&state);
    let app = http_api::router::build_router(state.clone());
    let http_addr: SocketAddr = config.bind_addr().parse().with_context(|| format!("invalid bind address: {}", config.bind_addr()))?;

    if config.use_tls {
        https_native::ensure_certificate(&config).await?;
        let tls_config = RustlsConfig::from_pem_file(&config.tls_cert, &config.tls_key)
            .await
            .with_context(|| format!("failed to load TLS certificate {} and key {}", config.tls_cert, config.tls_key))?;
        let https_addr: SocketAddr = format!("{}:{}", config.listen_host, config.tls_listen_port)
            .parse()
            .with_context(|| format!("invalid HTTPS bind address {}:{}", config.listen_host, config.tls_listen_port))?;

        tracing::info!(%https_addr, cert=%config.tls_cert, key=%config.tls_key, auto_tls=config.auto_tls, "starting native HTTPS server");
        let mut https_task = tokio::spawn({
            let https_app = app.clone();
            async move {
                axum_server::bind_rustls(https_addr, tls_config)
                    .serve(https_app.into_make_service())
                    .await
            }
        });

        if config.disable_http {
            tokio::select! {
                res = &mut https_task => { res.context("HTTPS task join failed")??; }
                _ = shutdown_signal(state.clone()) => { https_task.abort(); }
            }
            return Ok(());
        }

        let http_app = if config.redirect_http_to_https {
            tracing::info!(%http_addr, https_port=config.tls_listen_port, "starting HTTP to HTTPS redirect server");
            https_native::redirect_router(config.tls_listen_port)
        } else {
            tracing::info!(%http_addr, "starting arozos-rs HTTP server alongside HTTPS");
            app
        };
        let listener = tokio::net::TcpListener::bind(http_addr).await?;
        let mut http_task = tokio::spawn(async move { axum::serve(listener, http_app).await });

        tokio::select! {
            res = &mut http_task => { res.context("HTTP task join failed")??; }
            res = &mut https_task => { res.context("HTTPS task join failed")??; }
            _ = shutdown_signal(state.clone()) => {
                http_task.abort();
                https_task.abort();
            }
        }
        return Ok(());
    }

    if config.disable_http {
        anyhow::bail!("--disable_http requires --tls/--https to be enabled");
    }

    tracing::info!(%http_addr, "starting arozos-rs HTTP server");
    let listener = tokio::net::TcpListener::bind(http_addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state))
        .await?;
    Ok(())
}

async fn shutdown_signal(state: Arc<AppState>) {
    let ctrl_c = async { signal::ctrl_c().await.expect("failed to install Ctrl+C handler"); };
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! { _ = ctrl_c => {}, _ = terminate => {}, }
    state.shutdown().await;
}
