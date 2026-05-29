//! Original Go file: `mod/auth/accesscontrol/blacklist/blacklistmanager_test.go`
//! Package: `blacklist`; LOC: 168; SHA256: `b67aad0b26e4f1f00e355f16d18820605487e16f59213c0ec349c7cd81f21c7b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", package: "blacklist", go_loc: 168, functions: 5, types: 0, sha256: "b67aad0b26e4f1f00e355f16d18820605487e16f59213c0ec349c7cd81f21c7b" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/database",
    "os",
    "testing",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("setupSuite", "", 14),
    ("TestBlackList_IsBanned", "", 36),
    ("TestBlackList_ListBannedIpRanges", "", 75),
    ("TestBlackList_Ban_UnBan", "", 104),
    ("TestBlackList_InvalidIpRange", "", 143),
];

pub async fn setupsuite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", function: "setupSuite" })
}

pub async fn testblacklist_isbanned(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", function: "TestBlackList_IsBanned" })
}

pub async fn testblacklist_listbannedipranges(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", function: "TestBlackList_ListBannedIpRanges" })
}

pub async fn testblacklist_ban_unban(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", function: "TestBlackList_Ban_UnBan" })
}

pub async fn testblacklist_invalidiprange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklistmanager_test.go", function: "TestBlackList_InvalidIpRange" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
