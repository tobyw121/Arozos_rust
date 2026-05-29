//! Original Go file: `mod/auth/accesscontrol/blacklist/blacklist.go`
//! Package: `blacklist`; LOC: 109; SHA256: `7e0c669869435065f6281460d0b38e9230eb03cf57e7d48f51521a2f27e01c86`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accesscontrol/blacklist/blacklist.go", package: "blacklist", go_loc: 109, functions: 5, types: 1, sha256: "7e0c669869435065f6281460d0b38e9230eb03cf57e7d48f51521a2f27e01c86" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/auth/accesscontrol",
    "imuslab.com/arozos/mod/database",
    "log",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("BlackList", "struct", 22),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewBlacklistManager", "", 27),
    ("IsBanned", "bl *BlackList", 45),
    ("ListBannedIpRanges", "bl *BlackList", 63),
    ("Ban", "bl *BlackList", 81),
    ("UnBan", "bl *BlackList", 95),
];

pub async fn newblacklistmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklist.go", function: "NewBlacklistManager" })
}

pub async fn blacklist_isbanned(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklist.go", function: "BlackList.IsBanned" })
}

pub async fn blacklist_listbannedipranges(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklist.go", function: "BlackList.ListBannedIpRanges" })
}

pub async fn blacklist_ban(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklist.go", function: "BlackList.Ban" })
}

pub async fn blacklist_unban(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accesscontrol/blacklist/blacklist.go", function: "BlackList.UnBan" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
