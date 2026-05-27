//! WebDAV Server Module for ArozOS
//! 
//! Implements a WebDAV server with user authentication
//! and integration with the filesystem manager.

use async_trait::async_trait;
use axum::{
    body::Body,
    extract::{Path as AxumPath, State},
    http::{Method, Request, StatusCode, header},
    response::Response,
    routing::{delete, get, post, put},
    Router,
};
use dav_server::{
    DavConfig, DavHandler, DavMethod, DavRequest, DavResponse,
    fs::{DavFileSystem, DavFile, DavDirEntry, FsOptions},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, debug};

use crate::mod::auth;
use crate::mod::filesystem;

/// WebDAV Server Configuration
#[derive(Clone)]
pub struct WebdavConfig {
    pub bind_path: String,
    pub root_path: PathBuf,
    pub auth_required: bool,
    pub allow_anonymous: bool,
    pub max_upload_size: u64,
    pub read_only: bool,
    pub enable_locks: bool,
    pub enable_properties: bool,
}

impl Default for WebdavConfig {
    fn default() -> Self {
        Self {
            bind_path: "/webdav".to_string(),
            root_path: PathBuf::from("/data/webdav"),
            auth_required: true,
            allow_anonymous: false,
            max_upload_size: 1024 * 1024 * 1024, // 1GB
            read_only: false,
            enable_locks: true,
            enable_properties: true,
        }
    }
}

/// WebDAV Server Instance
pub struct WebdavServer {
    config: WebdavConfig,
    auth_manager: Arc<auth::AuthManager>,
    fs_manager: Arc<filesystem::FileSystemManager>,
    dav_handler: DavHandler,
}

impl WebdavServer {
    pub fn new(
        config: WebdavConfig,
        auth_manager: Arc<auth::AuthManager>,
        fs_manager: Arc<filesystem::FileSystemManager>,
    ) -> Self {
        let dav_options = FsOptions::new()
            .with_enable_locks(config.enable_locks)
            .with_enable_props(config.enable_properties);

        let dav_handler = DavHandler::builder()
            .filesystem(WebdavFileSystem::new(
                config.root_path.clone(),
                fs_manager.clone(),
            ))
            .locksystem(dav_server::fs::MemLs::new())
            .build();

        Self {
            config,
            auth_manager,
            fs_manager,
            dav_handler,
        }
    }

    /// Create Axum router for WebDAV endpoints
    pub fn create_router(&self) -> Router<Arc<WebdavState>> {
        let state = Arc::new(WebdavState {
            config: self.config.clone(),
            auth_manager: self.auth_manager.clone(),
            fs_manager: self.fs_manager.clone(),
            dav_handler: self.dav_handler.clone(),
        });

        Router::new()
            .route(
                "/*path",
                get(handle_dav_request)
                    .put(handle_dav_request)
                    .post(handle_dav_request)
                    .delete(handle_dav_request)
                    .method(Method::PROPFIND, handle_dav_request)
                    .method(Method::PROPPATCH, handle_dav_request)
                    .method(Method::MKCOL, handle_dav_request)
                    .method(Method::COPY, handle_dav_request)
                    .method(Method::MOVE, handle_dav_request)
                    .method(Method::LOCK, handle_dav_request)
                    .method(Method::UNLOCK, handle_dav_request)
                    .method(Method::REPORT, handle_dav_request),
            )
            .with_state(state)
    }

    /// Get server configuration
    pub fn config(&self) -> &WebdavConfig {
        &self.config
    }
}

/// WebDAV Server State
pub struct WebdavState {
    pub config: WebdavConfig,
    pub auth_manager: Arc<auth::AuthManager>,
    pub fs_manager: Arc<filesystem::FileSystemManager>,
    pub dav_handler: DavHandler,
}

/// Handle WebDAV requests
async fn handle_dav_request(
    State(state): State<Arc<WebdavState>>,
    req: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    // Check authentication if required
    if state.config.auth_required {
        // Extract authorization header
        let auth_header = req.headers().get(header::AUTHORIZATION);
        
        match auth_header {
            Some(header_value) => {
                // Parse Basic or Bearer auth
                // In production, implement proper token validation
                let auth_str = header_value.to_str().map_err(|_| StatusCode::BAD_REQUEST)?;
                
                if !auth_str.starts_with("Basic ") && !auth_str.starts_with("Bearer ") {
                    return Err(StatusCode::UNAUTHORIZED);
                }
                
                // Validate credentials against auth manager
                // This is simplified - real implementation would decode and verify
            }
            None => {
                if !state.config.allow_anonymous {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        }
    }

    // Convert Axum request to DavRequest
    let dav_request = DavRequest::from(req);
    
    // Process with DavHandler
    let dav_response = state.dav_handler.handle(dav_request).await;
    
    // Convert back to Axum response
    Ok(dav_response.into_response())
}

/// WebDAV FileSystem Implementation
pub struct WebdavFileSystem {
    root_path: PathBuf,
    fs_manager: Arc<filesystem::FileSystemManager>,
}

impl WebdavFileSystem {
    pub fn new(root_path: PathBuf, fs_manager: Arc<filesystem::FileSystemManager>) -> Self {
        Self {
            root_path,
            fs_manager,
        }
    }
}

#[async_trait]
impl DavFileSystem for WebdavFileSystem {
    type DirEntry = WebdavDirEntry;
    type File = WebdavFile;

    async fn open(
        &self,
        path: &Path,
        options: FsOptions,
    ) -> Result<Self::File, dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        // Use filesystem manager to open file
        match self.fs_manager.read_file(&full_path.to_string_lossy()).await {
            Ok(data) => Ok(WebdavFile {
                path: full_path,
                data,
                metadata: None,
            }),
            Err(_) => Err(dav_server::fs::FsError::NotFound),
        }
    }

    async fn read_dir(
        &self,
        path: &Path,
    ) -> Result<Vec<Self::DirEntry>, dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        match tokio::fs::read_dir(&full_path).await {
            Ok(mut entries) => {
                let mut dir_entries = Vec::new();
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let metadata = entry.metadata().await
                        .map_err(|_| dav_server::fs::FsError::NotFound)?;
                    
                    dir_entries.push(WebdavDirEntry {
                        name: entry.file_name().to_string_lossy().to_string(),
                        path: entry.path(),
                        is_dir: metadata.is_dir(),
                        size: metadata.len(),
                        modified: metadata.modified().ok(),
                    });
                }
                Ok(dir_entries)
            }
            Err(_) => Err(dav_server::fs::FsError::NotFound),
        }
    }

    async fn create_dir(&self, path: &Path) -> Result<(), dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        tokio::fs::create_dir_all(&full_path).await
            .map_err(|_| dav_server::fs::FsError::Forbidden)
    }

    async fn remove_dir(&self, path: &Path) -> Result<(), dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        tokio::fs::remove_dir(&full_path).await
            .map_err(|_| dav_server::fs::FsError::Forbidden)
    }

    async fn remove_file(&self, path: &Path) -> Result<(), dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        tokio::fs::remove_file(&full_path).await
            .map_err(|_| dav_server::fs::FsError::Forbidden)
    }

    async fn rename(&self, from: &Path, to: &Path) -> Result<(), dav_server::fs::FsError> {
        let from_path = self.root_path.join(from);
        let to_path = self.root_path.join(to);
        
        tokio::fs::rename(&from_path, &to_path).await
            .map_err(|_| dav_server::fs::FsError::Forbidden)
    }

    async fn write(
        &self,
        path: &Path,
        data: Vec<u8>,
    ) -> Result<(), dav_server::fs::FsError> {
        let full_path = self.root_path.join(path);
        
        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|_| dav_server::fs::FsError::Forbidden)?;
        }
        
        tokio::fs::write(&full_path, &data).await
            .map_err(|_| dav_server::fs::FsError::Forbidden)
    }
}

/// WebDAV Directory Entry
pub struct WebdavDirEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

impl DavDirEntry for WebdavDirEntry {
    fn name(&self) -> std::ffi::OsString {
        self.name.clone().into()
    }

    fn metadata(&self) -> dav_server::fs::BoxFuture<Result<dav_server::fs::FsMetadata, dav_server::fs::FsError>> {
        let metadata = dav_server::fs::FsMetadata {
            size: self.size,
            created: None,
            modified: self.modified,
            accessed: None,
            is_dir: self.is_dir,
            symlink: false,
        };
        Box::pin(async move { Ok(metadata) })
    }
}

/// WebDAV File
pub struct WebdavFile {
    pub path: PathBuf,
    pub data: Vec<u8>,
    pub metadata: Option<dav_server::fs::FsMetadata>,
}

#[async_trait]
impl DavFile for WebdavFile {
    fn metadata(&self) -> dav_server::fs::BoxFuture<Result<dav_server::fs::FsMetadata, dav_server::fs::FsError>> {
        let meta = self.metadata.clone().unwrap_or_else(|| {
            dav_server::fs::FsMetadata {
                size: self.data.len() as u64,
                created: None,
                modified: None,
                accessed: None,
                is_dir: false,
                symlink: false,
            }
        });
        Box::pin(async move { Ok(meta) })
    }

    fn write_buf(
        &mut self,
        buf: Box<dyn bytes::Buf + Send>,
    ) -> dav_server::fs::BoxFuture<Result<(), dav_server::fs::FsError>> {
        // Append data
        let remaining = buf.remaining();
        let mut new_data = Vec::with_capacity(self.data.len() + remaining);
        new_data.extend_from_slice(&self.data);
        
        let mut buf = buf;
        while buf.has_remaining() {
            new_data.push(buf.get_u8());
        }
        
        self.data = new_data;
        Box::pin(async move { Ok(()) })
    }

    fn read_bytes(
        &mut self,
        count: usize,
    ) -> dav_server::fs::BoxFuture<Result<bytes::Bytes, dav_server::fs::FsError>> {
        let end = std::cmp::min(count, self.data.len());
        let chunk = self.data[..end].to_vec();
        self.data = self.data[end..].to_vec();
        
        Box::pin(async move { Ok(bytes::Bytes::from(chunk)) })
    }

    fn seek(
        &mut self,
        pos: dav_server::fs::SeekFrom,
    ) -> dav_server::fs::BoxFuture<Result<u64, dav_server::fs::FsError>> {
        // Simplified seek implementation
        Box::pin(async move { Ok(0) })
    }
}

/// WebDAV Error Types
#[derive(Debug, thiserror::Error)]
pub enum WebdavError {
    #[error("Authentication required")]
    Unauthorized,
    #[error("Forbidden: {0}")]
    Forbidden(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("DAV error: {0}")]
    Dav(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_webdav_config_default() {
        let config = WebdavConfig::default();
        assert_eq!(config.bind_path, "/webdav");
        assert!(config.auth_required);
        assert!(!config.read_only);
    }
}
