//! Original Go file: `mod/share/shareEntry/shareEntry.go`
//! Package: `shareEntry`; LOC: 258; SHA256: `503cc7147d306b05f160e9d72e6c65314e36ef73336bbd5a79f08bc80fa6603a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/share/shareEntry/shareEntry.go", package: "shareEntry", go_loc: 258, functions: 12, types: 2, sha256: "503cc7147d306b05f160e9d72e6c65314e36ef73336bbd5a79f08bc80fa6603a" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/filesystem",
    "path/filepath",
    "strings",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ShareEntryTable", "struct", 23),
    ("ShareOption", "struct", 29),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewShareEntryTable", "", 40),
    ("CreateNewShare", "s *ShareEntryTable", 67),
    ("DeleteShareByPathHash", "s *ShareEntryTable", 120),
    ("DeleteShareByUUID", "s *ShareEntryTable", 146),
    ("GetShareUUIDFromPathHash", "s *ShareEntryTable", 170),
    ("GetShareObjectFromPathHash", "s *ShareEntryTable", 179),
    ("GetShareObjectFromUUID", "s *ShareEntryTable", 195),
    ("FileIsShared", "s *ShareEntryTable", 211),
    ("RemoveShareByPathHash", "s *ShareEntryTable", 216),
    ("RemoveShareByUUID", "s *ShareEntryTable", 229),
    ("ResolveShareOptionFromShareSubpath", "s *ShareEntryTable", 242),
    ("GetPathHash", "", 256),
];

pub async fn newshareentrytable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "NewShareEntryTable" })
}

pub async fn shareentrytable_createnewshare(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.CreateNewShare" })
}

pub async fn shareentrytable_deletesharebypathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.DeleteShareByPathHash" })
}

pub async fn shareentrytable_deletesharebyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.DeleteShareByUUID" })
}

pub async fn shareentrytable_getshareuuidfrompathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.GetShareUUIDFromPathHash" })
}

pub async fn shareentrytable_getshareobjectfrompathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.GetShareObjectFromPathHash" })
}

pub async fn shareentrytable_getshareobjectfromuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.GetShareObjectFromUUID" })
}

pub async fn shareentrytable_fileisshared(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.FileIsShared" })
}

pub async fn shareentrytable_removesharebypathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.RemoveShareByPathHash" })
}

pub async fn shareentrytable_removesharebyuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.RemoveShareByUUID" })
}

pub async fn shareentrytable_resolveshareoptionfromsharesubpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "ShareEntryTable.ResolveShareOptionFromShareSubpath" })
}

pub async fn getpathhash(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/share/shareEntry/shareEntry.go", function: "GetPathHash" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
