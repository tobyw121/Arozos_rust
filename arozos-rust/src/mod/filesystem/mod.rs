//! Filesystem Abstractions Module
//! 
//! This module provides a unified interface for various filesystem types
//! including local, WebDAV, SMB, SFTP, and FTP filesystems.
//! Ported from Go's mod/filesystem/filesystem.go

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::{AsyncRead, AsyncWrite};
use tracing::{info, warn, error};

/// File system abstraction trait - defines the interface for all filesystem types
#[async_trait]
pub trait FileSystemAbstraction: Send + Sync {
    /// Get the name of this filesystem
    fn name(&self) -> &str;
    
    // Fundamental Operations
    async fn chmod(&self, path: &str, mode: u32) -> Result<(), FilesystemError>;
    async fn chown(&self, path: &str, uid: u32, gid: u32) -> Result<(), FilesystemError>;
    async fn chtimes(&self, path: &str, atime: SystemTime, mtime: SystemTime) -> Result<(), FilesystemError>;
    async fn create(&self, path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError>;
    async fn mkdir(&self, path: &str, mode: u32) -> Result<(), FilesystemError>;
    async fn mkdir_all(&self, path: &str, mode: u32) -> Result<(), FilesystemError>;
    async fn open(&self, path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError>;
    async fn open_file(&self, path: &str, flags: OpenFlags, mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError>;
    async fn remove(&self, path: &str) -> Result<(), FilesystemError>;
    async fn remove_all(&self, path: &str) -> Result<(), FilesystemError>;
    async fn rename(&self, from: &str, to: &str) -> Result<(), FilesystemError>;
    async fn stat(&self, path: &str) -> Result<FileInfo, FilesystemError>;
    async fn close(&self) -> Result<(), FilesystemError>;
    
    // Utility Functions
    fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError>;
    fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError>;
    async fn file_exists(&self, path: &str) -> bool;
    async fn is_dir(&self, path: &str) -> bool;
    async fn glob(&self, pattern: &str) -> Result<Vec<String>, FilesystemError>;
    async fn get_file_size(&self, path: &str) -> Result<u64, FilesystemError>;
    async fn get_mod_time(&self, path: &str) -> Result<u64, FilesystemError>;
    async fn write_file(&self, path: &str, content: &[u8], mode: u32) -> Result<(), FilesystemError>;
    async fn read_file(&self, path: &str) -> Result<Vec<u8>, FilesystemError>;
    async fn read_dir(&self, path: &str) -> Result<Vec<DirEntry>, FilesystemError>;
    async fn write_stream<P: AsRef<Path>>(&self, path: P, reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError>;
    async fn read_stream(&self, path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError>;
    async fn walk<F>(&self, root: &str, callback: F) -> Result<(), FilesystemError>
    where
        F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send;
    async fn heartbeat(&self) -> Result<(), FilesystemError>;
}

/// Async file trait for file operations
#[async_trait]
pub trait AsyncFile: AsyncRead + AsyncWrite + Unpin + Send {
    fn path(&self) -> &str;
    async fn sync(&mut self) -> Result<(), FilesystemError>;
    async fn truncate(&mut self, size: u64) -> Result<(), FilesystemError>;
}

/// File information structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub mode: u32,
    pub modified: u64, // Unix timestamp
    pub accessed: u64,
    pub created: u64,
}

/// Directory entry structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub is_dir: bool,
    pub file_type: FileType,
}

/// File type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice,
    CharacterDevice,
    Fifo,
    Socket,
    Unknown,
}

/// Open flags for file operations
#[derive(Debug, Clone, Copy)]
pub struct OpenFlags {
    pub read: bool,
    pub write: bool,
    pub create: bool,
    pub truncate: bool,
    pub append: bool,
}

impl OpenFlags {
    pub const READ_ONLY: Self = Self {
        read: true,
        write: false,
        create: false,
        truncate: false,
        append: false,
    };
    
    pub const WRITE_ONLY: Self = Self {
        read: false,
        write: true,
        create: true,
        truncate: true,
        append: false,
    };
    
    pub const READ_WRITE: Self = Self {
        read: true,
        write: true,
        create: true,
        truncate: false,
        append: false,
    };
}

/// Filesystem error types
#[derive(Debug, thiserror::Error)]
pub enum FilesystemError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid path: {0}")]
    InvalidPath(String),
    #[error("Not a directory: {0}")]
    NotADirectory(String),
    #[error("Already exists: {0}")]
    AlreadyExists(String),
    #[error("Filesystem closed")]
    Closed,
    #[error("Network error: {0}")]
    Network(String),
    #[error("Unsupported operation: {0}")]
    Unsupported(String),
    #[error("Config error: {0}")]
    Config(String),
}

/// Filesystem option configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileSystemOption {
    pub name: String,
    pub uuid: String,
    pub path: String,
    pub access: AccessMode,
    pub hierarchy: HierarchyType,
    pub automount: bool,
    pub filesystem: String,
    pub mountdev: Option<String>,
    pub mountpt: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub port: Option<u16>,
    pub host: Option<String>,
}

/// Access mode enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessMode {
    ReadOnly,
    ReadWrite,
}

/// Hierarchy type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HierarchyType {
    Public,
    User,
    System,
}

/// Runtime persistence configuration
#[derive(Debug, Clone)]
pub struct RuntimePersistenceConfig {
    pub local_buffer_path: String,
}

/// Filesystem handler wrapping the abstraction
pub struct FileSystemHandler {
    pub name: String,
    pub uuid: String,
    pub path: String,
    pub hierarchy: HierarchyType,
    pub read_only: bool,
    pub require_buffer: bool,
    pub parent_uuid: Option<String>,
    pub initiation_time: u64,
    pub fs_abstraction: Arc<dyn FileSystemAbstraction>,
    pub filesystem_type: String,
    pub start_options: FileSystemOption,
    pub runtime_config: RuntimePersistenceConfig,
    pub closed: bool,
}

impl FileSystemHandler {
    /// Create a new filesystem handler from options
    pub fn new(
        option: FileSystemOption,
        runtime_config: RuntimePersistenceConfig,
    ) -> Result<Self, FilesystemError> {
        let fs_type = option.filesystem.to_lowercase();
        
        // Determine if buffering is required based on filesystem type
        let require_buffer = match fs_type.as_str() {
            "webdav" | "ftp" => true,
            _ => false,
        };
        
        // Create the appropriate filesystem abstraction
        let fs_abstraction: Arc<dyn FileSystemAbstraction> = match fs_type.as_str() {
            "local" | "" | "ext4" | "ext3" | "ext2" | "fat" | "vfat" | "ntfs" | "exfat" => {
                // Local filesystem
                Arc::new(local_fs::LocalFileSystem::new(
                    &option.uuid,
                    &option.path,
                    option.hierarchy,
                    option.access == AccessMode::ReadOnly,
                )?)
            }
            "webdav" => {
                // WebDAV filesystem
                Arc::new(webdav_fs::WebDAVFileSystem::new(
                    &option.uuid,
                    option.hierarchy,
                    &option.path,
                    option.username.as_deref().unwrap_or(""),
                    option.password.as_deref().unwrap_or(""),
                )?)
            }
            "smb" => {
                // SMB/CIFS filesystem
                Arc::new(smb_fs::SMBFileSystem::new(
                    &option.uuid,
                    option.hierarchy,
                    &option.host.unwrap_or_default(),
                    &option.path,
                    option.username.as_deref().unwrap_or(""),
                    option.password.as_deref().unwrap_or(""),
                )?)
            }
            "sftp" => {
                // SFTP filesystem
                Arc::new(sftp_fs::SFTPFileSystem::new(
                    &option.uuid,
                    option.hierarchy,
                    &option.host.unwrap_or_default(),
                    option.port.unwrap_or(22),
                    option.username.as_deref().unwrap_or(""),
                    option.password.as_deref().unwrap_or(""),
                )?)
            }
            "ftp" => {
                // FTP filesystem
                Arc::new(ftp_fs::FTPFileSystem::new(
                    &option.uuid,
                    option.hierarchy,
                    &option.host.unwrap_or_default(),
                    option.port.unwrap_or(21),
                    option.username.as_deref().unwrap_or(""),
                    option.password.as_deref().unwrap_or(""),
                )?)
            }
            _ => {
                return Err(FilesystemError::Config(format!(
                    "Unsupported filesystem type: {}",
                    fs_type
                )));
            }
        };
        
        Ok(Self {
            name: option.name.clone(),
            uuid: option.uuid.clone(),
            path: option.path.clone(),
            hierarchy: option.hierarchy,
            read_only: option.access == AccessMode::ReadOnly,
            require_buffer,
            parent_uuid: None,
            initiation_time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            fs_abstraction,
            filesystem_type: fs_type,
            start_options: option,
            runtime_config,
            closed: false,
        })
    }
    
    /// Close the filesystem handler
    pub async fn close(&mut self) -> Result<(), FilesystemError> {
        if !self.closed {
            self.fs_abstraction.close().await?;
            self.closed = true;
            info!("Filesystem handler {} closed", self.name);
        }
        Ok(())
    }
    
    /// Check if the filesystem is accessible
    pub async fn is_accessible(&self) -> bool {
        self.fs_abstraction.heartbeat().await.is_ok()
    }
}

// Submodules for different filesystem implementations

pub mod local_fs {
    use super::*;
    use tokio::fs::{self, File, OpenOptions};
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    pub struct LocalFileSystem {
        uuid: String,
        root_path: PathBuf,
        hierarchy: HierarchyType,
        read_only: bool,
    }
    
    impl LocalFileSystem {
        pub fn new(
            uuid: &str,
            root_path: &str,
            hierarchy: HierarchyType,
            read_only: bool,
        ) -> Result<Self, FilesystemError> {
            let root = PathBuf::from(root_path);
            
            // Ensure root path exists
            std::fs::create_dir_all(&root)?;
            
            // Create user hierarchy if needed
            if hierarchy == HierarchyType::User {
                let users_dir = root.join("users");
                std::fs::create_dir_all(&users_dir)?;
            }
            
            Ok(Self {
                uuid: uuid.to_string(),
                root_path: root,
                hierarchy,
                read_only,
            })
        }
    }
    
    #[async_trait]
    impl FileSystemAbstraction for LocalFileSystem {
        fn name(&self) -> &str {
            &self.uuid
        }
        
        async fn chmod(&self, path: &str, mode: u32) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = std::fs::Permissions::from_mode(mode);
                tokio::fs::set_permissions(&full_path, permissions).await?;
            }
            Ok(())
        }
        
        async fn chown(&self, _path: &str, _uid: u32, _gid: u32) -> Result<(), FilesystemError> {
            #[cfg(unix)]
            {
                use std::os::unix::fs::chown;
                if self.read_only {
                    return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
                }
                // Implementation for Unix systems
            }
            Ok(())
        }
        
        async fn chtimes(&self, path: &str, atime: SystemTime, mtime: SystemTime) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let file = File::open(&full_path).await?;
            // Note: tokio doesn't directly support setting times, would need to use std::fs
            Ok(())
        }
        
        async fn create(&self, path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(&full_path)
                .await?;
            Ok(Box::new(LocalFile::new(file, path)))
        }
        
        async fn mkdir(&self, path: &str, mode: u32) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            fs::create_dir(&full_path).await?;
            Ok(())
        }
        
        async fn mkdir_all(&self, path: &str, mode: u32) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            fs::create_dir_all(&full_path).await?;
            Ok(())
        }
        
        async fn open(&self, path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            if !full_path.exists() {
                return Err(FilesystemError::NotFound(path.to_string()));
            }
            let file = OpenOptions::new()
                .read(true)
                .write(!self.read_only)
                .open(&full_path)
                .await?;
            Ok(Box::new(LocalFile::new(file, path)))
        }
        
        async fn open_file(&self, path: &str, flags: OpenFlags, mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            if self.read_only && flags.write {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let mut opts = OpenOptions::new();
            opts.read(flags.read).write(flags.write);
            
            if flags.create {
                opts.create(true);
            }
            if flags.truncate {
                opts.truncate(true);
            }
            if flags.append {
                opts.append(true);
            }
            
            let file = opts.open(&full_path).await?;
            Ok(Box::new(LocalFile::new(file, path)))
        }
        
        async fn remove(&self, path: &str) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            if full_path.is_dir() {
                fs::remove_dir(&full_path).await?;
            } else {
                fs::remove_file(&full_path).await?;
            }
            Ok(())
        }
        
        async fn remove_all(&self, path: &str) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            fs::remove_dir_all(&full_path).await?;
            Ok(())
        }
        
        async fn rename(&self, from: &str, to: &str) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let from_path = self.root_path.join(from.trim_start_matches('/'));
            let to_path = self.root_path.join(to.trim_start_matches('/'));
            fs::rename(&from_path, &to_path).await?;
            Ok(())
        }
        
        async fn stat(&self, path: &str) -> Result<FileInfo, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let metadata = fs::metadata(&full_path).await?;
            
            let modified = metadata.modified()?
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let accessed = metadata.accessed()?
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let created = metadata.created()?
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            Ok(FileInfo {
                name: full_path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string(),
                path: path.to_string(),
                size: metadata.len(),
                is_dir: metadata.is_dir(),
                mode: 0o755, // Default mode
                modified,
                accessed,
                created,
            })
        }
        
        async fn close(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
        
        fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError> {
            let clean_path = virtual_path.trim_start_matches('/');
            Ok(self.root_path.join(clean_path).to_string_lossy().to_string())
        }
        
        fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError> {
            let real = PathBuf::from(real_path);
            if let Ok(relative) = real.strip_prefix(&self.root_path) {
                Ok(format!("/{}", relative.to_string_lossy()))
            } else {
                Err(FilesystemError::InvalidPath("Path not within filesystem root".to_string()))
            }
        }
        
        async fn file_exists(&self, path: &str) -> bool {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            full_path.exists()
        }
        
        async fn is_dir(&self, path: &str) -> bool {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            full_path.is_dir()
        }
        
        async fn glob(&self, pattern: &str) -> Result<Vec<String>, FilesystemError> {
            use glob::glob;
            let full_pattern = self.root_path.join(pattern.trim_start_matches('/')).to_string_lossy().to_string();
            let mut results = Vec::new();
            
            for entry in glob(&full_pattern)? {
                if let Ok(path) = entry {
                    if let Ok(relative) = path.strip_prefix(&self.root_path) {
                        results.push(format!("/{}", relative.to_string_lossy()));
                    }
                }
            }
            
            Ok(results)
        }
        
        async fn get_file_size(&self, path: &str) -> Result<u64, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let metadata = fs::metadata(&full_path).await?;
            Ok(metadata.len())
        }
        
        async fn get_mod_time(&self, path: &str) -> Result<u64, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let metadata = fs::metadata(&full_path).await?;
            Ok(metadata.modified()?
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs())
        }
        
        async fn write_file(&self, path: &str, content: &[u8], mode: u32) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            fs::write(&full_path, content).await?;
            Ok(())
        }
        
        async fn read_file(&self, path: &str) -> Result<Vec<u8>, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let content = fs::read(&full_path).await?;
            Ok(content)
        }
        
        async fn read_dir(&self, path: &str) -> Result<Vec<DirEntry>, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let mut entries = Vec::new();
            
            let mut dir = fs::read_dir(&full_path).await?;
            while let Some(entry) = dir.next_entry().await? {
                let file_type = entry.file_type().await?;
                entries.push(DirEntry {
                    name: entry.file_name()
                        .to_string_lossy()
                        .to_string(),
                    is_dir: file_type.is_dir(),
                    file_type: if file_type.is_dir() {
                        FileType::Directory
                    } else if file_type.is_file() {
                        FileType::File
                    } else if file_type.is_symlink() {
                        FileType::Symlink
                    } else {
                        FileType::Unknown
                    },
                });
            }
            
            Ok(entries)
        }
        
        async fn write_stream<P: AsRef<Path>>(&self, path: P, mut reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError> {
            if self.read_only {
                return Err(FilesystemError::PermissionDenied("Filesystem is read-only".to_string()));
            }
            
            let full_path = self.root_path.join(path.as_ref());
            if let Some(parent) = full_path.parent() {
                fs::create_dir_all(parent).await?;
            }
            
            let mut file = File::create(&full_path).await?;
            tokio::io::copy(&mut reader, &mut file).await?;
            Ok(())
        }
        
        async fn read_stream(&self, path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError> {
            let full_path = self.root_path.join(path.trim_start_matches('/'));
            let file = File::open(&full_path).await?;
            Ok(Box::new(file))
        }
        
        async fn walk<F>(&self, root: &str, mut callback: F) -> Result<(), FilesystemError>
        where
            F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send,
        {
            use futures::stream::{self, StreamExt};
            
            let root_path = self.root_path.join(root.trim_start_matches('/'));
            let mut entries = Vec::new();
            
            // Collect all entries first
            async fn collect_entries(dir: &Path, entries: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
                let mut queue = vec![dir.to_path_buf()];
                while let Some(current) = queue.pop() {
                    let mut dir_stream = fs::read_dir(&current).await?;
                    while let Some(entry) = dir_stream.next_entry().await? {
                        let path = entry.path();
                        entries.push(path.clone());
                        if entry.file_type().await?.is_dir() {
                            queue.push(path);
                        }
                    }
                }
                Ok(())
            }
            
            collect_entries(&root_path, &mut entries).await?;
            
            // Process each entry
            for entry_path in entries {
                if let Ok(metadata) = fs::metadata(&entry_path).await {
                    let virtual_path = self.real_to_virtual_path("", &entry_path.to_string_lossy())?;
                    let modified = metadata.modified()?
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    let info = FileInfo {
                        name: entry_path.file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("")
                            .to_string(),
                        path: virtual_path.clone(),
                        size: metadata.len(),
                        is_dir: metadata.is_dir(),
                        mode: 0o755,
                        modified,
                        accessed: modified,
                        created: modified,
                    };
                    
                    callback(&virtual_path, &info)?;
                }
            }
            
            Ok(())
        }
        
        async fn heartbeat(&self) -> Result<(), FilesystemError> {
            // Check if root path is accessible
            if self.root_path.exists() {
                Ok(())
            } else {
                Err(FilesystemError::NotFound("Root path not accessible".to_string()))
            }
        }
    }
    
    /// Local file wrapper
    pub struct LocalFile {
        file: File,
        path: String,
    }
    
    impl LocalFile {
        pub fn new(file: File, path: &str) -> Self {
            Self {
                file,
                path: path.to_string(),
            }
        }
    }
    
    #[async_trait]
    impl AsyncFile for LocalFile {
        fn path(&self) -> &str {
            &self.path
        }
        
        async fn sync(&mut self) -> Result<(), FilesystemError> {
            self.file.sync_all().await?;
            Ok(())
        }
        
        async fn truncate(&mut self, size: u64) -> Result<(), FilesystemError> {
            self.file.set_len(size).await?;
            Ok(())
        }
    }
    
    impl AsyncRead for LocalFile {
        fn poll_read(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &mut tokio::io::ReadBuf<'_>,
        ) -> std::task::Poll<std::io::Result<()>> {
            std::pin::Pin::new(&mut self.file).poll_read(cx, buf)
        }
    }
    
    impl AsyncWrite for LocalFile {
        fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
            buf: &[u8],
        ) -> std::task::Poll<Result<usize, std::io::Error>> {
            std::pin::Pin::new(&mut self.file).poll_write(cx, buf)
        }
        
        fn poll_flush(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            std::pin::Pin::new(&mut self.file).poll_flush(cx)
        }
        
        fn poll_shutdown(
            mut self: std::pin::Pin<&mut Self>,
            cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Result<(), std::io::Error>> {
            std::pin::Pin::new(&mut self.file).poll_shutdown(cx)
        }
    }
}

// Placeholder modules for other filesystem types
pub mod webdav_fs {
    use super::*;
    
    pub struct WebDAVFileSystem {
        uuid: String,
        hierarchy: HierarchyType,
        url: String,
        // Additional WebDAV client fields would go here
    }
    
    impl WebDAVFileSystem {
        pub fn new(
            uuid: &str,
            hierarchy: HierarchyType,
            url: &str,
            username: &str,
            password: &str,
        ) -> Result<Self, FilesystemError> {
            // Initialize WebDAV client
            Ok(Self {
                uuid: uuid.to_string(),
                hierarchy,
                url: url.to_string(),
            })
        }
    }
    
    #[async_trait]
    impl FileSystemAbstraction for WebDAVFileSystem {
        fn name(&self) -> &str {
            &self.uuid
        }
        
        // Implement all trait methods with WebDAV-specific logic
        // For brevity, returning unsupported errors as placeholders
        async fn chmod(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chmod not supported on WebDAV".to_string()))
        }
        
        async fn chown(&self, _path: &str, _uid: u32, _gid: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chown not supported on WebDAV".to_string()))
        }
        
        async fn chtimes(&self, _path: &str, _atime: SystemTime, _mtime: SystemTime) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chtimes not supported on WebDAV".to_string()))
        }
        
        async fn create(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("create requires buffered stream on WebDAV".to_string()))
        }
        
        async fn mkdir(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir placeholder".to_string()))
        }
        
        async fn mkdir_all(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir_all placeholder".to_string()))
        }
        
        async fn open(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open requires buffered stream on WebDAV".to_string()))
        }
        
        async fn open_file(&self, _path: &str, _flags: OpenFlags, _mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open_file requires buffered stream on WebDAV".to_string()))
        }
        
        async fn remove(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove placeholder".to_string()))
        }
        
        async fn remove_all(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove_all placeholder".to_string()))
        }
        
        async fn rename(&self, _from: &str, _to: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("rename placeholder".to_string()))
        }
        
        async fn stat(&self, _path: &str) -> Result<FileInfo, FilesystemError> {
            Err(FilesystemError::Unsupported("stat placeholder".to_string()))
        }
        
        async fn close(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
        
        fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError> {
            Ok(format!("{}{}", self.url, virtual_path))
        }
        
        fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError> {
            if real_path.starts_with(&self.url) {
                Ok(real_path[self.url.len()..].to_string())
            } else {
                Err(FilesystemError::InvalidPath("Path not within WebDAV root".to_string()))
            }
        }
        
        async fn file_exists(&self, _path: &str) -> bool {
            false // Placeholder
        }
        
        async fn is_dir(&self, _path: &str) -> bool {
            false // Placeholder
        }
        
        async fn glob(&self, _pattern: &str) -> Result<Vec<String>, FilesystemError> {
            Ok(vec![]) // Placeholder
        }
        
        async fn get_file_size(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_file_size placeholder".to_string()))
        }
        
        async fn get_mod_time(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_mod_time placeholder".to_string()))
        }
        
        async fn write_file(&self, _path: &str, _content: &[u8], _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_file requires stream on WebDAV".to_string()))
        }
        
        async fn read_file(&self, _path: &str) -> Result<Vec<u8>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_file requires stream on WebDAV".to_string()))
        }
        
        async fn read_dir(&self, _path: &str) -> Result<Vec<DirEntry>, FilesystemError> {
            Ok(vec![]) // Placeholder
        }
        
        async fn write_stream<P: AsRef<Path>>(&self, _path: P, _reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_stream placeholder".to_string()))
        }
        
        async fn read_stream(&self, _path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_stream placeholder".to_string()))
        }
        
        async fn walk<F>(&self, _root: &str, _callback: F) -> Result<(), FilesystemError>
        where
            F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send,
        {
            Err(FilesystemError::Unsupported("walk placeholder".to_string()))
        }
        
        async fn heartbeat(&self) -> Result<(), FilesystemError> {
            // Would check WebDAV server connectivity
            Ok(())
        }
    }
}

pub mod smb_fs {
    use super::*;
    
    pub struct SMBFileSystem {
        uuid: String,
        hierarchy: HierarchyType,
        // SMB client fields would go here
    }
    
    impl SMBFileSystem {
        pub fn new(
            uuid: &str,
            hierarchy: HierarchyType,
            host: &str,
            share: &str,
            username: &str,
            password: &str,
        ) -> Result<Self, FilesystemError> {
            Ok(Self {
                uuid: uuid.to_string(),
                hierarchy,
            })
        }
    }
    
    #[async_trait]
    impl FileSystemAbstraction for SMBFileSystem {
        fn name(&self) -> &str {
            &self.uuid
        }
        
        async fn chmod(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chmod not supported on SMB".to_string()))
        }
        
        async fn chown(&self, _path: &str, _uid: u32, _gid: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chown not supported on SMB".to_string()))
        }
        
        async fn chtimes(&self, _path: &str, _atime: SystemTime, _mtime: SystemTime) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chtimes not supported on SMB".to_string()))
        }
        
        async fn create(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("create placeholder".to_string()))
        }
        
        async fn mkdir(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir placeholder".to_string()))
        }
        
        async fn mkdir_all(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir_all placeholder".to_string()))
        }
        
        async fn open(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open placeholder".to_string()))
        }
        
        async fn open_file(&self, _path: &str, _flags: OpenFlags, _mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open_file placeholder".to_string()))
        }
        
        async fn remove(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove placeholder".to_string()))
        }
        
        async fn remove_all(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove_all placeholder".to_string()))
        }
        
        async fn rename(&self, _from: &str, _to: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("rename placeholder".to_string()))
        }
        
        async fn stat(&self, _path: &str) -> Result<FileInfo, FilesystemError> {
            Err(FilesystemError::Unsupported("stat placeholder".to_string()))
        }
        
        async fn close(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
        
        fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError> {
            Ok(virtual_path.to_string())
        }
        
        fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError> {
            Ok(real_path.to_string())
        }
        
        async fn file_exists(&self, _path: &str) -> bool {
            false
        }
        
        async fn is_dir(&self, _path: &str) -> bool {
            false
        }
        
        async fn glob(&self, _pattern: &str) -> Result<Vec<String>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn get_file_size(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_file_size placeholder".to_string()))
        }
        
        async fn get_mod_time(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_mod_time placeholder".to_string()))
        }
        
        async fn write_file(&self, _path: &str, _content: &[u8], _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_file placeholder".to_string()))
        }
        
        async fn read_file(&self, _path: &str) -> Result<Vec<u8>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_file placeholder".to_string()))
        }
        
        async fn read_dir(&self, _path: &str) -> Result<Vec<DirEntry>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn write_stream<P: AsRef<Path>>(&self, _path: P, _reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_stream placeholder".to_string()))
        }
        
        async fn read_stream(&self, _path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_stream placeholder".to_string()))
        }
        
        async fn walk<F>(&self, _root: &str, _callback: F) -> Result<(), FilesystemError>
        where
            F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send,
        {
            Err(FilesystemError::Unsupported("walk placeholder".to_string()))
        }
        
        async fn heartbeat(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
    }
}

pub mod sftp_fs {
    use super::*;
    
    pub struct SFTPFileSystem {
        uuid: String,
        hierarchy: HierarchyType,
        // SFTP client fields would go here
    }
    
    impl SFTPFileSystem {
        pub fn new(
            uuid: &str,
            hierarchy: HierarchyType,
            host: &str,
            port: u16,
            username: &str,
            password: &str,
        ) -> Result<Self, FilesystemError> {
            Ok(Self {
                uuid: uuid.to_string(),
                hierarchy,
            })
        }
    }
    
    #[async_trait]
    impl FileSystemAbstraction for SFTPFileSystem {
        fn name(&self) -> &str {
            &self.uuid
        }
        
        async fn chmod(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chmod placeholder".to_string()))
        }
        
        async fn chown(&self, _path: &str, _uid: u32, _gid: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chown placeholder".to_string()))
        }
        
        async fn chtimes(&self, _path: &str, _atime: SystemTime, _mtime: SystemTime) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chtimes placeholder".to_string()))
        }
        
        async fn create(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("create placeholder".to_string()))
        }
        
        async fn mkdir(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir placeholder".to_string()))
        }
        
        async fn mkdir_all(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir_all placeholder".to_string()))
        }
        
        async fn open(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open placeholder".to_string()))
        }
        
        async fn open_file(&self, _path: &str, _flags: OpenFlags, _mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open_file placeholder".to_string()))
        }
        
        async fn remove(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove placeholder".to_string()))
        }
        
        async fn remove_all(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove_all placeholder".to_string()))
        }
        
        async fn rename(&self, _from: &str, _to: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("rename placeholder".to_string()))
        }
        
        async fn stat(&self, _path: &str) -> Result<FileInfo, FilesystemError> {
            Err(FilesystemError::Unsupported("stat placeholder".to_string()))
        }
        
        async fn close(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
        
        fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError> {
            Ok(virtual_path.to_string())
        }
        
        fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError> {
            Ok(real_path.to_string())
        }
        
        async fn file_exists(&self, _path: &str) -> bool {
            false
        }
        
        async fn is_dir(&self, _path: &str) -> bool {
            false
        }
        
        async fn glob(&self, _pattern: &str) -> Result<Vec<String>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn get_file_size(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_file_size placeholder".to_string()))
        }
        
        async fn get_mod_time(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_mod_time placeholder".to_string()))
        }
        
        async fn write_file(&self, _path: &str, _content: &[u8], _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_file placeholder".to_string()))
        }
        
        async fn read_file(&self, _path: &str) -> Result<Vec<u8>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_file placeholder".to_string()))
        }
        
        async fn read_dir(&self, _path: &str) -> Result<Vec<DirEntry>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn write_stream<P: AsRef<Path>>(&self, _path: P, _reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_stream placeholder".to_string()))
        }
        
        async fn read_stream(&self, _path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_stream placeholder".to_string()))
        }
        
        async fn walk<F>(&self, _root: &str, _callback: F) -> Result<(), FilesystemError>
        where
            F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send,
        {
            Err(FilesystemError::Unsupported("walk placeholder".to_string()))
        }
        
        async fn heartbeat(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
    }
}

pub mod ftp_fs {
    use super::*;
    
    pub struct FTPFileSystem {
        uuid: String,
        hierarchy: HierarchyType,
        // FTP client fields would go here
    }
    
    impl FTPFileSystem {
        pub fn new(
            uuid: &str,
            hierarchy: HierarchyType,
            host: &str,
            port: u16,
            username: &str,
            password: &str,
        ) -> Result<Self, FilesystemError> {
            Ok(Self {
                uuid: uuid.to_string(),
                hierarchy,
            })
        }
    }
    
    #[async_trait]
    impl FileSystemAbstraction for FTPFileSystem {
        fn name(&self) -> &str {
            &self.uuid
        }
        
        async fn chmod(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chmod not supported on FTP".to_string()))
        }
        
        async fn chown(&self, _path: &str, _uid: u32, _gid: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chown not supported on FTP".to_string()))
        }
        
        async fn chtimes(&self, _path: &str, _atime: SystemTime, _mtime: SystemTime) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("chtimes not supported on FTP".to_string()))
        }
        
        async fn create(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("create requires stream on FTP".to_string()))
        }
        
        async fn mkdir(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir placeholder".to_string()))
        }
        
        async fn mkdir_all(&self, _path: &str, _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("mkdir_all placeholder".to_string()))
        }
        
        async fn open(&self, _path: &str) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open requires stream on FTP".to_string()))
        }
        
        async fn open_file(&self, _path: &str, _flags: OpenFlags, _mode: u32) -> Result<Box<dyn AsyncFile>, FilesystemError> {
            Err(FilesystemError::Unsupported("open_file requires stream on FTP".to_string()))
        }
        
        async fn remove(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove placeholder".to_string()))
        }
        
        async fn remove_all(&self, _path: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("remove_all placeholder".to_string()))
        }
        
        async fn rename(&self, _from: &str, _to: &str) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("rename placeholder".to_string()))
        }
        
        async fn stat(&self, _path: &str) -> Result<FileInfo, FilesystemError> {
            Err(FilesystemError::Unsupported("stat placeholder".to_string()))
        }
        
        async fn close(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
        
        fn virtual_to_real_path(&self, base: &str, virtual_path: &str) -> Result<String, FilesystemError> {
            Ok(virtual_path.to_string())
        }
        
        fn real_to_virtual_path(&self, base: &str, real_path: &str) -> Result<String, FilesystemError> {
            Ok(real_path.to_string())
        }
        
        async fn file_exists(&self, _path: &str) -> bool {
            false
        }
        
        async fn is_dir(&self, _path: &str) -> bool {
            false
        }
        
        async fn glob(&self, _pattern: &str) -> Result<Vec<String>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn get_file_size(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_file_size placeholder".to_string()))
        }
        
        async fn get_mod_time(&self, _path: &str) -> Result<u64, FilesystemError> {
            Err(FilesystemError::Unsupported("get_mod_time placeholder".to_string()))
        }
        
        async fn write_file(&self, _path: &str, _content: &[u8], _mode: u32) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_file requires stream on FTP".to_string()))
        }
        
        async fn read_file(&self, _path: &str) -> Result<Vec<u8>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_file requires stream on FTP".to_string()))
        }
        
        async fn read_dir(&self, _path: &str) -> Result<Vec<DirEntry>, FilesystemError> {
            Ok(vec![])
        }
        
        async fn write_stream<P: AsRef<Path>>(&self, _path: P, _reader: Box<dyn AsyncRead + Send>) -> Result<(), FilesystemError> {
            Err(FilesystemError::Unsupported("write_stream placeholder".to_string()))
        }
        
        async fn read_stream(&self, _path: &str) -> Result<Box<dyn AsyncRead + Send>, FilesystemError> {
            Err(FilesystemError::Unsupported("read_stream placeholder".to_string()))
        }
        
        async fn walk<F>(&self, _root: &str, _callback: F) -> Result<(), FilesystemError>
        where
            F: FnMut(&str, &FileInfo) -> Result<(), FilesystemError> + Send,
        {
            Err(FilesystemError::Unsupported("walk placeholder".to_string()))
        }
        
        async fn heartbeat(&self) -> Result<(), FilesystemError> {
            Ok(())
        }
    }
}
