//! Original Go file: `mod/network/neighbour/handler.go`
//! Package: `neighbour`; LOC: 76; SHA256: `ab175b5990a69a2161d745747177ddd4d8b6e8bc77e517142733bc60e7933590`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/neighbour/handler.go", package: "neighbour", go_loc: 76, functions: 3, types: 1, sha256: "ab175b5990a69a2161d745747177ddd4d8b6e8bc77e517142733bc60e7933590" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/network/mdns",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ScanResults", "struct", 17),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("HandleScanningRequest", "d *Discoverer", 24),
    ("HandleScanRecord", "d *Discoverer", 45),
    ("HandleWakeOnLan", "d *Discoverer", 62),
];

pub async fn discoverer_handlescanningrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/handler.go", function: "Discoverer.HandleScanningRequest" })
}

pub async fn discoverer_handlescanrecord(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/handler.go", function: "Discoverer.HandleScanRecord" })
}

pub async fn discoverer_handlewakeonlan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/handler.go", function: "Discoverer.HandleWakeOnLan" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
