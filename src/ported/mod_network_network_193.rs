//! Original Go file: `mod/network/network.go`
//! Package: `network`; LOC: 224; SHA256: `25e559dbae0b63ddecab295368d57c5a1bbbd81618f5c9c5014c94d1ca383102`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/network.go", package: "network", go_loc: 224, functions: 8, types: 1, sha256: "25e559dbae0b63ddecab295368d57c5a1bbbd81618f5c9c5014c94d1ca383102" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "gitlab.com/NebulousLabs/go-upnp",
    "imuslab.com/arozos/mod/utils",
    "io",
    "log",
    "net",
    "net/http",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("NICS", "struct", 16),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetNICInfo", "", 28),
    ("GetOutboundIP", "", 119),
    ("GetExternalIPAddr", "", 132),
    ("GetExternalIPAddrVia3rdPartyServices", "", 145),
    ("IsPublicIP", "", 161),
    ("IsIPv6Addr", "", 180),
    ("GetPing", "", 195),
    ("GetIpFromRequest", "", 199),
];

pub async fn getnicinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetNICInfo" })
}

pub async fn getoutboundip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetOutboundIP" })
}

pub async fn getexternalipaddr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetExternalIPAddr" })
}

pub async fn getexternalipaddrvia3rdpartyservices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetExternalIPAddrVia3rdPartyServices" })
}

pub async fn ispublicip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "IsPublicIP" })
}

pub async fn isipv6addr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "IsIPv6Addr" })
}

pub async fn getping(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetPing" })
}

pub async fn getipfromrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/network.go", function: "GetIpFromRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
