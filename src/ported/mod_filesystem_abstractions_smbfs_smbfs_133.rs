//! Original Go file: `mod/filesystem/abstractions/smbfs/smbfs.go`
//! Package: `smbfs`; LOC: 407; SHA256: `ce92a7fd98c30b398dbb0cc66dca647470a5e935dd63588ab9ac288b58814cd1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/smbfs/smbfs.go", package: "smbfs", go_loc: 407, functions: 33, types: 1, sha256: "ce92a7fd98c30b398dbb0cc66dca647470a5e935dd63588ab9ac288b58814cd1" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "github.com/hirochachacha/go-smb2",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "log",
    "net",
    "os",
    "path/filepath",
    "regexp",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ServerMessageBlockFileSystemAbstraction", "struct", 26),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewServerMessageBlockFileSystemAbstraction", "", 41),
    ("Chmod", "a ServerMessageBlockFileSystemAbstraction", 103),
    ("Chown", "a ServerMessageBlockFileSystemAbstraction", 108),
    ("Chtimes", "a ServerMessageBlockFileSystemAbstraction", 111),
    ("Create", "a ServerMessageBlockFileSystemAbstraction", 116),
    ("Mkdir", "a ServerMessageBlockFileSystemAbstraction", 125),
    ("MkdirAll", "a ServerMessageBlockFileSystemAbstraction", 130),
    ("Name", "a ServerMessageBlockFileSystemAbstraction", 135),
    ("Open", "a ServerMessageBlockFileSystemAbstraction", 138),
    ("OpenFile", "a ServerMessageBlockFileSystemAbstraction", 147),
    ("Remove", "a ServerMessageBlockFileSystemAbstraction", 156),
    ("RemoveAll", "a ServerMessageBlockFileSystemAbstraction", 161),
    ("Rename", "a ServerMessageBlockFileSystemAbstraction", 166),
    ("Stat", "a ServerMessageBlockFileSystemAbstraction", 171),
    ("Close", "a ServerMessageBlockFileSystemAbstraction", 175),
    ("VirtualPathToRealPath", "a ServerMessageBlockFileSystemAbstraction", 197),
    ("RealPathToVirtualPath", "a ServerMessageBlockFileSystemAbstraction", 213),
    ("FileExists", "a ServerMessageBlockFileSystemAbstraction", 225),
    ("IsDir", "a ServerMessageBlockFileSystemAbstraction", 235),
    ("Glob", "a ServerMessageBlockFileSystemAbstraction", 245),
    ("GetFileSize", "a ServerMessageBlockFileSystemAbstraction", 255),
    ("GetModTime", "a ServerMessageBlockFileSystemAbstraction", 264),
    ("WriteFile", "a ServerMessageBlockFileSystemAbstraction", 273),
    ("ReadFile", "a ServerMessageBlockFileSystemAbstraction", 277),
    ("ReadDir", "a ServerMessageBlockFileSystemAbstraction", 282),
    ("WriteStream", "a ServerMessageBlockFileSystemAbstraction", 299),
    ("ReadStream", "a ServerMessageBlockFileSystemAbstraction", 324),
    ("Walk", "a ServerMessageBlockFileSystemAbstraction", 334),
    ("Heartbeat", "a ServerMessageBlockFileSystemAbstraction", 350),
    ("CapacityInfo", "a *ServerMessageBlockFileSystemAbstraction", 361),
    ("toWinPath", "", 376),
    ("filterFilepath", "a ServerMessageBlockFileSystemAbstraction", 381),
    ("wildCardToRegexp", "", 394),
];

pub async fn newservermessageblockfilesystemabstraction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "NewServerMessageBlockFileSystemAbstraction" })
}

pub async fn servermessageblockfilesystemabstraction_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Chmod" })
}

pub async fn servermessageblockfilesystemabstraction_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Chown" })
}

pub async fn servermessageblockfilesystemabstraction_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Chtimes" })
}

pub async fn servermessageblockfilesystemabstraction_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Create" })
}

pub async fn servermessageblockfilesystemabstraction_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Mkdir" })
}

pub async fn servermessageblockfilesystemabstraction_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.MkdirAll" })
}

pub async fn servermessageblockfilesystemabstraction_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Name" })
}

pub async fn servermessageblockfilesystemabstraction_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Open" })
}

pub async fn servermessageblockfilesystemabstraction_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.OpenFile" })
}

pub async fn servermessageblockfilesystemabstraction_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Remove" })
}

pub async fn servermessageblockfilesystemabstraction_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.RemoveAll" })
}

pub async fn servermessageblockfilesystemabstraction_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Rename" })
}

pub async fn servermessageblockfilesystemabstraction_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Stat" })
}

pub async fn servermessageblockfilesystemabstraction_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Close" })
}

pub async fn servermessageblockfilesystemabstraction_virtualpathtorealpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.VirtualPathToRealPath" })
}

pub async fn servermessageblockfilesystemabstraction_realpathtovirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.RealPathToVirtualPath" })
}

pub async fn servermessageblockfilesystemabstraction_fileexists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.FileExists" })
}

pub async fn servermessageblockfilesystemabstraction_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.IsDir" })
}

pub async fn servermessageblockfilesystemabstraction_glob(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Glob" })
}

pub async fn servermessageblockfilesystemabstraction_getfilesize(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.GetFileSize" })
}

pub async fn servermessageblockfilesystemabstraction_getmodtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.GetModTime" })
}

pub async fn servermessageblockfilesystemabstraction_writefile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.WriteFile" })
}

pub async fn servermessageblockfilesystemabstraction_readfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.ReadFile" })
}

pub async fn servermessageblockfilesystemabstraction_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.ReadDir" })
}

pub async fn servermessageblockfilesystemabstraction_writestream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.WriteStream" })
}

pub async fn servermessageblockfilesystemabstraction_readstream(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.ReadStream" })
}

pub async fn servermessageblockfilesystemabstraction_walk(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Walk" })
}

pub async fn servermessageblockfilesystemabstraction_heartbeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.Heartbeat" })
}

pub async fn servermessageblockfilesystemabstraction_capacityinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.CapacityInfo" })
}

pub async fn towinpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "toWinPath" })
}

pub async fn servermessageblockfilesystemabstraction_filterfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "ServerMessageBlockFileSystemAbstraction.filterFilepath" })
}

pub async fn wildcardtoregexp(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbfs.go", function: "wildCardToRegexp" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
