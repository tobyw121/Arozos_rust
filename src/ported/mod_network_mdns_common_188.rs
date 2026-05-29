//! Original Go file: `mod/network/mdns/common.go`
//! Package: `mdns`; LOC: 27; SHA256: `18ce462d992fa03dd04cef5cc8d2e560274d6382d5701cedba7ae6676dd7315b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/mdns/common.go", package: "mdns", go_loc: 27, functions: 2, types: 0, sha256: "18ce462d992fa03dd04cef5cc8d2e560274d6382d5701cedba7ae6676dd7315b" };

pub const GO_IMPORTS: &[&str] = &[
    "net",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("stringInSlice", "", 5),
    ("getMacAddr", "", 14),
];

pub async fn stringinslice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/mdns/common.go", function: "stringInSlice" })
}

pub async fn getmacaddr(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/mdns/common.go", function: "getMacAddr" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
