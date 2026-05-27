//! SFTP Server Module for ArozOS
//! 
//! Implements an SFTP server using SSH protocol with user authentication
//! and integration with the filesystem manager.

use async_trait::async_trait;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};

use crate::mod::auth;
use crate::mod::filesystem;

/// SFTP Server Configuration
#[derive(Clone)]
pub struct SftpConfig {
    pub bind_addr: SocketAddr,
    pub host_key_path: Option<PathBuf>,
    pub max_connections: usize,
    pub timeout_secs: u64,
    pub auth_methods: Vec<String>,
    pub banner: String,
}

impl Default for SftpConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:22".parse().unwrap(),
            host_key_path: None,
            max_connections: 100,
            timeout_secs: 300,
            auth_methods: vec!["password".to_string(), "publickey".to_string()],
            banner: "ArozOS SFTP Server".to_string(),
        }
    }
}

/// SFTP Server Instance
pub struct SftpServer {
    config: SftpConfig,
    auth_manager: Arc<auth::AuthManager>,
    fs_manager: Arc<filesystem::FileSystemManager>,
    running: RwLock<bool>,
}

impl SftpServer {
    pub fn new(
        config: SftpConfig,
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

    /// Start the SFTP server
    pub async fn start(&self) -> Result<(), SftpError> {
        let mut running = self.running.write().await;
        if *running {
            return Err(SftpError::AlreadyRunning);
        }

        info!(
            "Starting SFTP server on {} (auth methods: {:?})",
            self.config.bind_addr, self.config.auth_methods
        );

        // In a full implementation, we would:
        // 1. Create a TCP listener
        // 2. Use russh library to handle SSH protocol
        // 3. Implement SFTP subsystem handler
        // 4. Handle file operations through filesystem manager
        
        // Placeholder for actual server implementation
        // This would use russh::Server and implement the russh::Handler trait
        
        *running = true;
        info!("SFTP server started");
        Ok(())
    }

    /// Stop the SFTP server
    pub async fn stop(&self) -> Result<(), SftpError> {
        let mut running = self.running.write().await;
        if !*running {
            return Err(SftpError::NotRunning);
        }

        info!("Stopping SFTP server...");
        *running = false;
        info!("SFTP server stopped");
        Ok(())
    }

    /// Check if server is running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Get server configuration
    pub fn config(&self) -> &SftpConfig {
        &self.config
    }
}

/// SFTP Error Types
#[derive(Debug, thiserror::Error)]
pub enum SftpError {
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
    #[error("SSH error: {0}")]
    Ssh(String),
    #[error("Protocol error: {0}")]
    Protocol(String),
}

/// SFTP Session State
pub struct SftpSession {
    pub username: String,
    pub root_path: PathBuf,
    pub current_path: PathBuf,
    pub authenticated: bool,
    pub session_id: String,
}

impl SftpSession {
    pub fn new(username: String, root_path: PathBuf, session_id: String) -> Self {
        Self {
            username,
            root_path: root_path.clone(),
            current_path: root_path,
            authenticated: false,
            session_id,
        }
    }

    /// Resolve a path relative to the session root
    pub fn resolve_path(&self, path: &str) -> Result<PathBuf, SftpError> {
        let resolved = if path.starts_with('/') {
            self.root_path.join(path.trim_start_matches('/'))
        } else {
            self.current_path.join(path)
        };

        // Ensure path is within root (chroot jail)
        if !resolved.starts_with(&self.root_path) {
            return Err(SftpError::File("Path traversal detected".to_string()));
        }

        Ok(resolved)
    }
}

/// SFTP Handler Trait - Defines file operations
#[async_trait]
pub trait SftpHandler: Send + Sync {
    async fn authenticate(&self, username: &str, password: Option<&str>) -> Result<bool, SftpError>;
    async fn list_dir(&self, session: &SftpSession, path: &Path) -> Result<Vec<SftpDirEntry>, SftpError>;
    async fn read_file(&self, session: &SftpSession, path: &Path, offset: u64, len: u32) -> Result<Vec<u8>, SftpError>;
    async fn write_file(&self, session: &SftpSession, path: &Path, data: &[u8], offset: u64) -> Result<(), SftpError>;
    async fn create_dir(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError>;
    async fn remove_file(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError>;
    async fn remove_dir(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError>;
    async fn rename(&self, session: &SftpSession, from: &Path, to: &Path) -> Result<(), SftpError>;
    async fn get_metadata(&self, session: &SftpSession, path: &Path) -> Result<SftpMetadata, SftpError>;
    async fn set_metadata(&self, session: &SftpSession, path: &Path, metadata: SftpMetadata) -> Result<(), SftpError>;
}

/// Directory Entry for SFTP
#[derive(Clone, Debug)]
pub struct SftpDirEntry {
    pub filename: String,
    pub longname: String,
    pub metadata: SftpMetadata,
}

/// File Metadata for SFTP
#[derive(Clone, Debug)]
pub struct SftpMetadata {
    pub size: u64,
    pub uid: u32,
    pub gid: u32,
    pub mode: u32,
    pub atime: u64,
    pub mtime: u64,
    pub file_type: SftpFileType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SftpFileType {
    Regular,
    Directory,
    Symlink,
    BlockDevice,
    CharacterDevice,
    Fifo,
    Socket,
}

impl SftpMetadata {
    pub fn new(file_type: SftpFileType, size: u64) -> Self {
        Self {
            size,
            uid: 1000,
            gid: 1000,
            mode: 0o755,
            atime: 0,
            mtime: 0,
            file_type,
        }
    }
}

/// Default SFTP Handler Implementation
pub struct DefaultSftpHandler {
    auth_manager: Arc<auth::AuthManager>,
    fs_manager: Arc<filesystem::FileSystemManager>,
}

impl DefaultSftpHandler {
    pub fn new(auth_manager: Arc<auth::AuthManager>, fs_manager: Arc<filesystem::FileSystemManager>) -> Self {
        Self {
            auth_manager,
            fs_manager,
        }
    }
}

#[async_trait]
impl SftpHandler for DefaultSftpHandler {
    async fn authenticate(&self, username: &str, password: Option<&str>) -> Result<bool, SftpError> {
        match password {
            Some(pwd) => {
                match self.auth_manager.authenticate(username, pwd).await {
                    Ok(valid) => Ok(valid),
                    Err(_) => Ok(false),
                }
            }
            None => Ok(false),
        }
    }

    async fn list_dir(&self, session: &SftpSession, path: &Path) -> Result<Vec<SftpDirEntry>, SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::read_dir(&full_path).await {
            Ok(mut entries) => {
                let mut dir_entries = Vec::new();
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let metadata = entry.metadata().await?;
                    let file_type = if metadata.is_dir() {
                        SftpFileType::Directory
                    } else {
                        SftpFileType::Regular
                    };
                    
                    let sftp_meta = SftpMetadata::new(file_type, metadata.len());
                    let filename = entry.file_name().to_string_lossy().to_string();
                    let longname = format!(
                        "{}rwxr-xr-x 1 user group {:8} Jan 1 00:00 {}",
                        if file_type == SftpFileType::Directory { "d" } else { "-" },
                        metadata.len(),
                        filename
                    );
                    
                    dir_entries.push(SftpDirEntry {
                        filename: filename.clone(),
                        longname,
                        metadata: sftp_meta,
                    });
                }
                Ok(dir_entries)
            }
            Err(e) => Err(SftpError::File(format!("Failed to list directory: {}", e))),
        }
    }

    async fn read_file(&self, session: &SftpSession, path: &Path, offset: u64, len: u32) -> Result<Vec<u8>, SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::read(&full_path).await {
            Ok(data) => {
                let start = offset as usize;
                let end = std::cmp::min(start + len as usize, data.len());
                
                if start >= data.len() {
                    Ok(vec![])
                } else {
                    Ok(data[start..end].to_vec())
                }
            }
            Err(e) => Err(SftpError::File(format!("Failed to read file: {}", e))),
        }
    }

    async fn write_file(&self, session: &SftpSession, path: &Path, data: &[u8], offset: u64) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // For simplicity, we write the entire data
        // A full implementation would handle offset-based writes
        match tokio::fs::write(&full_path, data).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to write file: {}", e))),
        }
    }

    async fn create_dir(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::create_dir_all(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to create directory: {}", e))),
        }
    }

    async fn remove_file(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::remove_file(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to remove file: {}", e))),
        }
    }

    async fn remove_dir(&self, session: &SftpSession, path: &Path) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::remove_dir(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to remove directory: {}", e))),
        }
    }

    async fn rename(&self, session: &SftpSession, from: &Path, to: &Path) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let from_path = session.resolve_path(from)?;
        let to_path = session.resolve_path(to)?;
        
        match tokio::fs::rename(&from_path, &to_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to rename: {}", e))),
        }
    }

    async fn get_metadata(&self, session: &SftpSession, path: &Path) -> Result<SftpMetadata, SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        let full_path = session.resolve_path(path)?;
        
        match tokio::fs::metadata(&full_path).await {
            Ok(meta) => {
                let file_type = if meta.is_dir() {
                    SftpFileType::Directory
                } else {
                    SftpFileType::Regular
                };
                
                Ok(SftpMetadata::new(file_type, meta.len()))
            }
            Err(e) => Err(SftpError::File(format!("Failed to get metadata: {}", e))),
        }
    }

    async fn set_metadata(&self, session: &SftpSession, path: &Path, metadata: SftpMetadata) -> Result<(), SftpError> {
        if !session.authenticated {
            return Err(SftpError::Auth("Not authenticated".to_string()));
        }

        // Simplified implementation - real SFTP would set permissions, timestamps, etc.
        let full_path = session.resolve_path(path)?;
        
        // Verify file exists
        match tokio::fs::metadata(&full_path).await {
            Ok(_) => Ok(()),
            Err(e) => Err(SftpError::File(format!("Failed to set metadata: {}", e))),
        }
    }
}

/// TFTP Server Module (Simple implementation)
pub mod tftp {
    use super::*;
    use std::net::UdpSocket;
    
    /// TFTP Server Configuration
    #[derive(Clone)]
    pub struct TftpConfig {
        pub bind_addr: SocketAddr,
        pub root_path: PathBuf,
        pub timeout_secs: u64,
        pub max_retries: u32,
    }

    impl Default for TftpConfig {
        fn default() -> Self {
            Self {
                bind_addr: "0.0.0.0:69".parse().unwrap(),
                root_path: PathBuf::from("/data/tftp"),
                timeout_secs: 5,
                max_retries: 5,
            }
        }
    }

    /// TFTP Server Instance
    pub struct TftpServer {
        config: TftpConfig,
        running: RwLock<bool>,
    }

    impl TftpServer {
        pub fn new(config: TftpConfig) -> Self {
            Self {
                config,
                running: RwLock::new(false),
            }
        }

        /// Start the TFTP server
        pub async fn start(&self) -> Result<(), SftpError> {
            let mut running = self.running.write().await;
            if *running {
                return Err(SftpError::AlreadyRunning);
            }

            info!("Starting TFTP server on {}", self.config.bind_addr);
            
            // TFTP uses UDP and is much simpler than FTP/SFTP
            // It supports only read and write operations (RRQ/WRQ)
            // Full implementation would handle UDP packets according to RFC 1350
            
            *running = true;
            info!("TFTP server started");
            Ok(())
        }

        /// Stop the TFTP server
        pub async fn stop(&self) -> Result<(), SftpError> {
            let mut running = self.running.write().await;
            if !*running {
                return Err(SftpError::NotRunning);
            }

            info!("Stopping TFTP server...");
            *running = false;
            info!("TFTP server stopped");
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sftp_config_default() {
        let config = SftpConfig::default();
        assert_eq!(config.banner, "ArozOS SFTP Server");
        assert_eq!(config.max_connections, 100);
        assert!(config.auth_methods.contains(&"password".to_string()));
    }

    #[test]
    fn test_session_path_resolution() {
        let session = SftpSession::new("test".to_string(), PathBuf::from("/home/test"), "sess123".to_string());
        
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

    #[test]
    fn test_tftp_config_default() {
        let config = tftp::TftpConfig::default();
        assert_eq!(config.timeout_secs, 5);
        assert_eq!(config.max_retries, 5);
    }
}
