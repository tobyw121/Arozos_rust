//! Original Go file: `mod/fileservers/servers/samba/handlers.go`
//! Package: `samba`; LOC: 630; SHA256: `629754e51c75b5a5d8a4ca9131404409471f0ee336c8f6fc8723461dabe99a0c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/samba/handlers.go", package: "samba", go_loc: 630, functions: 13, types: 0, sha256: "629754e51c75b5a5d8a4ca9131404409471f0ee336c8f6fc8723461dabe99a0c" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SmbdStates", "s *ShareManager", 14),
    ("ListSambaShares", "s *ShareManager", 53),
    ("AddSambaShare", "s *ShareManager", 68),
    ("DelUserSambaShare", "s *ShareManager", 190),
    ("DelSambaShare", "s *ShareManager", 242),
    ("NewSambaUser", "s *ShareManager", 279),
    ("DelSambaUser", "s *ShareManager", 302),
    ("ListSambaUsers", "s *ShareManager", 320),
    ("ActivateUserAccount", "s *ShareManager", 347),
    ("HandleUserSmbStatusList", "s *ShareManager", 441),
    ("DeactiveUserAccount", "s *ShareManager", 479),
    ("HandleAccessUserUpdate", "s *ShareManager", 518),
    ("HandleSharePathChange", "s *ShareManager", 575),
];

pub async fn sharemanager_smbdstates(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.SmbdStates" })
}

pub async fn sharemanager_listsambashares(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.ListSambaShares" })
}

pub async fn sharemanager_addsambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.AddSambaShare" })
}

pub async fn sharemanager_delusersambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.DelUserSambaShare" })
}

pub async fn sharemanager_delsambashare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.DelSambaShare" })
}

pub async fn sharemanager_newsambauser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.NewSambaUser" })
}

pub async fn sharemanager_delsambauser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.DelSambaUser" })
}

pub async fn sharemanager_listsambausers(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.ListSambaUsers" })
}

pub async fn sharemanager_activateuseraccount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.ActivateUserAccount" })
}

pub async fn sharemanager_handleusersmbstatuslist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.HandleUserSmbStatusList" })
}

pub async fn sharemanager_deactiveuseraccount(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.DeactiveUserAccount" })
}

pub async fn sharemanager_handleaccessuserupdate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.HandleAccessUserUpdate" })
}

pub async fn sharemanager_handlesharepathchange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/samba/handlers.go", function: "ShareManager.HandleSharePathChange" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
