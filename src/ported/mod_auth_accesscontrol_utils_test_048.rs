//! Original Go file: `mod/auth/accesscontrol/utils_test.go`
//! Package: `accesscontrol`; LOC: 96; SHA256: `166693d122317890d426158f70ee1c18f3e4e8a8d7212c2307047547d92798a3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/utils_test.go", package: "accesscontrol", go_loc: 96, functions: 4, types: 0, sha256: "166693d122317890d426158f70ee1c18f3e4e8a8d7212c2307047547d92798a3" };

pub const GO_IMPORTS: &[&str] = &[
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("TestBreakdownIpRange", "", 7),
    ("TestIpInRange", "", 28),
    ("TestValidateIpRange", "", 54),
    ("isEqual", "", 86),
];

pub async fn testbreakdowniprange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils_test.go", function: "TestBreakdownIpRange" })
}

pub async fn testipinrange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils_test.go", function: "TestIpInRange" })
}

pub async fn testvalidateiprange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils_test.go", function: "TestValidateIpRange" })
}

pub async fn isequal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/utils_test.go", function: "isEqual" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
