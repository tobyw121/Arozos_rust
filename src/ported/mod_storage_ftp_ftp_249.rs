//! Original Go file: `mod/storage/ftp/ftp.go`
//! Package: `ftp`; LOC: 90; SHA256: `3c93cdc7a044e4ecb1d96b695455b5df1699b3c8056f45bcd432943102940883`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/ftp/ftp.go", package: "ftp", go_loc: 90, functions: 4, types: 2, sha256: "3c93cdc7a044e4ecb1d96b695455b5df1699b3c8056f45bcd432943102940883" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/fclairamb/ftpserverlib",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/user",
    "log",
    "strconv",
    "strings",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Handler", "struct", 16),
    ("mainDriver", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewFTPHandler", "", 33),
    ("UpdateAccessableGroups", "", 63),
    ("Start", "f *Handler", 71),
    ("Close", "f *Handler", 85),
];

pub async fn newftphandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/ftp.go", function: "NewFTPHandler" })
}

pub async fn updateaccessablegroups(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/ftp.go", function: "UpdateAccessableGroups" })
}

pub async fn handler_start(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/ftp.go", function: "Handler.Start" })
}

pub async fn handler_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/ftp.go", function: "Handler.Close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
