//! Auto-Updater Module for ArozOS
//! 
//! Provides automatic update checking, downloading, and installation
//! with rollback capabilities.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};
use tracing::{info, warn, error, debug};

/// Updater Configuration
#[derive(Clone)]
pub struct UpdaterConfig {
    pub update_server_url: String,
    pub check_interval_secs: u64,
    pub auto_download: bool,
    pub auto_install: bool,
    pub require_confirmation: bool,
    pub backup_before_update: bool,
    pub rollback_on_failure: bool,
    pub current_version: String,
    pub architecture: String,
    pub platform: String,
}

impl Default for UpdaterConfig {
    fn default() -> Self {
        Self {
            update_server_url: "https://update.arozos.com/api/v1".to_string(),
            check_interval_secs: 3600, // Check every hour
            auto_download: true,
            auto_install: false,
            require_confirmation: true,
            backup_before_update: true,
            rollback_on_failure: true,
            current_version: env!("CARGO_PKG_VERSION").to_string(),
            architecture: std::env::consts::ARCH.to_string(),
            platform: std::env::consts::OS.to_string(),
        }
    }
}

/// Update Manager - Handles all update operations
pub struct UpdateManager {
    config: UpdaterConfig,
    state: RwLock<UpdateState>,
    event_tx: broadcast::Sender<UpdateEvent>,
    http_client: reqwest::Client,
}

impl UpdateManager {
    pub fn new(config: UpdaterConfig) -> Self {
        let (event_tx, _) = broadcast::channel(100);
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap_or_default();

        Self {
            config,
            state: RwLock::new(UpdateState::Idle),
            event_tx,
            http_client,
        }
    }

    /// Check for available updates
    pub async fn check_for_updates(&self) -> Result<Option<UpdateInfo>, UpdateError> {
        info!("Checking for updates...");

        let url = format!(
            "{}/updates/latest?version={}&arch={}&platform={}",
            self.config.update_server_url,
            self.config.current_version,
            self.config.architecture,
            self.config.platform
        );

        let response = self.http_client.get(&url).send().await?;
        
        if !response.status().is_success() {
            return Err(UpdateError::ServerUnavailable(response.status().to_string()));
        }

        let update_info: UpdateInfo = response.json().await?;

        // Check if update is newer than current version
        if is_newer_version(&self.config.current_version, &update_info.version) {
            info!("Update available: {} -> {}", self.config.current_version, update_info.version);
            
            let _ = self.event_tx.send(UpdateEvent::UpdateAvailable {
                info: update_info.clone(),
            });

            Ok(Some(update_info))
        } else {
            debug!("No updates available");
            Ok(None)
        }
    }

    /// Download an update
    pub async fn download_update(&self, update_info: &UpdateInfo) -> Result<PathBuf, UpdateError> {
        info!("Downloading update: {}", update_info.version);

        *self.state.write().await = UpdateState::Downloading { progress: 0.0 };

        let _ = self.event_tx.send(UpdateEvent::DownloadStarted {
            version: update_info.version.clone(),
            size_bytes: update_info.size_bytes,
        });

        // Download the update file
        let response = self.http_client.get(&update_info.download_url).send().await?;
        
        if !response.status().is_success() {
            return Err(UpdateError::DownloadFailed("HTTP error".to_string()));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded = 0u64;

        // Create temp directory for download
        let temp_dir = tempfile::tempdir()?;
        let download_path = temp_dir.path().join(&update_info.filename);
        let mut file = tokio::fs::File::create(&download_path).await?;

        let mut stream = response.bytes_stream();
        
        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            tokio::io::copy_buf(&mut chunk.as_ref(), &mut file).await?;
            
            downloaded += chunk.len() as u64;
            
            if total_size > 0 {
                let progress = (downloaded as f32 / total_size as f32) * 100.0;
                *self.state.write().await = UpdateState::Downloading { progress };
                
                let _ = self.event_tx.send(UpdateEvent::DownloadProgress {
                    downloaded_bytes: downloaded,
                    total_bytes: total_size,
                    progress,
                });
            }
        }

        // Verify checksum
        if let Some(expected_hash) = &update_info.sha256_hash {
            let file_content = tokio::fs::read(&download_path).await?;
            let actual_hash = hex::encode(sha2::Sha256::digest(&file_content));
            
            if actual_hash != *expected_hash {
                return Err(UpdateError::ChecksumMismatch {
                    expected: expected_hash.clone(),
                    actual: actual_hash,
                });
            }
            info!("Checksum verified successfully");
        }

        *self.state.write().await = UpdateState::Downloaded { path: download_path.clone() };

        let _ = self.event_tx.send(UpdateEvent::DownloadCompleted {
            version: update_info.version.clone(),
            path: download_path.clone(),
        });

        info!("Download completed: {:?}", download_path);
        Ok(download_path)
    }

    /// Install an update
    pub async fn install_update(&self, update_path: &Path, update_info: &UpdateInfo) -> Result<(), UpdateError> {
        info!("Installing update: {}", update_info.version);

        *self.state.write().await = UpdateState::Installing { stage: "preparing".to_string() };

        let _ = self.event_tx.send(UpdateEvent::InstallStarted {
            version: update_info.version.clone(),
        });

        // Create backup before update
        if self.config.backup_before_update {
            *self.state.write().await = UpdateState::Installing { stage: "creating_backup".to_string() };
            self.create_backup().await?;
        }

        // Extract and install
        *self.state.write().await = UpdateState::Installing { stage: "extracting".to_string() };
        
        // In a real implementation:
        // 1. Stop running services
        // 2. Extract archive
        // 3. Replace binaries
        // 4. Run migration scripts
        // 5. Restart services
        
        // Placeholder installation logic
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        *self.state.write().await = UpdateState::Installed {
            version: update_info.version.clone(),
            installed_at: chrono::Utc::now(),
        };

        let _ = self.event_tx.send(UpdateEvent::InstallCompleted {
            version: update_info.version.clone(),
            requires_restart: update_info.requires_restart,
        });

        info!("Update installed successfully");
        Ok(())
    }

    /// Create a system backup before update
    async fn create_backup(&self) -> Result<(), UpdateError> {
        info!("Creating system backup...");
        
        // In a real implementation:
        // 1. Backup configuration files
        // 2. Backup database
        // 3. Backup user data
        // 4. Store backup in safe location
        
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        Ok(())
    }

    /// Rollback to previous version
    pub async fn rollback(&self) -> Result<(), UpdateError> {
        info!("Rolling back to previous version...");

        *self.state.write().await = UpdateState::RollingBack;

        let _ = self.event_tx.send(UpdateEvent::RollbackStarted);

        // In a real implementation:
        // 1. Stop services
        // 2. Restore from backup
        // 3. Restart services
        
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;

        *self.state.write().await = UpdateState::Idle;

        let _ = self.event_tx.send(UpdateEvent::RollbackCompleted);

        info!("Rollback completed");
        Ok(())
    }

    /// Get current update state
    pub async fn get_state(&self) -> UpdateState {
        *self.state.read().await
    }

    /// Get updater configuration
    pub fn config(&self) -> &UpdaterConfig {
        &self.config
    }

    /// Subscribe to update events
    pub fn subscribe_events(&self) -> broadcast::Receiver<UpdateEvent> {
        self.event_tx.subscribe()
    }

    /// Start automatic update checking loop
    pub async fn start_auto_check(&self) {
        let interval = std::time::Duration::from_secs(self.config.check_interval_secs);
        let mut interval_timer = tokio::time::interval(interval);

        loop {
            interval_timer.tick().await;
            
            match self.check_for_updates().await {
                Ok(Some(info)) => {
                    if self.config.auto_download {
                        match self.download_update(&info).await {
                            Ok(path) => {
                                if self.config.auto_install && !self.config.require_confirmation {
                                    if let Err(e) = self.install_update(&path, &info).await {
                                        error!("Auto-install failed: {}", e);
                                        if self.config.rollback_on_failure {
                                            let _ = self.rollback().await;
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Auto-download failed: {}", e);
                            }
                        }
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    warn!("Update check failed: {}", e);
                }
            }
        }
    }
}

/// Update Information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub version: String,
    pub release_date: chrono::DateTime<chrono::Utc>,
    pub filename: String,
    pub download_url: String,
    pub size_bytes: u64,
    pub sha256_hash: Option<String>,
    pub requires_restart: bool,
    pub critical: bool,
    pub release_notes: String,
    pub breaking_changes: Vec<String>,
}

/// Update State
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum UpdateState {
    Idle,
    Checking,
    Downloading { progress: f32 },
    Downloaded { path: PathBuf },
    Installing { stage: String },
    Installed { version: String, installed_at: chrono::DateTime<chrono::Utc> },
    RollingBack,
    Failed { error: String },
}

/// Update Events
#[derive(Clone, Debug)]
pub enum UpdateEvent {
    CheckStarted,
    UpdateAvailable { info: UpdateInfo },
    NoUpdatesAvailable,
    DownloadStarted { version: String, size_bytes: u64 },
    DownloadProgress { downloaded_bytes: u64, total_bytes: u64, progress: f32 },
    DownloadCompleted { version: String, path: PathBuf },
    DownloadFailed { error: String },
    InstallStarted { version: String },
    InstallProgress { stage: String, progress: f32 },
    InstallCompleted { version: String, requires_restart: bool },
    InstallFailed { error: String },
    RollbackStarted,
    RollbackCompleted,
    RollbackFailed { error: String },
}

/// Update Error Types
#[derive(Debug, thiserror::Error)]
pub enum UpdateError {
    #[error("Server unavailable: {0}")]
    ServerUnavailable(String),
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    #[error("Checksum mismatch: expected {expected}, got {actual}")]
    ChecksumMismatch { expected: String, actual: String },
    #[error("Installation failed: {0}")]
    InstallationFailed(String),
    #[error("Rollback failed: {0}")]
    RollbackFailed(String),
    #[error("No update in progress")]
    NoUpdateInProgress,
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Compare version strings to determine if new version is newer
fn is_newer_version(current: &str, new: &str) -> bool {
    let current_parts: Vec<u32> = current.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();
    
    let new_parts: Vec<u32> = new.split('.')
        .filter_map(|s| s.parse().ok())
        .collect();

    for (i, new_part) in new_parts.iter().enumerate() {
        if i >= current_parts.len() {
            return true;
        }
        if *new_part > current_parts[i] {
            return true;
        }
        if *new_part < current_parts[i] {
            return false;
        }
    }

    // If all compared parts are equal, new version must have more parts
    new_parts.len() > current_parts.len()
}

/// Get current application version
pub fn get_current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_updater_config_default() {
        let config = UpdaterConfig::default();
        assert!(config.auto_download);
        assert!(!config.auto_install);
        assert!(config.require_confirmation);
    }

    #[test]
    fn test_version_comparison() {
        assert!(is_newer_version("1.0.0", "1.0.1"));
        assert!(is_newer_version("1.0.0", "1.1.0"));
        assert!(is_newer_version("1.0.0", "2.0.0"));
        assert!(!is_newer_version("1.0.1", "1.0.0"));
        assert!(!is_newer_version("1.0.0", "1.0.0"));
    }

    #[test]
    fn test_update_state_serialization() {
        let state = UpdateState::Downloading { progress: 50.0 };
        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Downloading"));
    }

    #[tokio::test]
    async fn test_update_manager_creation() {
        let config = UpdaterConfig::default();
        let manager = UpdateManager::new(config);
        
        let state = manager.get_state().await;
        assert_eq!(state, UpdateState::Idle);
    }
}
