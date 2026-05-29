use anyhow::{Context, Result};
use axum::{http::{HeaderMap, Uri, header::HOST}, response::{IntoResponse, Redirect}, routing::any, Router};
use rcgen::generate_simple_self_signed;
use serde::Serialize;
use std::{collections::BTreeSet, fs, net::{IpAddr, UdpSocket}, path::Path};

use crate::config::Config;

#[derive(Debug, Clone, Serialize)]
pub struct HttpsStatus {
    pub enabled: bool,
    pub auto_tls: bool,
    pub http_enabled: bool,
    pub redirect_http_to_https: bool,
    pub https_port: u16,
    pub cert_path: String,
    pub key_path: String,
    pub cert_exists: bool,
    pub key_exists: bool,
    pub provider: &'static str,
    pub note: &'static str,
}

pub fn status(config: &Config) -> HttpsStatus {
    HttpsStatus {
        enabled: config.use_tls,
        auto_tls: config.auto_tls,
        http_enabled: !config.disable_http,
        redirect_http_to_https: config.redirect_http_to_https,
        https_port: config.tls_listen_port,
        cert_path: config.tls_cert.clone(),
        key_path: config.tls_key.clone(),
        cert_exists: Path::new(&config.tls_cert).exists(),
        key_exists: Path::new(&config.tls_key).exists(),
        provider: if config.auto_tls { "native-self-signed" } else { "manual-pem" },
        note: "Native Rust HTTPS uses rustls. The automatic certificate is a local self-signed certificate; use a public CA certificate for browser-trusted Internet HTTPS.",
    }
}

pub async fn ensure_certificate(config: &Config) -> Result<()> {
    let cert_path = Path::new(&config.tls_cert);
    let key_path = Path::new(&config.tls_key);
    if cert_path.exists() && key_path.exists() {
        return Ok(());
    }
    if !config.auto_tls {
        anyhow::bail!(
            "TLS is enabled but certificate or key is missing. Pass --auto_tls to generate a local certificate, or pass --cert and --key. Missing cert_exists={} key_exists={}",
            cert_path.exists(),
            key_path.exists()
        );
    }
    generate_local_certificate(config)
}

pub fn generate_local_certificate(config: &Config) -> Result<()> {
    let mut names = BTreeSet::<String>::new();
    names.insert("localhost".to_string());
    names.insert("127.0.0.1".to_string());
    names.insert("::1".to_string());
    if !config.host_name.trim().is_empty() {
        names.insert(config.host_name.trim().to_string());
    }
    if !config.listen_host.trim().is_empty() && config.listen_host != "0.0.0.0" && config.listen_host != "::" {
        names.insert(config.listen_host.trim().to_string());
    }
    if let Some(ip) = best_lan_ip() {
        names.insert(ip.to_string());
    }
    for san in &config.tls_subject_alt_names {
        let san = san.trim();
        if !san.is_empty() {
            names.insert(san.to_string());
        }
    }
    let cert_names: Vec<String> = names.into_iter().collect();
    let cert = generate_simple_self_signed(cert_names)
        .context("failed to create local HTTPS certificate")?;
    if let Some(parent) = Path::new(&config.tls_cert).parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create TLS directory {}", parent.display()))?;
    }
    if let Some(parent) = Path::new(&config.tls_key).parent() {
        fs::create_dir_all(parent).with_context(|| format!("failed to create TLS key directory {}", parent.display()))?;
    }
    fs::write(&config.tls_cert, cert.serialize_pem()?)
        .with_context(|| format!("failed to write TLS certificate {}", config.tls_cert))?;
    fs::write(&config.tls_key, cert.serialize_private_key_pem())
        .with_context(|| format!("failed to write TLS key {}", config.tls_key))?;
    Ok(())
}

fn best_lan_ip() -> Option<IpAddr> {
    let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
    let _ = socket.connect("8.8.8.8:80");
    socket.local_addr().ok().map(|addr| addr.ip()).filter(|ip| !ip.is_loopback())
}

pub fn redirect_router(https_port: u16) -> Router {
    Router::new().fallback(any(move |headers: HeaderMap, uri: Uri| async move {
        let host = headers.get(HOST).and_then(|h| h.to_str().ok()).unwrap_or("localhost");
        let hostname = host.rsplit_once(':').map(|(name, _)| name).unwrap_or(host);
        let authority = if https_port == 443 {
            hostname.to_string()
        } else {
            format!("{}:{}", hostname, https_port)
        };
        let target = format!("https://{}{}", authority, uri);
        Redirect::temporary(&target).into_response()
    }))
}
