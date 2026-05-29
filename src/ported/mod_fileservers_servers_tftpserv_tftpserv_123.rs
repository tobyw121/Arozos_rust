//! Original Go file: `mod/fileservers/servers/tftpserv/tftpserv.go`
//! Package: `tftpserv`; LOC: 150; SHA256: `9b12815bbbf4a958818d4498fba41c3fd74e11be2e3678e283918a80192111a0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/tftpserv/tftpserv.go", package: "tftpserv", go_loc: 150, functions: 7, types: 3, sha256: "9b12815bbbf4a958818d4498fba41c3fd74e11be2e3678e283918a80192111a0" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/storage/tftp",
    "imuslab.com/arozos/mod/user",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ServerStatus", "struct", 11),
    ("ManagerOption", "struct", 18),
    ("Manager", "struct", 27),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewTFTPManager", "", 32),
    ("StartTftpServer", "m *Manager", 55),
    ("StopTftpServer", "m *Manager", 81),
    ("GetTftpServerStatus", "m *Manager", 90),
    ("IsTftpServerEnabled", "m *Manager", 120),
    ("TFTPServerToggle", "m *Manager", 124),
    ("TFTPGetEndpoints", "m *Manager", 138),
];

pub async fn newtftpmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "NewTFTPManager" })
}

pub async fn manager_starttftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.StartTftpServer" })
}

pub async fn manager_stoptftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.StopTftpServer" })
}

pub async fn manager_gettftpserverstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.GetTftpServerStatus" })
}

pub async fn manager_istftpserverenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.IsTftpServerEnabled" })
}

pub async fn manager_tftpservertoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.TFTPServerToggle" })
}

pub async fn manager_tftpgetendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/tftpserv/tftpserv.go", function: "Manager.TFTPGetEndpoints" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
