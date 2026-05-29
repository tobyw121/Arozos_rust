//! Original Go file: `mod/media/mediaserver/mediaserver.go`
//! Package: `mediaserver`; LOC: 478; SHA256: `4b867404d2dd293bdb82f4939adf40aab73f2e4acb8bb663667d07af1235f70b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/media/mediaserver/mediaserver.go", package: "mediaserver", go_loc: 478, functions: 8, types: 2, sha256: "4b867404d2dd293bdb82f4939adf40aab73f2e4acb8bb663667d07af1235f70b" };

pub const GO_IMPORTS: &[&str] = &[
    "crypto/md5",
    "encoding/hex",
    "errors",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/compatibility",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/metadata",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/media/transcoder",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "io",
    "net/http",
    "net/url",
    "os",
    "path/filepath",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Options", "struct", 36),
    ("Instance", "struct", 47),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewMediaServer", "", 53),
    ("SetVirtualPathResolver", "s *Instance", 63),
    ("ValidateSourceFile", "s *Instance", 68),
    ("ServeMediaMime", "s *Instance", 139),
    ("ServerMedia", "s *Instance", 172),
    ("ServeVideoWithTranscode", "s *Instance", 293),
    ("BufferRemoteFileToTmp", "s *Instance", 399),
    ("GetHashFromRemoteFile", "s *Instance", 460),
];

pub async fn newmediaserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "NewMediaServer" })
}

pub async fn instance_setvirtualpathresolver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.SetVirtualPathResolver" })
}

pub async fn instance_validatesourcefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.ValidateSourceFile" })
}

pub async fn instance_servemediamime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.ServeMediaMime" })
}

pub async fn instance_servermedia(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.ServerMedia" })
}

pub async fn instance_servevideowithtranscode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.ServeVideoWithTranscode" })
}

pub async fn instance_bufferremotefiletotmp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.BufferRemoteFileToTmp" })
}

pub async fn instance_gethashfromremotefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/media/mediaserver/mediaserver.go", function: "Instance.GetHashFromRemoteFile" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
