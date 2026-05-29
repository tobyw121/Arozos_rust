//! Original Go file: `mod/fileservers/servers/sftpserv/sftpserv.go`
//! Package: `sftpserv`; LOC: 223; SHA256: `08bc560ba50cbf1e68a99fb8c57773f2b24df1dd96200a26c58857adcc277284`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/sftpserv/sftpserv.go", package: "sftpserv", go_loc: 223, functions: 10, types: 2, sha256: "08bc560ba50cbf1e68a99fb8c57773f2b24df1dd96200a26c58857adcc277284" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/network/upnp",
    "imuslab.com/arozos/mod/storage/sftpserver",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ManagerOption", "struct", 18),
    ("Manager", "struct", 27),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSFTPServer", "", 33),
    ("newSFTPServerInstance", "", 45),
    ("closeInstance", "m *Manager", 74),
    ("HandleListeningPort", "m *Manager", 92),
    ("getUpnPEnabled", "", 126),
    ("HandleGetConnectedClients", "m *Manager", 135),
    ("HandleToogleUPnP", "m *Manager", 148),
    ("ServerToggle", "m *Manager", 192),
    ("IsEnabled", "m *Manager", 211),
    ("GetEndpoints", "m *Manager", 215),
];

pub async fn newsftpserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "NewSFTPServer" })
}

pub async fn newsftpserverinstance(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "newSFTPServerInstance" })
}

pub async fn manager_closeinstance(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.closeInstance" })
}

pub async fn manager_handlelisteningport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.HandleListeningPort" })
}

pub async fn getupnpenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "getUpnPEnabled" })
}

pub async fn manager_handlegetconnectedclients(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.HandleGetConnectedClients" })
}

pub async fn manager_handletoogleupnp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.HandleToogleUPnP" })
}

pub async fn manager_servertoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.ServerToggle" })
}

pub async fn manager_isenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.IsEnabled" })
}

pub async fn manager_getendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/sftpserv/sftpserv.go", function: "Manager.GetEndpoints" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
