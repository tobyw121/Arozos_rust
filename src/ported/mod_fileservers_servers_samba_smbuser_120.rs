//! Original Go file: `mod/fileservers/servers/samba/smbuser.go`
//! Package: `samba`; LOC: 156; SHA256: `419d0f17d080d4614fa675a803561ed9852f81ca84bdc78ae74266c307c6124e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/samba/smbuser.go", package: "samba", go_loc: 156, functions: 6, types: 1, sha256: "419d0f17d080d4614fa675a803561ed9852f81ca84bdc78ae74266c307c6124e" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "fmt",
    "os/exec",
    "regexp",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("UserInfo", "struct", 12),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("AddSambaUser", "s *ShareManager", 21),
    ("unixUserExists", "", 42),
    ("setupSmbUser", "s *ShareManager", 50),
    ("RemoveSmbUser", "s *ShareManager", 85),
    ("RemoveUnixUser", "s *ShareManager", 99),
    ("ListSambaUsersInfo", "s *ShareManager", 113),
];

pub async fn sharemanager_addsambauser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "ShareManager.AddSambaUser" })
}

pub async fn unixuserexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "unixUserExists" })
}

pub async fn sharemanager_setupsmbuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "ShareManager.setupSmbUser" })
}

pub async fn sharemanager_removesmbuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "ShareManager.RemoveSmbUser" })
}

pub async fn sharemanager_removeunixuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "ShareManager.RemoveUnixUser" })
}

pub async fn sharemanager_listsambausersinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/smbuser.go", function: "ShareManager.ListSambaUsersInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
