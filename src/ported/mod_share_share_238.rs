//! Original Go file: `mod/share/share.go`
//! Package: `share`; LOC: 1420; SHA256: `84f9d1745168e1b2ab09b6c048f41f6330294019afe2fa0f50944e497ffe464e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/share/share.go", package: "share", go_loc: 1420, functions: 25, types: 2, sha256: "84f9d1745168e1b2ab09b6c048f41f6330294019afe2fa0f50944e497ffe464e" };

pub const GO_IMPORTS: &[&str] = &[
    "compress/flate",
    "encoding/json",
    "errors",
    "fmt",
    "github.com/golang/freetype",
    "github.com/nfnt/resize",
    "github.com/satori/go.uuid",
    "image",
    "image/color",
    "image/draw",
    "image/jpeg",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/metadata",
    "imuslab.com/arozos/mod/share/shareEntry",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "io",
    "io/fs",
    "log",
    "math",
    "mime",
    "net/http",
    "net/url",
    "os",
    "path/filepath",
    "sort",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Options", "struct", 46),
    ("Manager", "struct", 54),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewShareManager", "", 59),
    ("HandleOPGServing", "s *Manager", 66),
    ("HandleShareAccess", "s *Manager", 264),
    ("HandleShareCheck", "s *Manager", 897),
    ("HandleCreateNewShare", "s *Manager", 946),
    ("HandleEditShare", "s *Manager", 987),
    ("HandleDeleteShare", "s *Manager", 1056),
    ("HandleListAllShares", "s *Manager", 1092),
    ("UserCanOpenShareInFileManager", "s *Manager", 1172),
    ("CreateNewShare", "s *Manager", 1192),
    ("ServePermissionDeniedPage", "", 1198),
    ("validateShareModes", "", 1217),
    ("ListAllShareByFshId", "s *Manager", 1248),
    ("ShareIsValid", "s *Manager", 1269),
    ("GetPathHashFromShare", "s *Manager", 1287),
    ("ValidateAndClearShares", "s *Manager", 1298),
    ("CanModifyShareEntry", "s *Manager", 1322),
    ("DeleteShareByVpath", "s *Manager", 1349),
    ("DeleteShareByUUID", "s *Manager", 1360),
    ("GetShareUUIDFromUserAndVpath", "s *Manager", 1373),
    ("GetShareObjectFromUserAndVpath", "s *Manager", 1381),
    ("GetShareObjectFromUUID", "s *Manager", 1389),
    ("FileIsShared", "s *Manager", 1393),
    ("RemoveShareByUUID", "s *Manager", 1402),
    ("getPathHashFromUsernameAndVpath", "", 1413),
];

pub async fn newsharemanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "NewShareManager" })
}

pub async fn manager_handleopgserving(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleOPGServing" })
}

pub async fn manager_handleshareaccess(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleShareAccess" })
}

pub async fn manager_handlesharecheck(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleShareCheck" })
}

pub async fn manager_handlecreatenewshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleCreateNewShare" })
}

pub async fn manager_handleeditshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleEditShare" })
}

pub async fn manager_handledeleteshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleDeleteShare" })
}

pub async fn manager_handlelistallshares(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.HandleListAllShares" })
}

pub async fn manager_usercanopenshareinfilemanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.UserCanOpenShareInFileManager" })
}

pub async fn manager_createnewshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.CreateNewShare" })
}

pub async fn servepermissiondeniedpage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "ServePermissionDeniedPage" })
}

pub async fn validatesharemodes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "validateShareModes" })
}

pub async fn manager_listallsharebyfshid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.ListAllShareByFshId" })
}

pub async fn manager_shareisvalid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.ShareIsValid" })
}

pub async fn manager_getpathhashfromshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.GetPathHashFromShare" })
}

pub async fn manager_validateandclearshares(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.ValidateAndClearShares" })
}

pub async fn manager_canmodifyshareentry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.CanModifyShareEntry" })
}

pub async fn manager_deletesharebyvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.DeleteShareByVpath" })
}

pub async fn manager_deletesharebyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.DeleteShareByUUID" })
}

pub async fn manager_getshareuuidfromuserandvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.GetShareUUIDFromUserAndVpath" })
}

pub async fn manager_getshareobjectfromuserandvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.GetShareObjectFromUserAndVpath" })
}

pub async fn manager_getshareobjectfromuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.GetShareObjectFromUUID" })
}

pub async fn manager_fileisshared(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.FileIsShared" })
}

pub async fn manager_removesharebyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "Manager.RemoveShareByUUID" })
}

pub async fn getpathhashfromusernameandvpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/share.go", function: "getPathHashFromUsernameAndVpath" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
