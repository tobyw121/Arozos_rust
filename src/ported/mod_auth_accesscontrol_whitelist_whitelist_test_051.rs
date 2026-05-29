//! Original Go file: `mod/auth/accesscontrol/whitelist/whitelist_test.go`
//! Package: `whitelist`; LOC: 169; SHA256: `4d4258fe0850ef71068fac170ad20c6d22705d6b1db8c957b21db0149997290a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/whitelist/whitelist_test.go", package: "whitelist", go_loc: 169, functions: 5, types: 0, sha256: "4d4258fe0850ef71068fac170ad20c6d22705d6b1db8c957b21db0149997290a" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/database",
    "os",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("setupSuite", "", 14),
    ("TestWhiteList_SetWhitelistEnabled", "", 39),
    ("TestWhiteList_IsWhitelisted", "", 65),
    ("TestWhiteList_ListWhitelistedIpRanges", "", 96),
    ("TestWhiteList_SetWhitelist_UnsetWhitelist", "", 126),
];

pub async fn setupsuite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist_test.go", function: "setupSuite" })
}

pub async fn testwhitelist_setwhitelistenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist_test.go", function: "TestWhiteList_SetWhitelistEnabled" })
}

pub async fn testwhitelist_iswhitelisted(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist_test.go", function: "TestWhiteList_IsWhitelisted" })
}

pub async fn testwhitelist_listwhitelistedipranges(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist_test.go", function: "TestWhiteList_ListWhitelistedIpRanges" })
}

pub async fn testwhitelist_setwhitelist_unsetwhitelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/whitelist/whitelist_test.go", function: "TestWhiteList_SetWhitelist_UnsetWhitelist" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
