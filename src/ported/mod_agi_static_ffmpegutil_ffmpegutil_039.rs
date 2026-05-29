//! Original Go file: `mod/agi/static/ffmpegutil/ffmpegutil.go`
//! Package: `ffmpegutil`; LOC: 98; SHA256: `f8db2cecd146a2241c182fd76ee00f93bf97f8783b2f5d70d3c8a9649c5c9867`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/static/ffmpegutil/ffmpegutil.go", package: "ffmpegutil", go_loc: 98, functions: 4, types: 0, sha256: "f8db2cecd146a2241c182fd76ee00f93bf97f8783b2f5d70d3c8a9649c5c9867" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/utils",
    "os",
    "os/exec",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("FFmpeg_conv", "", 30),
    ("isVideo", "", 79),
    ("isAudio", "", 86),
    ("isImage", "", 93),
];

pub async fn ffmpeg_conv(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/ffmpegutil/ffmpegutil.go", function: "FFmpeg_conv" })
}

pub async fn isvideo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/ffmpegutil/ffmpegutil.go", function: "isVideo" })
}

pub async fn isaudio(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/ffmpegutil/ffmpegutil.go", function: "isAudio" })
}

pub async fn isimage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/static/ffmpegutil/ffmpegutil.go", function: "isImage" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
