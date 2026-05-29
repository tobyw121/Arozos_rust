use axum::{
    body::{to_bytes, Body},
    extract::State,
    http::{header, HeaderMap, Request, StatusCode},
    response::{IntoResponse, Response},
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::{Path, PathBuf}, sync::Arc, time::UNIX_EPOCH};

use crate::{file_api, state::AppState, utils};

#[derive(Debug, Clone, Deserialize)]
struct ServiceConfig { enabled: bool }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDavRule {
    pub path: String,
    pub users: Vec<String>,
    pub groups: Vec<String>,
    pub read: bool,
    pub write: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebDavAccessConfig {
    pub allowed_paths: Vec<String>,
    pub denied_paths: Vec<String>,
    pub rules: Vec<WebDavRule>,
    pub allow_admin_all: bool,
    pub allow_browser_index: bool,
    pub require_auth: bool,
}

impl Default for WebDavAccessConfig {
    fn default() -> Self {
        Self {
            allowed_paths: vec!["user:/".to_string()],
            denied_paths: vec!["user:/../".to_string(), "host:/proc".to_string(), "host:/sys".to_string(), "host:/dev".to_string()],
            rules: vec![WebDavRule { path: "user:/".to_string(), users: Vec::new(), groups: vec!["administrator".to_string(), "user".to_string()], read: true, write: true }],
            allow_admin_all: true,
            allow_browser_index: true,
            require_auth: true,
        }
    }
}

#[derive(Debug, Clone)]
struct DavPrincipal { username: String, group: String, is_admin: bool }

#[derive(Debug, Clone)]
struct DavAccess { principal: Option<DavPrincipal>, vpath: String, can_read: bool, can_write: bool }

pub async fn handle(State(state): State<Arc<AppState>>, headers: HeaderMap, req: Request<Body>) -> Response {
    let method = req.method().as_str().to_ascii_uppercase();
    if method == "OPTIONS" {
        return (
            StatusCode::NO_CONTENT,
            [
                ("dav", "1, 2"),
                ("ms-author-via", "DAV"),
                ("allow", "OPTIONS, PROPFIND, GET, HEAD, PUT, DELETE, MKCOL, MOVE, COPY"),
            ],
        ).into_response();
    }

    if !webdav_enabled(&state) {
        return (StatusCode::SERVICE_UNAVAILABLE, "WebDAV is disabled in ArozOS settings").into_response();
    }

    let config = access_config(&state);
    let principal = authenticated_principal(&state, &headers);
    if config.require_auth && principal.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            [(header::WWW_AUTHENTICATE, "Basic realm=\"ArozOS WebDAV\"")],
            "authentication required",
        ).into_response();
    }

    let uri_path = req.uri().path().to_string();
    // Canonical mount is /webdav. There is intentionally no /webdave alias.
    let dav_path = uri_path.strip_prefix("/webdav").unwrap_or(&uri_path);
    let vpath = dav_virtual_path(dav_path);
    let access = evaluate_access(&config, principal, &vpath);
    let wants_write = matches!(method.as_str(), "PUT" | "DELETE" | "MKCOL" | "MOVE" | "COPY");
    if !access.can_read || (wants_write && !access.can_write) {
        return (StatusCode::FORBIDDEN, format!("WebDAV access denied for {vpath}")).into_response();
    }
    let target = match resolve_dav_path(&state, dav_path) {
        Ok(p) => p,
        Err(e) => return (StatusCode::BAD_REQUEST, e).into_response(),
    };

    match method.as_str() {
        "PROPFIND" => propfind(&state, &target, dav_path, &headers, &config, &access),
        "GET" => get_or_head(&target, false, &config, &access),
        "HEAD" => get_or_head(&target, true, &config, &access),
        "PUT" => put_file(&state, &target, req).await,
        "DELETE" => delete_path(&target),
        "MKCOL" => mkcol(&target),
        "MOVE" => copy_or_move(&state, &headers, &target, true, &config, &access),
        "COPY" => copy_or_move(&state, &headers, &target, false, &config, &access),
        _ => (StatusCode::METHOD_NOT_ALLOWED, "unsupported WebDAV method").into_response(),
    }
}

fn webdav_enabled(state: &AppState) -> bool {
    let file = utils::data_file(&state.config.system_root, "service_webdav.json");
    utils::read_json_file::<ServiceConfig>(&file).map(|c| c.enabled).unwrap_or(false)
}

fn access_config(state: &AppState) -> WebDavAccessConfig {
    utils::read_json_file(utils::data_file(&state.config.system_root, "webdav_access.json")).unwrap_or_default()
}

pub fn save_access_config(state: &AppState, cfg: &WebDavAccessConfig) -> Result<(), String> {
    utils::write_json_file(utils::data_file(&state.config.system_root, "webdav_access.json"), cfg).map_err(|e| e.to_string())
}

fn authenticated_principal(state: &AppState, headers: &HeaderMap) -> Option<DavPrincipal> {
    if let Some(user) = state.auth.current_user(headers) {
        return Some(DavPrincipal { username: user.username, group: user.group, is_admin: user.is_admin });
    }
    let auth = headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok())?;
    let encoded = auth.strip_prefix("Basic ")?;
    let decoded = general_purpose::STANDARD.decode(encoded.trim()).ok()?;
    let text = String::from_utf8(decoded).ok()?;
    let (user, pass) = text.split_once(':')?;
    if !state.auth.verify(user, pass) { return None; }
    state.auth.get_user(user).map(|u| DavPrincipal { username: u.username, group: u.group, is_admin: u.is_admin })
}

fn dav_virtual_path(path: &str) -> String {
    let mut clean = percent_decode(path);
    while clean.starts_with('/') { clean.remove(0); }
    if clean.is_empty() { "user:/".to_string() } else { format!("user:/{}", clean) }
}

fn normalize_vpath(s: &str) -> String { s.trim().replace('\\', "/").trim_end_matches('/').to_string() }
fn vpath_matches(path: &str, prefix: &str) -> bool {
    let p = normalize_vpath(path); let r = normalize_vpath(prefix);
    if r.is_empty() || r == "user:" || r == "user:/" { return p.starts_with("user:"); }
    p == r || p.starts_with(&(r + "/"))
}

fn evaluate_access(cfg: &WebDavAccessConfig, principal: Option<DavPrincipal>, vpath: &str) -> DavAccess {
    let mut can_read = cfg.allowed_paths.iter().any(|p| vpath_matches(vpath, p));
    let mut can_write = can_read;
    if cfg.denied_paths.iter().any(|p| vpath_matches(vpath, p)) { can_read = false; can_write = false; }
    if let Some(ref p) = principal {
        if cfg.allow_admin_all && p.is_admin { can_read = true; can_write = true; }
        for rule in &cfg.rules {
            if !vpath_matches(vpath, &rule.path) { continue; }
            let user_match = rule.users.is_empty() || rule.users.iter().any(|u| u == &p.username);
            let group_match = rule.groups.is_empty() || rule.groups.iter().any(|g| g == &p.group);
            if user_match && group_match { can_read = rule.read; can_write = rule.write; }
        }
    }
    DavAccess { principal, vpath: vpath.to_string(), can_read, can_write }
}

fn resolve_dav_path(state: &AppState, path: &str) -> Result<PathBuf, String> {
    let raw = dav_virtual_path(path);
    file_api::resolve(&state.config.root_directory, Some(&raw)).map_err(|e| e.to_string())
}

fn href_from_path(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
    if rel.is_empty() { "/webdav/".to_string() } else { format!("/webdav/{}/", rel.trim_end_matches('/')).replace("//", "/") }
}

fn propfind(state: &AppState, target: &Path, dav_path: &str, headers: &HeaderMap, cfg: &WebDavAccessConfig, access: &DavAccess) -> Response {
    let depth = headers.get("depth").and_then(|v| v.to_str().ok()).unwrap_or("1");
    let root = match file_api::ensure_root(&state.config.root_directory) {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };
    if !target.exists() { return (StatusCode::NOT_FOUND, "not found").into_response(); }
    let mut entries = vec![target.to_path_buf()];
    if depth != "0" && target.is_dir() {
        if let Ok(rd) = fs::read_dir(target) {
            for e in rd.flatten() { entries.push(e.path()); }
        }
    }
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<D:multistatus xmlns:D=\"DAV:\">\n");
    for path in entries {
        if let Ok(md) = fs::metadata(&path) {
            let is_dir = md.is_dir();
            let href = if path == target {
                let clean = percent_decode(dav_path).trim_matches('/').to_string();
                if clean.is_empty() { "/webdav/".to_string() } else if is_dir { format!("/webdav/{}/", xml_escape(&clean)) } else { format!("/webdav/{}", xml_escape(&clean)) }
            } else {
                xml_escape(&href_from_path(&root, &path))
            };
            let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| "ArozOS".into());
            let modified = md.modified().ok().and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
            let ctype = if is_dir { "httpd/unix-directory".to_string() } else { mime_guess::from_path(&path).first_or_octet_stream().essence_str().to_string() };
            let res_type = if is_dir { "<D:collection/>" } else { "" };
            xml.push_str(&format!(
                "<D:response><D:href>{}</D:href><D:propstat><D:prop><D:displayname>{}</D:displayname><D:getcontentlength>{}</D:getcontentlength><D:getcontenttype>{}</D:getcontenttype><D:getlastmodified>{}</D:getlastmodified><D:resourcetype>{}</D:resourcetype></D:prop><D:status>HTTP/1.1 200 OK</D:status></D:propstat></D:response>\n",
                href, xml_escape(&name), md.len(), xml_escape(&ctype), modified, res_type
            ));
        }
    }
    xml.push_str("</D:multistatus>");
    (StatusCode::from_u16(207).unwrap(), [(header::CONTENT_TYPE, "application/xml; charset=utf-8")], xml).into_response()
}

fn get_or_head(target: &Path, head: bool, cfg: &WebDavAccessConfig, access: &DavAccess) -> Response {
    if !target.exists() { return (StatusCode::NOT_FOUND, "not found").into_response(); }
    if target.is_dir() {
        let html = if head { String::new() } else if cfg.allow_browser_index { directory_index_html(target, access) } else { String::new() };
        return (StatusCode::OK, [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "no-store"),
        ], html).into_response();
    }
    let mime = mime_guess::from_path(target).first_or_octet_stream().essence_str().to_string();
    let disposition = format!("inline; filename=\"{}\"", target.file_name().and_then(|n| n.to_str()).unwrap_or("download"));
    if head { return (StatusCode::OK, [(header::CONTENT_TYPE, mime), (header::CONTENT_DISPOSITION, disposition)], Vec::<u8>::new()).into_response(); }
    match fs::read(target) {
        Ok(bytes) => (StatusCode::OK, [(header::CONTENT_TYPE, mime), (header::CONTENT_DISPOSITION, disposition)], bytes).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

fn directory_index_html(target: &Path, access: &DavAccess) -> String {
    let mut rows = Vec::new();
    let current_href = href_for_directory(target);
    if let Some(parent) = parent_href(&current_href) {
        rows.push(format!(
            r#"<tr><td class="icon">⬆</td><td><a href="{}">..</a></td><td>Folder</td><td class="num">-</td><td></td><td><a class="btn" href="{}">Open</a></td></tr>"#,
            parent, parent
        ));
    }
    if let Ok(rd) = fs::read_dir(target) {
        let mut entries: Vec<_> = rd.flatten().collect();
        entries.sort_by_key(|e| (!e.path().is_dir(), e.file_name().to_string_lossy().to_ascii_lowercase()));
        for e in entries {
            let path = e.path();
            let name = e.file_name().to_string_lossy().to_string();
            let href = format!("{}{}{}", current_href, url_encode_path_segment(&name), if path.is_dir() { "/" } else { "" });
            let meta = e.metadata().ok();
            let is_dir = meta.as_ref().map(|m| m.is_dir()).unwrap_or(false);
            let size = meta.as_ref().map(|m| if m.is_dir() { "-".into() } else { human_size(m.len()) }).unwrap_or_else(|| "-".into());
            let modified = meta.as_ref()
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs().to_string())
                .unwrap_or_default();
            let typ = if is_dir {
                "Folder".to_string()
            } else {
                mime_guess::from_path(&path).first_or_octet_stream().essence_str().to_string()
            };
            let icon = if is_dir { "📁" } else { "📄" };
            let action = if is_dir {
                format!(r#"<a class="btn" href="{}">Open</a>"#, href)
            } else {
                format!(r#"<a class="btn" href="{}" download>Download</a><a class="btn secondary" href="{}">Open</a>"#, href, href)
            };
            rows.push(format!(
                r#"<tr><td class="icon">{}</td><td><a href="{}">{}</a></td><td>{}</td><td class="num">{}</td><td>{}</td><td class="actions">{}</td></tr>"#,
                icon, href, xml_escape(&name), xml_escape(&typ), size, xml_escape(&modified), action
            ));
        }
    }
    let title = if current_href == "/webdav/" {
        "ArozOS WebDAV".to_string()
    } else {
        format!("ArozOS WebDAV - {}", xml_escape(&current_href))
    };
    format!(r#"<!doctype html>
<html><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{}</title>
<style>
:root{{--bg:#f7f8fb;--panel:#ffffff;--line:#d7dbe3;--text:#23272f;--muted:#6b7280;--brand:#2185d0;--brand2:#16ab39;}}
*{{box-sizing:border-box}} body{{margin:0;background:var(--bg);color:var(--text);font:14px/1.45 -apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,Arial,sans-serif}}
.topbar{{height:54px;background:#1b1c1d;color:#fff;display:flex;align-items:center;padding:0 22px;box-shadow:0 1px 6px rgba(0,0,0,.25)}}
.topbar .logo{{width:26px;height:26px;border-radius:6px;background:linear-gradient(135deg,#2185d0,#21ba45);margin-right:12px}} .topbar h1{{font-size:18px;margin:0;font-weight:600}}
.wrap{{max-width:1120px;margin:28px auto;padding:0 22px}} .crumb{{background:var(--panel);border:1px solid var(--line);border-radius:8px;padding:13px 16px;margin-bottom:16px;color:var(--muted)}}
.card{{background:var(--panel);border:1px solid var(--line);border-radius:10px;box-shadow:0 1px 3px rgba(0,0,0,.06);overflow:hidden}} table{{width:100%;border-collapse:collapse}} th,td{{padding:12px 14px;border-bottom:1px solid #edf0f5;text-align:left;vertical-align:middle}} th{{background:#f1f3f7;color:#4b5563;font-weight:600}} tr:hover td{{background:#f8fbff}} a{{color:#1678c2;text-decoration:none}} a:hover{{text-decoration:underline}} .icon{{width:44px;text-align:center;font-size:20px}} .num{{text-align:right;color:#4b5563;white-space:nowrap}} .actions{{white-space:nowrap;text-align:right}} .btn{{display:inline-block;border-radius:5px;background:var(--brand);color:white;padding:6px 10px;margin-left:6px;font-size:12px}} .btn:hover{{color:white;text-decoration:none;background:#1678c2}} .btn.secondary{{background:#767676}} .toolbar{{display:flex;justify-content:space-between;gap:12px;align-items:center;margin:0 0 14px}} .toolbar .hint{{color:var(--muted)}} .footer{{margin-top:14px;color:var(--muted);font-size:12px}}
@media(max-width:760px){{.hide-sm{{display:none}} th,td{{padding:9px 8px}} .actions{{display:block;text-align:left}}}}
</style></head><body>
<div class="topbar"><div class="logo"></div><h1>ArozOS WebDAV</h1></div>
<div class="wrap"><div class="toolbar"><div class="hint">Native Rust WebDAV directory index · user: {}</div><a class="btn secondary" href="/desktop.html">ArozOS Desktop</a></div>
<div class="crumb"><b>Location:</b> {} · <b>Access:</b> read={} write={}</div><div class="card"><table><thead><tr><th></th><th>Name</th><th class="hide-sm">Type</th><th>Size</th><th class="hide-sm">Modified Unix</th><th></th></tr></thead><tbody>{}</tbody></table></div>
<div class="footer">Use a WebDAV client for PROPFIND/PUT/MKCOL/MOVE/COPY. Browser view is a styled ArozOS-compatible index with download buttons.</div>
</div></body></html>"#, title, access.principal.as_ref().map(|p| p.username.as_str()).unwrap_or("anonymous"), xml_escape(&current_href), access.can_read, access.can_write, rows.join("\n"))
}

fn href_for_directory(path: &Path) -> String {
    let root = path.ancestors()
        .find(|p| p.file_name().and_then(|n| n.to_str()) == Some("files"))
        .unwrap_or(path);
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
    if rel.is_empty() {
        "/webdav/".into()
    } else {
        format!("/webdav/{}/", rel.trim_matches('/'))
    }
}
fn parent_href(current: &str) -> Option<String> {
    let trimmed = current.trim_end_matches('/');
    if trimmed == "/webdav" { return None; }
    let mut parts: Vec<&str> = trimmed.trim_start_matches("/webdav").trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    if parts.is_empty() { return None; }
    parts.pop();
    if parts.is_empty() { Some("/webdav/".into()) } else { Some(format!("/webdav/{}/", parts.join("/"))) }
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut v = bytes as f64;
    let mut unit = 0usize;
    while v >= 1024.0 && unit + 1 < UNITS.len() { v /= 1024.0; unit += 1; }
    if unit == 0 { format!("{} {}", bytes, UNITS[unit]) } else { format!("{:.1} {}", v, UNITS[unit]) }
}

fn url_encode_path_segment(input: &str) -> String {
    let mut out = String::new();
    for b in input.as_bytes() {
        match *b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(*b as char),
            b' ' => out.push_str("%20"),
            other => out.push_str(&format!("%{:02X}", other)),
        }
    }
    out
}

async fn put_file(_state: &AppState, target: &Path, req: Request<Body>) -> Response {
    if let Some(parent) = target.parent() { if let Err(e) = fs::create_dir_all(parent) { return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(); } }
    let body = req.into_body();
    let bytes = match to_bytes(body, 512 * 1024 * 1024).await {
        Ok(b) => b,
        Err(e) => return (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
    };
    match fs::write(target, &bytes) {
        Ok(()) => (StatusCode::CREATED, "created").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

fn delete_path(target: &Path) -> Response {
    if !target.exists() { return (StatusCode::NOT_FOUND, "not found").into_response(); }
    let result = if target.is_dir() { fs::remove_dir_all(target) } else { fs::remove_file(target) };
    match result { Ok(()) => (StatusCode::NO_CONTENT, "").into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
}

fn mkcol(target: &Path) -> Response {
    match fs::create_dir_all(target) { Ok(()) => (StatusCode::CREATED, "created").into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
}

fn copy_or_move(state: &AppState, headers: &HeaderMap, source: &Path, move_it: bool, cfg: &WebDavAccessConfig, access: &DavAccess) -> Response {
    let Some(dest) = headers.get("destination").and_then(|v| v.to_str().ok()) else { return (StatusCode::BAD_REQUEST, "missing Destination header").into_response(); };
    let path_part = dest.split("/webdav").nth(1).unwrap_or(dest);
    let dest_vpath = dav_virtual_path(path_part);
    let dest_access = evaluate_access(cfg, access.principal.clone(), &dest_vpath);
    if !dest_access.can_write { return (StatusCode::FORBIDDEN, format!("WebDAV destination denied for {dest_vpath}")).into_response(); }
    let dest_path = match resolve_dav_path(state, path_part) { Ok(p) => p, Err(e) => return (StatusCode::BAD_REQUEST, e).into_response() };
    if move_it {
        if let Some(parent) = dest_path.parent() { let _ = fs::create_dir_all(parent); }
        return match fs::rename(source, &dest_path) { Ok(()) => (StatusCode::CREATED, "moved").into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() };
    }
    let result = copy_recursive(source, &dest_path);
    match result { Ok(()) => (StatusCode::CREATED, "copied").into_response(), Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response() }
}

fn copy_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let e = entry?;
            copy_recursive(&e.path(), &dst.join(e.file_name()))?;
        }
        Ok(())
    } else {
        if let Some(parent) = dst.parent() { fs::create_dir_all(parent)?; }
        fs::copy(src, dst).map(|_| ())
    }
}

fn percent_decode(input: &str) -> String {
    let mut out = Vec::new();
    let bytes = input.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let (Some(h), Some(l)) = (hex(bytes[i + 1]), hex(bytes[i + 2])) {
                out.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).to_string()
}

fn hex(b: u8) -> Option<u8> {
    match b { b'0'..=b'9' => Some(b - b'0'), b'a'..=b'f' => Some(b - b'a' + 10), b'A'..=b'F' => Some(b - b'A' + 10), _ => None }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\"', "&quot;").replace('\'', "&apos;")
}
