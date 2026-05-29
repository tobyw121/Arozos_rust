use serde::Serialize;
use std::{env, fs, path::{Path, PathBuf}, process::Command};

#[derive(Debug, Clone, Serialize)]
pub struct ServiceAdapterInfo {
    pub platform: String,
    pub family: String,
    pub arch: String,
    pub service_manager: String,
    pub can_execute_service_ops: bool,
    pub requires_privilege: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ManagedServiceInfo {
    pub id: String,
    pub configured_enabled: bool,
    pub native_available: bool,
    pub active: bool,
    pub manager: String,
    pub primary_service_name: String,
    pub service_names: Vec<String>,
    pub executables: Vec<String>,
    pub found_executables: Vec<String>,
    pub status_text: String,
    pub start_command: Vec<String>,
    pub stop_command: Vec<String>,
    pub reload_command: Vec<String>,
    pub package_hint: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceActionResult {
    pub ok: bool,
    pub id: String,
    pub action: String,
    pub executed: bool,
    pub command: Vec<String>,
    pub stdout: String,
    pub stderr: String,
    pub status: Option<i32>,
    pub message: String,
}

pub fn adapter_info() -> ServiceAdapterInfo {
    let platform = env::consts::OS.to_string();
    let family = env::consts::FAMILY.to_string();
    let arch = env::consts::ARCH.to_string();
    let manager = detect_service_manager();
    let mut notes = Vec::new();
    notes.push("Rust stores ArozOS UI configuration internally and can optionally control native OS services.".to_string());
    notes.push("Native start/stop is disabled unless AROZOS_RS_ENABLE_SYSTEM_SERVICE_OPS=1 is set.".to_string());
    if manager == "unknown" {
        notes.push("No supported service manager was detected; UI state remains internal unless an adapter is added.".to_string());
    }
    ServiceAdapterInfo {
        platform,
        family,
        arch,
        service_manager: manager,
        can_execute_service_ops: service_ops_enabled(),
        requires_privilege: true,
        notes,
    }
}

pub fn service_status(id: &str, configured_enabled: bool) -> ManagedServiceInfo {
    let id = normalize_id(id);
    let manager = detect_service_manager();
    let service_names = service_candidates(id);
    let executables = executable_candidates(id);
    let found_executables: Vec<String> = executables.iter().filter(|e| command_exists(e)).map(|s| s.to_string()).collect();
    let (primary, status_text) = pick_service_candidate(&manager, &service_names).unwrap_or_else(|| {
        let primary = service_names.first().cloned().unwrap_or_else(|| id.to_string());
        let status = native_status_text(&manager, &primary);
        (primary, status)
    });
    let status_lower = status_text.to_ascii_lowercase();
    let native_active = (status_lower.contains("active") && !status_lower.contains("inactive") && !status_lower.contains("not active"))
        || status_lower.contains("running")
        || status_lower.contains("started");
    let native_available = !found_executables.is_empty() || manager_service_known(&manager, &primary, &status_text) || id == "webdav";
    ManagedServiceInfo {
        id: id.to_string(),
        configured_enabled,
        native_available,
        // active reflects the native OS daemon only. The requested ArozOS state is kept separately
        // in configured_enabled so disabling a service in the WebUI does not remain stuck "on"
        // merely because an unrelated OS daemon is still running.
        active: native_active,
        manager: manager.clone(),
        primary_service_name: primary.clone(),
        service_names,
        executables,
        found_executables,
        status_text,
        start_command: service_command(&manager, &primary, "start"),
        stop_command: service_command(&manager, &primary, "stop"),
        reload_command: service_command(&manager, &primary, "reload"),
        package_hint: package_hint(id),
    }
}

pub fn all_statuses(configured: &[(&str, bool)]) -> Vec<ManagedServiceInfo> {
    configured.iter().map(|(id, en)| service_status(id, *en)).collect()
}

pub fn perform_service_action(id: &str, action: &str) -> ServiceActionResult {
    let id = normalize_id(id).to_string();
    let manager = detect_service_manager();
    let candidates = service_candidates(&id);
    let primary = pick_service_candidate(&manager, &candidates).map(|(name, _)| name).unwrap_or_else(|| candidates.first().cloned().unwrap_or_else(|| id.clone()));
    let action = match action {
        "enable" | "start" | "activate" => "start",
        "disable" | "stop" | "deactivate" => "stop",
        "restart" => "restart",
        "reload" => "reload",
        "status" => "status",
        other => other,
    }.to_string();
    if id == "webdav" {
        return ServiceActionResult { ok: true, id, action, executed: false, command: Vec::new(), stdout: String::new(), stderr: String::new(), status: Some(0), message: "WebDAV is served by the built-in Rust /webdav endpoint; no external daemon is required".into() };
    }
    let command = service_command(&manager, &primary, &action);
    if command.is_empty() {
        return ServiceActionResult {
            ok: false,
            id,
            action,
            executed: false,
            command,
            stdout: String::new(),
            stderr: String::new(),
            status: None,
            message: format!("no service adapter for manager '{}'; install/configure the service manually", manager),
        };
    }
    if !service_ops_enabled() && action != "status" {
        return ServiceActionResult {
            ok: false,
            id,
            action,
            executed: false,
            command,
            stdout: String::new(),
            stderr: String::new(),
            status: None,
            message: "native service control requires root/Administrator or AROZOS_RS_ENABLE_SYSTEM_SERVICE_OPS=1; configuration was saved but the OS daemon was not started".to_string(),
        };
    }
    let mut cmd = Command::new(&command[0]);
    if command.len() > 1 { cmd.args(&command[1..]); }
    match cmd.output() {
        Ok(out) => ServiceActionResult {
            ok: out.status.success(),
            id,
            action,
            executed: true,
            command,
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            status: out.status.code(),
            message: if out.status.success() { "native service command completed".into() } else { "native service command failed".into() },
        },
        Err(e) => ServiceActionResult {
            ok: false,
            id,
            action,
            executed: true,
            command,
            stdout: String::new(),
            stderr: e.to_string(),
            status: None,
            message: "failed to launch native service command".into(),
        }
    }
}

pub fn local_addresses() -> Vec<String> {
    let mut out = Vec::new();
    #[cfg(unix)]
    {
        let commands = [
            "hostname -I 2>/dev/null",
            "ip -o -4 addr show scope global 2>/dev/null | awk '{print $4}' | cut -d/ -f1",
            "ifconfig 2>/dev/null | awk '/inet /{print $2}'",
        ];
        for script in commands {
            if let Ok(cmd) = Command::new("sh").arg("-c").arg(script).output() {
                for token in String::from_utf8_lossy(&cmd.stdout).split_whitespace() {
                    let cleaned = token.trim().trim_start_matches("addr:").to_string();
                    if cleaned.parse::<std::net::IpAddr>().is_ok() && !out.iter().any(|x| x == &cleaned) {
                        out.push(cleaned);
                    }
                }
            }
            if !out.is_empty() { break; }
        }
    }
    #[cfg(windows)]
    {
        if let Ok(cmd) = Command::new("powershell").args(["-NoProfile", "-Command", "Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -notlike '169.254.*'} | Select-Object -ExpandProperty IPAddress"]).output() {
            for token in String::from_utf8_lossy(&cmd.stdout).split_whitespace() {
                if token.parse::<std::net::IpAddr>().is_ok() && !out.iter().any(|x| x == token) {
                    out.push(token.to_string());
                }
            }
        }
    }
    out.retain(|a| !a.is_empty());
    out.sort();
    out.dedup();
    if out.is_empty() { out.push("127.0.0.1".into()); }
    out
}

fn service_ops_enabled() -> bool {
    if matches!(env::var("AROZOS_RS_ENABLE_SYSTEM_SERVICE_OPS").ok().as_deref(), Some("1") | Some("true") | Some("yes") | Some("on")) { return true; }
    is_privileged_process()
}

fn is_privileged_process() -> bool {
    #[cfg(unix)]
    {
        if let Ok(out) = Command::new("id").arg("-u").output() {
            return String::from_utf8_lossy(&out.stdout).trim() == "0";
        }
        false
    }
    #[cfg(windows)]
    {
        matches!(env::var("USERNAME").ok().as_deref(), Some("Administrator"))
    }
    #[cfg(not(any(unix, windows)))]
    { false }
}

fn normalize_id(id: &str) -> &str {
    match id {
        "smb" | "smbd" | "samba" => "samba",
        "ssh" | "sshd" | "sftp" => "sftp",
        "web_dav" | "webdav" => "webdav",
        "ftp" | "ftpd" => "ftp",
        "tftp" | "tftpd" => "tftp",
        "telnet" | "telnetd" => "telnet",
        other => other,
    }
}

fn detect_service_manager() -> String {
    if cfg!(target_os = "windows") { return "windows-sc".into(); }
    if cfg!(target_os = "macos") { return "launchctl".into(); }
    if command_exists("systemctl") && Path::new("/run/systemd/system").exists() { return "systemd".into(); }
    if command_exists("rc-service") { return "openrc".into(); }
    if command_exists("service") { return "sysvinit".into(); }
    "unknown".into()
}

fn command_exists(cmd: &str) -> bool {
    if cmd.contains('/') { return Path::new(cmd).exists(); }
    let paths = env::var_os("PATH").unwrap_or_default();
    for dir in env::split_paths(&paths) {
        let candidate = dir.join(cmd);
        if is_executable(&candidate) { return true; }
        #[cfg(windows)]
        {
            for ext in ["exe", "bat", "cmd", "ps1"] {
                if is_executable(&candidate.with_extension(ext)) { return true; }
            }
        }
    }
    for dir in common_binary_dirs() {
        if is_executable(&dir.join(cmd)) { return true; }
    }
    false
}

fn common_binary_dirs() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/usr/sbin"), PathBuf::from("/usr/bin"), PathBuf::from("/sbin"), PathBuf::from("/bin"),
        PathBuf::from("/usr/local/sbin"), PathBuf::from("/usr/local/bin"), PathBuf::from("/opt/homebrew/bin"), PathBuf::from("/opt/homebrew/sbin"),
    ]
}

fn is_executable(path: &Path) -> bool {
    fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
}

fn service_candidates(id: &str) -> Vec<String> {
    match normalize_id(id) {
        "samba" => vec!["smbd", "samba", "smb", "nmbd"],
        "ftp" => vec!["vsftpd", "proftpd", "pure-ftpd", "ftpd"],
        "sftp" => vec!["sshd", "ssh"],
        "webdav" => vec!["arozos-rs-webdav", "apache2", "httpd", "nginx", "caddy"],
        "tftp" => vec!["tftpd-hpa", "in.tftpd", "tftp"],
        "telnet" => vec!["telnetd", "inetd", "xinetd"],
        other => vec![other],
    }.into_iter().map(|s| s.to_string()).collect()
}

fn executable_candidates(id: &str) -> Vec<String> {
    match normalize_id(id) {
        "samba" => vec!["smbd", "nmbd", "samba", "testparm", "smbpasswd", "net"],
        "ftp" => vec!["vsftpd", "proftpd", "pure-ftpd", "ftpd"],
        "sftp" => vec!["sshd", "sftp-server", "internal-sftp"],
        "webdav" => vec!["apache2", "httpd", "nginx", "caddy", "rclone"],
        "tftp" => vec!["in.tftpd", "tftpd", "tftpd-hpa"],
        "telnet" => vec!["telnetd", "in.telnetd", "busybox", "inetd", "xinetd"],
        other => vec![other],
    }.into_iter().map(|s| s.to_string()).collect()
}

fn pick_service_candidate(manager: &str, candidates: &[String]) -> Option<(String, String)> {
    if manager == "unknown" { return None; }
    let mut fallback: Option<(String, String)> = None;
    for service in candidates {
        let status = native_status_text(manager, service);
        let lower = status.to_ascii_lowercase();
        let active = (lower.contains("active") && !lower.contains("inactive") && !lower.contains("not active"))
            || lower.contains("running")
            || lower.contains("started");
        if active { return Some((service.clone(), status)); }
        if manager_service_known(manager, service, &status) && fallback.is_none() {
            fallback = Some((service.clone(), status));
        }
    }
    fallback
}

fn manager_service_known(manager: &str, _service: &str, status: &str) -> bool {
    if manager == "unknown" { return false; }
    let s = status.to_ascii_lowercase();
    !(s.contains("could not be found") || s.contains("not-found") || s.contains("not found") || s.contains("does not exist") || s.contains("unrecognized service"))
}

fn native_status_text(manager: &str, service: &str) -> String {
    let cmd = service_command(manager, service, "status");
    if cmd.is_empty() { return "unknown".into(); }
    let mut p = Command::new(&cmd[0]);
    if cmd.len() > 1 { p.args(&cmd[1..]); }
    match p.output() {
        Ok(out) => {
            let mut text = String::from_utf8_lossy(&out.stdout).to_string();
            if text.trim().is_empty() { text = String::from_utf8_lossy(&out.stderr).to_string(); }
            text.lines().take(4).collect::<Vec<_>>().join(" | ")
        },
        Err(e) => e.to_string(),
    }
}

fn service_command(manager: &str, service: &str, action: &str) -> Vec<String> {
    match manager {
        "systemd" => vec!["systemctl".into(), action.into(), service.into()],
        "openrc" => vec!["rc-service".into(), service.into(), action.into()],
        "sysvinit" => vec!["service".into(), service.into(), action.into()],
        "launchctl" => {
            let verb = match action { "start" => "kickstart", "stop" => "bootout", "restart" => "kickstart", "reload" => "kickstart", "status" => "print", other => other };
            let label = match service { "sshd" | "ssh" => "system/com.openssh.sshd", "smbd" | "samba" => "system/com.apple.smbd", other => other };
            vec!["launchctl".into(), verb.into(), label.into()]
        },
        "windows-sc" => {
            let name = match service { "sshd" | "ssh" => "sshd", "smbd" | "samba" => "LanmanServer", "vsftpd" | "ftp" => "ftpsvc", other => other };
            let verb = match action { "start" => "start", "stop" => "stop", "restart" => "query", "reload" => "query", "status" => "query", other => other };
            vec!["sc".into(), verb.into(), name.into()]
        },
        _ => Vec::new(),
    }
}

fn package_hint(id: &str) -> String {
    let os = env::consts::OS;
    match (os, normalize_id(id)) {
        ("linux", "samba") => "Debian/Ubuntu: apt install samba; Alpine: apk add samba; Arch: pacman -S samba".into(),
        ("linux", "ftp") => "Debian/Ubuntu: apt install vsftpd; Alpine: apk add vsftpd; Arch: pacman -S vsftpd".into(),
        ("linux", "sftp") => "Debian/Ubuntu: apt install openssh-server; Alpine: apk add openssh; Arch: pacman -S openssh".into(),
        ("linux", "webdav") => "Use the built-in Rust WebDAV compatibility endpoint, or install apache2/nginx/caddy with WebDAV module.".into(),
        ("macos", "samba") => "macOS includes SMB file sharing; enable via System Settings > General > Sharing or launchctl.".into(),
        ("macos", "sftp") => "macOS includes OpenSSH; enable Remote Login in Sharing settings.".into(),
        ("windows", "samba") => "Windows SMB is provided by LanmanServer; enable File and Printer Sharing.".into(),
        ("windows", "sftp") => "Install OpenSSH Server optional feature and start service sshd.".into(),
        ("linux", "telnet") => "Telnet is insecure. If required for legacy devices: Debian/Ubuntu: apt install telnetd/openbsd-inetd; Alpine: apk add busybox-extras; Arch: pacman -S inetutils".into(),
        ("macos", "telnet") => "Install/enable a Telnet daemon only for isolated legacy environments; SSH/SFTP is recommended.".into(),
        ("windows", "telnet") => "Enable the Telnet Server optional feature only in trusted legacy environments; SSH/SFTP is recommended.".into(),
        (_, "webdav") => "Use the built-in Rust WebDAV compatibility endpoint where native WebDAV service is unavailable.".into(),
        _ => "Install a native server package for this platform or use Rust-managed compatibility mode.".into(),
    }
}
