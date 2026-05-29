//! Original Go file: `mod/network/ssdp/ssdp.go`
//! Package: `ssdp`; LOC: 168; SHA256: `2b5511325efbd0e15093520eb0052201a4396ae72a6c1b823a06080530e47724`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/ssdp/ssdp.go", package: "ssdp", go_loc: 168, functions: 6, types: 2, sha256: "2b5511325efbd0e15093520eb0052201a4396ae72a6c1b823a06080530e47724" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/koron/go-ssdp",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net",
    "net/http",
    "os/exec",
    "runtime",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SSDPOption", "struct", 18),
    ("SSDPHost", "struct", 29),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSSDPHost", "", 37),
    ("Start", "a *SSDPHost", 73),
    ("Close", "a *SSDPHost", 100),
    ("handleSSDP", "a *SSDPHost", 110),
    ("getFirstNetworkInterfaceName", "", 131),
    ("pkg_exists", "", 159),
];

pub async fn newssdphost(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "NewSSDPHost" })
}

pub async fn ssdphost_start(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "SSDPHost.Start" })
}

pub async fn ssdphost_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "SSDPHost.Close" })
}

pub async fn ssdphost_handlessdp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "SSDPHost.handleSSDP" })
}

pub async fn getfirstnetworkinterfacename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "getFirstNetworkInterfaceName" })
}

pub async fn pkg_exists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/ssdp/ssdp.go", function: "pkg_exists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
