//! Original Go file: `mod/filesystem/localversion/localversion.go`
//! Package: `localversion`; LOC: 225; SHA256: `a93166b7b22da2de1851cab96fcebc3fccb557f9b29f74bf90580028992a17cf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/localversion/localversion.go", package: "localversion", go_loc: 225, functions: 7, types: 2, sha256: "a93166b7b22da2de1851cab96fcebc3fccb557f9b29f74bf90580028992a17cf" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem",
    "os",
    "path/filepath",
    "sort",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("FileSnapshot", "struct", 22),
    ("VersionList", "struct", 31),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetFileVersionData", "", 37),
    ("RestoreFileHistory", "", 72),
    ("RemoveFileHistory", "", 141),
    ("RemoveAllRelatedFileHistory", "", 150),
    ("CreateFileSnapshot", "", 161),
    ("CleanExpiredVersionBackups", "", 184),
    ("inLocalVersionFolder", "", 222),
];

pub async fn getfileversiondata(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "GetFileVersionData" })
}

pub async fn restorefilehistory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "RestoreFileHistory" })
}

pub async fn removefilehistory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "RemoveFileHistory" })
}

pub async fn removeallrelatedfilehistory(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "RemoveAllRelatedFileHistory" })
}

pub async fn createfilesnapshot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "CreateFileSnapshot" })
}

pub async fn cleanexpiredversionbackups(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "CleanExpiredVersionBackups" })
}

pub async fn inlocalversionfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/localversion/localversion.go", function: "inLocalVersionFolder" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
