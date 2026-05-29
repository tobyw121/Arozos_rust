//! Original Go file: `mod/network/mdns/mdns.go`
//! Package: `mdns`; LOC: 229; SHA256: `ff79c9d92bf730ac17ae67b1bf552b53be6516c06f810315e050c89a85c12a2f`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/mdns/mdns.go", package: "mdns", go_loc: 229, functions: 3, types: 2, sha256: "ff79c9d92bf730ac17ae67b1bf552b53be6516c06f810315e050c89a85c12a2f" };

pub const GO_IMPORTS: &[&str] = &[
    "context",
    "github.com/grandcat/zeroconf",
    "log",
    "net",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("MDNSHost", "struct", 13),
    ("NetworkHost", "struct", 19),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewMDNS", "", 34),
    ("Close", "m *MDNSHost", 111),
    ("Scan", "m *MDNSHost", 119),
];

pub async fn newmdns(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/mdns/mdns.go", function: "NewMDNS" })
}

pub async fn mdnshost_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/mdns/mdns.go", function: "MDNSHost.Close" })
}

pub async fn mdnshost_scan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/mdns/mdns.go", function: "MDNSHost.Scan" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
