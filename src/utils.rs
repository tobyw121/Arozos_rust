use axum::{body::{to_bytes, Body}, http::Request};
use serde::{de::DeserializeOwned, Serialize};
use std::{collections::HashMap, fs, io, path::{Path, PathBuf}, process::Command};

pub fn file_exists(path: impl AsRef<Path>) -> bool { fs::metadata(path).is_ok() }

pub fn sanitize_relative_path(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|part| !part.is_empty() && *part != "." && *part != "..")
        .collect::<Vec<_>>()
        .join("/")
}

pub fn percent_decode(input: &str) -> String {
    let mut out = Vec::with_capacity(input.len());
    let mut bytes = input.as_bytes().iter().copied();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let a = bytes.next();
            let c = bytes.next();
            if let (Some(a), Some(c)) = (a, c) {
                let hex_bytes = [a, c];
                if let Ok(hex) = std::str::from_utf8(&hex_bytes) {
                    if let Ok(v) = u8::from_str_radix(hex, 16) {
                        out.push(v);
                        continue;
                    }
                }
                out.push(b'%'); out.push(a); out.push(c);
            } else {
                out.push(b'%');
                if let Some(a) = a { out.push(a); }
                if let Some(c) = c { out.push(c); }
            }
        } else if b == b'+' {
            out.push(b' ');
        } else {
            out.push(b);
        }
    }
    String::from_utf8_lossy(&out).to_string()
}

pub fn query_to_map(query: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for pair in query.split('&').filter(|p| !p.is_empty()) {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        map.insert(percent_decode(k), percent_decode(v));
    }
    map
}

pub async fn request_params(req: Request<Body>, max_body: usize) -> (HashMap<String, String>, Option<serde_json::Value>) {
    let (parts, body) = req.into_parts();
    let mut params = parts.uri.query().map(query_to_map).unwrap_or_default();
    let content_type = parts.headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("").to_ascii_lowercase();
    let mut json_body = None;
    if let Ok(bytes) = to_bytes(body, max_body).await {
        if !bytes.is_empty() {
            if content_type.contains("application/x-www-form-urlencoded") || content_type.contains("text/plain") || content_type.is_empty() {
                if let Ok(s) = std::str::from_utf8(&bytes) {
                    for (k, v) in query_to_map(s) { params.entry(k).or_insert(v); }
                }
            } else if content_type.contains("application/json") {
                if let Ok(v) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                    flatten_json_into_params(&mut params, &v);
                    json_body = Some(v);
                }
            }
        }
    }
    (params, json_body)
}

fn flatten_json_into_params(params: &mut HashMap<String, String>, v: &serde_json::Value) {
    if let serde_json::Value::Object(obj) = v {
        for (k, val) in obj {
            match val {
                serde_json::Value::String(s) => { params.entry(k.clone()).or_insert_with(|| s.clone()); }
                serde_json::Value::Bool(b) => { params.entry(k.clone()).or_insert_with(|| b.to_string()); }
                serde_json::Value::Number(n) => { params.entry(k.clone()).or_insert_with(|| n.to_string()); }
                _ => {}
            }
        }
    }
}

pub fn read_json_file<T: DeserializeOwned>(path: impl AsRef<Path>) -> io::Result<T> {
    let data = fs::read_to_string(path)?;
    serde_json::from_str(&data).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

pub fn write_json_file<T: Serialize>(path: impl AsRef<Path>, value: &T) -> io::Result<()> {
    if let Some(parent) = path.as_ref().parent() { fs::create_dir_all(parent)?; }
    let data = serde_json::to_vec_pretty(value).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    fs::write(path, data)
}

pub fn data_file(system_root: &str, name: &str) -> PathBuf {
    PathBuf::from(system_root).join("rust-port-data").join(name)
}

pub fn truthy(v: Option<&String>) -> bool {
    v.map(|s| matches!(s.to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on" | "enable" | "enabled")).unwrap_or(false)
}

pub fn run_command(program: &str, args: &[&str]) -> serde_json::Value {
    match Command::new(program).args(args).output() {
        Ok(out) => serde_json::json!({
            "ok": out.status.success(),
            "status": out.status.code(),
            "stdout": String::from_utf8_lossy(&out.stdout),
            "stderr": String::from_utf8_lossy(&out.stderr),
        }),
        Err(e) => serde_json::json!({"ok": false, "error": e.to_string(), "program": program}),
    }
}
