//! Original Go file: `mod/fileservers/servers/samba/samba.go`
//! Package: `samba`; LOC: 417; SHA256: `386fa0b4974f54f4eecfadc1717185d5a9d30af3ea99268046b4bb6599aa3b81`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/samba/samba.go", package: "samba", go_loc: 417, functions: 12, types: 2, sha256: "386fa0b4974f54f4eecfadc1717185d5a9d30af3ea99268046b4bb6599aa3b81" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "os",
    "runtime",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ShareManager", "struct", 24),
    ("ShareConfig", "struct", 29),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSambaShareManager", "", 39),
    ("ReadSambaShares", "s *ShareManager", 55),
    ("ShareNameExists", "s *ShareManager", 120),
    ("FilterSystemCreatedShares", "s *ShareManager", 136),
    ("CreateNewSambaShare", "s *ShareManager", 148),
    ("RemoveSambaShareConfig", "s *ShareManager", 197),
    ("ShareExists", "s *ShareManager", 274),
    ("AddUserToSambaShare", "s *ShareManager", 302),
    ("UserCanAccessShare", "s *ShareManager", 331),
    ("GetUsersShare", "s *ShareManager", 336),
    ("RemoveUserFromSambaShare", "s *ShareManager", 354),
    ("GetShareByName", "s *ShareManager", 402),
];

pub async fn newsambasharemanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "NewSambaShareManager" })
}

pub async fn sharemanager_readsambashares(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.ReadSambaShares" })
}

pub async fn sharemanager_sharenameexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.ShareNameExists" })
}

pub async fn sharemanager_filtersystemcreatedshares(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.FilterSystemCreatedShares" })
}

pub async fn sharemanager_createnewsambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.CreateNewSambaShare" })
}

pub async fn sharemanager_removesambashareconfig(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.RemoveSambaShareConfig" })
}

pub async fn sharemanager_shareexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.ShareExists" })
}

pub async fn sharemanager_addusertosambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.AddUserToSambaShare" })
}

pub async fn sharemanager_usercanaccessshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.UserCanAccessShare" })
}

pub async fn sharemanager_getusersshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.GetUsersShare" })
}

pub async fn sharemanager_removeuserfromsambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.RemoveUserFromSambaShare" })
}

pub async fn sharemanager_getsharebyname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/samba.go", function: "ShareManager.GetShareByName" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
