//! Original Go file: `mod/filesystem/abstractions/smbfs/smbFileWrapper.go`
//! Package: `smbfs`; LOC: 109; SHA256: `a5ad680bca59a9eef991341fe0abaf6c277c460f5e72148abde079e9f1e2bcc1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", package: "smbfs", go_loc: 109, functions: 24, types: 2, sha256: "a5ad680bca59a9eef991341fe0abaf6c277c460f5e72148abde079e9f1e2bcc1" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/hirochachacha/go-smb2",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("smbfsFile", "struct", 12),
    ("smbDirEntry", "struct", 85),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSmbFsFile", "", 16),
    ("Chdir", "f *smbfsFile", 22),
    ("Chmod", "f *smbfsFile", 25),
    ("Chown", "f *smbfsFile", 28),
    ("Close", "f *smbfsFile", 31),
    ("Name", "f *smbfsFile", 34),
    ("Read", "f *smbfsFile", 37),
    ("ReadAt", "f *smbfsFile", 40),
    ("ReadDir", "f *smbfsFile", 43),
    ("Readdirnames", "f *smbfsFile", 46),
    ("ReadFrom", "f *smbfsFile", 57),
    ("Readdir", "f *smbfsFile", 60),
    ("Seek", "f *smbfsFile", 63),
    ("Stat", "f *smbfsFile", 66),
    ("Sync", "f *smbfsFile", 69),
    ("Truncate", "f *smbfsFile", 72),
    ("Write", "f *smbfsFile", 75),
    ("WriteAt", "f *smbfsFile", 78),
    ("WriteString", "f *smbfsFile", 81),
    ("newDirEntryFromFileInfo", "", 89),
    ("Name", "de smbDirEntry", 95),
    ("IsDir", "de smbDirEntry", 99),
    ("Type", "de smbDirEntry", 103),
    ("Info", "de smbDirEntry", 107),
];

pub async fn newsmbfsfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "NewSmbFsFile" })
}

pub async fn smbfsfile_chdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Chdir" })
}

pub async fn smbfsfile_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Chmod" })
}

pub async fn smbfsfile_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Chown" })
}

pub async fn smbfsfile_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Close" })
}

pub async fn smbfsfile_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Name" })
}

pub async fn smbfsfile_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Read" })
}

pub async fn smbfsfile_readat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.ReadAt" })
}

pub async fn smbfsfile_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.ReadDir" })
}

pub async fn smbfsfile_readdirnames(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Readdirnames" })
}

pub async fn smbfsfile_readfrom(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.ReadFrom" })
}

pub async fn smbfsfile_readdir_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Readdir" })
}

pub async fn smbfsfile_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Seek" })
}

pub async fn smbfsfile_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Stat" })
}

pub async fn smbfsfile_sync(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Sync" })
}

pub async fn smbfsfile_truncate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Truncate" })
}

pub async fn smbfsfile_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.Write" })
}

pub async fn smbfsfile_writeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.WriteAt" })
}

pub async fn smbfsfile_writestring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbfsFile.WriteString" })
}

pub async fn newdirentryfromfileinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "newDirEntryFromFileInfo" })
}

pub async fn smbdirentry_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbDirEntry.Name" })
}

pub async fn smbdirentry_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbDirEntry.IsDir" })
}

pub async fn smbdirentry_type(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbDirEntry.Type" })
}

pub async fn smbdirentry_info(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/smbfs/smbFileWrapper.go", function: "smbDirEntry.Info" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
