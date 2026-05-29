use axum::{body::Body, extract::{Multipart, Query, State}, http::{header, Request, StatusCode}, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use std::{collections::{HashMap, HashSet}, fs, io::{self, Cursor, Seek, Write}, path::{Path, PathBuf}, sync::Arc, time::UNIX_EPOCH};
use crate::{state::AppState, utils};

const PARAM_BODY_LIMIT: usize = 16 * 1024 * 1024;

#[derive(Debug, Serialize)]
struct FsEntry {
    // Rust-native field names used by the migration API
    name: String,
    path: String,
    is_dir: bool,
    size: u64,
    modified_unix: Option<u64>,
    readonly: bool,
    mime: String,

    // ArozOS / Go-compatible field names expected by ao_module.js and legacy apps
    #[serde(rename = "Filename")]
    filename: String,
    #[serde(rename = "Filepath")]
    filepath: String,
    #[serde(rename = "Ext")]
    ext: String,
    #[serde(rename = "Filesize")]
    filesize: u64,
    #[serde(rename = "Displaysize")]
    display_size: String,
    #[serde(rename = "IsDir")]
    is_dir_go: bool,
    #[serde(rename = "IsHidden")]
    is_hidden: bool,
    #[serde(rename = "MimeType")]
    mime_type: String,
    #[serde(rename = "ModTime")]
    mod_time: u64,
    #[serde(rename = "Realpath")]
    real_path: String,
    #[serde(rename = "ReadOnly")]
    readonly_go: bool,
}

#[derive(Debug, Clone)]
pub struct HostRoot {
    pub name: String,
    pub path: PathBuf,
    pub kind: String,
    pub icon: String,
}

pub fn user_path_to_relative(raw: &str) -> String {
    let mut s = raw.trim().replace('\\', "/");
    if let Some((_, tail)) = s.split_once(":/") { s = tail.to_string(); }
    while s.starts_with('/') { s.remove(0); }
    utils::sanitize_relative_path(&s)
}

pub fn ensure_root(root: &str) -> io::Result<PathBuf> {
    let p = PathBuf::from(root);
    if !p.exists() { fs::create_dir_all(&p)?; }
    p.canonicalize()
}

fn raw_param(raw: Option<&String>) -> String { raw.cloned().unwrap_or_else(|| "user:/".to_string()) }
fn is_host_path(raw: &str) -> bool { raw.starts_with("host:/") || raw.starts_with("file://") || raw.starts_with("local:/") }
pub fn hostfs_write_enabled() -> bool { std::env::var("AROZOS_RS_ENABLE_HOST_FS_WRITE").map(|v| utils::truthy(Some(&v))).unwrap_or(false) }
pub fn hostfs_read_enabled() -> bool { std::env::var("AROZOS_RS_DISABLE_HOST_FS_READ").map(|v| !utils::truthy(Some(&v))).unwrap_or(true) }

fn decode_mountinfo_path(raw: &str) -> String {
    raw.replace("\\040", " ")
        .replace("\\011", "\t")
        .replace("\\012", "\n")
        .replace("\\134", "\\")
}

fn resolve_host_path(raw: &str) -> io::Result<PathBuf> {
    if !hostfs_read_enabled() { return Err(io::Error::new(io::ErrorKind::PermissionDenied, "host:/ browsing is disabled")); }
    let mut s = raw.trim().replace('\\', "/");
    if let Some(tail) = s.strip_prefix("host:/") {
        let tail = tail.trim_start_matches('/');
        #[cfg(target_os = "windows")]
        {
            if tail.len() >= 3 && tail.as_bytes()[1] == b':' && tail.as_bytes()[2] == b'/' {
                s = tail.to_string();
            } else if tail.is_empty() {
                s = std::env::var("USERPROFILE").unwrap_or_else(|_| "C:/".to_string());
            } else {
                s = format!("/{}", tail);
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            s = format!("/{}", tail);
        }
    }
    if let Some(tail) = s.strip_prefix("local:/") { s = format!("/{}", tail.trim_start_matches('/')); }
    if let Some(tail) = s.strip_prefix("file://") { s = tail.to_string(); }
    let p = PathBuf::from(if s.is_empty() { "/".to_string() } else { s });
    if p.exists() { p.canonicalize() } else {
        let parent = p.parent().unwrap_or(Path::new("/")).canonicalize().unwrap_or_else(|_| PathBuf::from("/"));
        Ok(parent.join(p.file_name().unwrap_or_default()))
    }
}

pub fn resolve(root: &str, raw: Option<&String>) -> io::Result<PathBuf> {
    let raw_s = raw_param(raw);
    if is_host_path(&raw_s) { return resolve_host_path(&raw_s); }
    let base = ensure_root(root)?;
    let rel = user_path_to_relative(&raw_s);
    let joined = base.join(rel);
    if joined.exists() {
        let canon = joined.canonicalize()?;
        if canon.starts_with(&base) { Ok(canon) } else { Err(io::Error::new(io::ErrorKind::PermissionDenied, "path escapes root")) }
    } else {
        let parent = joined.parent().unwrap_or(&base).canonicalize().unwrap_or(base.clone());
        if parent.starts_with(&base) { Ok(joined) } else { Err(io::Error::new(io::ErrorKind::PermissionDenied, "path escapes root")) }
    }
}

fn host_vpath(path: &Path) -> String {
    let abs = path.to_string_lossy().replace('\\', "/");
    #[cfg(target_os = "windows")]
    {
        format!("host:/{}", abs.trim_start_matches('/'))
    }
    #[cfg(not(target_os = "windows"))]
    {
        if abs == "/" { "host:/".to_string() } else { format!("host:/{}", abs.trim_start_matches('/')) }
    }
}

fn vpath_for(root: &Path, path: &Path) -> String {
    if path.starts_with(root) {
        let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
        if rel.is_empty() { "user:/".to_string() } else { format!("user:/{}", rel) }
    } else {
        host_vpath(path)
    }
}

fn human_size(size: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut value = size as f64;
    let mut unit = 0usize;
    while value >= 1024.0 && unit < UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 { format!("{} {}", size, UNITS[unit]) } else { format!("{:.1} {}", value, UNITS[unit]) }
}

fn metadata_to_entry(root: &Path, path: &Path) -> io::Result<FsEntry> {
    let md = fs::metadata(path)?;
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| vpath_for(root, path));
    let filepath = vpath_for(root, path);
    let modified_unix = md.modified().ok().and_then(|m| m.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs());
    let mod_time = modified_unix.unwrap_or(0);
    let mime = if md.is_dir() { "inode/directory".to_string() } else { mime_guess::from_path(path).first_or_octet_stream().essence_str().to_string() };
    let ext = if md.is_dir() { String::new() } else { path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default() };
    let readonly = md.permissions().readonly() || (filepath.starts_with("host:/") && !hostfs_write_enabled());
    Ok(FsEntry {
        name: name.clone(),
        path: filepath.clone(),
        is_dir: md.is_dir(),
        size: md.len(),
        modified_unix,
        readonly,
        mime: mime.clone(),
        filename: name.clone(),
        filepath,
        ext,
        filesize: md.len(),
        display_size: if md.is_dir() { "Folder".to_string() } else { human_size(md.len()) },
        is_dir_go: md.is_dir(),
        is_hidden: name.starts_with('.'),
        mime_type: mime,
        mod_time,
        real_path: path.to_string_lossy().to_string(),
        readonly_go: readonly,
    })
}

fn host_root_key(path: &Path) -> String {
    let mut key = path.to_string_lossy().replace('\\', "/");
    while key.len() > 1 && key.ends_with('/') { key.pop(); }
    #[cfg(target_os="windows")]
    { key = key.to_lowercase(); }
    key
}

fn add_host_root(out: &mut Vec<HostRoot>, seen: &mut HashSet<String>, name: impl Into<String>, path: PathBuf, kind: impl Into<String>, icon: impl Into<String>) {
    if !path.exists() || !path.is_dir() { return; }
    let stable = path.canonicalize().unwrap_or(path);
    let key = host_root_key(&stable);
    if seen.insert(key) {
        out.push(HostRoot { name: name.into(), path: stable, kind: kind.into(), icon: icon.into() });
    }
}

fn is_linux_pseudo_fs(fstype: &str) -> bool {
    let pseudo = [
        "proc", "sysfs", "devtmpfs", "devpts", "tmpfs", "cgroup", "cgroup2", "pstore",
        "securityfs", "debugfs", "tracefs", "configfs", "fusectl", "mqueue", "hugetlbfs",
        "bpf", "nsfs", "autofs", "binfmt_misc", "rpc_pipefs", "ramfs", "overlay"
    ];
    pseudo.contains(&fstype)
}

fn is_linux_physical_fs(fstype: &str) -> bool {
    let physical = [
        "ext2", "ext3", "ext4", "btrfs", "xfs", "zfs", "vfat", "exfat", "ntfs", "ntfs3",
        "fuseblk", "iso9660", "udf", "bcachefs", "apfs", "hfs", "hfsplus"
    ];
    physical.contains(&fstype)
}

fn is_noisy_linux_mount(mp: &str) -> bool {
    if mp == "/" { return false; }
    if mp.starts_with("/run/media/") { return false; }
    let noisy_prefixes = [
        "/proc", "/sys", "/dev", "/snap", "/var/lib/docker", "/var/lib/containers",
        "/run/user", "/tmp", "/var/tmp", "/var/cache", "/var/lib/snapd", "/var/lib/flatpak"
    ];
    noisy_prefixes.iter().any(|p| mp == *p || mp.starts_with(&format!("{}/", p.trim_end_matches('/'))))
}

fn is_interesting_linux_mount(mp: &str, fstype: &str) -> bool {
    if mp.is_empty() { return false; }
    if is_linux_pseudo_fs(fstype) { return false; }
    if is_noisy_linux_mount(mp) { return false; }
    if mp == "/" || mp == "/boot/efi" { return true; }

    // Removable/media-style mount points are what users expect in a Thunar-like sidebar.
    if mp.starts_with("/media/") || mp.starts_with("/run/media/") || mp.starts_with("/mnt/") { return true; }

    // Keep platform places as roots, but avoid exposing every technical bind mount below them.
    if matches!(mp, "/media" | "/run/media" | "/mnt" | "/home" | "/srv" | "/opt") { return true; }

    // A separate /home or user-home partition should still be visible, but exact duplicates are
    // removed later through the canonical path key.
    if mp.starts_with("/home/") && mp.matches('/').count() <= 2 { return true; }

    // For less common layouts such as /data or /storage, only show likely real filesystems.
    is_linux_physical_fs(fstype)
}

fn current_platform_id() -> &'static str {
    #[cfg(target_os="linux")]
    { "linux" }
    #[cfg(target_os="macos")]
    { "macos" }
    #[cfg(target_os="windows")]
    { "windows" }
    #[cfg(all(target_family="unix", not(any(target_os="linux", target_os="macos"))))]
    { "unix" }
    #[cfg(not(any(target_family="unix", target_os="windows")))]
    { "unknown" }
}

fn current_platform_name() -> &'static str {
    #[cfg(target_os="linux")]
    { "Linux" }
    #[cfg(target_os="macos")]
    { "macOS" }
    #[cfg(target_os="windows")]
    { "Windows" }
    #[cfg(all(target_family="unix", not(any(target_os="linux", target_os="macos"))))]
    { "Unix" }
    #[cfg(not(any(target_family="unix", target_os="windows")))]
    { "Unknown" }
}

pub fn mounted_roots() -> Vec<HostRoot> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();

    #[cfg(target_os="linux")]
    {
        add_host_root(&mut out, &mut seen, "File System", PathBuf::from("/"), "filesystem", "server");
        if let Ok(home) = std::env::var("HOME") {
            add_host_root(&mut out, &mut seen, "Home", PathBuf::from(home), "place", "home");
        }
        for candidate in ["/media", "/run/media", "/mnt", "/home", "/srv", "/opt"] {
            add_host_root(&mut out, &mut seen, candidate, PathBuf::from(candidate), "place", "folder");
        }
        if let Ok(text) = fs::read_to_string("/proc/self/mountinfo") {
            for line in text.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() < 10 { continue; }
                let mp = decode_mountinfo_path(parts[4]);
                let fstype = parts.iter().position(|part| *part == "-").and_then(|idx| parts.get(idx + 1)).copied().unwrap_or("");
                if !is_interesting_linux_mount(&mp, fstype) { continue; }
                let p = PathBuf::from(&mp);
                let name = if mp == "/" { "File System".to_string() } else { p.file_name().and_then(|n| n.to_str()).unwrap_or(&mp).to_string() };
                add_host_root(&mut out, &mut seen, name, p, "mount", "drive");
            }
        }
    }
    #[cfg(target_os="macos")]
    {
        add_host_root(&mut out, &mut seen, "File System", PathBuf::from("/"), "filesystem", "server");
        if let Ok(home) = std::env::var("HOME") {
            add_host_root(&mut out, &mut seen, "Home", PathBuf::from(home), "place", "home");
        }
        if let Ok(rd) = fs::read_dir("/Volumes") {
            for e in rd.flatten() {
                add_host_root(&mut out, &mut seen, e.file_name().to_string_lossy().to_string(), e.path(), "volume", "drive");
            }
        }
    }
    #[cfg(target_os="windows")]
    {
        if let Ok(profile) = std::env::var("USERPROFILE") {
            add_host_root(&mut out, &mut seen, "Benutzerprofil", PathBuf::from(profile), "place", "home");
        }
        for letter in b'A'..=b'Z' {
            let p = format!("{}:/", letter as char);
            if Path::new(&p).exists() {
                add_host_root(&mut out, &mut seen, format!("{}:", letter as char), PathBuf::from(&p), "drive", "drive");
            }
        }
    }
    #[cfg(all(target_family="unix", not(any(target_os="linux", target_os="macos"))))]
    {
        add_host_root(&mut out, &mut seen, "File System", PathBuf::from("/"), "filesystem", "server");
        if let Ok(home) = std::env::var("HOME") {
            add_host_root(&mut out, &mut seen, "Home", PathBuf::from(home), "place", "home");
        }
        for candidate in ["/media", "/run/media", "/mnt", "/home", "/srv", "/opt", "/Volumes"] {
            add_host_root(&mut out, &mut seen, candidate, PathBuf::from(candidate), "place", "folder");
        }
    }

    out.sort_by(|a,b| {
        let rank = |r: &HostRoot| match r.name.as_str() {
            "File System" => 0,
            "Home" | "Benutzerprofil" => 1,
            _ => 2,
        };
        rank(a).cmp(&rank(b)).then_with(|| a.path.cmp(&b.path))
    });
    out
}

fn root_json(name: &str, path: &str, backend: &str, kind: &str, real_path: &str, readonly: bool, icon: &str) -> serde_json::Value {
    let display_path = if backend == "hostfs" { real_path } else { path };
    serde_json::json!({
        "name": name, "Name": name, "RootName": name,
        "path": path, "Path": path, "RootPath": path,
        "backend": backend, "Backend": backend,
        "type": kind, "Type": kind,
        "real_path": real_path, "RealPath": real_path,
        "display_path": display_path, "DisplayPath": display_path,
        "platform": current_platform_id(), "Platform": current_platform_id(),
        "platformName": current_platform_name(), "PlatformName": current_platform_name(),
        "ReadOnly": readonly,
        "Icon": icon
    })
}

pub fn roots_array(root_directory: &str) -> io::Result<Vec<serde_json::Value>> {
    let root = ensure_root(root_directory)?;
    let mut roots = vec![root_json("User Files", "user:/", "localfs", "user", &root.to_string_lossy(), false, "folder")];
    for host_root in mounted_roots() {
        let vpath = host_vpath(&host_root.path);
        roots.push(root_json(&host_root.name, &vpath, "hostfs", &host_root.kind, &host_root.path.to_string_lossy(), !hostfs_write_enabled(), &host_root.icon));
    }
    Ok(roots)
}

fn default_user_folders(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for folder in ["Desktop", "Documents", "Download", "Music", "Photo", "Video", "Web"] {
        let p = root.join(folder);
        let _ = fs::create_dir_all(&p);
        out.push(p);
    }
    out
}

fn list_user_root_dirs(root_directory: &str) -> io::Result<Vec<FsEntry>> {
    let root = ensure_root(root_directory)?;
    let mut paths = default_user_folders(&root);
    if let Ok(rd) = fs::read_dir(&root) {
        for entry in rd.flatten() {
            let p = entry.path();
            if p.is_dir() && !paths.iter().any(|existing| existing == &p) {
                paths.push(p);
            }
        }
    }
    let mut entries = Vec::new();
    for path in paths {
        if let Ok(entry) = metadata_to_entry(&root, &path) { entries.push(entry); }
    }
    entries.sort_by(|a,b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(entries)
}

fn wants_object(q: &HashMap<String,String>) -> bool {
    q.get("format").map(|v| v.eq_ignore_ascii_case("object") || v.eq_ignore_ascii_case("json")).unwrap_or(false)
        || q.get("modern").map(|v| utils::truthy(Some(v))).unwrap_or(false)
}

fn json_error(status: StatusCode, msg: impl Into<String>) -> Response {
    (status, Json(serde_json::json!({"ok": false, "error": msg.into()}))).into_response()
}

pub async fn list_roots(State(state): State<Arc<AppState>>, Query(q): Query<HashMap<String,String>>) -> Response {
    if q.get("user").map(|v| utils::truthy(Some(v)) || v.eq_ignore_ascii_case("true")).unwrap_or(false) {
        return match list_user_root_dirs(&state.config.root_directory) {
            Ok(entries) => (StatusCode::OK, Json(serde_json::json!(entries))).into_response(),
            Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        };
    }
    match roots_array(&state.config.root_directory) {
        Ok(roots) => {
            if wants_object(&q) {
                (StatusCode::OK, Json(serde_json::json!({
                    "ok": true,
                    "roots": roots.clone(),
                    "drives": roots,
                    "platform": current_platform_id(),
                    "platformName": current_platform_name(),
                    "hostFsReadEnabled": hostfs_read_enabled(),
                    "hostFsWriteEnabled": hostfs_write_enabled()
                }))).into_response()
            } else {
                (StatusCode::OK, Json(serde_json::json!(roots))).into_response()
            }
        },
        Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn list_drives(State(state): State<Arc<AppState>>, Query(q): Query<HashMap<String,String>>) -> Response {
    match roots_array(&state.config.root_directory) {
        Ok(roots) => {
            if wants_object(&q) {
                (StatusCode::OK, Json(serde_json::json!({
                    "ok": true,
                    "roots": roots.clone(),
                    "drives": roots,
                    "platform": current_platform_id(),
                    "platformName": current_platform_name(),
                    "hostFsReadEnabled": hostfs_read_enabled(),
                    "hostFsWriteEnabled": hostfs_write_enabled()
                }))).into_response()
            } else {
                (StatusCode::OK, Json(serde_json::json!(roots))).into_response()
            }
        },
        Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
    }
}

pub async fn list_dir(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let raw = q.get("dir").or_else(|| q.get("path")).or_else(|| q.get("target"));
    let root = match ensure_root(&state.config.root_directory) { Ok(r) => r, Err(e) => return json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
    let dir = match resolve(&state.config.root_directory, raw) { Ok(p) => p, Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()) };
    let mut entries = Vec::new();
    let rd = match fs::read_dir(&dir) { Ok(rd) => rd, Err(e) => return json_error(StatusCode::NOT_FOUND, e.to_string()) };
    for entry in rd.flatten() { if let Ok(fe) = metadata_to_entry(&root, &entry.path()) { entries.push(fe); } }
    entries.sort_by(|a,b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase())));
    if wants_object(&q) {
        (StatusCode::OK, Json(serde_json::json!({"ok": true, "path": raw.cloned().unwrap_or_else(|| "user:/".to_string()), "files": entries}))).into_response()
    } else {
        (StatusCode::OK, Json(serde_json::json!(entries))).into_response()
    }
}

pub async fn list_dir_hash(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let raw = q.get("dir").or_else(|| q.get("path")).or_else(|| q.get("target"));
    let dir = match resolve(&state.config.root_directory, raw) { Ok(p) => p, Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()) };
    use sha2::Digest;
    let mut h = sha2::Sha256::new();
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            h.update(e.file_name().to_string_lossy().as_bytes());
            if let Ok(md) = e.metadata() { h.update(md.len().to_le_bytes()); }
        }
    }
    (StatusCode::OK, Json(serde_json::json!({"ok": true, "hash": format!("{:x}", h.finalize())}))).into_response()
}

pub async fn get_properties(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let raw = q.get("path").or_else(|| q.get("file")).or_else(|| q.get("target")).or_else(|| q.get("dir"));
    let root = match ensure_root(&state.config.root_directory) { Ok(r) => r, Err(e) => return json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
    let path = match resolve(&state.config.root_directory, raw) { Ok(p) => p, Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()) };
    match metadata_to_entry(&root, &path) {
        Ok(entry) => (StatusCode::OK, Json(serde_json::json!({"ok": true, "properties": entry}))).into_response(),
        Err(e) => json_error(StatusCode::NOT_FOUND, e.to_string()),
    }
}

pub async fn new_item(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    if !q.contains_key("type") && !q.contains_key("opr") && !q.contains_key("name") && !q.contains_key("filename") {
        return (StatusCode::OK, Json(serde_json::json!([
            {"Desc":"Text Document","Ext":"txt"},
            {"Desc":"Markdown Document","Ext":"md"},
            {"Desc":"HTML Document","Ext":"html"},
            {"Desc":"JavaScript File","Ext":"js"},
            {"Desc":"CSS Stylesheet","Ext":"css"},
            {"Desc":"JSON File","Ext":"json"}
        ]))).into_response();
    }
    let base_raw = q.get("dir").or_else(|| q.get("path")).or_else(|| q.get("src")).or_else(|| q.get("target"));
    if base_raw.map(|s| is_host_path(s)).unwrap_or(false) && !hostfs_write_enabled() { return json_error(StatusCode::FORBIDDEN, "host:/ writes are disabled. Set AROZOS_RS_ENABLE_HOST_FS_WRITE=1 to create files on OS drives."); }
    let name = q.get("name").or_else(|| q.get("filename")).cloned().unwrap_or_else(|| "new_item".to_string());
    if name.contains('/') || name.contains('\\') || name == "." || name == ".." { return json_error(StatusCode::BAD_REQUEST, "invalid name"); }
    let mut path = match resolve(&state.config.root_directory, base_raw) { Ok(p) => p, Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()) };
    if path.is_file() { path.pop(); }
    let target = path.join(name);
    let item_type = q.get("type").or_else(|| q.get("opr")).map(|s| s.as_str()).unwrap_or("file");
    let result = if item_type.eq_ignore_ascii_case("folder") || item_type.eq_ignore_ascii_case("dir") || item_type.eq_ignore_ascii_case("mkdir") {
        fs::create_dir_all(&target)
    } else {
        fs::OpenOptions::new().create_new(true).write(true).open(&target).map(|_| ())
    };
    match result { Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true, "path": target.to_string_lossy()}))).into_response(), Err(e) => json_error(StatusCode::BAD_REQUEST, e.to_string()) }
}

pub async fn file_opr(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    match file_opr_inner(&state.config.root_directory, &q) {
        Ok(value) => (StatusCode::OK, Json(value)).into_response(),
        Err(e) => json_error(StatusCode::BAD_REQUEST, e.to_string()),
    }
}

pub fn file_opr_inner(root: &str, q: &HashMap<String,String>) -> io::Result<serde_json::Value> {
    let op = q.get("opr").or_else(|| q.get("op")).or_else(|| q.get("operation")).map(|s| s.as_str()).unwrap_or("");
    let src_raw = q.get("src").or_else(|| q.get("source")).or_else(|| q.get("path")).or_else(|| q.get("file"));
    if src_raw.map(|s| is_host_path(s)).unwrap_or(false) && !hostfs_write_enabled() && !matches!(op, "stat" | "properties" | "list") {
        return Err(io::Error::new(io::ErrorKind::PermissionDenied, "host:/ writes are disabled. Set AROZOS_RS_ENABLE_HOST_FS_WRITE=1 to allow OS-drive modifications."));
    }
    let src = resolve(root, src_raw)?;
    match op {
        "delete" | "remove" | "trash" => {
            if src.is_dir() { fs::remove_dir_all(&src)? } else { fs::remove_file(&src)? };
            Ok(serde_json::json!({"ok": true}))
        },
        "rename" => {
            let new_name = q.get("newname").or_else(|| q.get("newName")).or_else(|| q.get("dest")).or_else(|| q.get("target")).cloned().unwrap_or_default();
            if new_name.is_empty() || new_name.contains('/') || new_name.contains('\\') { return Err(io::Error::new(io::ErrorKind::InvalidInput, "invalid new name")); }
            let dest = src.with_file_name(new_name);
            fs::rename(&src, &dest)?;
            Ok(serde_json::json!({"ok": true, "dest": dest.to_string_lossy()}))
        },
        "move" => {
            let dest = resolve(root, q.get("dest").or_else(|| q.get("target")))?;
            let final_dest = if dest.is_dir() { dest.join(src.file_name().unwrap_or_default()) } else { dest };
            fs::rename(&src, &final_dest)?;
            Ok(serde_json::json!({"ok": true, "dest": final_dest.to_string_lossy()}))
        },
        "copy" => {
            let dest = resolve(root, q.get("dest").or_else(|| q.get("target")))?;
            let final_dest = if dest.is_dir() { dest.join(src.file_name().unwrap_or_default()) } else { dest };
            if src.is_dir() { copy_dir_recursive(&src, &final_dest)?; } else { fs::copy(&src, &final_dest).map(|_| ())?; }
            Ok(serde_json::json!({"ok": true, "dest": final_dest.to_string_lossy()}))
        },
        "mkdir" => { fs::create_dir_all(&src)?; Ok(serde_json::json!({"ok": true})) },
        "touch" => { fs::OpenOptions::new().create(true).append(true).open(&src).map(|_| ())?; Ok(serde_json::json!({"ok": true})) },
        other => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("unsupported file operation: {other}"))),
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let target = dst.join(entry.file_name());
        if ty.is_dir() { copy_dir_recursive(&entry.path(), &target)?; } else { fs::copy(entry.path(), target).map(|_| ())?; }
    }
    Ok(())
}

pub async fn validate_file_opr(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let src_raw = q.get("src").or_else(|| q.get("source")).or_else(|| q.get("path")).or_else(|| q.get("file"));
    match resolve(&state.config.root_directory, src_raw) {
        Ok(path) => (StatusCode::OK, Json(serde_json::json!({"ok": true, "exists": path.exists(), "path": path.to_string_lossy()}))).into_response(),
        Err(e) => json_error(StatusCode::BAD_REQUEST, e.to_string()),
    }
}

pub async fn search(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let keyword = q.get("keyword").or_else(|| q.get("q")).or_else(|| q.get("search")).cloned().unwrap_or_default().to_lowercase();
    if keyword.is_empty() { return json_error(StatusCode::BAD_REQUEST, "missing keyword"); }
    let start = q.get("path").or_else(|| q.get("dir"));
    let root = match ensure_root(&state.config.root_directory) { Ok(r) => r, Err(e) => return json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()) };
    let start_path = match resolve(&state.config.root_directory, start) { Ok(p) if p.is_dir() => p, _ => root.clone() };
    let mut stack = vec![start_path];
    let mut results = Vec::new();
    while let Some(dir) = stack.pop() {
        if results.len() >= 500 { break; }
        if let Ok(rd) = fs::read_dir(&dir) {
            for e in rd.flatten() {
                let p = e.path();
                let name = e.file_name().to_string_lossy().to_lowercase();
                if name.contains(&keyword) { if let Ok(fe) = metadata_to_entry(&root, &p) { results.push(fe); } }
                if p.is_dir() && results.len() < 500 { stack.push(p); }
            }
        }
    }
    (StatusCode::OK, Json(serde_json::json!({"ok": true, "results": results}))).into_response()
}

pub async fn upload(State(state): State<Arc<AppState>>, Query(q): Query<HashMap<String,String>>, mut multipart: Multipart) -> Response {
    let dest_raw = q.get("dir").or_else(|| q.get("path")).or_else(|| q.get("target"));
    if dest_raw.map(|s| is_host_path(s)).unwrap_or(false) && !hostfs_write_enabled() { return json_error(StatusCode::FORBIDDEN, "host:/ writes are disabled. Set AROZOS_RS_ENABLE_HOST_FS_WRITE=1 to upload to OS drives."); }
    let dest_dir = match resolve(&state.config.root_directory, dest_raw) { Ok(p) => p, Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()) };
    if let Err(e) = fs::create_dir_all(&dest_dir) { return json_error(StatusCode::BAD_REQUEST, e.to_string()); }
    let mut saved = Vec::new();
    while let Ok(Some(field)) = multipart.next_field().await {
        let name = field.file_name().map(|s| s.to_string()).unwrap_or_else(|| field.name().unwrap_or("upload.bin").to_string());
        let safe = utils::sanitize_relative_path(&name).replace('/', "_");
        let target = dest_dir.join(if safe.is_empty() { "upload.bin".to_string() } else { safe });
        match field.bytes().await {
            Ok(bytes) => {
                if let Err(e) = fs::write(&target, &bytes) { return json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()); }
                saved.push(serde_json::json!({"name": name, "path": target.to_string_lossy(), "size": bytes.len()}));
            },
            Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()),
        }
    }
    (StatusCode::OK, Json(serde_json::json!({"ok": true, "files": saved}))).into_response()
}

pub async fn lowmem_upload(State(state): State<Arc<AppState>>, Query(q): Query<HashMap<String,String>>, multipart: Multipart) -> Response { upload(State(state), Query(q), multipart).await }

fn parse_download_targets(q: &HashMap<String,String>) -> Vec<String> {
    for key in ["files", "filelist", "srcs", "paths"] {
        if let Some(raw) = q.get(key) {
            if let Ok(list) = serde_json::from_str::<Vec<String>>(raw) {
                return list.into_iter().filter(|s| !s.trim().is_empty()).collect();
            }
            return raw.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        }
    }
    q.get("file").or_else(|| q.get("path")).or_else(|| q.get("src")).or_else(|| q.get("vpath")).or_else(|| q.get("target")).map(|s| vec![s.clone()]).unwrap_or_default()
}

fn safe_download_name(path: &Path, fallback: &str) -> String {
    path.file_name().and_then(|n| n.to_str()).filter(|s| !s.is_empty()).unwrap_or(fallback).replace('"', "'").replace('\r', "").replace('\n', "")
}

fn add_path_to_zip<W: Write + Seek>(writer: &mut zip::ZipWriter<W>, path: &Path, base: &Path, options: zip::write::SimpleFileOptions) -> io::Result<()> {
    let raw_name = path.strip_prefix(base).unwrap_or(path).to_string_lossy().replace('\\', "/");
    let name = raw_name.trim_start_matches('/');
    if path.is_dir() {
        if !name.is_empty() { writer.add_directory(format!("{}/", name.trim_end_matches('/')), options)?; }
        for entry in fs::read_dir(path)? { add_path_to_zip(writer, &entry?.path(), base, options)?; }
    } else {
        let zip_name = if name.is_empty() { safe_download_name(path, "download") } else { name.to_string() };
        writer.start_file(zip_name, options)?;
        let mut f = fs::File::open(path)?;
        io::copy(&mut f, writer)?;
    }
    Ok(())
}

fn zip_targets(paths: &[PathBuf]) -> io::Result<Vec<u8>> {
    let cursor = Cursor::new(Vec::new());
    let mut writer = zip::ZipWriter::new(cursor);
    let options = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    for path in paths {
        let base = if paths.len() == 1 && path.is_dir() {
            path.parent().unwrap_or(Path::new("/")).to_path_buf()
        } else {
            path.parent().unwrap_or(Path::new("/")).to_path_buf()
        };
        add_path_to_zip(&mut writer, path, &base, options)?;
    }
    let cursor = writer.finish()?;
    Ok(cursor.into_inner())
}

pub async fn download(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (q, _) = utils::request_params(req, PARAM_BODY_LIMIT).await;
    let targets = parse_download_targets(&q);
    if targets.is_empty() { return json_error(StatusCode::BAD_REQUEST, "missing file/path parameter"); }
    let mut paths = Vec::new();
    for target in targets {
        match resolve(&state.config.root_directory, Some(&target)) {
            Ok(path) => {
                if !path.exists() { return json_error(StatusCode::NOT_FOUND, format!("not found: {target}")); }
                paths.push(path);
            },
            Err(e) => return json_error(StatusCode::BAD_REQUEST, e.to_string()),
        }
    }

    if paths.len() == 1 && paths[0].is_file() {
        let path = paths.remove(0);
        let filename = safe_download_name(&path, "download");
        match tokio::fs::read(&path).await {
            Ok(bytes) => {
                let mime = mime_guess::from_path(&path).first_or_octet_stream().to_string();
                let disposition = format!("attachment; filename=\"{}\"", filename);
                (StatusCode::OK, [(header::CONTENT_TYPE, mime), (header::CONTENT_DISPOSITION, disposition)], bytes).into_response()
            },
            Err(e) => json_error(StatusCode::NOT_FOUND, e.to_string()),
        }
    } else {
        let filename = if paths.len() == 1 { format!("{}.zip", safe_download_name(&paths[0], "download")) } else { "download.zip".to_string() };
        match zip_targets(&paths) {
            Ok(bytes) => (StatusCode::OK, [(header::CONTENT_TYPE, "application/zip".to_string()), (header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}\"", filename))], bytes).into_response(),
            Err(e) => json_error(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
        }
    }
}

pub async fn preference() -> Response { (StatusCode::OK, Json(serde_json::json!({"ok": true, "view":"list", "sort":"name", "showHidden": false}))).into_response() }
pub async fn trash_status() -> Response { (StatusCode::OK, Json(serde_json::json!({"ok": true, "files": []}))).into_response() }
pub async fn ongoing() -> Response { (StatusCode::OK, Json(serde_json::json!({"ok": true, "tasks": []}))).into_response() }
pub async fn version_history() -> Response { (StatusCode::OK, Json(serde_json::json!({"ok": true, "versions": []}))).into_response() }
pub async fn file_permission() -> Response { (StatusCode::OK, Json(serde_json::json!({"ok": true, "read": true, "write": true, "owner": true}))).into_response() }
pub async fn thumbnail() -> Response { (StatusCode::OK, [(header::CONTENT_TYPE, "application/json")], Body::from(r#"{"ok":true,"thumbnail":null}"#)).into_response() }

pub async fn not_yet_ported() -> Response {
    (StatusCode::NOT_IMPLEMENTED, Json(serde_json::json!({"ok": false, "error": "this endpoint is represented in Rust but not functionally ported yet"}))).into_response()
}
