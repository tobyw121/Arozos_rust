
use axum::{extract::State, response::IntoResponse, Json};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use std::{fs, path::PathBuf, process::Command, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use crate::state::AppState;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub product: &'static str,
    pub migration: &'static str,
    pub version: &'static str,
    pub host_name: String,
    pub vendor: &'static str,
    pub endpoint_count: usize,
    pub go_file_count: usize,
}

pub async fn info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(SystemInfo {
        product: "AlpNAS Desktop / ArozOS",
        migration: "rust",
        version: env!("CARGO_PKG_VERSION"),
        host_name: state.config.host_name.clone(),
        vendor: "AlpNAS Project",
        endpoint_count: crate::http_api::endpoints::ENDPOINTS.len(),
        go_file_count: crate::ported::GO_FILE_COUNT,
    })
}

fn read_web_asset_base64(state: &AppState, rel: &str) -> String {
    let p = PathBuf::from(&state.config.web_root).join(rel);
    fs::read(p).map(|b| general_purpose::STANDARD.encode(b)).unwrap_or_default()
}

pub async fn get_aroz_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let vendor_icon = read_web_asset_base64(&state, "img/public/vendor_icon.png");
    Json(serde_json::json!({
        "HostName": &state.config.host_name,
        "DeviceVendor": "AlpNAS Project",
        "DeviceModel": "AlpNAS",
        "DeviceModelDesc": "Diskless Alpine NAS Platform",
        "DeviceUUID": &state.config.system_uuid,
        "UUID": &state.config.system_uuid,
        "BuildVersion": env!("CARGO_PKG_VERSION"),
        "InternalVersion": env!("CARGO_PKG_VERSION"),
        "Version": env!("CARGO_PKG_VERSION"),
        "ProductName": "AlpNAS",
        "Runtime": "rust",
        "VendorIcon": vendor_icon,
        "VendorLogo": vendor_icon
    }))
}

pub async fn ping() -> impl IntoResponse {
    Json(serde_json::json!({"ok": true, "pong": true, "service": "arozos-rs"}))
}

pub async fn request_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(serde_json::json!({
        "DeviceVendor": "AlpNAS Project",
        "DeviceModel": "ALPNAS",
        "DeviceModelDesc": "Diskless Alpine NAS Platform",
        "HostName": &state.config.host_name,
        "UUID": &state.config.system_uuid,
        "Version": env!("CARGO_PKG_VERSION"),
        "Runtime": "rust"
    }))
}

pub async fn boot_flags(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(serde_json::json!({
        "host": &state.config.listen_host,
        "port": state.config.listen_port,
        "tls": state.config.use_tls,
        "allow_upnp": state.config.allow_upnp,
        "allow_ssdp": state.config.allow_ssdp,
        "allow_mdns": state.config.allow_mdns,
        "gzip": state.config.enable_gzip,
        "root": &state.config.root_directory,
        "tmp": &state.config.tmp_directory,
        "dir_list": state.config.enable_dir_listing,
        "homepage": state.config.allow_homepage,
        "allow_cluster": state.config.allow_clustering,
        "allow_iot": state.config.allow_iot
    }))
}

pub async fn runtime_info(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let started = state.started_at.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let uptime = SystemTime::now().duration_since(state.started_at).map(|d| d.as_secs()).unwrap_or(0);
    Json(serde_json::json!({
        "StartupTime": started,
        "ContinuesRuntime": uptime,
        "started_at_unix": started,
        "uptime_seconds": uptime,
        "runtime": "rust"
    }))
}

pub async fn cpu_info() -> impl IntoResponse {
    let text = fs::read_to_string("/proc/cpuinfo").unwrap_or_default();
    let mut model = String::new();
    let mut freq = String::new();
    let mut instruction = String::new();
    let mut hardware = String::new();
    let mut revision = String::new();
    let mut cores = 0usize;
    for line in text.lines() {
        if line.starts_with("processor") { cores += 1; }
        let lower = line.to_ascii_lowercase();
        if model.is_empty() && lower.starts_with("model name") { model = line.split_once(':').map(|(_,v)| v.trim().to_string()).unwrap_or_default(); }
        if freq.is_empty() && lower.starts_with("cpu mhz") { freq = line.split_once(':').map(|(_,v)| v.trim().to_string()).unwrap_or_default(); }
        if instruction.is_empty() && (lower.starts_with("flags") || lower.starts_with("features")) { instruction = line.split_once(':').map(|(_,v)| v.trim().to_string()).unwrap_or_default(); }
        if hardware.is_empty() && lower.starts_with("hardware") { hardware = line.split_once(':').map(|(_,v)| v.trim().to_string()).unwrap_or_default(); }
        if revision.is_empty() && lower.starts_with("revision") { revision = line.split_once(':').map(|(_,v)| v.trim().to_string()).unwrap_or_default(); }
    }
    if model.is_empty() { model = std::env::consts::ARCH.to_string(); }
    if freq.is_empty() { freq = "0".into(); }
    if instruction.is_empty() { instruction = "generic".into(); }
    if hardware.is_empty() { hardware = format!("{} {}", std::env::consts::OS, std::env::consts::ARCH); }
    if revision.is_empty() { revision = env!("CARGO_PKG_VERSION").into(); }
    if cores == 0 { cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1); }
    Json(serde_json::json!({
        "Model": model,
        "Freq": freq,
        "Instruction": instruction,
        "Hardware": hardware,
        "Revision": revision,
        "Cores": cores,
        "cores": cores,
        "model": model,
        "source": if text.is_empty() {"std"} else {"/proc/cpuinfo"}
    }))
}

pub async fn ram_info() -> impl IntoResponse {
    let mut total_kb = 0u64;
    if let Ok(text) = fs::read_to_string("/proc/meminfo") {
        for line in text.lines() {
            if line.starts_with("MemTotal:") {
                total_kb = line.split_whitespace().nth(1).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0);
                break;
            }
        }
    }
    Json(serde_json::json!(total_kb * 1024))
}

pub async fn drive_stat() -> impl IntoResponse {
    let mut rows = Vec::new();
    if let Ok(out) = Command::new("df").args(["-kP"]).output() {
        let s = String::from_utf8_lossy(&out.stdout);
        for (idx,line) in s.lines().enumerate() {
            if idx == 0 { continue; }
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() >= 6 {
                let total = parts[1].parse::<u64>().unwrap_or(0) * 1024;
                let used = parts[2].parse::<u64>().unwrap_or(0) * 1024;
                let free = parts[3].parse::<u64>().unwrap_or(0) * 1024;
                rows.push(serde_json::json!({
                    "DriveLetter": parts[5],
                    "FileSystem": parts[0],
                    "TotalSpace": total,
                    "UsedSpace": used,
                    "FreeSpace": free,
                    "MountPoint": parts[5],
                    "filesystem": parts[0],
                    "blocks_kb": parts[1],
                    "used_kb": parts[2],
                    "available_kb": parts[3],
                    "capacity": parts[4],
                    "mounted_on": parts[5]
                }));
            }
        }
    }
    Json(serde_json::json!(rows))
}

pub async fn ifconfig() -> impl IntoResponse {
    let mut interfaces = Vec::new();
    if let Ok(dir) = fs::read_dir("/sys/class/net") {
        for entry in dir.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            let base = entry.path();
            let mac = fs::read_to_string(base.join("address")).unwrap_or_default().trim().to_string();
            let operstate = fs::read_to_string(base.join("operstate")).unwrap_or_default().trim().to_string();
            interfaces.push(format!("{} ({}, {})", name, mac, operstate));
        }
    }
    Json(serde_json::json!(interfaces))
}

pub async fn license() -> impl IntoResponse {
    let text = fs::read_to_string("LICENSE").or_else(|_| fs::read_to_string("resources/LICENSE")).unwrap_or_else(|_| "License file not found in runtime directory".to_string());
    Json(serde_json::json!({"license": text}))
}

pub async fn settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    Json(serde_json::json!({
        "host_name": &state.config.host_name,
        "web_root": &state.config.web_root,
        "system_root": &state.config.system_root,
        "root_directory": &state.config.root_directory,
        "features": {
            "hardware_management": state.config.allow_hardware_management,
            "power_management": state.config.allow_power_management,
            "homepage": state.config.allow_homepage,
            "public_registry": state.config.allow_public_registry,
            "clustering": state.config.allow_clustering,
            "iot": state.config.allow_iot
        }
    }))
}
