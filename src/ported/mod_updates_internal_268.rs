//! Original Go file: `mod/updates/internal.go`
//! Package: `updates`; LOC: 131; SHA256: `5d41391671d986def20ac3c743a50ee730f4bd88568423fa78bc88de33138664`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/updates/internal.go", package: "updates", go_loc: 131, functions: 6, types: 0, sha256: "5d41391671d986def20ac3c743a50ee730f4bd88568423fa78bc88de33138664" };

pub const GO_IMPORTS: &[&str] = &[
    "archive/tar",
    "compress/gzip",
    "crypto/sha1",
    "encoding/hex",
    "errors",
    "io",
    "net/http",
    "os",
    "path/filepath",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getFileSize", "", 17),
    ("getDownloadFileSize", "", 26),
    ("downloadFile", "", 36),
    ("extractTarGz", "", 66),
    ("getSHA1Hash", "", 108),
    ("readCheckSumFile", "", 122),
];

pub async fn getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "getFileSize" })
}

pub async fn getdownloadfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "getDownloadFileSize" })
}

pub async fn downloadfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "downloadFile" })
}

pub async fn extracttargz(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "extractTarGz" })
}

pub async fn getsha1hash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "getSHA1Hash" })
}

pub async fn readchecksumfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/updates/internal.go", function: "readCheckSumFile" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
