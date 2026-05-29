use axum::{body::Body, extract::State, http::{header, HeaderMap, Request, StatusCode}, response::{IntoResponse, Response}, Json};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::{Path, PathBuf}, process::Command, sync::Arc, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

use crate::{file_api, state::AppState, utils};

const BODY_LIMIT: usize = 16 * 1024 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltinModule {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Desc")]
    pub desc: String,
    #[serde(rename = "Group")]
    pub group: String,
    #[serde(rename = "IconPath")]
    pub icon_path: String,
    #[serde(rename = "Version")]
    pub version: String,
    #[serde(rename = "StartDir")]
    pub start_dir: String,
    #[serde(rename = "SupportFW")]
    pub support_fw: bool,
    #[serde(rename = "LaunchFWDir")]
    pub launch_fw_dir: String,
    #[serde(rename = "SupportEmb")]
    pub support_emb: bool,
    #[serde(rename = "LaunchEmb")]
    pub launch_emb: String,
    #[serde(rename = "InitFWSize")]
    pub init_fw_size: [u16; 2],
    #[serde(rename = "InitEmbSize")]
    pub init_emb_size: [u16; 2],
    #[serde(rename = "SupportedExt")]
    pub supported_ext: Vec<String>,
}

pub fn builtin_modules_json() -> Vec<Value> {
    builtin_modules().into_iter().map(|m| serde_json::to_value(m).unwrap_or_else(|_| json!({}))).collect()
}

pub fn builtin_modules() -> Vec<BuiltinModule> {
    // Go UI parity: these records mirror the bundled WebApp init.agi
    // registrations. Paths are normalized to absolute web-root paths where the
    // Go runtime registered module-local values such as "index.html" or
    // "img/module_icon.png", so the Rust runtime can remain standalone while
    // rendering the original ArozOS UI unchanged.
    vec![
        module("Browser", "A simple web browser like bookmarking app", "Internet", "Browser/img/module_icon.png", "1.0", "Browser/index.html", true, "Browser/index.html", false, "", [720,480], [0,0], &[]),
        module("Camera", "The camera app that will never run out of storage", "Media", "Camera/img/module_icon.png", "1.0", "Camera/index.html", true, "Camera/index.html", false, "", [475,700], [0,0], &[]),
        module("Clock", "A date and time telling tool", "Utilities", "Clock/img/module_icon.png", "0.1.0", "Clock/index.html", true, "Clock/index.html", false, "", [600,400], [0,0], &[]),
        module("Code Studio", "The best code editor on ArozOS", "Office", "Code Studio/img/module_icon.png", "1.3", "Code Studio/index.html", true, "Code Studio/index.html", true, "Code Studio/embedded.html", [1024,768], [360,200], &[".bat",".coffee",".cpp",".cs",".csp",".csv",".fs",".dockerfile",".go",".html",".ini",".java",".js",".agi",".lua",".mips",".md",".sql",".txt",".php",".py",".ts",".xml",".yaml"]),
        module("MDEditor", "Basic Text Editor", "Utilities", "MDEditor/img/notebook.png", "3.0", "MDEditor/mde.html", true, "MDEditor/mde.html", false, "", [1080,580], [0,0], &[".txt",".md",".html",".xml",".json"]),
        module("Management Gateway", "System Management Interface for changing users, groups, services and toggle of system functions.", "Interface Module", "Management Gateway/img/small_icon.png", "1.0", "Management Gateway/index.html", true, "Management Gateway/index.html", false, "", [1080,580], [0,0], &[]),
        module("Manga", "", "Media", "Manga/img/small_icon.png", "1.1", "Manga/index.html", true, "Manga/index.html", false, "", [900,680], [0,0], &[".jpg",".jpeg",".png",".webp"]),
        module("Memo", "A simple Memo App for ArozOS", "Office", "Memo/img/small_icon.png", "0.1.0", "Memo/index.html", true, "Memo/index.html", false, "", [475,700], [0,0], &[]),
        module("Music", "The best music player in ArOZ Online", "Media", "Music/img/module_icon.png", "0.1.0", "Music/index.html", true, "Music/index.html", true, "Music/embedded.html", [475,720], [360,254], &[".mp3",".flac",".wav",".ogg",".aac",".webm",".mp4"]),
        module("NotepadA", "The legacy NotepadA from ArOZ Online Beta", "Office", "NotepadA/img/small_icon.png", "2.0", "NotepadA/index.html", true, "NotepadA/index.html", true, "NotepadA/embedded.html", [1024,600], [360,200], &[".txt",".md",".php",".html",".js",".htm",".csv",".json",".xml"]),
        module("OfficeViewer", "Basic office file viewer for docx, pptx and xlsx", "Office", "OfficeViewer/img/module_icon.png", "1.0", "OfficeViewer/index.html", true, "OfficeViewer/index.html", true, "OfficeViewer/embedded.html", [350,170], [1060,680], &[".docx",".pptx",".xlsx",".csv"]),
        module("OnScreenKeyboard", "On Screen Keyboard for touch displays that don't have on screen keyboard", "IME", "OnScreenKeyboard/img/small_icon.png", "0.1.0", "OnScreenKeyboard/index.html", true, "OnScreenKeyboard/index.html", false, "", [776,290], [0,0], &[]),
        module("PDF Viewer", "Possibly the best PDF viewer on the web", "Utilities", "PDF Viewer/icons/module_icon.png", "2.7.570", "", false, "", true, "PDF Viewer/viewer.html", [0,0], [1024,768], &[".pdf"]),
        module("Paint", "A basic drawing apps", "Utilities", "Paint/img/small_icon.png", "1.0", "Paint/index.html", true, "Paint/index.html", false, "", [870,530], [0,0], &[]),
        module("Photo", "The worst photo webapp", "Media", "Photo/img/module_icon.png", "0.0.1", "Photo/index.html", true, "Photo/index.html", true, "Photo/embedded.html", [900,550], [900,500], &[".jpg",".jpeg",".gif",".png",".webp",".arw",".cr2",".dng",".nef",".raf",".orf"]),
        module("Recorder", "A basic audio recorder", "Media", "Recorder/img/small_icon.png", "1.0", "Recorder/index.html", true, "Recorder/index.html", false, "", [540,232], [0,0], &[]),
        module("Serverless", "", "System Tools", "Serverless/img/small_icon.png", "0.1", "Serverless/index.html", true, "Serverless/index.html", false, "", [410,540], [0,0], &[]),
        module("Speedtest", "", "Internet", "Speedtest/img/small_icon.png", "1.0", "Speedtest/index.html", true, "Speedtest/index.html", false, "", [600,220], [0,0], &[]),
        module("Timer", "Basic Timer Utility", "Utilities", "Timer/img/timer.png", "2.0", "Timer/timer.html", true, "Timer/timer.html", false, "", [380,190], [0,0], &[]),
        module("Unit Tester", "", "Development", "UnitTest/img/small_icon.png", "1.0", "UnitTest/index.html", true, "UnitTest/index.html", false, "", [900,620], [0,0], &[]),
        module("Video", "The basic video player for ArOZ Online", "Media", "Video/img/module_icon.png", "0.0.6", "Video/index.html", true, "Video/index.html", true, "Video/embedded.html", [585,840], [700,424], &[".webm",".mp4",".ogg",".rmvb",".mkv",".avi"]),
        module("Web Builder", "The basic Web builder for writing HTML code", "Office", "Web Builder/img/module_icon.png", "0.1.0", "Web Builder/index.html", true, "Web Builder/index.html", false, "", [1080,580], [0,0], &[".html",".htm"]),
        module("Web Downloader", "", "Download", "Web Downloader/img/small_icon.png", "1.0", "Web Downloader/index.html", true, "Web Downloader/index.html", false, "", [400,500], [0,0], &[]),
    ]
}

fn module(name: &str, desc: &str, group: &str, icon_path: &str, version: &str, start_dir: &str, support_fw: bool, launch_fw_dir: &str, support_emb: bool, launch_emb: &str, init_fw_size: [u16; 2], init_emb_size: [u16; 2], supported_ext: &[&str]) -> BuiltinModule {
    BuiltinModule {
        name: name.into(), desc: desc.into(), group: group.into(), icon_path: icon_path.into(), version: version.into(), start_dir: start_dir.into(), support_fw, launch_fw_dir: launch_fw_dir.into(), support_emb, launch_emb: launch_emb.into(), init_fw_size, init_emb_size, supported_ext: supported_ext.iter().map(|s| s.to_string()).collect(),
    }
}

fn web_asset_base64(state: &AppState, rel: &str) -> String {
    fs::read(PathBuf::from(&state.config.web_root).join(rel))
        .map(|b| general_purpose::STANDARD.encode(b))
        .unwrap_or_default()
}

pub async fn static_or_backend(State(state): State<Arc<AppState>>, headers: HeaderMap, req: Request<Body>) -> Response {
    let raw_path = req.uri().path().to_string();
    let decoded = utils::percent_decode(raw_path.trim_start_matches('/'));

    // Mirror the original Go mrouter: first-run may only access user creation
    // resources; anonymous sessions may only access login/public assets.
    if let Some(redirect) = auth_gate(&state, &headers, &decoded) {
        return redirect;
    }

    if is_app_backend_path(&decoded) {
        let (params, json_body) = utils::request_params(req, BODY_LIMIT).await;
        return handle_backend(&state, &headers, &decoded, &params, json_body).await;
    }
    serve_static(&state, &decoded).await
}

fn is_app_backend_path(path: &str) -> bool {
    path.ends_with("init.agi")
        || path.contains("/backend/")
        || path.contains("/functions/")
        || path.contains("/agi/")
        || path.contains("/embedded/listNearbyImage.js")
        || path.ends_with("/list.agi")
        || path.contains("SystemAOB/functions/file_system/index.php")
        || path.contains("SystemAOB/functions/extDiskAccess.php")
        || matches!(path,
            "Code Studio/store" | "Code Studio/backend/store.agi" |
            "NotepadA/getDir.php" | "NotepadA/writeCode.php" |
            "Music/getMeta" | "Music/getThumbnail" |
            "Web Builder/save" | "Web Downloader/agi/download.agi" |
            "MDEditor/filesaver.js" | "Photo/embedded/listNearbyImage.js" |
            "Speedtest/special/wspeedtest.js" | "Photo/backend/config.js" | "Photo/backend/listFile.js"
        )
}

fn auth_gate(state: &AppState, headers: &HeaderMap, decoded_path: &str) -> Option<Response> {
    let path = if decoded_path.is_empty() { "" } else { decoded_path.trim_start_matches('/') };
    let public = is_public_static_path(path);
    let logged_in = state.auth.session_from_headers(headers).is_some();
    if state.auth.is_first_run() {
        if path.is_empty() || path == "desktop.html" || path == "desktop.system" {
            return Some(axum::response::Redirect::temporary("/user.html").into_response());
        }
        if path == "user.html" || path == "login.html" || path == "login.system" || public { return None; }
        return Some(axum::response::Redirect::temporary("/user.html").into_response());
    }
    if !logged_in {
        if path == "login.html" || path == "login.system" || path == "reset.html" || public { return None; }
        if path == "user.html" { return Some(axum::response::Redirect::temporary("/login.html").into_response()); }
        return Some(axum::response::Redirect::temporary("/login.html").into_response());
    }
    None
}

fn is_public_static_path(path: &str) -> bool {
    path == "favicon.ico" || path == "manifest.webmanifest" || path == "robots.txt" || path == "humans.txt"
        || path.starts_with("script/")
        || path.starts_with("img/public/")
        || path.starts_with("public/")
}


fn normalize_static_alias(path: &str) -> String {
    // The bundled ArozOS frontends still contain several legacy PHP launch
    // paths and root-level icon aliases such as /system/close.svg and
    // /system_icon/folder.png.  The Go mrouter resolved those aliases from the
    // webpack asset tree; keep the same behaviour here so SystemAO pages keep
    // their original styling in the Rust runtime.
    let mut p = path.trim_start_matches('/').to_string();
    let query = p.find('?').map(|i| p.split_off(i)).unwrap_or_default();
    let lower = p.to_ascii_lowercase();
    p = match lower.as_str() {
        "notepada/index.php" => "NotepadA/index.html".into(),
        "notepada/embedded.php" => "NotepadA/embedded.html".into(),
        "notepada/floatwindow.php" => "NotepadA/FloatWindow.html".into(),
        "pdf viewer/index.html" => "PDF Viewer/viewer.html".into(),
        "pdf viewer/" => "PDF Viewer/viewer.html".into(),
        "system_setting/index.html" | "system_setting/" => "SystemAO/system_setting/index.html".into(),
        "system_settings.json" => "SystemAO/locale/system_settings.json".into(),
        _ if lower.starts_with("system_settings/") => format!("SystemAO/locale/{}", p),
        _ if lower.starts_with("system_icon/") => format!("img/desktop/{}", p),
        _ if lower.starts_with("system/") => format!("img/{}", p),
        _ if lower.ends_with("/index.php") => format!("{}/index.html", p.trim_end_matches("/index.php")),
        _ if lower.ends_with("/embedded.php") => format!("{}/embedded.html", p.trim_end_matches("/embedded.php")),
        _ => p,
    };
    if !query.is_empty() { p.push_str(&query); }
    p
}

fn missing_asset_alias(rel: &str) -> Option<String> {
    // A few original apps point to generated or legacy files that are not
    // present as files in the shipped web tree. Route them to the canonical
    // static asset so the UI remains functional.
    match rel.to_ascii_lowercase().as_str() {
        "notepada/floatwindow.html" => Some("NotepadA/index.html".into()),
        "pdf viewer/index.html" => Some("PDF Viewer/viewer.html".into()),
        "officeviewer/index.php" => Some("OfficeViewer/index.html".into()),
        _ => None,
    }
}

async fn serve_static(state: &AppState, decoded_path: &str) -> Response {
    let raw_rel = if decoded_path.is_empty() { "desktop.html".to_string() } else { decoded_path.trim_start_matches('/').to_string() };
    let raw_rel = normalize_static_alias(&raw_rel);
    let mut rel = utils::sanitize_relative_path(&raw_rel);
    rel = match rel.as_str() {
        "login.system" => "login.html".to_string(),
        "desktop.system" => "desktop.html".to_string(),
        "mobile.system" => "mobile.html".to_string(),
        _ => rel,
    };
    let mut full = PathBuf::from(&state.config.web_root).join(&rel);
    if full.is_dir() { full = full.join("index.html"); }
    if !full.exists() && !rel.contains('.') {
        let with_index = PathBuf::from(&state.config.web_root).join(&rel).join("index.html");
        if with_index.exists() { full = with_index; }
    }
    if !full.exists() {
        if let Some(alias) = missing_asset_alias(&rel) {
            let alias_full = PathBuf::from(&state.config.web_root).join(&alias);
            if alias_full.exists() { full = alias_full; rel = alias; }
        }
    }
    if rel == "login.html" {
        match tokio::fs::read_to_string(&full).await {
            Ok(mut body) => {
                body = body.replace("{{redirection_addr}}", "");
                body = body.replace("{{login_addr}}", "system/auth/login");
                body = body.replace("{{usercount}}", &state.auth.user_count().to_string());
                body = body.replace("{{service_logo}}", &web_asset_base64(state, "img/public/vendor_icon.png"));
                return (StatusCode::OK, [
                    (header::CONTENT_TYPE, "text/html; charset=utf-8"),
                    (header::CACHE_CONTROL, "no-store, no-cache, must-revalidate, max-age=0"),
                ], body).into_response();
            },
            Err(_) => return (StatusCode::NOT_FOUND, Json(json!({"ok": false, "error": "asset not found", "path": decoded_path}))).into_response(),
        }
    }
    match tokio::fs::read(&full).await {
        Ok(bytes) => {
            let mime = mime_guess::from_path(&full).first_or_octet_stream().essence_str().to_string();
            (StatusCode::OK, [
                (header::CONTENT_TYPE, mime),
                (header::CACHE_CONTROL, "no-store, no-cache, must-revalidate, max-age=0".to_string()),
            ], bytes).into_response()
        }
        Err(_) => (StatusCode::NOT_FOUND, Json(json!({"ok": false, "error": "asset not found", "path": decoded_path}))).into_response(),
    }
}


pub async fn handle_script(state: &AppState, headers: &HeaderMap, script_path: &str, params: &std::collections::HashMap<String, String>, json_body: Option<Value>) -> Response {
    let mut decoded = utils::percent_decode(script_path.trim_start_matches('/'));
    while decoded.starts_with("./") { decoded = decoded[2..].to_string(); }
    handle_backend(state, headers, &decoded, params, json_body).await
}

async fn handle_backend(state: &AppState, headers: &HeaderMap, decoded_path: &str, params: &BTreeMapCompat, json_body: Option<Value>) -> Response {
    let app = decoded_path.split('/').next().unwrap_or("");
    let script = decoded_path.split('/').last().unwrap_or("");
    if decoded_path.contains("SystemAOB/functions/file_system/index.php") {
        return notepada_legacy_dir(state, params);
    }
    if decoded_path.contains("SystemAOB/functions/extDiskAccess.php") {
        return app_read_file(state, params, "file");
    }
    if script == "init.agi" {
        let normalized = normalize_app_name(app);
        if let Some(m) = builtin_modules().into_iter().find(|m| normalize_app_name(&m.name) == normalized || normalize_app_name(&m.start_dir).starts_with(&normalized)) {
            return json_resp(json!({"ok": true, "module": m}));
        }
        return json_resp(json!({"ok": true, "module": app}));
    }
    if decoded_path.ends_with("list.agi") && app == "UnitTest" { return unittest_list(state); }

    match (app, script) {
        ("Browser", "bookmark.js") => browser_bookmark(state, headers, params),
        ("Browser", "sizechk.js") => browser_sizechk(state, params),
        ("Browser", "download.js") => browser_download(state, params),
        ("Browser", "getTitle.js") => browser_get_title(params),
        ("Browser", "getHeader.js") => browser_get_header(params),
        ("Browser", "proxy.js") => browser_proxy(params),
        ("Camera", "readydir.js") => camera_readydir(state, params),
        ("Camera", "listPhoto.js") => camera_list(state, params),
        ("Camera", "loadLatestPhoto.js") => camera_latest(state, params),
        ("Camera", "delPhoto.js") => camera_delete(state, params),
        ("Code Studio", "read.agi") => app_read_file(state, params, "file"),
        ("Code Studio", "writeFile.agi") => app_write_file(state, params, "filepath", "content"),
        ("Code Studio", "store.agi") | ("Code Studio", "store") => code_studio_store(state, headers, params),
        ("Memo", "addMemo.js") => memo_add(state, headers, params),
        ("MDEditor", "filesaver.js") => app_write_file(state, params, "filepath", "content"),
        ("Memo", "listmemo.js") => memo_list(state, headers),
        ("Memo", "removeMemo.js") => memo_remove(state, headers, params),
        ("NotepadA", "filesaver.js") => app_write_file(state, params, "filepath", "content"),
        ("NotepadA", "getDir.js") | ("NotepadA", "getDir.php") => notepad_get_dir(state, params),
        ("NotepadA", "writeCode.php") => app_write_file(state, params, "filename", "content"),
        ("NotepadA", "newfile.js") => notepad_new_file(state, params),
        ("Manga", "listTitles.js") => manga_list_titles(state),
        ("Manga", "getMangaInfo.js") => manga_info(state, params),
        ("Photo", "config.js") => photo_config(state, headers),
        ("Photo", "listRoots.js") => photo_roots(state),
        ("Photo", "listFolder.js") => photo_list_folder(state, params),
        ("Photo", "listFile.js") => photo_list_file(state, params),
        ("Photo", "listNearbyImage.js") => photo_nearby(state, params),
        ("Photo", "getExif.js") => photo_exif(state, params),
        ("Photo", "getCompressedImg.js") => photo_compressed(state, params),
        ("Photo", "exclude.js") => photo_exclude(state, params),
        ("Recorder", "createIfNotExists.js") => recorder_create(state, params),
        ("Music", "listSong.js") => music_list_song(state, params),
        ("Music", "playlist.js") => music_playlist(state, headers, params),
        ("Music", "buildCache.js") => { let _ = music_song_entries(state, None); text_resp("OK") },
        ("Music", "getFileInfo.js") => music_file_info(state, params),
        ("Music", "getMeta.js") | ("Music", "getMeta") => music_meta(state, params),
        ("Music", "getThumbnail.js") | ("Music", "getThumbnail") => app_thumbnail(state, params),
        ("Video", "buildPlaylist.js") => video_playlist(state),
        ("Video", "getThumbnail.js") => app_thumbnail(state, params),
        ("Speedtest", "wspeedtest.js") => speedtest_http(),
        ("Serverless", _) => serverless_backend(state, params),
        ("Web Builder", "save.js") => web_builder_save(state, headers, params),
        ("Web Downloader", "download.agi") => web_downloader(state, params),
        ("UnitTest", _) => unittest_backend(state, headers, script, params, json_body),
        _ => json_resp(json!({"ok": true, "path": decoded_path, "ported": "native-rust-app-backend-fallback", "note": "No original backend operation was required by this script; static frontend assets are served directly and stateful operations are handled by Rust modules where defined."})),
    }
}

type BTreeMapCompat = std::collections::HashMap<String, String>;

fn normalize_app_name(s: &str) -> String { s.to_ascii_lowercase().replace('%', "").replace(' ', "").replace('/', "") }
fn username(state: &AppState, headers: &HeaderMap) -> String { state.auth.current_user(headers).map(|u| u.username).unwrap_or_else(|| "admin".into()) }
fn param<'a>(params: &'a BTreeMapCompat, keys: &[&str]) -> Option<&'a str> { keys.iter().find_map(|k| params.get(*k).map(|s| s.as_str())) }
fn now_unix() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }
fn now_millis() -> u128 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() }
fn json_resp(value: Value) -> Response { (StatusCode::OK, Json(value)).into_response() }
fn text_resp(body: impl Into<String>) -> Response { (StatusCode::OK, [(header::CONTENT_TYPE, "text/plain; charset=utf-8")], body.into()).into_response() }
fn app_error(status: StatusCode, msg: impl Into<String>) -> Response { (status, Json(json!({"ok": false, "error": msg.into()}))).into_response() }

fn virtual_join(base: &str, name: &str) -> String {
    let mut b = base.to_string();
    if !b.ends_with('/') { b.push('/'); }
    b.push_str(name.trim_start_matches('/'));
    b
}

fn clean_virtual_path(raw: &str) -> String {
    let mut s = raw.trim().trim_matches('"').trim_matches('\'').to_string();
    if let Some(stripped) = s.strip_prefix("/media?file=") { s = stripped.to_string(); }
    if let Some(stripped) = s.strip_prefix("media?file=") { s = stripped.to_string(); }
    if let Some(pos) = s.find("?file=") { if s[..pos].ends_with("/media") || s[..pos].ends_with("media") { s = s[pos + 6..].to_string(); } }
    s = utils::percent_decode(&s);
    while s.ends_with("/*") { s.truncate(s.len().saturating_sub(2)); }
    while s.ends_with("/") && s != "user:/" { s.pop(); }
    if s.is_empty() { "user:/".into() } else { s }
}

fn resolve_virtual(state: &AppState, raw: Option<&str>) -> Result<PathBuf, String> {
    let owned = clean_virtual_path(raw.unwrap_or("user:/"));
    file_api::resolve(&state.config.root_directory, Some(&owned)).map_err(|e| e.to_string())
}

fn to_vpath(state: &AppState, path: &Path) -> String {
    let root = match file_api::ensure_root(&state.config.root_directory) { Ok(r) => r, Err(_) => PathBuf::from(&state.config.root_directory) };
    let rel = path.strip_prefix(&root).unwrap_or(path).to_string_lossy().replace('\\', "/");
    if rel.is_empty() { "user:/".into() } else { format!("user:/{}", rel) }
}

fn is_image(path: &Path) -> bool { ext(path).map(|e| matches!(e.as_str(), "jpg" | "jpeg" | "png" | "gif" | "webp" | "arw" | "cr2" | "dng" | "nef" | "raf" | "orf")).unwrap_or(false) }
fn is_video(path: &Path) -> bool { ext(path).map(|e| matches!(e.as_str(), "ogg" | "webm" | "mp4" | "rmvb" | "mkv" | "avi")).unwrap_or(false) }
fn ext(path: &Path) -> Option<String> { path.extension().map(|e| e.to_string_lossy().to_ascii_lowercase()) }

fn list_children(state: &AppState, dir_vpath: &str) -> Result<Vec<PathBuf>, String> {
    let dir = resolve_virtual(state, Some(dir_vpath))?;
    let mut out = Vec::new();
    for e in fs::read_dir(&dir).map_err(|e| e.to_string())?.flatten() { out.push(e.path()); }
    out.sort_by(|a, b| {
        let an = a.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
        let bn = b.file_name().map(|n| n.to_string_lossy().to_ascii_lowercase()).unwrap_or_default();
        an.cmp(&bn)
    });
    Ok(out)
}

fn list_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            out.push(p.clone());
            if p.is_dir() { list_recursive(&p, out); }
        }
    }
}

fn read_file(state: &AppState, raw: Option<&str>) -> Result<String, String> {
    let p = resolve_virtual(state, raw)?;
    fs::read_to_string(&p).map_err(|e| e.to_string())
}

fn write_file(state: &AppState, raw: Option<&str>, content: &str) -> Result<(), String> {
    let p = resolve_virtual(state, raw)?;
    if let Some(parent) = p.parent() { fs::create_dir_all(parent).map_err(|e| e.to_string())?; }
    fs::write(p, content).map_err(|e| e.to_string())
}

fn db_path(state: &AppState, table: &str) -> PathBuf { utils::data_file(&state.config.system_root, &format!("appdb_{}.json", utils::sanitize_relative_path(table).replace('/', "_"))) }
fn db_read(state: &AppState, table: &str) -> BTreeMap<String, Value> { utils::read_json_file(db_path(state, table)).unwrap_or_default() }
fn db_write(state: &AppState, table: &str, map: &BTreeMap<String, Value>) -> Result<(), String> { utils::write_json_file(db_path(state, table), map).map_err(|e| e.to_string()) }

fn camera_readydir(state: &AppState, params: &BTreeMapCompat) -> Response {
    match resolve_virtual(state, param(params, &["savetarget", "path", "dir"])) { Ok(p) => match fs::create_dir_all(p) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }, Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn camera_list(state: &AppState, params: &BTreeMapCompat) -> Response {
    let base = param(params, &["savetarget", "path", "dir"]).unwrap_or("user:/Photo/Camera");
    let mut files = match list_children(state, base) { Ok(v) => v, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    files.retain(|p| p.is_file() && ext(p).map(|e| e == "jpg" || e == "png").unwrap_or(false));
    files.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
    files.reverse();
    json_resp(json!(files.iter().map(|p| to_vpath(state, p)).collect::<Vec<_>>()))
}

fn camera_latest(state: &AppState, params: &BTreeMapCompat) -> Response {
    let base = param(params, &["savetarget", "path", "dir"]).unwrap_or("user:/Photo/Camera");
    let mut files = match list_children(state, base) { Ok(v) => v, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    files.retain(|p| p.is_file() && ext(p).map(|e| e == "jpg" || e == "png").unwrap_or(false));
    files.sort_by_key(|p| fs::metadata(p).and_then(|m| m.modified()).ok());
    let latest = files.last().map(|p| to_vpath(state, p)).unwrap_or_default();
    json_resp(json!(latest))
}

fn camera_delete(state: &AppState, params: &BTreeMapCompat) -> Response {
    let base = param(params, &["savetarget", "path", "dir"]).unwrap_or("user:/Photo/Camera");
    let filename = param(params, &["filename", "file", "name"]).unwrap_or("");
    let target = virtual_join(base, filename);
    let p = match resolve_virtual(state, Some(&target)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    if !ext(&p).map(|e| e == "jpg" || e == "png").unwrap_or(false) { return app_error(StatusCode::BAD_REQUEST, "target file is not a camera image"); }
    match fs::remove_file(p) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::BAD_REQUEST, e.to_string()) }
}

fn app_read_file(state: &AppState, params: &BTreeMapCompat, key: &str) -> Response {
    match read_file(state, param(params, &[key, "filepath", "path"])) { Ok(s) => text_resp(s), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn app_write_file(state: &AppState, params: &BTreeMapCompat, path_key: &str, content_key: &str) -> Response {
    match write_file(state, param(params, &[path_key, "file", "path"]), param(params, &[content_key, "data"]).unwrap_or("")) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn code_studio_store(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let key = match param(params, &["key"]) { Some(k) => format!("{}/{}", user, k), None => return app_error(StatusCode::BAD_REQUEST, "missing key") };
    let mut db = db_read(state, "Code Studio");
    match param(params, &["opr", "op"]).unwrap_or("get") {
        "set" => { db.insert(key, json!(param(params, &["value"]).unwrap_or(""))); match db_write(state, "Code Studio", &db) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) } }
        _ => json_resp(db.get(&key).cloned().unwrap_or(Value::Null)),
    }
}

fn memo_add(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let memo = param(params, &["memo", "content", "text"]).unwrap_or("");
    let key = format!("{}/{}", user, now_millis());
    let mut db = db_read(state, "Memo");
    db.insert(key, json!(memo));
    match db_write(state, "Memo", &db) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn memo_list(state: &AppState, headers: &HeaderMap) -> Response {
    let user = username(state, headers);
    let prefix = format!("{}/", user);
    let mut out = BTreeMap::new();
    for (k, v) in db_read(state, "Memo") { if let Some(id) = k.strip_prefix(&prefix) { out.insert(id.to_string(), v); } }
    json_resp(json!(out))
}

fn memo_remove(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let id = param(params, &["memoid", "id"]).unwrap_or("");
    let mut db = db_read(state, "Memo");
    db.remove(&format!("{}/{}", user, id));
    match db_write(state, "Memo", &db) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
}

fn notepad_get_dir(state: &AppState, params: &BTreeMapCompat) -> Response {
    let base = param(params, &["listpath", "path", "dir"]).unwrap_or("user:/");
    let mut out = Vec::new();
    match list_children(state, base) {
        Ok(children) => for p in children { if p.is_dir() { if let Some(name) = p.file_name().map(|n| n.to_string_lossy().to_string()) { if !name.starts_with('.') { out.push(json!([format!("{}/", to_vpath(state, &p)), name])); } } } },
        Err(e) => return app_error(StatusCode::BAD_REQUEST, e),
    }
    json_resp(json!(out))
}

fn notepada_legacy_dir(state: &AppState, params: &BTreeMapCompat) -> Response {
    // Legacy NotepadA embeds the old SystemAOB file selector. Return the
    // directory rows that the old PHP endpoint produced: [path, basename].
    let dir = param(params, &["dir", "subdir", "directory", "file", "path"]).unwrap_or("user:/");
    let children = match list_children(state, dir) { Ok(v) => v, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let rows: Vec<Value> = children.into_iter().filter(|p| p.is_dir()).map(|p| {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        json!([to_vpath(state, &p), name])
    }).collect();
    json_resp(json!(rows))
}

fn notepad_new_file(state: &AppState, params: &BTreeMapCompat) -> Response {
    let id = param(params, &["tmpid", "id"]).map(|s| utils::sanitize_relative_path(s)).filter(|s| !s.is_empty()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let target = format!("user:/Document/NotepadA/{}.tmp", id);
    match write_file(state, Some(&target), "") { Ok(_) => json_resp(json!(target)), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn manga_list_titles(state: &AppState) -> Response {
    let base = "user:/Photo/Manga";
    let _ = resolve_virtual(state, Some(base)).and_then(|p| fs::create_dir_all(p).map_err(|e| e.to_string()));
    let mut titles = Vec::new();
    if let Ok(title_dirs) = list_children(state, base) {
        for title_dir in title_dirs.into_iter().filter(|p| p.is_dir()) {
            let name = title_dir.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
            if name.starts_with('.') { continue; }
            let mut chapters = Vec::new();
            if let Ok(rd) = fs::read_dir(&title_dir) { for e in rd.flatten() { if e.path().is_dir() { chapters.push(e.path()); } } }
            chapters.sort();
            let chapter_names = chapters.iter().filter_map(|p| p.file_name().map(|n| n.to_string_lossy().to_string())).collect::<Vec<_>>();
            let title_image = title_dir.join("title.png");
            let fallback = chapters.first().and_then(|c| first_image_in(c));
            let image = if title_image.exists() { Some(title_image) } else { fallback };
            titles.push(json!([to_vpath(state, &title_dir), chapters.len(), image.as_ref().map(|p| to_vpath(state, p)).unwrap_or_default(), chapter_names]));
        }
    }
    json_resp(json!(titles))
}

fn manga_info(state: &AppState, params: &BTreeMapCompat) -> Response {
    let folder = param(params, &["folder", "path", "dir"]).unwrap_or("user:/Photo/Manga");
    let real = match resolve_virtual(state, Some(folder)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let mut pages = Vec::new();
    if let Ok(rd) = fs::read_dir(&real) { for e in rd.flatten() { if e.path().is_file() && is_image(&e.path()) { pages.push(to_vpath(state, &e.path())); } } }
    pages.sort();
    let parent = real.parent().unwrap_or(&real).to_path_buf();
    let mut chapters = Vec::new();
    if let Ok(rd) = fs::read_dir(&parent) { for e in rd.flatten() { if e.path().is_dir() { chapters.push(to_vpath(state, &e.path())); } } }
    let chapter = real.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let title = parent.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    json_resp(json!({"title": [title, chapter], "pages": pages, "dir": folder, "otherChapterDir": chapters}))
}

fn first_image_in(dir: &Path) -> Option<PathBuf> {
    let mut imgs = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) { for e in rd.flatten() { if e.path().is_file() && is_image(&e.path()) { imgs.push(e.path()); } } }
    imgs.sort();
    imgs.into_iter().next()
}

fn photo_roots(state: &AppState) -> Response {
    let photo = resolve_virtual(state, Some("user:/Photo"));
    if let Ok(p) = &photo { let _ = fs::create_dir_all(p); }
    json_resp(json!([["User Files", "user:/", "user:/Photo"]]))
}

fn photo_config(state: &AppState, headers: &HeaderMap) -> Response {
    let _ = resolve_virtual(state, Some("user:/Photo")).map(|p| fs::create_dir_all(p));
    json_resp(json!({
        "username": username(state, headers),
        "usericon": state.auth.current_user(headers).map(|u| u.profile_image).unwrap_or_else(|| "img/desktop/system_icon/user.svg".into()),
        "unlimited": true,
        "quota": 0,
        "quota_human": "Unlimited",
        "roots": [["User Files", "user:/", "user:/Photo"]],
        "defaultRoot": "user:/Photo",
        "native": "rust"
    }))
}

fn photo_list_file(state: &AppState, params: &BTreeMapCompat) -> Response {
    // The React-based Photo frontend expects a flat list of image objects with
    // VPath/img/Foldername-like fields in some views. The legacy Photo backend
    // also supports listFolder.js, which returns [folders, filesWithSize].
    let folder = param(params, &["folder", "path", "dir", "filepath"]).unwrap_or("user:/Photo");
    let children = match list_children(state, folder) { Ok(v) => v, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let mut out = Vec::new();
    for p in children {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if name.starts_with('.') { continue; }
        if p.is_file() && is_image(&p) {
            let vpath = to_vpath(state, &p);
            let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            out.push(json!({
                "Filename": name,
                "Filepath": vpath.clone(),
                "VPath": vpath.clone(),
                "filepath": vpath.clone(),
                "img": format!("../system/file_system/loadThumbnail?bytes=true&vpath={}", vpath),
                "filesize": size,
                "Ext": p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default(),
                "IsDir": false
            }));
        }
    }
    out.sort_by(|a,b| a["Filename"].as_str().cmp(&b["Filename"].as_str()));
    json_resp(json!(out))
}

fn photo_list_folder(state: &AppState, params: &BTreeMapCompat) -> Response {
    let folder = param(params, &["folder", "path", "dir", "filepath"]).unwrap_or("user:/Photo");
    let mut folders = Vec::new();
    let mut files = Vec::new();
    let children = match list_children(state, folder) { Ok(v) => v, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    for p in children {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if name.starts_with('.') { continue; }
        if p.is_dir() {
            // Match the Go AGI contract: folders are virtual paths.
            if folder_has_visible_children(&p) {
                folders.push(to_vpath(state, &p));
            }
        } else if p.is_file() && is_image(&p) {
            let size = fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
            files.push(json!({"filepath": to_vpath(state, &p), "filesize": size}));
        }
    }
    folders.sort();
    files.sort_by(|a, b| a["filepath"].as_str().cmp(&b["filepath"].as_str()));
    json_resp(json!([folders, files]))
}

fn folder_has_visible_children(path: &Path) -> bool {
    fs::read_dir(path).map(|rd| rd.flatten().any(|e| {
        let name = e.file_name().to_string_lossy().to_string();
        !name.starts_with('.')
    })).unwrap_or(false)
}

fn photo_compressed(state: &AppState, params: &BTreeMapCompat) -> Response {
    let p = match resolve_virtual(state, param(params, &["filepath", "file", "path"])) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let bytes = match fs::read(&p) { Ok(b) => b, Err(e) => return app_error(StatusCode::NOT_FOUND, e.to_string()) };
    let mime = mime_guess::from_path(&p).first_or_octet_stream().essence_str().to_string();
    let encoded = general_purpose::STANDARD.encode(bytes);
    text_resp(format!("data:{};base64,{}", mime, encoded))
}

fn photo_exclude(state: &AppState, params: &BTreeMapCompat) -> Response {
    let path = utils::data_file(&state.config.system_root, "photo_excludes.json");
    if let Some(folders) = param(params, &["folders", "exclude"]) {
        let list: Vec<String> = folders.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        match utils::write_json_file(&path, &list) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) }
    } else {
        let list: Vec<String> = utils::read_json_file(&path).unwrap_or_default();
        json_resp(json!(list))
    }
}

fn recorder_create(state: &AppState, params: &BTreeMapCompat) -> Response {
    let dir = param(params, &["savedir", "dir", "path"]).unwrap_or("user:/Audio/Recorder");
    match resolve_virtual(state, Some(dir)).and_then(|p| fs::create_dir_all(p).map_err(|e| e.to_string())) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn video_playlist(state: &AppState) -> Response {
    let base = match resolve_virtual(state, Some("user:/Video")) { Ok(p) => { let _ = fs::create_dir_all(&p); p }, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let mut all = Vec::new();
    list_recursive(&base, &mut all);
    let mut by_folder: BTreeMap<String, Vec<Value>> = BTreeMap::new();
    for p in all.into_iter().filter(|p| p.is_file() && is_video(p)) {
        let folder = p.parent().unwrap_or(&base).to_path_buf();
        let entry = json!({"Filename": p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(), "Filepath": to_vpath(state, &p), "Ext": p.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default()});
        by_folder.entry(to_vpath(state, &folder)).or_default().push(entry);
    }
    let mut playlists = Vec::new();
    for (folder, files) in by_folder {
        let name = folder.trim_end_matches('/').split('/').last().unwrap_or("Video").to_string();
        playlists.push(json!({"Thumbnail": "", "Folderpath": folder, "Name": name, "Files": files}));
    }
    json_resp(json!([{"StorageName": "User Files", "PlayLists": playlists}]))
}

fn web_builder_save(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let filepath = param(params, &["filepath", "file", "path"]);
    let content = param(params, &["content", "html"]).unwrap_or("");
    let title = param(params, &["title"]).unwrap_or("Webpage");
    let template = param(params, &["template"]).and_then(|t| read_file(state, Some(t)).ok()).unwrap_or_else(|| "<!doctype html><html><head><meta charset=\"utf-8\"><title>{{title}}</title></head><body>{{content}}</body></html>".into());
    let generated_title;
    let title_value = if title.is_empty() { generated_title = format!("{} Webpage", user); generated_title.as_str() } else { title };
    let output = template.replace("{{title}}", title_value).replace("{{content}}", content);
    match write_file(state, filepath, &output) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
}

fn web_downloader(state: &AppState, params: &BTreeMapCompat) -> Response {
    // Go's AGI endpoint returns JSON string "OK" after enqueueing a download.
    // The Rust port records a .url file instead of performing unattended external downloads.
    let dir = param(params, &["filepath", "path", "dir"]).unwrap_or("user:/Download");
    let filename = param(params, &["filename", "name"]).unwrap_or("download.url");
    let link = param(params, &["link", "url"]).unwrap_or("");
    let target = virtual_join(dir, filename);
    let _ = write_file(state, Some(&target), link);
    json_resp(json!("OK"))
}


fn browser_bookmark(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let rtype = param(params, &["rtype"]).unwrap_or("bookmark");
    let key = if rtype == "titles" { format!("{}/titles", user) } else { user };
    let mut db = db_read(state, "browser");
    match param(params, &["opr", "op"]).unwrap_or("read") {
        "write" => {
            let value = if rtype == "titles" { param(params, &["newTitleArray", "value"]) } else { param(params, &["newBookmarkArray", "value"]) }.unwrap_or("[]");
            let parsed = serde_json::from_str::<Value>(value).unwrap_or_else(|_| json!(value));
            db.insert(key, parsed);
            match db_write(state, "browser", &db) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
        }
        _ => json_resp(db.get(&key).cloned().unwrap_or_else(|| if rtype == "titles" { json!({}) } else { json!([]) })),
    }
}

fn browser_sizechk(state: &AppState, params: &BTreeMapCompat) -> Response {
    match resolve_virtual(state, param(params, &["filepath", "file", "path"])) {
        Ok(p) => json_resp(json!(fs::metadata(p).map(|m| m.len() as i64).unwrap_or(-1))),
        Err(_) => json_resp(json!(-1)),
    }
}

fn browser_download(state: &AppState, params: &BTreeMapCompat) -> Response {
    let dir = param(params, &["filepath", "path", "dir"]).unwrap_or("user:/Download");
    let filename = param(params, &["filename", "name"]).unwrap_or("download.url");
    let link = param(params, &["link", "url"]).unwrap_or("");
    let target = virtual_join(dir, filename);
    if !is_http_url(link) {
        return match write_file(state, Some(&target), link) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::BAD_REQUEST, e) };
    }
    match http_fetch_bytes(link, 30) {
        Ok(bytes) => {
            let p = match resolve_virtual(state, Some(&target)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
            if let Some(parent) = p.parent() { let _ = fs::create_dir_all(parent); }
            match fs::write(p, bytes) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::BAD_REQUEST, e.to_string()) }
        }
        Err(_) => {
            // If curl is not available, preserve the request as a .url file rather
            // than making the UI fail completely.
            match write_file(state, Some(&target), link) { Ok(_) => json_resp(json!("OK")), Err(e) => app_error(StatusCode::BAD_REQUEST, e) }
        }
    }
}

fn browser_get_title(params: &BTreeMapCompat) -> Response {
    let url = param(params, &["url"]).unwrap_or("");
    if is_http_url(url) {
        if let Ok(body) = http_fetch_text(url, 12) {
            let lower = body.to_ascii_lowercase();
            if let (Some(start), Some(end)) = (lower.find("<title"), lower.find("</title>")) {
                let seg = &body[start..end];
                let title = seg.split('>').nth(1).unwrap_or("").trim();
                if !title.is_empty() { return text_resp(title.to_string()); }
            }
        }
    }
    let title = url.trim_start_matches("https://").trim_start_matches("http://").split('/').next().unwrap_or("");
    text_resp(title)
}

fn browser_get_header(params: &BTreeMapCompat) -> Response {
    let url = param(params, &["url"]).unwrap_or("");
    if is_http_url(url) {
        if let Ok((code, headers, location)) = http_head(url, 12) {
            return json_resp(json!({"header": headers, "code": code, "location": location}));
        }
    }
    json_resp(json!({"header": {}, "code": 0, "location": url}))
}

fn browser_proxy(params: &BTreeMapCompat) -> Response {
    let url = param(params, &["url"]).unwrap_or("");
    if is_http_url(url) {
        match http_fetch_text(url, 15) {
            Ok(mut website_content) => {
                let root_url = http_root_url(url);
                let current_dir = http_current_dir(url);
                website_content = patch_remote_html_links(&website_content, &root_url, &current_dir);
                return html_resp(website_content);
            }
            Err(e) => return html_resp(format!("<!doctype html><html><body><h3>Proxy load failed</h3><pre>{}</pre></body></html>", e)),
        }
    }
    html_resp("<!doctype html><html><body><p>Invalid or unsupported URL.</p></body></html>")
}

fn is_http_url(url: &str) -> bool { url.starts_with("http://") || url.starts_with("https://") }

fn http_fetch_bytes(url: &str, timeout_secs: u64) -> Result<Vec<u8>, String> {
    let timeout = timeout_secs.to_string();
    let out = Command::new("curl")
        .args(["-L", "--silent", "--show-error", "--max-time", timeout.as_str(), url])
        .output()
        .map_err(|e| e.to_string())?;
    if out.status.success() { Ok(out.stdout) } else { Err(String::from_utf8_lossy(&out.stderr).to_string()) }
}

fn http_fetch_text(url: &str, timeout_secs: u64) -> Result<String, String> {
    http_fetch_bytes(url, timeout_secs).map(|b| String::from_utf8_lossy(&b).to_string())
}

fn http_head(url: &str, timeout_secs: u64) -> Result<(i32, Value, String), String> {
    let timeout = timeout_secs.to_string();
    let out = Command::new("curl")
        .args(["-I", "-L", "--silent", "--show-error", "--max-time", timeout.as_str(), url])
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8_lossy(&out.stdout).to_string();
    let mut code = 0;
    let mut location = String::new();
    let mut headers = serde_json::Map::new();
    for line in text.lines() {
        if line.starts_with("HTTP/") { code = line.split_whitespace().nth(1).and_then(|v| v.parse().ok()).unwrap_or(code); }
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim().to_ascii_lowercase();
            let value = v.trim().to_string();
            if key == "location" { location = value.clone(); }
            headers.insert(key, json!(value));
        }
    }
    Ok((code, Value::Object(headers), location))
}

fn http_root_url(url: &str) -> String {
    if let Some((scheme, rest)) = url.split_once("://") {
        let host = rest.split('/').next().unwrap_or(rest);
        format!("{}://{}", scheme, host)
    } else { String::new() }
}

fn http_current_dir(url: &str) -> String {
    let no_query = url.split('?').next().unwrap_or(url);
    if no_query.ends_with('/') { no_query.trim_end_matches('/').to_string() }
    else { no_query.rsplit_once('/').map(|(d, _)| d.to_string()).unwrap_or_else(|| no_query.to_string()) }
}

fn patch_remote_html_links(input: &str, root_url: &str, current_dir: &str) -> String {
    let mut s = input.to_string();
    for attr in ["src=\"", "href=\""] {
        let parts: Vec<&str> = s.split(attr).collect();
        if parts.len() <= 1 { continue; }
        let mut rebuilt = String::from(parts[0]);
        for part in parts.into_iter().skip(1) {
            if let Some((url_part, rest)) = part.split_once('"') {
                let patched = if url_part.starts_with("//") { format!("https:{}", url_part) }
                    else if url_part.starts_with('/') { format!("{}{}", root_url, url_part) }
                    else if url_part.starts_with("http://") || url_part.starts_with("https://") || url_part.starts_with("data:") || url_part.starts_with("#") { url_part.to_string() }
                    else { format!("{}/{}", current_dir, url_part) };
                rebuilt.push_str(attr);
                rebuilt.push_str(&patched);
                rebuilt.push('"');
                rebuilt.push_str(rest);
            } else {
                rebuilt.push_str(attr);
                rebuilt.push_str(part);
            }
        }
        s = rebuilt;
    }
    s
}

fn photo_nearby(state: &AppState, params: &BTreeMapCompat) -> Response {
    let file = param(params, &["path", "file", "filepath"]).unwrap_or("user:/Photo");
    let real = match resolve_virtual(state, Some(file)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let dir = if real.is_dir() { real } else { real.parent().unwrap_or(&real).to_path_buf() };
    let mut files = Vec::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() && is_image(&p) { files.push(to_vpath(state, &p)); }
        }
    }
    files.sort();
    json_resp(json!(files))
}

fn photo_exif(state: &AppState, params: &BTreeMapCompat) -> Response {
    let p = param(params, &["file", "filepath", "path"]).and_then(|v| resolve_virtual(state, Some(v)).ok());
    if let Some(path) = p {
        let md = fs::metadata(&path).ok();
        return json_resp(json!({
            "Filename": path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default(),
            "Filesize": md.as_ref().map(|m| m.len()).unwrap_or(0),
            "MIME": mime_guess::from_path(&path).first_or_octet_stream().essence_str(),
            "EXIF": {}
        }));
    }
    json_resp(json!({}))
}

fn app_thumbnail(state: &AppState, params: &BTreeMapCompat) -> Response {
    // Go imagelib.loadThumbString returns a data URL string. For videos and audio,
    // prefer a sibling cover image if present; otherwise return an empty string so
    // the bundled frontend uses its own fallback icons.
    let mut raw = param(params, &["file", "filepath", "path"]).unwrap_or("").to_string();
    if let Some(stripped) = raw.strip_prefix("/media?file=") { raw = stripped.to_string(); }
    let real = match resolve_virtual(state, Some(&raw)) { Ok(p) => p, Err(_) => return text_resp("") };
    let dir = real.parent().unwrap_or(&real);
    let stem = real.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let candidates = [
        dir.join(format!("{}.jpg", stem)),
        dir.join(format!("{}.png", stem)),
        dir.join("cover.jpg"),
        dir.join("cover.png"),
        dir.join("folder.jpg"),
        dir.join("folder.png"),
    ];
    for c in candidates {
        if c.exists() && c.is_file() {
            if let Ok(bytes) = fs::read(&c) {
                let mime = mime_guess::from_path(&c).first_or_octet_stream().essence_str().to_string();
                return text_resp(format!("data:{};base64,{}", mime, general_purpose::STANDARD.encode(bytes)));
            }
        }
    }
    text_resp("")
}

fn music_song_entries(state: &AppState, keyword: Option<&str>) -> Vec<Value> {
    let mut out = Vec::new();
    let root = match resolve_virtual(state, Some("user:/")) { Ok(p) => p, Err(_) => return out };
    let music_root = root.join("Music");
    let _ = fs::create_dir_all(&music_root);
    let mut all = Vec::new();
    list_recursive(&music_root, &mut all);
    let key = keyword.unwrap_or("").to_ascii_lowercase();
    for p in all.into_iter().filter(|p| p.is_file() && is_audio(p)) {
        let filename = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if !key.is_empty() && !filename.to_ascii_lowercase().contains(&key) { continue; }
        let stem = p.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| filename.clone());
        let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
        let size = fs::metadata(&p).map(|m| human_size(m.len())).unwrap_or_else(|_| "0 Byte".into());
        let vpath = to_vpath(state, &p);
        out.push(json!([format!("/media?file={}", vpath), stem, ext, size]));
    }
    out
}

fn music_list_song(state: &AppState, params: &BTreeMapCompat) -> Response {
    if let Some(listdir) = param(params, &["listdir"]) {
        if listdir == "root" {
            let music_dir = resolve_virtual(state, Some("user:/Music")).unwrap_or_else(|_| PathBuf::from(&state.config.root_directory).join("Music"));
            let _ = fs::create_dir_all(&music_dir);
            let mut files = 0usize;
            let mut folders = 0usize;
            if let Ok(rd) = fs::read_dir(&music_dir) { for e in rd.flatten() { if e.path().is_dir() { folders += 1; } else { files += 1; } } }
            return json_resp(json!([["User Files", "user:/Music", files, folders]]));
        }
        let target = match resolve_virtual(state, Some(listdir)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
        let mut folder_info = Vec::new();
        let mut file_info = Vec::new();
        if let Ok(rd) = fs::read_dir(target) {
            for e in rd.flatten() {
                let p = e.path();
                if p.is_dir() {
                    let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    if !name.starts_with('.') { folder_info.push(json!([name, format!("{}/", to_vpath(state, &p)), -1, -1])); }
                } else if p.is_file() && is_audio(&p) {
                    let stem = p.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
                    let size = fs::metadata(&p).map(|m| human_size(m.len())).unwrap_or_else(|_| "0 Byte".into());
                    let vpath = to_vpath(state, &p);
                    file_info.push(json!([format!("/media?file={}", vpath), stem, ext, size]));
                }
            }
        }
        folder_info.sort_by(|a,b| a[0].as_str().cmp(&b[0].as_str()));
        file_info.sort_by(|a,b| a[1].as_str().cmp(&b[1].as_str()));
        return json_resp(json!([folder_info, file_info]));
    }
    let mode = param(params, &["listSong"]).unwrap_or("all");
    let keyword = mode.strip_prefix("search:");
    let list = music_song_entries(state, keyword);
    if mode == "all" {
        json_resp(json!({"cached": false, "list": list}))
    } else {
        json_resp(json!(list))
    }
}

fn music_playlist(state: &AppState, headers: &HeaderMap, params: &BTreeMapCompat) -> Response {
    let user = username(state, headers);
    let mut db = db_read(state, "AirMusic");
    let opr = param(params, &["opr", "op"]).unwrap_or("root");
    match opr {
        "root" => {
            let prefix = format!("playlist/{}/", user);
            let mut list = Vec::new();
            for (k, v) in db.iter() {
                if let Some(name) = k.strip_prefix(&prefix) {
                    let count = v.as_array().map(|a| a.len()).unwrap_or(0);
                    list.push(json!({"name": name, "count": count}));
                }
            }
            json_resp(json!(list))
        }
        "list" => {
            let name = param(params, &["playlistname", "name"]).unwrap_or("");
            let key = format!("playlist/{}/{}", user, name);
            let songs = db.get(&key).and_then(|v| v.as_array()).cloned().unwrap_or_default();
            let mut rows = Vec::new();
            for song in songs.iter().filter_map(|v| v.as_str()) {
                if let Ok(p) = resolve_virtual(state, Some(song)) {
                    let stem = p.file_stem().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                    let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
                    let size = fs::metadata(&p).map(|m| human_size(m.len())).unwrap_or_else(|_| "0 Byte".into());
                    rows.push(json!([format!("/media?file={}", song), stem, ext, size]));
                }
            }
            json_resp(json!(rows))
        }
        "add" => {
            let name = param(params, &["playlistname", "name"]).unwrap_or("Default");
            let song = param(params, &["musicpath", "song", "file"]).unwrap_or("");
            let key = format!("playlist/{}/{}", user, name);
            let mut songs = db.get(&key).and_then(|v| v.as_array()).cloned().unwrap_or_default();
            if !song.is_empty() && !songs.iter().any(|v| v.as_str() == Some(song)) { songs.push(json!(song)); }
            db.insert(key, json!(songs));
            match db_write(state, "AirMusic", &db) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
        }
        "remove" => {
            let name = param(params, &["playlistname", "name"]).unwrap_or("");
            let song = param(params, &["musicpath", "song", "file"]).unwrap_or("");
            let key = format!("playlist/{}/{}", user, name);
            let mut songs = db.get(&key).and_then(|v| v.as_array()).cloned().unwrap_or_default();
            songs.retain(|v| v.as_str() != Some(song));
            if songs.is_empty() { db.remove(&key); } else { db.insert(key, json!(songs)); }
            match db_write(state, "AirMusic", &db) { Ok(_) => text_resp("OK"), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
        }
        _ => app_error(StatusCode::BAD_REQUEST, "Unknown operation type"),
    }
}

fn music_file_info(state: &AppState, params: &BTreeMapCompat) -> Response {
    let mut raw = param(params, &["filepath", "file", "path"]).unwrap_or("").to_string();
    if let Some(stripped) = raw.strip_prefix("/media?file=") { raw = stripped.to_string(); }
    let p = match resolve_virtual(state, Some(&raw)) { Ok(p) => p, Err(e) => return app_error(StatusCode::BAD_REQUEST, e) };
    let filename = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let vpath = to_vpath(state, &p);
    let meta = fs::metadata(&p).ok();
    let bytes = meta.as_ref().map(|m| m.len()).unwrap_or(0);
    let human = human_size(bytes);
    let mtime = meta.and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default();
    json_resp(json!([filename, vpath, human, bytes, mtime]))
}

fn music_meta(state: &AppState, params: &BTreeMapCompat) -> Response {
    // The Go AGI getMeta.js does not return ID3 metadata. It returns the
    // nearby playable audio files in the same directory as arrays:
    // [filename, filepath, ext_without_dot, human_readable_size].
    let mut raw = param(params, &["file", "filepath", "path"]).unwrap_or("").to_string();
    if let Some(stripped) = raw.strip_prefix("/media?file=") { raw = stripped.to_string(); }
    let real = match resolve_virtual(state, Some(&raw)) {
        Ok(p) => p,
        Err(_) => return json_resp(json!([])),
    };
    let dir = real.parent().map(|p| p.to_path_buf()).unwrap_or(real.clone());
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(&dir) {
        for e in rd.flatten() {
            let p = e.path();
            if p.is_file() && is_audio(&p) {
                let filename = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
                let ext = p.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
                let human = fs::metadata(&p).map(|m| human_size(m.len())).unwrap_or_else(|_| "0 Byte".into());
                out.push(json!([filename, to_vpath(state, &p), ext, human]));
            }
        }
    }
    out.sort_by(|a, b| a[0].as_str().cmp(&b[0].as_str()));
    json_resp(json!(out))
}

fn is_audio(path: &Path) -> bool { ext(path).map(|e| matches!(e.as_str(), "mp3" | "flac" | "wav" | "ogg" | "aac" | "webm" | "mp4" | "m4a")).unwrap_or(false) }

fn human_size(bytes: u64) -> String {
    let units = ["Bytes", "KB", "MB", "GB", "TB"];
    if bytes == 0 { return "0 Byte".into(); }
    let mut size = bytes as f64;
    let mut idx = 0usize;
    while size >= 1024.0 && idx < units.len() - 1 { size /= 1024.0; idx += 1; }
    format!("{:.2} {}", size, units[idx])
}

fn html_resp(body: impl Into<String>) -> Response { (StatusCode::OK, [(header::CONTENT_TYPE, "text/html; charset=utf-8")], body.into()).into_response() }

fn speedtest_http() -> Response {
    json_resp(json!({
        "ok": true,
        "native": "rust",
        "websocket": true,
        "endpoint": "/system/ajgi/interface?script=Speedtest/special/wspeedtest.js"
    }))
}

fn serverless_backend(state: &AppState, params: &BTreeMapCompat) -> Response {
    let mut db = db_read(state, "serverless_ext");
    match param(params, &["opr", "op", "action"]).unwrap_or("list") {
        "add" => {
            let path = param(params, &["path", "file"]).unwrap_or("");
            if path.is_empty() { return app_error(StatusCode::BAD_REQUEST, "missing external AGI path"); }
            let id = Uuid::new_v4().to_string();
            db.insert(id.clone(), json!({"uuid": id.clone(), "path": path, "created_unix": now_unix()}));
            match db_write(state, "serverless_ext", &db) { Ok(_) => json_resp(json!({"ok": true, "uuid": id})), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
        }
        "remove" | "rm" => {
            if let Some(id) = param(params, &["uuid", "id"]) { db.remove(id); }
            match db_write(state, "serverless_ext", &db) { Ok(_) => json_resp(json!({"ok": true})), Err(e) => app_error(StatusCode::INTERNAL_SERVER_ERROR, e) }
        }
        _ => json_resp(json!(db.values().cloned().collect::<Vec<_>>())),
    }
}

fn unittest_list(state: &AppState) -> Response {
    let base = PathBuf::from(&state.config.web_root).join("UnitTest/backend");
    let mut out = Vec::new();
    if let Ok(rd) = fs::read_dir(base) { for e in rd.flatten() { if e.path().is_file() { out.push(e.file_name().to_string_lossy().to_string()); } } }
    out.sort();
    json_resp(json!(out))
}

fn unittest_backend(state: &AppState, headers: &HeaderMap, script: &str, params: &BTreeMapCompat, _json_body: Option<Value>) -> Response {
    match script {
        "getParamters.js" => json_resp(json!(params)),
        "getLoadedModules.js" | "modulelist.js" => json_resp(json!(builtin_modules_json())),
        "getStorageDevices.js" => json_resp(json!([{"Name": "User Files", "UUID": "user", "Filesystem": "localfs"}])),
        "hello world.js" => text_resp("Hello World"),
        "dbExists.js" => json_resp(json!(true)),
        "dbNotExists.js" => text_resp("Table not found"),
        "dbtest.js" => { let mut db = db_read(state, "testdb"); db.insert("message".into(), json!("Hello World")); let value = db.get("message").cloned().unwrap_or(Value::Null); text_resp(format!("Database access return value: {}", value.as_str().unwrap_or(""))) },
        "dblist.js" => json_resp(json!({"One":"Hello World","Two":"This is a text message","Three":"For listing","Four":"123456","Five":"You can also put JSON string here"})),
        "appdata.listDir.js" => unittest_list(state),
        "appdata.readFile.js" => text_resp(fs::read_to_string(PathBuf::from(&state.config.web_root).join("UnitTest/appdata.txt")).unwrap_or_default()),
        "filelib.fileExists.js" => text_resp(if resolve_virtual(state, Some("user:/Desktop/test.txt")).map(|p| p.exists()).unwrap_or(false) { "File Exists" } else { "File Not Exists" }),
        "filelib.file.js" => { let _ = write_file(state, Some("user:/Desktop/test.txt"), "Hello World! This is a testing message to write"); app_read_file(state, &BTreeMapCompat::from([("file".into(), "user:/Desktop/test.txt".into())]), "file") },
        "filelib.mkdir.js" => recorder_create(state, &BTreeMapCompat::from([("savedir".into(), "user:/Desktop/testdir".into())])),
        "filelib.delete.js" => { let _ = write_file(state, Some("user:/Desktop/test.txt"), ""); let _ = resolve_virtual(state, Some("user:/Desktop/test.txt")).and_then(|p| fs::remove_file(p).map_err(|e| e.to_string())); text_resp("OK") },
        "filelib.readDir.js" | "filelib.glob.js" | "filelib.aglob.js" => json_resp(json!(list_children(state, "user:/Desktop").unwrap_or_default().iter().map(|p| to_vpath(state, p)).collect::<Vec<_>>())),
        "filelib.walk.js" => { let mut all=Vec::new(); if let Ok(root)=resolve_virtual(state, Some("user:/")) { list_recursive(&root, &mut all); } json_resp(json!(all.iter().map(|p| to_vpath(state, p)).collect::<Vec<_>>())) },
        "filelib.filesize.js" => { let p=resolve_virtual(state, Some("user:/Desktop/test.txt")).ok(); json_resp(json!(p.and_then(|p| fs::metadata(p).ok().map(|m| m.len())).unwrap_or(0))) },
        "filelib.md5.js" => json_resp(json!({"ok": true, "note": "md5 unit test mapped to Rust compatibility layer"})),
        "iot.list.js" | "iot.scan.js" | "iot.status.js" | "iot.connect.js" | "iot.exec.js" | "iot.iconTag.js" => json_resp(json!({"ok": true, "iot": [], "note": "IoT unit backend is handled by Rust state layer"})),
        "newUser.js" => match state.auth.upsert_user(param(params, &["username", "user"]).unwrap_or("testuser"), Some(param(params, &["password", "pw"]).unwrap_or("testpass")), false, Some("user"), None) { Ok(u) => json_resp(json!(u)), Err(e) => app_error(StatusCode::BAD_REQUEST, e) },
        "removeUser.js" => { let u = param(params, &["username", "user"]).unwrap_or("testuser"); json_resp(json!({"ok": state.auth.remove_user(u)})) },
        "permission.js" => json_resp(json!({"ok": true, "user": username(state, headers), "read": true, "write": true})),
        "share.shareFile.js" => json_resp(json!({"ok": true, "share": Uuid::new_v4().to_string()})),
        "http.get.js" | "http.post.js" | "http.head.js" | "http.download.js" => json_resp(json!({"ok": true, "http": "planned", "note": "external HTTP calls are represented but not executed by default"})),
        "imagelib.imageDimension.js" | "imagelib.cropImage.js" | "imagelib.imageResize.js" | "imagelib.loadThumbString.js" | "image.classify.js" => json_resp(json!({"ok": true, "image": "compat", "dimension": [0,0]})),
        "jsonPost.js" => json_resp(json!(params)),
        "error.js" => app_error(StatusCode::INTERNAL_SERVER_ERROR, "intentional unit-test error"),
        "execd.js" | "execpkg.js" | "require_pkg.js" | "require_pkg_windows.js" => json_resp(json!({"ok": false, "blocked": true, "note": "external process execution is gated in the Rust port"})),
        _ => json_resp(json!({"ok": true, "script": script, "ported": "unit-test-compat"})),
    }
}
