//! Media Transcoder Module for ArozOS
//! 
//! Provides video/audio transcoding capabilities using FFmpeg
//! for on-the-fly media conversion and optimization.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{RwLock, broadcast};
use tracing::{info, warn, error, debug};

/// Transcoder Configuration
#[derive(Clone)]
pub struct TranscoderConfig {
    pub ffmpeg_path: Option<PathBuf>,
    pub ffprobe_path: Option<PathBuf>,
    pub temp_dir: PathBuf,
    pub max_concurrent_jobs: usize,
    pub default_video_codec: String,
    pub default_audio_codec: String,
    pub default_container: String,
    pub hardware_acceleration: bool,
    pub acceleration_method: String, // "nvenc", "vaapi", "qsv", etc.
}

impl Default for TranscoderConfig {
    fn default() -> Self {
        Self {
            ffmpeg_path: None, // Will search in PATH
            ffprobe_path: None,
            temp_dir: PathBuf::from("/tmp/arozos/transcode"),
            max_concurrent_jobs: 2,
            default_video_codec: "libx264".to_string(),
            default_audio_codec: "aac".to_string(),
            default_container: "mp4".to_string(),
            hardware_acceleration: false,
            acceleration_method: "auto".to_string(),
        }
    }
}

/// Media Transcoder Manager
pub struct Transcoder {
    config: TranscoderConfig,
    active_jobs: RwLock<Vec<TranscodeJob>>,
    job_tx: broadcast::Sender<TranscodeEvent>,
    semaphore: Arc<tokio::sync::Semaphore>,
}

impl Transcoder {
    pub fn new(config: TranscoderConfig) -> Self {
        let (job_tx, _) = broadcast::channel(100);
        let semaphore = Arc::new(tokio::sync::Semaphore::new(config.max_concurrent_jobs));

        Self {
            config,
            active_jobs: RwLock::new(Vec::new()),
            job_tx,
            semaphore,
        }
    }

    /// Get media file information
    pub async fn probe(&self, input_path: &Path) -> Result<MediaInfo, TranscodeError> {
        let ffprobe = self.config.ffprobe_path.as_deref().unwrap_or(std::path::Path::new("ffprobe"));

        let output = Command::new(ffprobe)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                input_path.to_str().ok_or(TranscodeError::InvalidPath)?,
            ])
            .output()
            .await?;

        if !output.status.success() {
            return Err(TranscodeError::ProbeFailed(String::from_utf8_lossy(&output.stderr).to_string()));
        }

        let info: MediaInfo = serde_json::from_slice(&output.stdout)?;
        Ok(info)
    }

    /// Start a transcoding job
    pub async fn transcode(&self, request: TranscodeRequest) -> Result<String, TranscodeError> {
        // Acquire semaphore slot
        let _permit = self.semaphore.acquire().await
            .map_err(|_| TranscodeError::Shutdown)?;

        let job_id = uuid::Uuid::new_v4().to_string();
        
        info!("Starting transcode job {}: {} -> {}", job_id, request.input.display(), request.output.display());

        // Build FFmpeg command
        let mut cmd = Command::new(self.config.ffmpeg_path.as_deref().unwrap_or(std::path::Path::new("ffmpeg")));
        
        // Input
        cmd.arg("-i").arg(&request.input);

        // Video codec
        if let Some(video_codec) = &request.video_codec {
            cmd.arg("-c:v").arg(video_codec);
            
            // Add hardware acceleration flags if enabled
            if self.config.hardware_acceleration && video_codec == "h264_nvenc" {
                cmd.arg("-preset").arg("p2");
                cmd.arg("-tune").arg("hq");
            }
        } else {
            cmd.arg("-c:v").arg(&self.config.default_video_codec);
        }

        // Audio codec
        if let Some(audio_codec) = &request.audio_codec {
            cmd.arg("-c:a").arg(audio_codec);
        } else {
            cmd.arg("-c:a").arg(&self.config.default_audio_codec);
        }

        // Bitrate
        if let Some(video_bitrate) = request.video_bitrate {
            cmd.arg("-b:v").arg(format!("{}k", video_bitrate));
        }

        if let Some(audio_bitrate) = request.audio_bitrate {
            cmd.arg("-b:a").arg(format!("{}k", audio_bitrate));
        }

        // Resolution scaling
        if let Some((width, height)) = request.resolution {
            cmd.arg("-vf").arg(format!("scale={}:{}", width, height));
        }

        // Frame rate
        if let Some(fps) = request.framerate {
            cmd.arg("-r").arg(fps.to_string());
        }

        // Output options
        cmd.arg("-y") // Overwrite output
           .arg(&request.output);

        // Configure pipes
        cmd.stdout(Stdio::piped())
            .stderr(Stdio::piped());

        // Spawn process
        let mut child = cmd.spawn()?;

        // Create job entry
        let job = TranscodeJob {
            id: job_id.clone(),
            request: request.clone(),
            status: TranscodeStatus::Running,
            progress: 0.0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            error_message: None,
        };

        {
            let mut jobs = self.active_jobs.write().await;
            jobs.push(job);
        }

        // Monitor progress
        let job_id_clone = job_id.clone();
        let event_tx = self.job_tx.clone();
        let stderr = child.stderr.take();

        tokio::spawn(async move {
            if let Some(stderr) = stderr {
                let reader = BufReader::new(stderr);
                let mut lines = reader.lines();
                
                while let Ok(Some(line)) = lines.next_line().await {
                    // Parse FFmpeg progress from stderr
                    if line.contains("time=") {
                        if let Some(progress) = parse_ffmpeg_progress(&line) {
                            let _ = event_tx.send(TranscodeEvent::Progress {
                                job_id: job_id_clone.clone(),
                                progress,
                            });
                        }
                    }
                }
            }

            // Wait for process completion
            let status = child.wait().await;
            
            match status {
                Ok(exit_status) if exit_status.success() => {
                    let _ = event_tx.send(TranscodeEvent::Completed {
                        job_id: job_id_clone.clone(),
                        output_path: request.output.clone(),
                    });
                }
                Ok(exit_status) => {
                    let _ = event_tx.send(TranscodeEvent::Failed {
                        job_id: job_id_clone.clone(),
                        error: format!("Process exited with code: {:?}", exit_status.code()),
                    });
                }
                Err(e) => {
                    let _ = event_tx.send(TranscodeEvent::Failed {
                        job_id: job_id_clone.clone(),
                        error: e.to_string(),
                    });
                }
            }
        });

        // Notify job started
        let _ = self.job_tx.send(TranscodeEvent::Started {
            job_id: job_id.clone(),
            request: request.clone(),
        });

        Ok(job_id)
    }

    /// Cancel a transcoding job
    pub async fn cancel(&self, job_id: &str) -> Result<(), TranscodeError> {
        let mut jobs = self.active_jobs.write().await;
        
        if let Some(pos) = jobs.iter().position(|j| j.id == job_id) {
            let job = jobs.remove(pos);
            // In a full implementation, we would track the child process
            // and send SIGTERM to kill it
            
            let _ = self.job_tx.send(TranscodeEvent::Cancelled {
                job_id: job.id,
            });
            
            Ok(())
        } else {
            Err(TranscodeError::JobNotFound(job_id.to_string()))
        }
    }

    /// Get job status
    pub async fn get_job_status(&self, job_id: &str) -> Option<TranscodeJob> {
        let jobs = self.active_jobs.read().await;
        jobs.iter().find(|j| j.id == job_id).cloned()
    }

    /// List all active jobs
    pub async fn list_jobs(&self) -> Vec<TranscodeJob> {
        let jobs = self.active_jobs.read().await;
        jobs.clone()
    }

    /// Subscribe to transcode events
    pub fn subscribe_events(&self) -> broadcast::Receiver<TranscodeEvent> {
        self.job_tx.subscribe()
    }

    /// Check if FFmpeg is available
    pub async fn check_ffmpeg() -> Result<FfmpegVersion, TranscodeError> {
        let output = Command::new("ffmpeg")
            .arg("-version")
            .output()
            .await?;

        if !output.status.success() {
            return Err(TranscodeError::FfmpegNotFound);
        }

        let version_output = String::from_utf8_lossy(&output.stdout);
        
        // Parse version string
        let version = version_output
            .lines()
            .next()
            .unwrap_or("unknown")
            .to_string();

        Ok(FfmpegVersion {
            raw: version_output.to_string(),
            version,
        })
    }
}

/// Transcode Request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscodeRequest {
    pub input: PathBuf,
    pub output: PathBuf,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub video_bitrate: Option<u32>, // in kbps
    pub audio_bitrate: Option<u32>, // in kbps
    pub resolution: Option<(u32, u32)>, // (width, height)
    pub framerate: Option<f32>,
    pub preset: Option<String>,
    pub crf: Option<u32>, // Constant Rate Factor (0-51, lower = better quality)
}

/// Media Information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaInfo {
    pub format: FormatInfo,
    pub video_streams: Vec<VideoStreamInfo>,
    pub audio_streams: Vec<AudioStreamInfo>,
    pub subtitle_streams: Vec<SubtitleStreamInfo>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FormatInfo {
    pub filename: String,
    pub duration_secs: f64,
    pub size_bytes: u64,
    pub bit_rate: Option<u64>,
    pub container_format: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VideoStreamInfo {
    pub index: u32,
    pub codec: String,
    pub profile: Option<String>,
    pub width: u32,
    pub height: u32,
    pub framerate: Option<f32>,
    pub bitrate: Option<u64>,
    pub pixel_format: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AudioStreamInfo {
    pub index: u32,
    pub codec: String,
    pub sample_rate: Option<u32>,
    pub channels: Option<u32>,
    pub bitrate: Option<u64>,
    pub language: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SubtitleStreamInfo {
    pub index: u32,
    pub codec: String,
    pub language: Option<String>,
}

/// Transcode Job Status
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TranscodeJob {
    pub id: String,
    pub request: TranscodeRequest,
    pub status: TranscodeStatus,
    pub progress: f32, // 0.0 to 100.0
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TranscodeStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// Transcode Events
#[derive(Clone, Debug)]
pub enum TranscodeEvent {
    Started {
        job_id: String,
        request: TranscodeRequest,
    },
    Progress {
        job_id: String,
        progress: f32,
    },
    Completed {
        job_id: String,
        output_path: PathBuf,
    },
    Failed {
        job_id: String,
        error: String,
    },
    Cancelled {
        job_id: String,
    },
}

/// FFmpeg Version Info
#[derive(Clone, Debug)]
pub struct FfmpegVersion {
    pub raw: String,
    pub version: String,
}

/// Transcode Error Types
#[derive(Debug, thiserror::Error)]
pub enum TranscodeError {
    #[error("FFmpeg not found")]
    FfmpegNotFound,
    #[error("Job not found: {0}")]
    JobNotFound(String),
    #[error("Invalid path")]
    InvalidPath,
    #[error("Probe failed: {0}")]
    ProbeFailed(String),
    #[error("Transcode failed: {0}")]
    TranscodeFailed(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("System shutdown")]
    Shutdown,
}

/// Parse FFmpeg progress from log line
fn parse_ffmpeg_progress(line: &str) -> Option<f32> {
    // Example: time=00:01:23.45
    if let Some(time_pos) = line.find("time=") {
        let time_str = &line[time_pos + 5..];
        if let Some(end_pos) = time_str.find(' ') {
            let time_str = &time_str[..end_pos];
            
            // Parse HH:MM:SS.xx format
            let parts: Vec<&str> = time_str.split(':').collect();
            if parts.len() == 3 {
                if let (Ok(h), Ok(m), Ok(s)) = (
                    parts[0].parse::<f32>(),
                    parts[1].parse::<f32>(),
                    parts[2].parse::<f32>(),
                ) {
                    let total_seconds = h * 3600.0 + m * 60.0 + s;
                    // This is a simplified progress calculation
                    // Real implementation would need total duration
                    return Some(total_seconds);
                }
            }
        }
    }
    None
}

/// Generate thumbnail from video
pub async fn generate_thumbnail(
    input_path: &Path,
    output_path: &Path,
    timestamp_secs: f32,
) -> Result<(), TranscodeError> {
    let output = Command::new("ffmpeg")
        .args([
            "-ss", &timestamp_secs.to_string(),
            "-i", input_path.to_str().ok_or(TranscodeError::InvalidPath)?,
            "-vframes", "1",
            "-q:v", "2",
            "-y",
            output_path.to_str().ok_or(TranscodeError::InvalidPath)?,
        ])
        .output()
        .await?;

    if !output.status.success() {
        return Err(TranscodeError::TranscodeFailed(
            String::from_utf8_lossy(&output.stderr).to_string()
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcoder_config_default() {
        let config = TranscoderConfig::default();
        assert_eq!(config.default_video_codec, "libx264");
        assert_eq!(config.default_audio_codec, "aac");
        assert_eq!(config.max_concurrent_jobs, 2);
    }

    #[test]
    fn test_parse_progress() {
        let line = "frame=  125 fps=0.0 q=0.0 size=       0kB time=00:00:05.00 bitrate=N/A";
        let progress = parse_ffmpeg_progress(line);
        assert!(progress.is_some());
        assert_eq!(progress.unwrap(), 5.0);
    }

    #[tokio::test]
    async fn test_check_ffmpeg() {
        // This test will fail if FFmpeg is not installed
        let result = Transcoder::check_ffmpeg().await;
        // Don't assert success - just verify it doesn't panic
        let _ = result;
    }
}
