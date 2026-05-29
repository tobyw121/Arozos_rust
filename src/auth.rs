use axum::{body::Body, extract::State, http::{header, HeaderMap, Request, StatusCode}, response::{IntoResponse, Response}, Json};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::{HashMap, HashSet}, path::PathBuf, sync::{Arc, RwLock}, time::{SystemTime, UNIX_EPOCH}};
use uuid::Uuid;

use crate::{state::AppState, utils};

pub const MIN_PASSWORD_LEN: usize = 8;
pub const PASSWORD_POLICY_TEXT: &str = "Password must be at least 8 characters. Letters, numbers, spaces and symbols are allowed; line breaks and control characters are not allowed.";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub username: String,
    pub display_name: String,
    pub group: String,
    pub is_admin: bool,
    pub disabled: bool,
    pub profile_image: String,
    pub created_unix: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserSession { pub username: String, pub is_admin: bool }

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct PersistedAuth {
    users: HashMap<String, UserRecord>,
    passwords: HashMap<String, String>,
}

#[derive(Debug, Default)]
pub struct AuthService {
    auth_file: PathBuf,
    users: RwLock<HashMap<String, UserRecord>>,
    passwords: RwLock<HashMap<String, String>>,
    sessions: RwLock<HashMap<String, UserSession>>,
    whitelist_enabled: RwLock<bool>,
    whitelist: RwLock<HashSet<String>>,
    blacklist_enabled: RwLock<bool>,
    blacklist: RwLock<HashSet<String>>,
    autologin_tokens: RwLock<HashMap<String, String>>,
    login_log: RwLock<Vec<serde_json::Value>>,
}

impl AuthService {
    pub fn new(system_root: &str) -> Self {
        let auth_file = utils::data_file(system_root, "auth_users.json");
        let persisted: PersistedAuth = utils::read_json_file(&auth_file).unwrap_or_default();
        Self {
            auth_file,
            users: RwLock::new(persisted.users),
            passwords: RwLock::new(persisted.passwords),
            sessions: RwLock::new(HashMap::new()),
            whitelist_enabled: RwLock::new(false),
            whitelist: RwLock::new(HashSet::new()),
            blacklist_enabled: RwLock::new(false),
            blacklist: RwLock::new(HashSet::new()),
            autologin_tokens: RwLock::new(HashMap::new()),
            login_log: RwLock::new(Vec::new()),
        }
    }

    fn save_persisted(&self) {
        let payload = PersistedAuth {
            users: self.users.read().unwrap().clone(),
            passwords: self.passwords.read().unwrap().clone(),
        };
        if let Err(e) = utils::write_json_file(&self.auth_file, &payload) {
            tracing::warn!(error=%e, path=%self.auth_file.display(), "failed to persist auth database");
        }
    }

    pub fn user_count(&self) -> usize { self.users.read().unwrap().len() }
    pub fn is_first_run(&self) -> bool { self.user_count() == 0 }

    pub fn verify(&self, username: &str, password: &str) -> bool {
        let username = normalize_username(username);
        let users = self.users.read().unwrap();
        if users.get(&username).map(|u| u.disabled).unwrap_or(true) { return false; }
        self.passwords.read().unwrap().get(&username).map(|h| h == &hash_password(password)).unwrap_or(false)
    }

    pub fn create_session(&self, username: String) -> String {
        let username = normalize_username(&username);
        let token = Uuid::new_v4().to_string();
        let is_admin = self.users.read().unwrap().get(&username).map(|u| u.is_admin).unwrap_or(false);
        self.sessions.write().unwrap().insert(token.clone(), UserSession { username, is_admin });
        token
    }

    pub fn session_from_headers(&self, headers: &HeaderMap) -> Option<UserSession> {
        let cookie = headers.get(header::COOKIE)?.to_str().ok()?;
        for part in cookie.split(';') {
            let p = part.trim();
            if let Some(token) = p.strip_prefix("arozos_session=") {
                return self.sessions.read().unwrap().get(token).cloned();
            }
        }
        None
    }

    pub fn remove_session(&self, token: &str) { self.sessions.write().unwrap().remove(token); }

    pub fn current_user(&self, headers: &HeaderMap) -> Option<UserRecord> {
        let session = self.session_from_headers(headers)?;
        self.users.read().unwrap().get(&session.username).cloned()
    }

    pub fn list_users(&self) -> Vec<UserRecord> {
        let mut users: Vec<_> = self.users.read().unwrap().values().cloned().collect();
        users.sort_by(|a,b| a.username.cmp(&b.username));
        users
    }

    pub fn get_user(&self, username: &str) -> Option<UserRecord> { self.users.read().unwrap().get(&normalize_username(username)).cloned() }

    pub fn user_exists(&self, username: &str) -> bool { self.users.read().unwrap().contains_key(&normalize_username(username)) }

    pub fn upsert_user(&self, username: &str, password: Option<&str>, is_admin: bool, group: Option<&str>, display_name: Option<&str>) -> Result<UserRecord, String> {
        let username = normalize_username(username);
        validate_username(&username)?;
        let first_user = self.user_count() == 0;
        let is_admin = is_admin || first_user || group.map(|g| g.eq_ignore_ascii_case("administrator")).unwrap_or(false);
        let effective_group = group.unwrap_or(if is_admin { "administrator" } else { "user" });
        let now = now_unix();
        let user_out = {
            let mut users = self.users.write().unwrap();
            let user = users.entry(username.clone()).or_insert_with(|| UserRecord {
                username: username.clone(),
                display_name: display_name.unwrap_or(&username).to_string(),
                group: effective_group.to_string(),
                is_admin,
                disabled: false,
                profile_image: "img/desktop/system_icon/user.svg".to_string(),
                created_unix: now,
            });
            if let Some(display_name) = display_name { user.display_name = display_name.to_string(); }
            user.group = effective_group.to_string();
            user.is_admin = is_admin;
            user.clone()
        };
        if let Some(password) = password.filter(|p| !p.is_empty()) {
            validate_password(password)?;
            self.passwords.write().unwrap().insert(username, hash_password(password));
        }
        self.save_persisted();
        Ok(user_out)
    }


    pub fn remove_user(&self, username: &str) -> bool {
        let username = normalize_username(username);
        let removed = {
            self.passwords.write().unwrap().remove(&username);
            self.users.write().unwrap().remove(&username).is_some()
        };
        if removed { self.save_persisted(); }
        removed
    }

    pub fn set_disabled(&self, username: &str, disabled: bool) -> bool {
        let changed = {
            let mut users = self.users.write().unwrap();
            if let Some(u) = users.get_mut(&normalize_username(username)) { u.disabled = disabled; true } else { false }
        };
        if changed { self.save_persisted(); }
        changed
    }

    pub fn set_whitelist_enabled(&self, enabled: bool) { *self.whitelist_enabled.write().unwrap() = enabled; }
    pub fn set_blacklist_enabled(&self, enabled: bool) { *self.blacklist_enabled.write().unwrap() = enabled; }
    pub fn whitelist_state(&self) -> serde_json::Value { serde_json::json!({"enabled": *self.whitelist_enabled.read().unwrap(), "entries": self.whitelist.read().unwrap().iter().cloned().collect::<Vec<_>>()}) }
    pub fn blacklist_state(&self) -> serde_json::Value { serde_json::json!({"enabled": *self.blacklist_enabled.read().unwrap(), "entries": self.blacklist.read().unwrap().iter().cloned().collect::<Vec<_>>()}) }
    pub fn whitelist_add(&self, ip: String) { self.whitelist.write().unwrap().insert(ip); }
    pub fn whitelist_remove(&self, ip: &str) { self.whitelist.write().unwrap().remove(ip); }
    pub fn blacklist_add(&self, ip: String) { self.blacklist.write().unwrap().insert(ip); }
    pub fn blacklist_remove(&self, ip: &str) { self.blacklist.write().unwrap().remove(ip); }

    pub fn create_autologin_token(&self, username: &str) -> Option<(String, String)> {
        if !self.user_exists(username) { return None; }
        let token = Uuid::new_v4().to_string();
        self.autologin_tokens.write().unwrap().insert(token.clone(), normalize_username(username));
        Some((token, username.to_string()))
    }
    pub fn remove_autologin_token(&self, token: &str) -> bool { self.autologin_tokens.write().unwrap().remove(token).is_some() }
    pub fn autologin_tokens(&self) -> Vec<serde_json::Value> { self.autologin_tokens.read().unwrap().iter().map(|(t,u)| serde_json::json!({"token": t, "username": u})).collect() }
    pub fn username_for_token(&self, token: &str) -> Option<String> { self.autologin_tokens.read().unwrap().get(token).cloned() }

    fn log_login(&self, username: &str, ok: bool, reason: &str) {
        let mut log = self.login_log.write().unwrap();
        log.push(serde_json::json!({"time": now_unix(), "username": username, "ok": ok, "reason": reason}));
        if log.len() > 5000 { let drain = log.len() - 5000; log.drain(0..drain); }
    }
    pub fn login_log(&self) -> Vec<serde_json::Value> { self.login_log.read().unwrap().clone() }
}

fn now_unix() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs() }

fn normalize_username(username: &str) -> String { username.trim().to_ascii_lowercase() }

fn validate_username(username: &str) -> Result<(), String> {
    if username.len() < 2 || username.len() > 64 { return Err("username length must be 2..64".into()); }
    if !username.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-' || c == '.') { return Err("username contains invalid characters".into()); }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), String> {
    if password.chars().count() < MIN_PASSWORD_LEN {
        return Err(format!("Password too short. Must be at least {} characters.", MIN_PASSWORD_LEN));
    }
    if password.chars().any(|c| c.is_control()) {
        return Err("Password contains invalid control characters.".into());
    }
    Ok(())
}

fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

fn first_param<'a>(params: &'a std::collections::HashMap<String,String>, keys: &[&str]) -> Option<&'a String> { keys.iter().find_map(|k| params.get(*k)) }

#[derive(Serialize)]
struct LoginResponse<'a> { ok: bool, username: &'a str }

pub async fn login(State(state): State<Arc<AppState>>, req: Request<Body>) -> Response {
    let (params, _) = utils::request_params(req, 1024 * 1024).await;
    let token_value = first_param(&params, &["token", "key", "autologin"]).cloned();
    let token_login = token_value.as_deref().and_then(|t| state.auth.username_for_token(t));
    if token_value.is_some() && token_login.is_none() {
        return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"ok": false, "error": "invalid autologin token"}))).into_response();
    }
    let username = token_login.clone().unwrap_or_else(|| first_param(&params, &["username", "user", "u"]).cloned().unwrap_or_else(|| "admin".to_string()));
    let password = first_param(&params, &["password", "pw", "p"]).cloned().unwrap_or_else(|| "admin".to_string());
    if token_login.is_some() || state.auth.verify(&username, &password) {
        let token = state.auth.create_session(username.clone());
        state.auth.log_login(&username, true, "login");
        let cookie = format!("arozos_session={}; Path=/; HttpOnly; SameSite=Lax", token);
        (StatusCode::OK, [(header::SET_COOKIE, cookie)], Json(LoginResponse { ok: true, username: &username })).into_response()
    } else {
        state.auth.log_login(&username, false, "invalid credentials");
        (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"ok": false, "error": "invalid credentials"}))).into_response()
    }
}

pub async fn logout(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    if let Some(cookie) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) {
        for part in cookie.split(';') {
            if let Some(token) = part.trim().strip_prefix("arozos_session=") { state.auth.remove_session(token); }
        }
    }
    (StatusCode::OK, [(header::SET_COOKIE, "arozos_session=; Path=/; Max-Age=0".to_string())], Json(serde_json::json!({"ok": true}))).into_response()
}

pub async fn check_login(State(state): State<Arc<AppState>>, headers: HeaderMap) -> Response {
    let logged_in = state.auth.session_from_headers(&headers).is_some();
    (StatusCode::OK, Json(serde_json::json!(logged_in))).into_response()
}


pub async fn register(State(state): State<Arc<AppState>>, headers: HeaderMap, req: Request<Body>) -> Response {
    let (params, _) = utils::request_params(req, 1024 * 1024).await;
    let username = first_param(&params, &["username", "user", "u"]).cloned().unwrap_or_default();
    let password = first_param(&params, &["password", "pw", "p"]).map(|s| s.as_str()).unwrap_or("");
    let first_user = state.auth.is_first_run();

    if username.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "Error. Missing 'username' paramter").into_response();
    }
    if password.is_empty() {
        return (StatusCode::BAD_REQUEST, "Error. Missing 'password' paramter").into_response();
    }
    if let Err(e) = validate_password(password) {
        return (StatusCode::BAD_REQUEST, format!("Error. {} {}", e, PASSWORD_POLICY_TEXT)).into_response();
    }

    if !first_user {
        match state.auth.current_user(&headers) {
            Some(user) if user.is_admin => {},
            Some(_) => return (StatusCode::FORBIDDEN, "Error. Permission Denied").into_response(),
            None => return (StatusCode::UNAUTHORIZED, "Error. Login is needed to create new user").into_response(),
        }
    }

    let requested_group = first_param(&params, &["group"]).map(|s| s.as_str()).unwrap_or("user");
    let effective_group = if first_user { "administrator" } else { requested_group };
    let requested_admin = first_user || effective_group.eq_ignore_ascii_case("administrator") || utils::truthy(first_param(&params, &["admin", "isAdmin"]));

    match state.auth.upsert_user(&username, Some(password), requested_admin, Some(effective_group), first_param(&params, &["displayName", "display_name", "nickname"]).map(|s| s.as_str())) {
        Ok(user) => {
            // The original Go first-run flow creates the user and then sends the browser to login.
            // It does not silently authenticate the new account, so keep that behavior here.
            let base = PathBuf::from(&state.config.root_directory).join("users").join(&user.username);
            for folder in ["Desktop", "Document", "Documents", "Download", "Downloads", "Music", "Photo", "Video", "Audio", "Web"] {
                let _ = std::fs::create_dir_all(base.join(folder));
            }
            "OK".into_response()
        },
        Err(e) => (StatusCode::BAD_REQUEST, format!("Error. {}", e)).into_response(),
    }
}
