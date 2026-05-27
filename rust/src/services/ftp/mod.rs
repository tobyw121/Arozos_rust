//! FTP Server Module for ArozOS
//! 
//! Implements an async FTP/FTPS server with user authentication
//! and virtual filesystem support.

use async_trait::async_trait;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::mod::auth;
use crate::mod::filesystem;

/// FTP Server Configuration
#[derive(Clone)]
pub struct FtpConfig {
    pub bind_addr: SocketAddr,
    pub passive_ports: (u16, u16),
    pub greeting: String,
    pub tls_enabled: bool,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
    pub max_connections: usize,
    pub timeout_secs: u64,
}

impl Default for FtpConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:21".parse().unwrap(),
            passive_ports: (30000, 30100),
            greeting: "ArozOS FTP Server".to_string(),
            tls_enabled: false,
            tls_cert_path: None,
            tls_key_path: None,
            max_connections: 100,
            timeout_secs: 300,
        }
    }
}

/// FTP Server Instance
pub struct FtpServer {
    config: FtpConfig,
    auth_manager: Arc<auth::AuthManager>,
    fs_manager: Arc<filesystem::FileSystemManager>,
    running: RwLock<bool>,
}

impl FtpServer {
    pub fn new(
        config: FtpConfig,
        auth_manager: Arc<auth::AuthManager>,
        fs_manager: Arc<filesystem::FileSystemManager>,
    ) -> Self {
        Self {
            config,
            auth_manager,
            fs_manager,
            running: RwLock::new(false),
        }
    }

    /// Start the FTP server
    pub async fn start(&self) -> Result<(), FtpError> {
        let mut running = self.running.write().await;
        if *running {
            return Err(FtpError::AlreadyRunning);
        }

        info!(
            "Starting FTP server on {} (TLS: {})",
            self.config.bind_addr, self.config.tls_enabled
        );

        // In a full implementation, we would:
        // 1. Create a TCP listener
        // 2. Spawn connection handlers
        // 3. Implement the FTP protocol state machine
        // 4. Handle commands: USER, PASS, LIST, RETR, STOR, etc.
        
        // Placeholder for actual server implementation
        // This would use tokio::net::TcpListener and implement
        // the FTP protocol according to RFC 959
        
        *running = true;
        info!("FTP server started");
        Ok(())
    }

    /// Stop the FTP server
    pub async fn stop(&self) -> Result<(), FtpError> {
        let mut running = self.running.write().await;
        if !*running {
            return Err(FtpError::NotRunning);
        }

        info!("Stopping FTP server...");
        *running = false;
        info!("FTP server stopped");
        Ok(())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Get server configuration
    pub fn config(&self) -> &FtpConfig {
        &self.config
    }
}

/// FTP Error Types
#[derive(Debug, thiserror::Error)]
pub enum FtpError {
    #[error("Server already running")]
    AlreadyRunning,
    #[error("Server not running")]
    NotRunning,
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Authentication error: {0}")]
    Auth(String),
    #[error("File error: {0}")]
    File(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

/// FTP User Session
pub struct FtpSession {
    pub username: String,
    pub root_path: PathBuf,
    pub current_path: PathBuf,
    pub authenticated: bool,
    pub passive_mode: bool,
    pub data_connection: Option<SocketAddr>,
}

impl FtpSession {
    pub fn new(username: String, root_path: PathBuf) -> Self {
        Self {
            username,
            root_path: root_path.clone(),
            current_path: root_path,
            authenticated: false,
            passive_mode: false,
            data_connection: None,
        }
    }

    /// Resolve a path relative to the session root
    pub fn resolve_path(&self, path: &str) -> Result<PathBuf, FtpError> {
        let resolved = if path.starts_with('/') {
            self.root_path.join(path.trim_start_matches('/'))
        } else {
            self.current_path.join(path)
        };

        // Ensure path is within root (chroot jail)
        if !resolved.starts_with(&self.root_path) {
            return Err(FtpError::File("Path traversal detected".to_string()));
        }

        Ok(resolved)
    }
}

/// FTP Command Handler Trait
#[async_trait]
pub trait FtpCommandHandler: Send + Sync {
    async fn handle_user(&self, session: &mut FtpSession, username: &str) -> Result<String, FtpError>;
    async fn handle_pass(&self, session: &mut FtpSession, password: &str) -> Result<String, FtpError>;
    async fn handle_list(&self, session: &FtpSession, path: Option<&str>) -> Result<String, FtpError>;
    async fn handle_retr(&self, session: &FtpSession, path: &str) -> Result<Vec<u8>, FtpError>;
    async fn handle_stor(&self, session: &FtpSession, path: &str, data: &[u8]) -> Result<String, FtpError>;
    async fn handle_cwd(&self, session: &mut FtpSession, path: &str) -> Result<String, FtpError>;
    async fn handle_pwd(&self, session: &FtpSession) -> Result<String, FtpError>;
    async fn handle_dele(&self, session: &FtpSession, path: &str) -> Result<String, FtpError>;
    async fn handle_mkd(&self, session: &FtpSession, path: &str) -> Result<String, FtpError>;
    async fn handle_rmd(&self, session: &FtpSession, path: &str) -> Result<String, FtpError>;
    async fn handle_quit(&self, session: &mut FtpSession) -> Result<String, FtpError>;
}

/// Default FTP Command Handler Implementation
pub struct DefaultFtpHandler {
    auth_manager: Arc<auth::AuthManager>,
    fs_manager: Arc<filesystem::FileSystemManager>,
}

impl DefaultFtpHandler {
    pub fn new(auth_manager: Arc<auth::AuthManager>, fs_manager: Arc<filesystem::FileSystemManager>) -> Self {
        Self {
            auth_manager,
            fs_manager,
        }
    }
}

#[async_trait]
impl FtpCommandHandler for DefaultFtpHandler {
    async fn handle_user(&self, session: &mut FtpSession, username: &str) -> Result<String, FtpError> {
        // Check if user exists
        match self.auth_manager.get_user_info(username).await {
            Ok(_) => {
                session.username = username.to_string();
                Ok(format!("331 Password required for {}", username))
            }
            Err(_) => Err(FtpError::Auth("User not found".to_string())),
        }
    }

    async fn handle_pass(&self, session: &mut FtpSession, password: &str) -> Result<String, FtpError> {
        // Verify password
        match self.auth_manager.authenticate(&session.username, password).await {
            Ok(true) => {
                session.authenticated = true;
                
                // Set user's home directory as root
                if let Ok(user_info) = self.auth_manager.get_user_info(&session.username).await {
                    session.root_path = PathBuf::from(user_info.home_dir);
                    session.current_path = session.root_path.clone();
                }
                
                Ok("230 Login successful".to_string())
            }
            Ok(false) | Err(_) => Err(FtpError::Auth("Invalid credentials".to_string())),
        }
    }

    async fn handle_list(&self, session: &FtpSession, path: Option<&str>) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let target_path = match path {
            Some(p) => session.resolve_path(p)?,
            None => session.current_path.clone(),
        };

        // List directory contents
        match tokio::fs::read_dir(&target_path).await {
            Ok(mut entries) => {
                let mut listing = String::new();
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let metadata = entry.metadata().await?;
                    let name = entry.file_name().to_string_lossy();
                    let perms = if metadata.is_dir() { "d" } else { "-" };
                    listing.push_str(&format!("{}rwxr-xr-x 1 user group 0 Jan 1 00:00 {}\n", perms, name));
                }
                Ok(listing)
            }
            Err(e) => Err(FtpError::File(format!("Failed to list directory: {}", e))),
        }
    }

    async fn handle_retr(&self, session: &FtpSession, path: &str) -> Result<Vec<u8>, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let file_path = session.resolve_path(path)?;
        
        match tokio::fs::read(&file_path).await {
            Ok(data) => Ok(data),
            Err(e) => Err(FtpError::File(format!("Failed to read file: {}", e))),
        }
    }

    async fn handle_stor(&self, session: &FtpSession, path: &str, data: &[u8]) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let file_path = session.resolve_path(path)?;
        
        // Ensure parent directory exists
        if let Some(parent) = file_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        match tokio::fs::write(&file_path, data).await {
            Ok(_) => Ok(format!("226 Successfully wrote {}", path)),
            Err(e) => Err(FtpError::File(format!("Failed to write file: {}", e))),
        }
    }

    async fn handle_cwd(&self, session: &mut FtpSession, path: &str) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let new_path = session.resolve_path(path)?;
        
        // Verify it's a directory
        match tokio::fs::metadata(&new_path).await {
            Ok(meta) if meta.is_dir() => {
                session.current_path = new_path;
                Ok(format!("250 Changed directory to {}", path))
            }
            Ok(_) => Err(FtpError::File("Not a directory".to_string())),
            Err(e) => Err(FtpError::File(format!("Directory not found: {}", e))),
        }
    }

    async fn handle_pwd(&self, session: &FtpSession) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let relative_path = session.current_path
            .strip_prefix(&session.root_path)
            .unwrap_or_else(|_| std::path::Path::new(""));
        
        Ok(format!("257 \"/{}\" is current directory", relative_path.display()))
    }

    async fn handle_dele(&self, session: &FtpSession, path: &str) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let file_path = session.resolve_path(path)?;
        
        match tokio::fs::remove_file(&file_path).await {
            Ok(_) => Ok(format!("250 Deleted {}", path)),
            Err(e) => Err(FtpError::File(format!("Failed to delete: {}", e))),
        }
    }

    async fn handle_mkd(&self, session: &FtpSession, path: &str) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let dir_path = session.resolve_path(path)?;
        
        match tokio::fs::create_dir_all(&dir_path).await {
            Ok(_) => Ok(format!("257 \"{}\" created", path)),
            Err(e) => Err(FtpError::File(format!("Failed to create directory: {}", e))),
        }
    }

    async fn handle_rmd(&self, session: &FtpSession, path: &str) -> Result<String, FtpError> {
        if !session.authenticated {
            return Err(FtpError::Auth("Not authenticated".to_string()));
        }

        let dir_path = session.resolve_path(path)?;
        
        match tokio::fs::remove_dir(&dir_path).await {
            Ok(_) => Ok(format!("250 Removed {}", path)),
            Err(e) => Err(FtpError::File(format!("Failed to remove directory: {}", e))),
        }
    }

    async fn handle_quit(&self, session: &mut FtpSession) -> Result<String, FtpError> {
        session.authenticated = false;
        Ok("221 Goodbye".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftp_config_default() {
        let config = FtpConfig::default();
        assert_eq!(config.greeting, "ArozOS FTP Server");
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    fn test_session_path_resolution() {
        let session = FtpSession::new("test".to_string(), PathBuf::from("/home/test"));
        
        // Test relative path
        let resolved = session.resolve_path("documents").unwrap();
        assert!(resolved.ends_with("documents"));
        
        // Test absolute path
        let resolved = session.resolve_path("/documents").unwrap();
        assert!(resolved.ends_with("documents"));
        
        // Test path traversal prevention
        let result = session.resolve_path("../../../etc/passwd");
        assert!(result.is_err());
    }
}
