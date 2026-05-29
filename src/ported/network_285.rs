//! Original Go file: `network.go`
//! Package: `main`; LOC: 613; SHA256: `6a71fe621b5de76c3aeb10ac43b847c1b8357df089c4b0daac65ecaeec7a9d90`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "network.go", package: "main", go_loc: 613, functions: 8, types: 0, sha256: "6a71fe621b5de76c3aeb10ac43b847c1b8357df089c4b0daac65ecaeec7a9d90" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/fileservers/servers/dirserv",
    "imuslab.com/arozos/mod/fileservers/servers/ftpserv",
    "imuslab.com/arozos/mod/fileservers/servers/samba",
    "imuslab.com/arozos/mod/fileservers/servers/sftpserv",
    "imuslab.com/arozos/mod/fileservers/servers/tftpserv",
    "imuslab.com/arozos/mod/fileservers/servers/webdavserv",
    "imuslab.com/arozos/mod/network",
    "imuslab.com/arozos/mod/network/mdns",
    "imuslab.com/arozos/mod/network/netstat",
    "imuslab.com/arozos/mod/network/ssdp",
    "imuslab.com/arozos/mod/network/upnp",
    "imuslab.com/arozos/mod/network/websocket",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "imuslab.com/arozos/mod/www",
    "log",
    "net/http",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NetworkServiceInit", "", 44),
    ("StartNetworkServices", "", 120),
    ("StopNetworkServices", "", 223),
    ("FileServerInit", "", 253),
    ("NetworkHandleFileServerToggle", "", 504),
    ("NetworkHandleGetFileServerServiceList", "", 544),
    ("NetworkHandleGetFileServerStatus", "", 550),
    ("NetworkHandleGetFileServerEndpoints", "", 575),
];

pub async fn networkserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "NetworkServiceInit" })
}

pub async fn startnetworkservices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "StartNetworkServices" })
}

pub async fn stopnetworkservices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "StopNetworkServices" })
}

pub async fn fileserverinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "FileServerInit" })
}

pub async fn networkhandlefileservertoggle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "NetworkHandleFileServerToggle" })
}

pub async fn networkhandlegetfileserverservicelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "NetworkHandleGetFileServerServiceList" })
}

pub async fn networkhandlegetfileserverstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "NetworkHandleGetFileServerStatus" })
}

pub async fn networkhandlegetfileserverendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.go", function: "NetworkHandleGetFileServerEndpoints" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
