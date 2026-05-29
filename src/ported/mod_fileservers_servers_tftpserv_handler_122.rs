//! Original Go file: `mod/fileservers/servers/tftpserv/handler.go`
//! Package: `tftpserv`; LOC: 95; SHA256: `ebccb2e2a8a3a1895ace50c09b4bebeb4e63be4989ae4b2bc2e6b81d43dcab50`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/tftpserv/handler.go", package: "tftpserv", go_loc: 95, functions: 5, types: 0, sha256: "ebccb2e2a8a3a1895ace50c09b4bebeb4e63be4989ae4b2bc2e6b81d43dcab50" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleTFTPServerStart", "m *Manager", 12),
    ("HandleTFTPServerStop", "m *Manager", 22),
    ("HandleTFTPServerStatus", "m *Manager", 28),
    ("HandleTFTPPort", "m *Manager", 40),
    ("HandleTFTPDefaultUser", "m *Manager", 71),
];

pub async fn manager_handletftpserverstart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/handler.go", function: "Manager.HandleTFTPServerStart" })
}

pub async fn manager_handletftpserverstop(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/handler.go", function: "Manager.HandleTFTPServerStop" })
}

pub async fn manager_handletftpserverstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/handler.go", function: "Manager.HandleTFTPServerStatus" })
}

pub async fn manager_handletftpport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/handler.go", function: "Manager.HandleTFTPPort" })
}

pub async fn manager_handletftpdefaultuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/handler.go", function: "Manager.HandleTFTPDefaultUser" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
