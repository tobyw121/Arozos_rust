use crate::{config::Config, state::AppState, utils};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::HashMap, fs, io, path::{Path, PathBuf}, sync::Mutex};
use tokio::{
    fs as tokio_fs,
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream, UdpSocket},
    task::JoinHandle,
    time::{timeout, Duration},
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProtocolConfig {
    pub name: String,
    pub enabled: bool,
    pub port: Option<u16>,
    pub upnp: bool,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolRuntimeStatus {
    pub service: String,
    pub builtin: bool,
    pub running: bool,
    pub configured_enabled: bool,
    pub port: u16,
    pub bind_addr: String,
    pub mode: String,
    pub note: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProtocolApplyResult {
    pub ok: bool,
    pub service: String,
    pub builtin: bool,
    pub action: String,
    pub port: u16,
    pub running: bool,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Default)]
struct AuthDb {
    #[serde(default)]
    passwords: HashMap<String, String>,
}

static FTP_TASK: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));
static TFTP_TASK: Lazy<Mutex<Option<JoinHandle<()>>>> = Lazy::new(|| Mutex::new(None));

pub fn start_enabled_services(state: &AppState) {
    for name in ["ftp", "tftp"] {
        let cfg = load_config(&state.config, name, default_port(name));
        if cfg.enabled {
            let result = apply_service_change(state, name, true);
            tracing::info!(service=%name, ok=%result.ok, port=%result.port, message=%result.message, "applied builtin protocol service state");
        }
    }
}

pub fn runtime_status(state: &AppState, service: &str) -> ProtocolRuntimeStatus {
    let service = normalize_service(service);
    let cfg = load_config(&state.config, service, default_port(service));
    let port = cfg.port.unwrap_or_else(|| default_port(service).unwrap_or(0));
    let running = match service {
        "ftp" => FTP_TASK.lock().ok().and_then(|g| g.as_ref().map(|h| !h.is_finished())).unwrap_or(false),
        "tftp" => TFTP_TASK.lock().ok().and_then(|g| g.as_ref().map(|h| !h.is_finished())).unwrap_or(false),
        "webdav" => cfg.enabled,
        _ => false,
    };
    let (builtin, mode, note) = match service {
        "ftp" => (true, "builtin-rust-passive-ftp", "Built-in Rust FTP server; uses passive mode and the ArozOS root directory."),
        "tftp" => (true, "builtin-rust-tftp", "Built-in Rust TFTP server; unauthenticated by protocol design, root-limited."),
        "webdav" => (true, "builtin-rust-http-webdav", "Built-in Rust WebDAV endpoint at /webdav/."),
        "sftp" => (false, "os-sshd-adapter", "SFTP is the SSH subsystem. Rust controls/configures the OS sshd service instead of faking an HTTP alias."),
        "samba" => (false, "os-samba-adapter", "SMB/CIFS is provided by the OS Samba daemon; Rust writes state/config plans and can control smbd when privileged."),
        "telnet" => (false, "os-telnetd-adapter", "Telnet is an OS telnetd/inetd service; Rust never exposes it as an HTTP alias."),
        _ => (false, "unknown", "No built-in protocol runtime is registered for this service."),
    };
    ProtocolRuntimeStatus {
        service: service.to_string(),
        builtin,
        running,
        configured_enabled: cfg.enabled,
        port,
        bind_addr: "0.0.0.0".into(),
        mode: mode.into(),
        note: note.into(),
    }
}

pub fn apply_service_change(state: &AppState, service: &str, enabled: bool) -> ProtocolApplyResult {
    let service = normalize_service(service).to_string();
    match service.as_str() {
        "ftp" => apply_ftp(&state.config, enabled),
        "tftp" => apply_tftp(&state.config, enabled),
        "webdav" => ProtocolApplyResult {
            ok: true,
            service,
            builtin: true,
            action: if enabled { "enable" } else { "disable" }.into(),
            port: state.config.listen_port,
            running: enabled,
            message: if enabled { "WebDAV is served natively by the Rust HTTP router at /webdav/." } else { "WebDAV disabled in ArozOS settings." }.into(),
        },
        "sftp" => ProtocolApplyResult {
            ok: true,
            service,
            builtin: false,
            action: if enabled { "enable" } else { "disable" }.into(),
            port: load_config(&state.config, "sftp", Some(22)).port.unwrap_or(22),
            running: false,
            message: "SFTP is implemented through the OS sshd subsystem. Use the native service adapter/config plan; no HTTP alias is used.".into(),
        },
        "samba" => ProtocolApplyResult {
            ok: true,
            service,
            builtin: false,
            action: if enabled { "enable" } else { "disable" }.into(),
            port: 445,
            running: false,
            message: "SMB/CIFS is implemented through the OS Samba daemon. Use the native service adapter/config plan; no HTTP alias is used.".into(),
        },
        "telnet" => ProtocolApplyResult {
            ok: true,
            service,
            builtin: false,
            action: if enabled { "enable" } else { "disable" }.into(),
            port: load_config(&state.config, "telnet", Some(23)).port.unwrap_or(23),
            running: false,
            message: "Telnet is implemented through a native OS telnetd/inetd adapter only; it is not exposed as an HTTP alias.".into(),
        },
        other => ProtocolApplyResult { ok: false, service: other.into(), builtin: false, action: if enabled { "enable" } else { "disable" }.into(), port: 0, running: false, message: "unknown protocol service".into() },
    }
}

fn apply_ftp(config: &Config, enabled: bool) -> ProtocolApplyResult {
    let cfg = load_config(config, "ftp", Some(2121));
    let port = cfg.port.unwrap_or(2121);
    if !enabled {
        stop_task(&FTP_TASK);
        return ProtocolApplyResult { ok: true, service: "ftp".into(), builtin: true, action: "stop".into(), port, running: false, message: "built-in Rust FTP server stopped".into() };
    }
    stop_task(&FTP_TASK);
    let root = PathBuf::from(&config.root_directory);
    let system_root = config.system_root.clone();
    let bind = format!("0.0.0.0:{port}");
    let handle = tokio::spawn(async move {
        if let Err(e) = run_ftp_server(bind.clone(), root, system_root).await {
            tracing::error!(service="ftp", error=%e, "built-in FTP server stopped");
        }
    });
    *FTP_TASK.lock().expect("ftp task mutex") = Some(handle);
    ProtocolApplyResult { ok: true, service: "ftp".into(), builtin: true, action: "start".into(), port, running: true, message: "built-in Rust FTP server start requested".into() }
}

fn apply_tftp(config: &Config, enabled: bool) -> ProtocolApplyResult {
    let cfg = load_config(config, "tftp", Some(6969));
    let port = cfg.port.unwrap_or(6969);
    if !enabled {
        stop_task(&TFTP_TASK);
        return ProtocolApplyResult { ok: true, service: "tftp".into(), builtin: true, action: "stop".into(), port, running: false, message: "built-in Rust TFTP server stopped".into() };
    }
    stop_task(&TFTP_TASK);
    let root = PathBuf::from(&config.root_directory);
    let bind = format!("0.0.0.0:{port}");
    let handle = tokio::spawn(async move {
        if let Err(e) = run_tftp_server(bind.clone(), root).await {
            tracing::error!(service="tftp", error=%e, "built-in TFTP server stopped");
        }
    });
    *TFTP_TASK.lock().expect("tftp task mutex") = Some(handle);
    ProtocolApplyResult { ok: true, service: "tftp".into(), builtin: true, action: "start".into(), port, running: true, message: "built-in Rust TFTP server start requested".into() }
}

fn stop_task(slot: &Lazy<Mutex<Option<JoinHandle<()>>>>) {
    if let Ok(mut guard) = slot.lock() {
        if let Some(handle) = guard.take() { handle.abort(); }
    }
}

fn load_config(config: &Config, name: &str, default_port: Option<u16>) -> ProtocolConfig {
    let path = utils::data_file(&config.system_root, &format!("service_{}.json", name));
    let mut cfg: ProtocolConfig = utils::read_json_file(&path).unwrap_or_else(|_| ProtocolConfig { name: name.into(), enabled: false, port: default_port, upnp: false, options: serde_json::json!({}) });
    if cfg.name.is_empty() { cfg.name = name.into(); }
    if cfg.port.is_none() { cfg.port = default_port; }
    cfg
}

fn default_port(service: &str) -> Option<u16> {
    match normalize_service(service) {
        "ftp" => Some(2121),
        "tftp" => Some(6969),
        "sftp" => Some(22),
        "telnet" => Some(23),
        _ => None,
    }
}

fn normalize_service(service: &str) -> &str {
    let s = service.trim();
    if s.eq_ignore_ascii_case("smb") || s.eq_ignore_ascii_case("smbd") || s.eq_ignore_ascii_case("samba") { return "samba"; }
    if s.eq_ignore_ascii_case("ssh") || s.eq_ignore_ascii_case("sshd") || s.eq_ignore_ascii_case("sftp") { return "sftp"; }
    if s.eq_ignore_ascii_case("webdav") { return "webdav"; }
    if s.eq_ignore_ascii_case("ftp") { return "ftp"; }
    if s.eq_ignore_ascii_case("tftp") { return "tftp"; }
    if s.eq_ignore_ascii_case("telnet") || s.eq_ignore_ascii_case("telnetd") { return "telnet"; }
    service
}

async fn run_ftp_server(bind: String, root: PathBuf, system_root: String) -> io::Result<()> {
    fs::create_dir_all(&root)?;
    let listener = TcpListener::bind(&bind).await?;
    tracing::info!(addr=%bind, root=%root.display(), "built-in Rust FTP server listening");
    loop {
        let (socket, peer) = listener.accept().await?;
        let root = root.clone();
        let system_root = system_root.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_ftp_client(socket, root, system_root).await {
                tracing::debug!(peer=%peer, error=%e, "FTP client session ended");
            }
        });
    }
}

async fn handle_ftp_client(socket: TcpStream, root: PathBuf, system_root: String) -> io::Result<()> {
    let local_ip = socket.local_addr().ok().map(|a| a.ip().to_string()).unwrap_or_else(|| "127.0.0.1".into());
    let (reader_half, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader_half);
    let mut username = String::new();
    let mut authenticated = false;
    let mut cwd = String::from("/");
    let mut passive: Option<TcpListener> = None;
    let mut rename_from: Option<PathBuf> = None;
    ftp_reply(&mut writer, 220, "ArozOS Rust FTP ready").await?;
    let mut line = String::new();
    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 { break; }
        let trimmed = line.trim_end_matches(|c| c == '\r' || c == '\n');
        let (cmd, arg) = split_ftp_command(trimmed);
        let cmd = cmd.to_ascii_uppercase();
        match cmd.as_str() {
            "USER" => { username = arg.unwrap_or("").to_string(); ftp_reply(&mut writer, 331, "Password required").await?; }
            "PASS" => {
                let password = arg.unwrap_or("");
                authenticated = verify_password(&system_root, &username, password);
                if authenticated { ftp_reply(&mut writer, 230, "Login successful").await?; } else { ftp_reply(&mut writer, 530, "Login incorrect").await?; }
            }
            "SYST" => ftp_reply(&mut writer, 215, "UNIX Type: L8").await?,
            "FEAT" => { writer.write_all(b"211-Features\r\n UTF8\r\n SIZE\r\n MDTM\r\n PASV\r\n EPSV\r\n211 End\r\n").await?; }
            "OPTS" => ftp_reply(&mut writer, 200, "OK").await?,
            "NOOP" => ftp_reply(&mut writer, 200, "OK").await?,
            "TYPE" => ftp_reply(&mut writer, 200, "Type set").await?,
            "PWD" | "XPWD" => ftp_reply(&mut writer, 257, &format!("\"{}\"", cwd)).await?,
            "CWD" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let target = safe_path(&root, &cwd, arg.unwrap_or(""));
                if target.is_dir() {
                    cwd = virtual_path(&root, &target);
                    ftp_reply(&mut writer, 250, "Directory changed").await?;
                } else { ftp_reply(&mut writer, 550, "Not a directory").await?; }
            }
            "CDUP" => { cwd = parent_virtual_path(&cwd); ftp_reply(&mut writer, 200, "Directory changed").await?; }
            "PASV" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let listener = TcpListener::bind("0.0.0.0:0").await?;
                let port = listener.local_addr()?.port();
                passive = Some(listener);
                let ip = local_ip.replace(':', ".").replace('.', ",");
                ftp_reply(&mut writer, 227, &format!("Entering Passive Mode ({},{},{})", ip, port / 256, port % 256)).await?;
            }
            "EPSV" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let listener = TcpListener::bind("0.0.0.0:0").await?;
                let port = listener.local_addr()?.port();
                passive = Some(listener);
                ftp_reply(&mut writer, 229, &format!("Entering Extended Passive Mode (|||{}|)", port)).await?;
            }
            "LIST" | "NLST" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let Some(listener) = passive.take() else { ftp_reply(&mut writer, 425, "Use PASV first").await?; continue; };
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                ftp_reply(&mut writer, 150, "Opening data connection").await?;
                let (mut data, _) = listener.accept().await?;
                let listing = ftp_listing(&root, &path, cmd == "NLST").unwrap_or_else(|_| String::new());
                data.write_all(listing.as_bytes()).await?;
                data.shutdown().await.ok();
                ftp_reply(&mut writer, 226, "Transfer complete").await?;
            }
            "RETR" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let Some(listener) = passive.take() else { ftp_reply(&mut writer, 425, "Use PASV first").await?; continue; };
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                if !path.is_file() { ftp_reply(&mut writer, 550, "File not found").await?; continue; }
                ftp_reply(&mut writer, 150, "Opening data connection").await?;
                let (mut data, _) = listener.accept().await?;
                let mut file = tokio_fs::File::open(path).await?;
                tokio::io::copy(&mut file, &mut data).await?;
                data.shutdown().await.ok();
                ftp_reply(&mut writer, 226, "Transfer complete").await?;
            }
            "STOR" => {
                if !authenticated { ftp_reply(&mut writer, 530, "Login first").await?; continue; }
                let Some(listener) = passive.take() else { ftp_reply(&mut writer, 425, "Use PASV first").await?; continue; };
                let path = safe_path(&root, &cwd, arg.unwrap_or("upload.bin"));
                if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
                ftp_reply(&mut writer, 150, "Opening data connection").await?;
                let (mut data, _) = listener.accept().await?;
                let mut file = tokio_fs::File::create(path).await?;
                tokio::io::copy(&mut data, &mut file).await?;
                ftp_reply(&mut writer, 226, "Transfer complete").await?;
            }
            "DELE" => {
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                match fs::remove_file(path) { Ok(_) => ftp_reply(&mut writer, 250, "Deleted").await?, Err(_) => ftp_reply(&mut writer, 550, "Delete failed").await? }
            }
            "MKD" | "XMKD" => {
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                match fs::create_dir_all(path) { Ok(_) => ftp_reply(&mut writer, 257, "Directory created").await?, Err(_) => ftp_reply(&mut writer, 550, "Create directory failed").await? }
            }
            "RMD" | "XRMD" => {
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                match fs::remove_dir_all(path) { Ok(_) => ftp_reply(&mut writer, 250, "Directory removed").await?, Err(_) => ftp_reply(&mut writer, 550, "Remove directory failed").await? }
            }
            "RNFR" => { rename_from = Some(safe_path(&root, &cwd, arg.unwrap_or(""))); ftp_reply(&mut writer, 350, "Ready for RNTO").await?; }
            "RNTO" => {
                let Some(src) = rename_from.take() else { ftp_reply(&mut writer, 503, "Use RNFR first").await?; continue; };
                let dst = safe_path(&root, &cwd, arg.unwrap_or(""));
                match fs::rename(src, dst) { Ok(_) => ftp_reply(&mut writer, 250, "Renamed").await?, Err(_) => ftp_reply(&mut writer, 550, "Rename failed").await? }
            }
            "SIZE" => {
                let path = safe_path(&root, &cwd, arg.unwrap_or(""));
                match fs::metadata(path) { Ok(m) => ftp_reply(&mut writer, 213, &m.len().to_string()).await?, Err(_) => ftp_reply(&mut writer, 550, "File not found").await? }
            }
            "MDTM" => ftp_reply(&mut writer, 213, "19700101000000").await?,
            "QUIT" => { ftp_reply(&mut writer, 221, "Bye").await?; break; }
            _ => ftp_reply(&mut writer, 502, "Command not implemented").await?,
        }
    }
    Ok(())
}

async fn ftp_reply<W: AsyncWrite + Unpin>(writer: &mut W, code: u16, msg: &str) -> io::Result<()> {
    writer.write_all(format!("{} {}\r\n", code, msg).as_bytes()).await
}

fn split_ftp_command(line: &str) -> (&str, Option<&str>) {
    match line.split_once(' ') { Some((c, a)) => (c, Some(a.trim())), None => (line, None) }
}

fn verify_password(system_root: &str, username: &str, password: &str) -> bool {
    if username.trim().is_empty() { return false; }
    let path = utils::data_file(system_root, "auth_users.json");
    let Ok(db) = utils::read_json_file::<AuthDb>(&path) else { return false; };
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    let digest = format!("{:x}", h.finalize());
    db.passwords.get(&username.trim().to_ascii_lowercase()).map(|p| p == &digest).unwrap_or(false)
}

fn safe_path(root: &Path, cwd: &str, arg: &str) -> PathBuf {
    let raw = arg.trim().trim_matches('"');
    let mut rel = if raw.starts_with('/') { raw.trim_start_matches('/').to_string() } else {
        let base = cwd.trim_start_matches('/').trim_end_matches('/');
        if raw.is_empty() { base.to_string() } else if base.is_empty() { raw.to_string() } else { format!("{}/{}", base, raw) }
    };
    rel = utils::sanitize_relative_path(&utils::percent_decode(&rel));
    root.join(rel)
}

fn virtual_path(root: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(root).unwrap_or(path).to_string_lossy().replace('\\', "/");
    if rel.is_empty() { "/".into() } else { format!("/{}", rel) }
}

fn parent_virtual_path(cwd: &str) -> String {
    let mut parts: Vec<&str> = cwd.trim_matches('/').split('/').filter(|s| !s.is_empty()).collect();
    parts.pop();
    if parts.is_empty() { "/".into() } else { format!("/{}", parts.join("/")) }
}

fn ftp_listing(root: &Path, path: &Path, names_only: bool) -> io::Result<String> {
    let target = if path.is_dir() { path.to_path_buf() } else { path.parent().unwrap_or(root).to_path_buf() };
    let mut out = String::new();
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        let name = entry.file_name().to_string_lossy().to_string();
        if names_only {
            out.push_str(&name);
            out.push_str("\r\n");
        } else {
            let kind = if meta.is_dir() { 'd' } else { '-' };
            out.push_str(&format!("{}rw-r--r-- 1 owner group {:>12} Jan 01 00:00 {}\r\n", kind, meta.len(), name));
        }
    }
    Ok(out)
}

async fn run_tftp_server(bind: String, root: PathBuf) -> io::Result<()> {
    fs::create_dir_all(&root)?;
    let socket = UdpSocket::bind(&bind).await?;
    tracing::info!(addr=%bind, root=%root.display(), "built-in Rust TFTP server listening");
    let mut buf = vec![0u8; 2048];
    loop {
        let (len, peer) = socket.recv_from(&mut buf).await?;
        let packet = buf[..len].to_vec();
        let root = root.clone();
        tokio::spawn(async move {
            if let Err(e) = handle_tftp_request(packet, peer, root).await {
                tracing::debug!(error=%e, "TFTP request failed");
            }
        });
    }
}

async fn handle_tftp_request(packet: Vec<u8>, peer: std::net::SocketAddr, root: PathBuf) -> io::Result<()> {
    if packet.len() < 4 { return Ok(()); }
    let opcode = u16::from_be_bytes([packet[0], packet[1]]);
    let filename = parse_tftp_filename(&packet[2..]).unwrap_or_default();
    let path = root.join(utils::sanitize_relative_path(&utils::percent_decode(&filename)));
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    match opcode {
        1 => tftp_send_file(socket, peer, path).await,
        2 => tftp_recv_file(socket, peer, path).await,
        _ => Ok(()),
    }
}

fn parse_tftp_filename(data: &[u8]) -> Option<String> {
    let end = data.iter().position(|b| *b == 0)?;
    Some(String::from_utf8_lossy(&data[..end]).to_string())
}

async fn tftp_send_file(socket: UdpSocket, peer: std::net::SocketAddr, path: PathBuf) -> io::Result<()> {
    let mut file = match tokio_fs::File::open(path).await { Ok(f) => f, Err(_) => { tftp_error(&socket, peer, 1, "file not found").await?; return Ok(()); } };
    let mut block: u16 = 1;
    loop {
        let mut data = vec![0u8; 512];
        let n = file.read(&mut data).await?;
        data.truncate(n);
        let mut pkt = Vec::with_capacity(4 + data.len());
        pkt.extend_from_slice(&3u16.to_be_bytes());
        pkt.extend_from_slice(&block.to_be_bytes());
        pkt.extend_from_slice(&data);
        socket.send_to(&pkt, peer).await?;
        let mut ack = [0u8; 8];
        let _ = timeout(Duration::from_secs(5), socket.recv_from(&mut ack)).await;
        if n < 512 { break; }
        block = block.wrapping_add(1);
    }
    Ok(())
}

async fn tftp_recv_file(socket: UdpSocket, peer: std::net::SocketAddr, path: PathBuf) -> io::Result<()> {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent)?; }
    let mut file = tokio_fs::File::create(path).await?;
    let mut expected: u16 = 1;
    tftp_ack(&socket, peer, 0).await?;
    let mut buf = vec![0u8; 1024];
    loop {
        let Ok(Ok((len, from))) = timeout(Duration::from_secs(10), socket.recv_from(&mut buf)).await else { break; };
        if from != peer || len < 4 { continue; }
        let opcode = u16::from_be_bytes([buf[0], buf[1]]);
        let block = u16::from_be_bytes([buf[2], buf[3]]);
        if opcode == 3 && block == expected {
            file.write_all(&buf[4..len]).await?;
            tftp_ack(&socket, peer, block).await?;
            expected = expected.wrapping_add(1);
            if len < 516 { break; }
        }
    }
    Ok(())
}

async fn tftp_ack(socket: &UdpSocket, peer: std::net::SocketAddr, block: u16) -> io::Result<()> {
    let mut pkt = Vec::with_capacity(4);
    pkt.extend_from_slice(&4u16.to_be_bytes());
    pkt.extend_from_slice(&block.to_be_bytes());
    socket.send_to(&pkt, peer).await.map(|_| ())
}

async fn tftp_error(socket: &UdpSocket, peer: std::net::SocketAddr, code: u16, message: &str) -> io::Result<()> {
    let mut pkt = Vec::new();
    pkt.extend_from_slice(&5u16.to_be_bytes());
    pkt.extend_from_slice(&code.to_be_bytes());
    pkt.extend_from_slice(message.as_bytes());
    pkt.push(0);
    socket.send_to(&pkt, peer).await.map(|_| ())
}
