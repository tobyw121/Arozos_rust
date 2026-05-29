//! Original Go file: `mod/filesystem/abstractions/sftpfs/sftpFileUtil.go`
//! Package: `sftpfs`; LOC: 127; SHA256: `3baaf44bb06598840f7907038268fb592fd5573601fa4658b70695b3f1778816`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", package: "sftpfs", go_loc: 127, functions: 24, types: 2, sha256: "3baaf44bb06598840f7907038268fb592fd5573601fa4658b70695b3f1778816" };

pub const GO_IMPORTS: &[&str] = &[
    "github.com/pkg/sftp",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("sftpFsFile", "struct", 18),
    ("SftpDirEntry", "struct", 103),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("newSftpFsFile", "", 24),
    ("Chdir", "f *sftpFsFile", 32),
    ("Chmod", "f *sftpFsFile", 35),
    ("Chown", "f *sftpFsFile", 38),
    ("Close", "f *sftpFsFile", 41),
    ("Name", "f *sftpFsFile", 44),
    ("Read", "f *sftpFsFile", 47),
    ("ReadAt", "f *sftpFsFile", 50),
    ("Readdirnames", "f *sftpFsFile", 54),
    ("ReadDir", "f *sftpFsFile", 62),
    ("ReadFrom", "f *sftpFsFile", 69),
    ("Readdir", "f *sftpFsFile", 72),
    ("Seek", "f *sftpFsFile", 75),
    ("Stat", "f *sftpFsFile", 78),
    ("Sync", "f *sftpFsFile", 81),
    ("Truncate", "f *sftpFsFile", 84),
    ("Write", "f *sftpFsFile", 87),
    ("WriteAt", "f *sftpFsFile", 90),
    ("WriteString", "f *sftpFsFile", 93),
    ("newDirEntryFromFileInfo", "", 107),
    ("Name", "de SftpDirEntry", 113),
    ("IsDir", "de SftpDirEntry", 117),
    ("Type", "de SftpDirEntry", 121),
    ("Info", "de SftpDirEntry", 125),
];

pub async fn newsftpfsfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "newSftpFsFile" })
}

pub async fn sftpfsfile_chdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Chdir" })
}

pub async fn sftpfsfile_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Chmod" })
}

pub async fn sftpfsfile_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Chown" })
}

pub async fn sftpfsfile_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Close" })
}

pub async fn sftpfsfile_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Name" })
}

pub async fn sftpfsfile_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Read" })
}

pub async fn sftpfsfile_readat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.ReadAt" })
}

pub async fn sftpfsfile_readdirnames(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Readdirnames" })
}

pub async fn sftpfsfile_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.ReadDir" })
}

pub async fn sftpfsfile_readfrom(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.ReadFrom" })
}

pub async fn sftpfsfile_readdir_2(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Readdir" })
}

pub async fn sftpfsfile_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Seek" })
}

pub async fn sftpfsfile_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Stat" })
}

pub async fn sftpfsfile_sync(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Sync" })
}

pub async fn sftpfsfile_truncate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Truncate" })
}

pub async fn sftpfsfile_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.Write" })
}

pub async fn sftpfsfile_writeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.WriteAt" })
}

pub async fn sftpfsfile_writestring(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "sftpFsFile.WriteString" })
}

pub async fn newdirentryfromfileinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "newDirEntryFromFileInfo" })
}

pub async fn sftpdirentry_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "SftpDirEntry.Name" })
}

pub async fn sftpdirentry_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "SftpDirEntry.IsDir" })
}

pub async fn sftpdirentry_type(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "SftpDirEntry.Type" })
}

pub async fn sftpdirentry_info(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/abstractions/sftpfs/sftpFileUtil.go", function: "SftpDirEntry.Info" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
