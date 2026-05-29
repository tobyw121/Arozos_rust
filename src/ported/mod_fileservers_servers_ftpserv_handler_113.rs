//! Original Go file: `mod/fileservers/servers/ftpserv/handler.go`
//! Package: `ftpserv`; LOC: 154; SHA256: `97c135735e302bfa8025377d0bb3d6131704dba29ad8e870827509a116211058`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/ftpserv/handler.go", package: "ftpserv", go_loc: 154, functions: 7, types: 0, sha256: "97c135735e302bfa8025377d0bb3d6131704dba29ad8e870827509a116211058" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/storage/ftp",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleFTPServerStart", "m *Manager", 14),
    ("HandleFTPServerStop", "m *Manager", 23),
    ("HandleFTPServerStatus", "m *Manager", 29),
    ("HandleFTPUPnP", "m *Manager", 41),
    ("HandleFTPAccessUpdate", "m *Manager", 59),
    ("HandleFTPSetPort", "m *Manager", 83),
    ("HandleFTPPassiveModeSettings", "m *Manager", 113),
];

pub async fn manager_handleftpserverstart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPServerStart" })
}

pub async fn manager_handleftpserverstop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPServerStop" })
}

pub async fn manager_handleftpserverstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPServerStatus" })
}

pub async fn manager_handleftpupnp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPUPnP" })
}

pub async fn manager_handleftpaccessupdate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPAccessUpdate" })
}

pub async fn manager_handleftpsetport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPSetPort" })
}

pub async fn manager_handleftppassivemodesettings(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/handler.go", function: "Manager.HandleFTPPassiveModeSettings" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
