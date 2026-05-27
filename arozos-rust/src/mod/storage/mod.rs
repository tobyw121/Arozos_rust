//! Storage Management Module
//! 
//! This module provides storage pool management and generic interfacing
//! to various filesystems. It allows grouping multiple filesystem handlers
//! into logical storage pools with permission controls.
//! Ported from Go's mod/storage/storage.go

use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::mod::filesystem::{FileSystemHandler, FileSystemAbstraction, FilesystemError, FileInfo, DirEntry};
use tokio::io::{AsyncRead, AsyncWrite};

/// Storage Pool - groups multiple filesystem handlers for an owner
pub struct StoragePool {
    /// Owner of the storage pool (also acts as resolver's username)
    pub owner: String,
    /// Permissions for users other than the owner
    pub other_permission: AccessLevel,
    /// Storage pool accessible by this owner
    pub storages: Vec<Arc<FileSystemHandler>>,
}

/// Access level enumeration (from highest to lowest)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessLevel {
    ReadWrite,
    ReadOnly,
    Denied,
}

impl AccessLevel {
    /// Check if this access level has higher or equal permission than another
    pub fn has_higher_or_equal_permission(&self, other: &AccessLevel) -> bool {
        match (self, other) {
            // ReadWrite is highest
            (AccessLevel::ReadWrite, _) => true,
            // ReadOnly is lower than ReadWrite but higher than Denied
            (AccessLevel::ReadOnly, AccessLevel::ReadWrite) => false,
            (AccessLevel::ReadOnly, _) => true,
            // Denied is lowest
            (AccessLevel::Denied, AccessLevel::Denied) => true,
            (AccessLevel::Denied, _) => false,
        }
    }
    
    /// Convert from string representation
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "readwrite" | "rw" => AccessLevel::ReadWrite,
            "readonly" | "ro" => AccessLevel::ReadOnly,
            "denied" | "none" => AccessLevel::Denied,
            _ => AccessLevel::ReadOnly, // Default to readonly
        }
    }
    
    /// Convert to string representation
    pub fn to_str(&self) -> &'static str {
        match self {
            AccessLevel::ReadWrite => "readwrite",
            AccessLevel::ReadOnly => "readonly",
            AccessLevel::Denied => "denied",
        }
    }
}

impl StoragePool {
    /// Create a new storage pool with given filesystem handlers
    pub fn new(
        fs_handlers: Vec<Arc<FileSystemHandler>>,
        owner: &str,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            owner: owner.to_string(),
            other_permission: AccessLevel::ReadOnly,
            storages: fs_handlers,
        })
    }
    
    /// Create a new storage pool with custom permission level
    pub fn with_permission(
        fs_handlers: Vec<Arc<FileSystemHandler>>,
        owner: &str,
        other_permission: AccessLevel,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            owner: owner.to_string(),
            other_permission,
            storages: fs_handlers,
        })
    }
    
    /// Check if this storage pool contains a particular disk ID
    pub fn contains_disk_id(&self, disk_id: &str) -> bool {
        self.storages.iter().any(|fsh| fsh.uuid == disk_id)
    }
    
    /// Compare permissions with another storage pool
    pub fn has_higher_or_equal_permission_than(&self, other: &StoragePool) -> bool {
        self.other_permission.has_higher_or_equal_permission(&other.other_permission)
    }
    
    /// Get filesystem handler from virtual path
    pub fn get_fs_handler_from_virtual_path(
        &self,
        vpath: &str,
    ) -> Result<(Arc<FileSystemHandler>, String), StorageError> {
        let (fshid, subpath) = Self::get_id_from_virtual_path(vpath)?;
        let fsh = self.get_fs_handler_by_uuid(&fshid)?;
        Ok((fsh, subpath))
    }
    
    /// Get filesystem handler by UUID
    pub fn get_fs_handler_by_uuid(
        &self,
        uuid: &str,
    ) -> Result<Arc<FileSystemHandler>, StorageError> {
        // Filter out the ":/" from uuid if exists
        let clean_uuid = uuid.split(':').next().unwrap_or(uuid);
        
        for fsh in &self.storages {
            if fsh.uuid == clean_uuid {
                return Ok(fsh.clone());
            }
        }
        
        Err(StorageError::NotFound(format!(
            "Filesystem handler with UUID '{}' not found",
            clean_uuid
        )))
    }
    
    /// Parse virtual path to extract filesystem ID and subpath
    /// Virtual path format: :[fshid]:/[subpath]
    fn get_id_from_virtual_path(vpath: &str) -> Result<(String, String), StorageError> {
        let path = vpath.trim_start_matches('/');
        
        if !path.starts_with(':') {
            return Err(StorageError::InvalidPath(
                "Virtual path must start with ':'".to_string()
            ));
        }
        
        let parts: Vec<&str> = path[1..].splitn(2, ':').collect();
        if parts.len() < 2 {
            return Err(StorageError::InvalidPath(
                "Invalid virtual path format".to_string()
            ));
        }
        
        let fshid = parts[0].to_string();
        let subpath = parts[1].trim_start_matches('/').to_string();
        
        Ok((fshid, if subpath.is_empty() { "/".to_string() } else { format!("/{}", subpath) }))
    }
    
    /// List all storage UUIDs in this pool
    pub fn list_storage_uuids(&self) -> Vec<String> {
        self.storages.iter().map(|fsh| fsh.uuid.clone()).collect()
    }
    
    /// Get total number of storages in this pool
    pub fn storage_count(&self) -> usize {
        self.storages.len()
    }
    
    /// Check if the pool is empty
    pub fn is_empty(&self) -> bool {
        self.storages.is_empty()
    }
    
    /// Add a new filesystem handler to the pool
    pub fn add_storage(&mut self, fsh: Arc<FileSystemHandler>) {
        self.storages.push(fsh);
        info!("Added storage {} to pool for owner {}", fsh.name, self.owner);
    }
    
    /// Remove a filesystem handler from the pool by UUID
    pub fn remove_storage(&mut self, uuid: &str) -> Result<(), StorageError> {
        let initial_len = self.storages.len();
        self.storages.retain(|fsh| fsh.uuid != uuid);
        
        if self.storages.len() == initial_len {
            Err(StorageError::NotFound(format!(
                "Storage with UUID '{}' not found in pool",
                uuid
            )))
        } else {
            info!("Removed storage {} from pool for owner {}", uuid, self.owner);
            Ok(())
        }
    }
    
    /// Get the owner of this storage pool
    pub fn owner(&self) -> &str {
        &self.owner
    }
    
    /// Set permission level for other users
    pub fn set_other_permission(&mut self, permission: AccessLevel) {
        self.other_permission = permission;
    }
    
    /// Get permission level for other users
    pub fn other_permission(&self) -> AccessLevel {
        self.other_permission
    }
}

/// Storage error types
#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Storage not found: {0}")]
    NotFound(String),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Filesystem error: {0}")]
    Filesystem(#[from] FilesystemError),
    #[error("Storage pool is empty")]
    EmptyPool,
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// Storage manager - manages multiple storage pools
pub struct StorageManager {
    /// Map of owner to their storage pools
    pools: tokio::sync::RwLock<std::collections::HashMap<String, StoragePool>>,
}

impl StorageManager {
    /// Create a new storage manager
    pub fn new() -> Self {
        Self {
            pools: tokio::sync::RwLock::new(std::collections::HashMap::new()),
        }
    }
    
    /// Register a new storage pool for an owner
    pub async fn register_pool(&self, pool: StoragePool) -> Result<(), StorageError> {
        let owner = pool.owner.clone();
        let mut pools = self.pools.write().await;
        
        if pools.contains_key(&owner) {
            return Err(StorageError::OperationFailed(format!(
                "Storage pool already exists for owner '{}'",
                owner
            )));
        }
        
        pools.insert(owner, pool);
        info!("Registered storage pool for owner {}", owner);
        Ok(())
    }
    
    /// Get a storage pool by owner
    pub async fn get_pool(&self, owner: &str) -> Option<StoragePool> {
        let pools = self.pools.read().await;
        pools.get(owner).cloned()
    }
    
    /// Get a mutable reference to a storage pool by owner
    pub async fn get_pool_mut(&self, owner: &str) -> Option<&mut StoragePool> {
        let mut pools = self.pools.write().await;
        pools.get_mut(owner)
    }
    
    /// Remove a storage pool by owner
    pub async fn remove_pool(&self, owner: &str) -> Result<(), StorageError> {
        let mut pools = self.pools.write().await;
        
        if pools.remove(owner).is_some() {
            info!("Removed storage pool for owner {}", owner);
            Ok(())
        } else {
            Err(StorageError::NotFound(format!(
                "No storage pool found for owner '{}'",
                owner
            )))
        }
    }
    
    /// List all owners with storage pools
    pub async fn list_owners(&self) -> Vec<String> {
        let pools = self.pools.read().await;
        pools.keys().cloned().collect()
    }
    
    /// Get total number of storage pools
    pub async fn pool_count(&self) -> usize {
        let pools = self.pools.read().await;
        pools.len()
    }
    
    /// Find which pool contains a specific filesystem UUID
    pub async fn find_pool_by_disk_id(&self, disk_id: &str) -> Option<String> {
        let pools = self.pools.read().await;
        
        for (owner, pool) in pools.iter() {
            if pool.contains_disk_id(disk_id) {
                return Some(owner.clone());
            }
        }
        
        None
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}

/// High-level storage operations that work across filesystems
pub struct StorageOperations {
    pool: Arc<StoragePool>,
}

impl StorageOperations {
    /// Create new storage operations for a pool
    pub fn new(pool: Arc<StoragePool>) -> Self {
        Self { pool }
    }
    
    /// Read a file from the storage pool
    pub async fn read_file(&self, virtual_path: &str) -> Result<Vec<u8>, StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        let content = fsh.fs_abstraction.read_file(&subpath).await?;
        Ok(content)
    }
    
    /// Write a file to the storage pool
    pub async fn write_file(
        &self,
        virtual_path: &str,
        content: &[u8],
        mode: u32,
    ) -> Result<(), StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        
        // Check write permission
        if fsh.read_only {
            return Err(StorageError::PermissionDenied(
                "Storage is read-only".to_string()
            ));
        }
        
        fsh.fs_abstraction.write_file(&subpath, content, mode).await?;
        Ok(())
    }
    
    /// Delete a file from the storage pool
    pub async fn delete_file(&self, virtual_path: &str) -> Result<(), StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        
        if fsh.read_only {
            return Err(StorageError::PermissionDenied(
                "Storage is read-only".to_string()
            ));
        }
        
        fsh.fs_abstraction.remove(&subpath).await?;
        Ok(())
    }
    
    /// List directory contents
    pub async fn list_dir(&self, virtual_path: &str) -> Result<Vec<DirEntry>, StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        let entries = fsh.fs_abstraction.read_dir(&subpath).await?;
        Ok(entries)
    }
    
    /// Get file information
    pub async fn stat(&self, virtual_path: &str) -> Result<FileInfo, StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        let info = fsh.fs_abstraction.stat(&subpath).await?;
        Ok(info)
    }
    
    /// Create a directory
    pub async fn mkdir(&self, virtual_path: &str, mode: u32) -> Result<(), StorageError> {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(virtual_path)?;
        
        if fsh.read_only {
            return Err(StorageError::PermissionDenied(
                "Storage is read-only".to_string()
            ));
        }
        
        fsh.fs_abstraction.mkdir_all(&subpath, mode).await?;
        Ok(())
    }
    
    /// Rename/move a file or directory
    pub async fn rename(
        &self,
        from_path: &str,
        to_path: &str,
    ) -> Result<(), StorageError> {
        let (from_fsh, from_subpath) = self.pool.get_fs_handler_from_virtual_path(from_path)?;
        let (to_fsh, to_subpath) = self.pool.get_fs_handler_from_virtual_path(to_path)?;
        
        // Check if both paths are on the same filesystem
        if from_fsh.uuid != to_fsh.uuid {
            return Err(StorageError::OperationFailed(
                "Cannot rename across different filesystems".to_string()
            ));
        }
        
        if from_fsh.read_only {
            return Err(StorageError::PermissionDenied(
                "Storage is read-only".to_string()
            ));
        }
        
        from_fsh.fs_abstraction.rename(&from_subpath, &to_subpath).await?;
        Ok(())
    }
    
    /// Copy a file (potentially across filesystems)
    pub async fn copy_file(
        &self,
        from_path: &str,
        to_path: &str,
    ) -> Result<(), StorageError> {
        let (from_fsh, from_subpath) = self.pool.get_fs_handler_from_virtual_path(from_path)?;
        let (to_fsh, to_subpath) = self.pool.get_fs_handler_from_virtual_path(to_path)?;
        
        if to_fsh.read_only {
            return Err(StorageError::PermissionDenied(
                "Destination storage is read-only".to_string()
            ));
        }
        
        // Read from source
        let content = from_fsh.fs_abstraction.read_file(&from_subpath).await?;
        
        // Write to destination
        to_fsh.fs_abstraction.write_file(&to_subpath, &content, 0o644).await?;
        
        Ok(())
    }
    
    /// Walk through all files in a directory tree
    pub async fn walk<F>(&self, root_path: &str, mut callback: F) -> Result<(), StorageError>
    where
        F: FnMut(&str, &FileInfo) -> Result<(), StorageError> + Send,
    {
        let (fsh, subpath) = self.pool.get_fs_handler_from_virtual_path(root_path)?;
        
        fsh.fs_abstraction
            .walk(&subpath, |path, info| {
                // Convert back to virtual path
                let vpath = format!(":{}:{}", fsh.uuid, path);
                callback(&vpath, info)
            })
            .await?;
        
        Ok(())
    }
    
    /// Get total size of all files in a path
    pub async fn get_total_size(&self, virtual_path: &str) -> Result<u64, StorageError> {
        let mut total_size: u64 = 0;
        
        self.walk(virtual_path, |_path, info| {
            if !info.is_dir {
                total_size += info.size;
            }
            Ok(())
        })
        .await?;
        
        Ok(total_size)
    }
}

// HTTP Handlers for storage operations (Axum integration)

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use crate::AppState;

pub async fn read_file_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
) -> impl IntoResponse {
    // Get the user's storage pool
    // Note: In real implementation, get owner from auth context
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.read_file(&virtual_path).await {
                Ok(content) => (StatusCode::OK, content).into_response(),
                Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn write_file_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.write_file(&virtual_path, &body, 0o644).await {
                Ok(_) => (StatusCode::OK, "File written successfully").into_response(),
                Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn delete_file_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
) -> impl IntoResponse {
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.delete_file(&virtual_path).await {
                Ok(_) => (StatusCode::OK, "File deleted successfully").into_response(),
                Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn list_dir_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
) -> impl IntoResponse {
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.list_dir(&virtual_path).await {
                Ok(entries) => Json(entries).into_response(),
                Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn stat_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
) -> impl IntoResponse {
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.stat(&virtual_path).await {
                Ok(info) => Json(info).into_response(),
                Err(e) => (StatusCode::NOT_FOUND, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn mkdir_handler(
    State(state): State<Arc<AppState>>,
    Path(virtual_path): Path<String>,
) -> impl IntoResponse {
    let owner = "default";
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.mkdir(&virtual_path, 0o755).await {
                Ok(_) => (StatusCode::OK, "Directory created successfully").into_response(),
                Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}

pub async fn rename_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let owner = "default";
    
    let from_path = payload.get("from").and_then(|v| v.as_str()).unwrap_or("");
    let to_path = payload.get("to").and_then(|v| v.as_str()).unwrap_or("");
    
    if from_path.is_empty() || to_path.is_empty() {
        return (StatusCode::BAD_REQUEST, "Missing 'from' or 'to' parameter").into_response();
    }
    
    match state.storage_manager.get_pool(owner).await {
        Some(pool) => {
            let ops = StorageOperations::new(Arc::new(pool));
            match ops.rename(from_path, to_path).await {
                Ok(_) => (StatusCode::OK, "File/directory renamed successfully").into_response(),
                Err(e) => (StatusCode::BAD_REQUEST, e.to_string()).into_response(),
            }
        }
        None => (StatusCode::NOT_FOUND, "Storage pool not found").into_response(),
    }
}
