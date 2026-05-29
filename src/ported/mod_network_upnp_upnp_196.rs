//! Original Go file: `mod/network/upnp/upnp.go`
//! Package: `upnp`; LOC: 133; SHA256: `e5b061ba37d5fd99f987cf5c9283c42d78c896d2fdb5097f0060731bed8187d3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/upnp/upnp.go", package: "upnp", go_loc: 133, functions: 5, types: 1, sha256: "e5b061ba37d5fd99f987cf5c9283c42d78c896d2fdb5097f0060731bed8187d3" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "gitlab.com/NebulousLabs/go-upnp",
    "log",
    "sync",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("UPnPClient", "struct", 19),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewUPNPClient", "", 26),
    ("ForwardPort", "u *UPnPClient", 56),
    ("ClosePort", "u *UPnPClient", 77),
    ("RenewForwardRules", "u *UPnPClient", 109),
    ("Close", "u *UPnPClient", 123),
];

pub async fn newupnpclient(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/upnp/upnp.go", function: "NewUPNPClient" })
}

pub async fn upnpclient_forwardport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/upnp/upnp.go", function: "UPnPClient.ForwardPort" })
}

pub async fn upnpclient_closeport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/upnp/upnp.go", function: "UPnPClient.ClosePort" })
}

pub async fn upnpclient_renewforwardrules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/upnp/upnp.go", function: "UPnPClient.RenewForwardRules" })
}

pub async fn upnpclient_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/upnp/upnp.go", function: "UPnPClient.Close" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
