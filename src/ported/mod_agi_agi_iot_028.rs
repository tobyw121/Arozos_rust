//! Original Go file: `mod/agi/agi.iot.go`
//! Package: `agi`; LOC: 294; SHA256: `e13bc5da5e93baad96d4de6330c30b7df47528057997039576e8ed75eb76c41d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.iot.go", package: "agi", go_loc: 294, functions: 2, types: 0, sha256: "e13bc5da5e93baad96d4de6330c30b7df47528057997039576e8ed75eb76c41d" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "github.com/robertkrimen/otto",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/iot",
    "log",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("IoTLibRegister", "g *Gateway", 21),
    ("injectIoTFunctions", "g *Gateway", 28),
];

pub async fn gateway_iotlibregister(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.iot.go", function: "Gateway.IoTLibRegister" })
}

pub async fn gateway_injectiotfunctions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.iot.go", function: "Gateway.injectIoTFunctions" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
