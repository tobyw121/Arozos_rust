//! Original Go file: `mod/fileservers/servers/ftpserv/ftpserv.go`
//! Package: `ftpserv`; LOC: 260; SHA256: `fe166c151e755a3662c62bea43799c50319727d12e111c1652c908df9fc6ad66`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/ftpserv/ftpserv.go", package: "ftpserv", go_loc: 260, functions: 7, types: 3, sha256: "fe166c151e755a3662c62bea43799c50319727d12e111c1652c908df9fc6ad66" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/network/upnp",
    "imuslab.com/arozos/mod/storage/ftp",
    "imuslab.com/arozos/mod/user",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ServerStatus", "struct", 14),
    ("ManagerOption", "struct", 25),
    ("Manager", "struct", 36),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewFTPManager", "", 41),
    ("StartFtpServer", "m *Manager", 64),
    ("StopFtpServer", "m *Manager", 151),
    ("GetFtpServerStatus", "m *Manager", 160),
    ("IsFtpServerEnabled", "m *Manager", 230),
    ("FTPServerToggle", "m *Manager", 234),
    ("FTPGetEndpoints", "m *Manager", 248),
];

pub async fn newftpmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "NewFTPManager" })
}

pub async fn manager_startftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.StartFtpServer" })
}

pub async fn manager_stopftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.StopFtpServer" })
}

pub async fn manager_getftpserverstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.GetFtpServerStatus" })
}

pub async fn manager_isftpserverenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.IsFtpServerEnabled" })
}

pub async fn manager_ftpservertoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.FTPServerToggle" })
}

pub async fn manager_ftpgetendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/ftpserv/ftpserv.go", function: "Manager.FTPGetEndpoints" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
