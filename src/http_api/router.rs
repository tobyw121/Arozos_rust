use axum::{body::Body, extract::{State, Query, ws::{Message, WebSocket, WebSocketUpgrade}}, http::{HeaderMap, Request, StatusCode}, response::{IntoResponse, Redirect, Response}, routing::{any, get, post}, Json, Router};
use serde::Serialize;
use std::{collections::HashMap, sync::Arc, time::Instant};
use tower_http::trace::TraceLayer;

use crate::{app_api, auth, file_api, legacy_api, state::AppState, system_info, webdav_server};
use super::endpoints::{self, EndpointSpec};

#[derive(Debug, Serialize)]
struct StubResponse<'a> {
    ok: bool,
    path: &'a str,
    access: &'a str,
    source: &'a str,
    handler: &'a str,
    status: &'a str,
}

pub fn build_router(state: Arc<AppState>) -> Router {
    let mut app = Router::new()
        .route("/", get(root_handler))
        .route("/system/info", get(system_info::info))
        .route("/system/id/ping", any(system_info::ping))
        .route("/system/id/requestInfo", any(system_info::request_info))
        .route("/system/info/getArOZInfo", any(system_info::get_aroz_info))
        .route("/system/info/getCPUinfo", any(system_info::cpu_info))
        .route("/system/info/getRAMinfo", any(system_info::ram_info))
        .route("/system/info/getRuntimeInfo", any(system_info::runtime_info))
        .route("/system/info/getDriveStat", any(system_info::drive_stat))
        .route("/system/info/ifconfig", any(system_info::ifconfig))
        .route("/system/info/license", any(system_info::license))
        .route("/system/bootflags", any(system_info::boot_flags))
        .route("/system/setting/list", any(legacy_endpoint))
        .route("/system/auth/login", any(auth::login))
        .route("/api/auth/login", any(auth::login))
        .route("/system/auth/logout", any(auth::logout))
        .route("/system/auth/register", any(auth::register))
        .route("/system/auth/passwordPolicy", any(legacy_endpoint))
        .route("/system/auth/checkLogin", any(auth::check_login))
        .route("/system/file_system/listRoots", any(file_api::list_roots))
        .route("/system/file_system/listDrives", any(file_api::list_drives))
        .route("/system/file_system/listDir", any(file_api::list_dir))
        .route("/system/file_system/getProperties", any(file_api::get_properties))
        .route("/system/file_system/newItem", any(file_api::new_item))
        .route("/system/file_system/fileOpr", any(file_api::file_opr))
        .route("/system/file_system/search", any(file_api::search))
        .route("/system/file_system/download", any(file_api::download))
        .route("/system/file_system/downloadFile", any(file_api::download))
        .route("/system/file_system/upload", post(file_api::upload))
        .route("/system/file_system/lowmemUpload", post(file_api::lowmem_upload))
        .route("/system/file_system/listDirHash", any(file_api::list_dir_hash))
        .route("/system/file_system/validateFileOpr", any(file_api::validate_file_opr))
        .route("/system/file_system/listMounts", any(legacy_endpoint))
        .route("/system/file_system/hostRoots", any(legacy_endpoint))
        .route("/system/file_system/platformDrives", any(legacy_endpoint))
        .route("/system/network/webdav/accessConfig", any(legacy_endpoint))
        .route("/system/network/webdav/permissions", any(legacy_endpoint))
        .route("/system/network/webdav/pathRules", any(legacy_endpoint))
        .route("/system/network/webdav/addPath", any(legacy_endpoint))
        .route("/system/network/webdav/removePath", any(legacy_endpoint))
        .route("/system/network/webdav/addRule", any(legacy_endpoint))
        .route("/system/network/webdav/removeRule", any(legacy_endpoint))
        .route("/system/network/nginx/configPreview", any(legacy_endpoint))
        .route("/system/network/nginx/configPlan", any(legacy_endpoint))
        .route("/system/network/reverseproxy/nginx", any(legacy_endpoint))
        .route("/system/network/nginx/writeConfig", any(legacy_endpoint))
        .route("/system/network/https/status", any(legacy_endpoint))
        .route("/system/network/https/configPreview", any(legacy_endpoint))
        .route("/system/network/https/generateSelfSigned", any(legacy_endpoint))
        .route("/system/network/reverseproxy/native", any(legacy_endpoint))
        .route("/system/ajgi/interface", get(ajgi_interface_get).post(ajgi_interface_post))
        .route("/api/ajgi/interface", get(ajgi_interface_get).post(ajgi_interface_post))
        .route("/media", any(legacy_endpoint))
        .route("/media/", any(legacy_endpoint))
        .route("/media/download/", any(legacy_endpoint))
        .route("/media/getMime/", any(legacy_endpoint))
        .route("/media/transcode/", any(legacy_endpoint))
        .route("/media/*path", any(legacy_endpoint))
        .route("/webdav", any(webdav_server::handle))
        .route("/webdav/", any(webdav_server::handle))
        .route("/webdav/*path", any(webdav_server::handle));

    for endpoint in endpoints::ENDPOINTS {
        if matches!(endpoint.path, "/" | "/system/info" | "/system/auth/login" | "/api/auth/login" | "/system/auth/logout" | "/system/auth/checkLogin" | "/system/id/ping" | "/system/id/requestInfo" | "/system/info/getArOZInfo" | "/system/info/getCPUinfo" | "/system/info/getRAMinfo" | "/system/info/getRuntimeInfo" | "/system/info/getDriveStat" | "/system/info/ifconfig" | "/system/info/license" | "/system/bootflags" | "/system/setting/list" | "/system/file_system/listRoots" | "/system/file_system/listDrives" | "/system/file_system/listDir" | "/system/file_system/getProperties" | "/system/file_system/newItem" | "/system/file_system/fileOpr" | "/system/file_system/search" | "/system/file_system/download" | "/system/file_system/downloadFile" | "/system/file_system/upload" | "/system/file_system/lowmemUpload" | "/system/file_system/listDirHash" | "/system/file_system/validateFileOpr" | "/system/auth/register" | "/system/auth/passwordPolicy" | "/system/ajgi/interface" | "/api/ajgi/interface" | "/media" | "/media/" | "/media/download/" | "/media/getMime/" | "/media/transcode/") {
            continue;
        }
        app = app.route(endpoint.path, any(legacy_endpoint));
    }

    app
        .fallback(any(app_api::static_or_backend))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}

async fn root_handler(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    if state.auth.is_first_run() { return Redirect::temporary("/user.html").into_response(); }
    if state.auth.session_from_headers(&headers).is_none() { return Redirect::temporary("/login.html").into_response(); }
    Redirect::temporary("/desktop.html").into_response()
}

async fn ajgi_interface_get(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    ws: Option<WebSocketUpgrade>,
) -> Response {
    let script = params.get("script").cloned().unwrap_or_default();
    if script.contains("Speedtest/special/wspeedtest.js") {
        if let Some(ws) = ws {
            return ws.on_upgrade(speedtest_socket).into_response();
        }
        return (StatusCode::UPGRADE_REQUIRED, Json(serde_json::json!({
            "ok": false,
            "error": "websocket upgrade required",
            "script": script
        }))).into_response();
    }
    if !script.is_empty() {
        return app_api::handle_script(&state, &headers, &script, &params, None).await;
    }
    (StatusCode::OK, Json(serde_json::json!({
        "ok": true,
        "runtime": "rust",
        "endpoints": ["/system/ajgi/interface", "/api/ajgi/interface"],
        "apps": app_api::builtin_modules_json()
    }))).into_response()
}


async fn ajgi_interface_post(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    req: Request<Body>,
) -> Response {
    let (params, json_body) = crate::utils::request_params(req, 16 * 1024 * 1024).await;
    let script = params.get("script").cloned().unwrap_or_default();
    if script.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "ok": false,
            "error": "missing AGI script parameter"
        }))).into_response();
    }
    app_api::handle_script(&state, &headers, &script, &params, json_body).await
}

async fn speedtest_socket(mut socket: WebSocket) {
    let _ = socket.send(Message::Text("DWL/UPL?".into())).await;
    while let Some(Ok(msg)) = socket.recv().await {
        let Message::Text(text) = msg else { continue; };
        match text.as_str() {
            "DWL" => {
                let payload = "DATA:".to_string() + &"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".repeat(1024);
                let start = Instant::now();
                let mut sent = 0usize;
                for _ in 0..16usize {
                    if socket.send(Message::Text(payload.clone())).await.is_err() { return; }
                    sent += payload.len();
                }
                let secs = start.elapsed().as_secs_f64().max(0.001);
                let _ = socket.send(Message::Text(format!("TIME_DIFF={secs:.3}"))).await;
                let _ = socket.send(Message::Text(format!("TTL_SIZE={:.2} KB", sent as f64 / 1024.0))).await;
                let _ = socket.send(Message::Text(format!("TTL_TIME={secs:.3}s"))).await;
                let _ = socket.send(Message::Text(format!("TTL_BANDWIDTH={:.2} Mbps", (sent as f64 * 8.0) / secs / 1_000_000.0))).await;
                let _ = socket.close().await;
                return;
            }
            "UPL" => {
                let _ = socket.send(Message::Text("UPL".into())).await;
            }
            "PING" => {
                let now = chrono_like_millis();
                let _ = socket.send(Message::Text(format!("{now},{now}"))).await;
            }
            "stop" => {
                let _ = socket.send(Message::Text("Stopped.".into())).await;
                let _ = socket.close().await;
                return;
            }
            _ => {}
        }
    }
}

fn chrono_like_millis() -> u128 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis()
}

async fn legacy_endpoint(State(state): State<Arc<AppState>>, headers: HeaderMap, req: Request<Body>) -> Response {
    legacy_api::handle(State(state), headers, req).await
}
