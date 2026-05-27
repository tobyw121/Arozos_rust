//! Authentication Gateway Module
//! 
//! This module handles user authentication, session management, and access control.
//! Ported from Go's mod/auth/auth.go

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::AppState;

/// Authentication Agent managing sessions and user validation
pub struct AuthAgent {
    /// Session configuration
    pub session_name: String,
    pub session_key: Vec<u8>,
    pub session_timeout_secs: u64,
    
    /// Database reference (using sled)
    pub db: Arc<sled::Db>,
    
    /// Token store for temporary access tokens
    pub token_store: Arc<RwLock<tokio::sync::HashMap<String, TokenInfo>>>,
    
    /// Exponential delay handler for brute-force protection
    pub exp_delay_handler: ExpLoginHandler,
    
    /// Access control lists
    pub whitelist_manager: WhitelistManager,
    pub blacklist_manager: BlacklistManager,
    
    /// Logger for auth events
    pub logger: AuthLogger,
    
    /// Account switcher
    pub switchable_account_manager: SwitchableAccountPoolManager,
}

/// Token information with expiry
#[derive(Clone, Debug)]
pub struct TokenInfo {
    pub username: String,
    pub created_at: std::time::SystemTime,
    pub expires_at: std::time::SystemTime,
}

/// Exponential login delay handler to prevent brute-force attacks
pub struct ExpLoginHandler {
    // Maps username + IP to retry count and last attempt time
    attempts: Arc<RwLock<tokio::sync::HashMap<String, LoginAttempt>>>,
    base_delay_secs: u64,
    max_delay_secs: u64,
}

#[derive(Clone, Debug)]
pub struct LoginAttempt {
    pub count: u32,
    pub last_attempt: std::time::SystemTime,
}

/// Whitelist manager for allowed IPs
pub struct WhitelistManager {
    pub enabled: bool,
    db: Arc<sled::Db>,
}

/// Blacklist manager for banned IPs
pub struct BlacklistManager {
    pub enabled: bool,
    db: Arc<sled::Db>,
}

/// Auth event logger
pub struct AuthLogger {
    db: Arc<sled::Db>,
}

/// Switchable account pool manager
pub struct SwitchableAccountPoolManager {
    db: Arc<sled::Db>,
}

/// User session data
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionData {
    pub authenticated: bool,
    pub username: Option<String>,
    pub remember_me: bool,
}

/// Login request payload
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    #[serde(default)]
    pub rmbme: bool, // remember me
}

/// Register request payload
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub group: String,
}

impl AuthAgent {
    /// Create a new authentication agent
    pub fn new(
        session_name: String,
        session_key: Vec<u8>,
        db: Arc<sled::Db>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Ensure auth table exists
        let auth_tree = db.open_tree("auth")?;
        
        // Create components
        let exp_delay_handler = ExpLoginHandler::new(2, 10800);
        let whitelist_manager = WhitelistManager::new(db.clone());
        let blacklist_manager = BlacklistManager::new(db.clone());
        let logger = AuthLogger::new(db.clone())?;
        let switchable_account_manager = SwitchableAccountPoolManager::new(db.clone());
        
        Ok(Self {
            session_name,
            session_key,
            session_timeout_secs: 3600,
            db,
            token_store: Arc::new(RwLock::new(tokio::sync::HashMap::new())),
            exp_delay_handler,
            whitelist_manager,
            blacklist_manager,
            logger,
            switchable_account_manager,
        })
    }
    
    /// Hash a password using SHA-512
    pub fn hash_password(password: &str) -> String {
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(password.as_bytes());
        hex::encode(hasher.finalize())
    }
    
    /// Validate username format
    pub fn validate_username(username: &str) -> Result<(), &'static str> {
        if username.is_empty() {
            return Err("Username not defined or empty");
        }
        if username.contains('/') || username.contains('\\') {
            return Err("Username contains invalid path characters");
        }
        if username.contains("..") {
            return Err("Username contains invalid path sequence");
        }
        Ok(())
    }
    
    /// Normalize username by trimming whitespace
    pub fn normalize_username(username: &str) -> String {
        username.trim().to_string()
    }
    
    /// Validate username and password against database
    pub async fn validate_credentials(&self, username: &str, password: &str) -> Result<bool, String> {
        let hashed_password = Self::hash_password(password);
        let candidates = vec![
            username.to_string(),
            Self::normalize_username(username),
        ];
        
        let auth_tree = self.db.open_tree("auth").map_err(|e| e.to_string())?;
        
        for candidate in candidates {
            if candidate.is_empty() {
                continue;
            }
            
            let key = format!("passhash/{}", candidate);
            if let Ok(Some(stored_hash)) = auth_tree.get(key.as_bytes()) {
                let stored_hash_str = String::from_utf8_lossy(&stored_hash);
                if stored_hash_str == hashed_password {
                    return Ok(true);
                }
            }
        }
        
        Ok(false)
    }
    
    /// Check if user is authenticated via session
    pub fn check_auth(&self, headers: &HeaderMap) -> bool {
        // In a real implementation, this would parse session cookies
        // For now, we check for an Authorization header with a token
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    // Check token store
                    let token_store = futures::executor::block_on(self.token_store.read());
                    if let Some(token_info) = token_store.get(token) {
                        if token_info.expires_at > std::time::SystemTime::now() {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Get current username from session
    pub fn get_username(&self, headers: &HeaderMap) -> Option<String> {
        if !self.check_auth(headers) {
            return None;
        }
        
        // Extract username from token (simplified)
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let token_store = futures::executor::block_on(self.token_store.read());
                    if let Some(token_info) = token_store.get(token) {
                        return Some(token_info.username.clone());
                    }
                }
            }
        }
        None
    }
    
    /// Handle login request
    pub async fn handle_login(&self, request: LoginRequest, client_ip: &str) -> Result<String, String> {
        let username = Self::normalize_username(&request.username);
        
        // Validate username
        Self::validate_username(&username).map_err(|e| e.to_string())?;
        
        // Check exponential delay handler
        let (allowed, retry_in) = self.exp_delay_handler.allow_access(&username, client_ip).await;
        if !allowed {
            return Err(format!("Too many requests! Next retry in {} seconds", retry_in));
        }
        
        // Validate credentials
        let password_valid = self.validate_credentials(&username, &request.password).await?;
        
        if !password_valid {
            // Increment retry count
            self.exp_delay_handler.add_attempt(&username, client_ip).await;
            self.logger.log_auth_event(client_ip, &username, false).await;
            return Err("Invalid username or password".to_string());
        }
        
        // Check IP access (whitelist/blacklist)
        if !self.validate_ip_access(client_ip)? {
            return Err("IP address not allowed".to_string());
        }
        
        // Reset retry count on successful login
        self.exp_delay_handler.reset_attempts(&username, client_ip).await;
        
        // Generate session token
        let token = self.generate_token(&username, request.rmbme).await?;
        
        // Log successful login
        self.logger.log_auth_event(client_ip, &username, true).await;
        info!("User {} logged in from {}", username, client_ip);
        
        Ok(token)
    }
    
    /// Generate a new access token
    async fn generate_token(&self, username: &str, remember_me: bool) -> Result<String, String> {
        use rand::Rng;
        
        let token = hex::encode(rand::thread_rng().gen::<[u8; 32]>());
        
        let expiry_secs = if remember_me {
            3600 * 24 * 7 // 1 week
        } else {
            3600 // 1 hour
        };
        
        let now = std::time::SystemTime::now();
        let expires_at = now + std::time::Duration::from_secs(expiry_secs);
        
        let token_info = TokenInfo {
            username: username.to_string(),
            created_at: now,
            expires_at,
        };
        
        let mut token_store = self.token_store.write().await;
        token_store.insert(token.clone(), token_info);
        
        Ok(token)
    }
    
    /// Handle logout
    pub async fn handle_logout(&self, headers: &HeaderMap) -> Result<(), String> {
        if let Some(username) = self.get_username(headers) {
            info!("User {} logged out", username);
        }
        
        // Remove token
        if let Some(auth_header) = headers.get("Authorization") {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];
                    let mut token_store = self.token_store.write().await;
                    token_store.remove(token);
                }
            }
        }
        
        Ok(())
    }
    
    /// Validate IP access against whitelist/blacklist
    pub fn validate_ip_access(&self, ip: &str) -> Result<bool, String> {
        if self.whitelist_manager.enabled && !self.whitelist_manager.is_whitelisted(ip)? {
            return Err("Your IP is not whitelisted on this host".to_string());
        }
        
        if self.blacklist_manager.enabled && self.blacklist_manager.is_banned(ip)? {
            return Err("Your IP is banned by this host".to_string());
        }
        
        Ok(true)
    }
    
    /// Create a new user account
    pub async fn create_user(&self, username: &str, password: &str, groups: &[String]) -> Result<(), String> {
        Self::validate_username(username).map_err(|e| e.to_string())?;
        
        let auth_tree = self.db.open_tree("auth").map_err(|e| e.to_string())?;
        
        // Check if user already exists
        let key = format!("passhash/{}", username);
        if auth_tree.contains_key(key.as_bytes()).map_err(|e| e.to_string())? {
            return Err("User already exists".to_string());
        }
        
        // Store hashed password
        let hashed_password = Self::hash_password(password);
        auth_tree.insert(key.as_bytes(), hashed_password.as_bytes())
            .map_err(|e| e.to_string())?;
        
        // Store groups
        for group in groups {
            let group_key = format!("group/{}/{}", username, group);
            auth_tree.insert(group_key.as_bytes(), "true".as_bytes())
                .map_err(|e| e.to_string())?;
        }
        
        info!("New user {} added to system", username);
        Ok(())
    }
    
    /// Delete a user account
    pub async fn delete_user(&self, username: &str) -> Result<(), String> {
        let auth_tree = self.db.open_tree("auth").map_err(|e| e.to_string())?;
        
        // Remove all user-related keys
        let prefix = format!("{}/", username);
        let mut batch = sled::Batch::default();
        
        for item in auth_tree.scan_prefix(prefix.as_bytes()) {
            let (key, _) = item.map_err(|e| e.to_string())?;
            batch.remove(key);
        }
        
        auth_tree.apply_batch(batch).map_err(|e| e.to_string())?;
        
        // Remove autologin tokens
        self.remove_autologin_tokens(username).await;
        
        info!("User {} removed from system", username);
        Ok(())
    }
    
    /// Get user count
    pub fn get_user_count(&self) -> Result<usize, String> {
        let auth_tree = self.db.open_tree("auth").map_err(|e| e.to_string())?;
        let mut count = 0;
        
        for item in auth_tree.scan_prefix(b"passhash/") {
            if item.is_ok() {
                count += 1;
            }
        }
        
        Ok(count)
    }
    
    /// List all usernames
    pub fn list_users(&self) -> Result<Vec<String>, String> {
        let auth_tree = self.db.open_tree("auth").map_err(|e| e.to_string())?;
        let mut users = Vec::new();
        
        for item in auth_tree.scan_prefix(b"passhash/") {
            let (key, _) = item.map_err(|e| e.to_string())?;
            if let Some(key_str) = std::str::from_utf8(&key).ok() {
                if let Some(username) = key_str.strip_prefix("passhash/") {
                    users.push(username.to_string());
                }
            }
        }
        
        Ok(users)
    }
    
    /// Clear expired tokens from store
    pub async fn clear_token_store(&self) {
        let now = std::time::SystemTime::now();
        let mut token_store = self.token_store.write().await;
        
        token_store.retain(|_, info| info.expires_at > now);
    }
    
    async fn remove_autologin_tokens(&self, _username: &str) {
        // Implementation for removing autologin tokens
        // Would iterate through autologin token store and remove matching tokens
    }
}

impl ExpLoginHandler {
    pub fn new(base_delay_secs: u64, max_delay_secs: u64) -> Self {
        Self {
            attempts: Arc::new(RwLock::new(tokio::sync::HashMap::new())),
            base_delay_secs,
            max_delay_secs,
        }
    }
    
    pub async fn allow_access(&self, username: &str, ip: &str) -> (bool, u64) {
        let key = format!("{}:{}", username, ip);
        let attempts = self.attempts.read().await;
        
        if let Some(attempt) = attempts.get(&key) {
            let elapsed = attempt.last_attempt.elapsed().unwrap_or_default().as_secs();
            let delay = self.calculate_delay(attempt.count);
            
            if elapsed < delay {
                return (false, delay - elapsed);
            }
        }
        
        (true, 0)
    }
    
    pub async fn add_attempt(&self, username: &str, ip: &str) {
        let key = format!("{}:{}", username, ip);
        let mut attempts = self.attempts.write().await;
        
        let entry = attempts.entry(key).or_insert(LoginAttempt {
            count: 0,
            last_attempt: std::time::SystemTime::now(),
        });
        
        entry.count += 1;
        entry.last_attempt = std::time::SystemTime::now();
    }
    
    pub async fn reset_attempts(&self, username: &str, ip: &str) {
        let key = format!("{}:{}", username, ip);
        let mut attempts = self.attempts.write().await;
        attempts.remove(&key);
    }
    
    fn calculate_delay(&self, count: u32) -> u64 {
        let delay = self.base_delay_secs * (1 << count.min(10)); // Cap at 2^10
        delay.min(self.max_delay_secs)
    }
}

impl WhitelistManager {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Self {
            enabled: false,
            db,
        }
    }
    
    pub fn is_whitelisted(&self, ip: &str) -> Result<bool, String> {
        // Check database for whitelist entries
        let tree = self.db.open_tree("auth:whitelist").map_err(|e| e.to_string())?;
        Ok(tree.contains_key(ip.as_bytes()).map_err(|e| e.to_string())?)
    }
}

impl BlacklistManager {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Self {
            enabled: false,
            db,
        }
    }
    
    pub fn is_banned(&self, ip: &str) -> Result<bool, String> {
        // Check database for blacklist entries
        let tree = self.db.open_tree("auth:blacklist").map_err(|e| e.to_string())?;
        Ok(tree.contains_key(ip.as_bytes()).map_err(|e| e.to_string())?)
    }
}

impl AuthLogger {
    pub fn new(db: Arc<sled::Db>) -> Result<Self, Box<dyn std::error::Error>> {
        db.open_tree("auth:logs")?;
        Ok(Self { db })
    }
    
    pub async fn log_auth_event(&self, ip: &str, username: &str, success: bool) {
        let tree = match self.db.open_tree("auth:logs") {
            Ok(t) => t,
            Err(_) => return,
        };
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        let log_entry = format!(
            "{{\"timestamp\":{},\"ip\":\"{}\",\"username\":\"{}\",\"success\":{}}}",
            timestamp, ip, username, success
        );
        
        let _ = tree.insert(format!("log_{}", timestamp).as_bytes(), log_entry.as_bytes());
    }
}

impl SwitchableAccountPoolManager {
    pub fn new(db: Arc<sled::Db>) -> Self {
        Self { db }
    }
}

// HTTP Handlers for Axum router

pub async fn login_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let client_ip = headers
        .get("X-Forwarded-For")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .split(',')
        .next()
        .unwrap_or("unknown")
        .trim()
        .to_string();
    
    match state.auth_agent.handle_login(payload, &client_ip).await {
        Ok(token) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                "Authorization",
                format!("Bearer {}", token).parse().unwrap(),
            );
            (headers, StatusCode::OK, "Login successful").into_response()
        }
        Err(e) => {
            warn!("Login failed: {}", e);
            (StatusCode::UNAUTHORIZED, e).into_response()
        }
    }
}

pub async fn logout_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match state.auth_agent.handle_logout(&headers).await {
        Ok(_) => (StatusCode::OK, "Logout successful").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

pub async fn check_auth_handler(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if state.auth_agent.check_auth(&headers) {
        Json("true").into_response()
    } else {
        Json("false").into_response()
    }
}

pub async fn register_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Check if any users exist - if not, allow registration without auth
    let user_count = state.auth_agent.get_user_count().unwrap_or(0);
    
    if user_count > 0 && !state.auth_agent.check_auth(&HeaderMap::new()) {
        return (
            StatusCode::UNAUTHORIZED,
            "Login is needed to create new user"
        ).into_response();
    }
    
    let groups = vec![payload.group.clone()];
    match state.auth_agent.create_user(&payload.username, &payload.password, &groups).await {
        Ok(_) => (StatusCode::OK, "User created successfully").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// Background task to clean up expired tokens
pub async fn token_cleanup_task(auth_agent: Arc<AuthAgent>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // 5 minutes
    
    loop {
        interval.tick().await;
        auth_agent.clear_token_store().await;
        info!("Cleaned up expired tokens");
    }
}
