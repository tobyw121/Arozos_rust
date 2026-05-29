//! Original Go file: `mod/auth/accesscontrol/utils.go`
//! Package: `accesscontrol`; LOC: 133; SHA256: `e40f9355409b40d464f6206e656babc34151b02db4f1001a7eb6594c370a46e9`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/utils.go", package: "accesscontrol", go_loc: 133, functions: 3, types: 0, sha256: "e40f9355409b40d464f6206e656babc34151b02db4f1001a7eb6594c370a46e9" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "errors",
    "net",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("BreakdownIpRange", "", 12),
    ("IpInRange", "", 50),
    ("ValidateIpRange", "", 86),
];

pub async fn breakdowniprange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils.go", function: "BreakdownIpRange" })
}

pub async fn ipinrange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils.go", function: "IpInRange" })
}

pub async fn validateiprange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils.go", function: "ValidateIpRange" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
