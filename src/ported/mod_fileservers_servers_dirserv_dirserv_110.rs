//! Original Go file: `mod/fileservers/servers/dirserv/dirserv.go`
//! Package: `dirserv`; LOC: 178; SHA256: `82a849b91e155237e3815320f59c93e80b1671a6f7dcbbf2fe3b945d7af3c42a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/dirserv/dirserv.go", package: "dirserv", go_loc: 178, functions: 5, types: 2, sha256: "82a849b91e155237e3815320f59c93e80b1671a6f7dcbbf2fe3b945d7af3c42a" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/user",
    "io",
    "net/http",
    "net/url",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Option", "struct", 24),
    ("Manager", "struct", 31),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDirectoryServer", "", 37),
    ("DirServerEnabled", "m *Manager", 52),
    ("Toggle", "m *Manager", 56),
    ("ListEndpoints", "m *Manager", 62),
    ("ServerWebFileRequest", "m *Manager", 76),
];

pub async fn newdirectoryserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/dirserv.go", function: "NewDirectoryServer" })
}

pub async fn manager_dirserverenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/dirserv.go", function: "Manager.DirServerEnabled" })
}

pub async fn manager_toggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/dirserv.go", function: "Manager.Toggle" })
}

pub async fn manager_listendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/dirserv.go", function: "Manager.ListEndpoints" })
}

pub async fn manager_serverwebfilerequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/dirserv/dirserv.go", function: "Manager.ServerWebFileRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
