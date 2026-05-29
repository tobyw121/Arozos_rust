//! Original Go file: `mod/storage/sftpserver/vroot.go`
//! Package: `sftpserver`; LOC: 530; SHA256: `c79aa7293833e06c3ee546cca1e31d35337cd5f584b99ff13b250af71a99195e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/sftpserver/vroot.go", package: "sftpserver", go_loc: 530, functions: 49, types: 6, sha256: "c79aa7293833e06c3ee546cca1e31d35337cd5f584b99ff13b250af71a99195e" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/pkg/sftp",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io",
    "io/fs",
    "os",
    "path",
    "path/filepath",
    "sort",
    "strings",
    "syscall",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("root", "struct", 21),
    ("rootFolder", "struct", 28),
    ("rootEntry", "struct", 36),
    ("sftpFileInterface", "interface", 65),
    ("wrappedArozFile", "struct", 77),
    ("listerat", "[", 319),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewVrootEmulatedDirEntry", "", 40),
    ("Name", "r *rootEntry", 46),
    ("Size", "r *rootEntry", 49),
    ("Mode", "r *rootEntry", 52),
    ("ModTime", "r *rootEntry", 55),
    ("IsDir", "r *rootEntry", 58),
    ("Sys", "r *rootEntry", 61),
    ("newArozFileWrapper", "", 81),
    ("Name", "f *wrappedArozFile", 85),
    ("Size", "f *wrappedArozFile", 89),
    ("Mode", "f *wrappedArozFile", 97),
    ("ModTime", "f *wrappedArozFile", 105),
    ("IsDir", "f *wrappedArozFile", 113),
    ("Sys", "f *wrappedArozFile", 121),
    ("ReadAt", "f *wrappedArozFile", 125),
    ("WriteAt", "f *wrappedArozFile", 129),
    ("GetNewSFTPRoot", "", 133),
    ("getFshFromID", "fs *root", 143),
    ("Fileread", "fs *root", 153),
    ("Filewrite", "fs *root", 163),
    ("OpenFile", "fs *root", 182),
    ("Filecmd", "fs *root", 197),
    ("rename", "fs *root", 233),
    ("PosixRename", "fs *root", 269),
    ("StatVFS", "fs *root", 273),
    ("mkdir", "fs *root", 277),
    ("rmdir", "fs *root", 286),
    ("link", "fs *root", 294),
    ("symlink", "fs *root", 300),
    ("unlink", "fs *root", 304),
    ("ListAt", "f listerat", 322),
    ("Filelist", "fs *root", 334),
    ("readdir", "fs *root", 357),
    ("readlink", "fs *root", 396),
    ("Lstat", "fs *root", 401),
    ("Realpath", "fs *root", 410),
    ("getFshAndSubpathFromSFTPPathname", "fs *root", 418),
    ("lfetch", "fs *root", 452),
    ("fetch", "fs *root", 481),
    ("Name", "f *rootFolder", 490),
    ("Size", "f *rootFolder", 491),
    ("Mode", "f *rootFolder", 494),
    ("ModTime", "f *rootFolder", 497),
    ("IsDir", "f *rootFolder", 498),
    ("Sys", "f *rootFolder", 499),
    ("ReadAt", "f *rootFolder", 503),
    ("WriteAt", "f *rootFolder", 507),
    ("cleanPath", "", 520),
    ("cleanPathWithBase", "", 524),
];

pub async fn newvrootemulateddirentry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "NewVrootEmulatedDirEntry" })
}

pub async fn rootentry_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.Name" })
}

pub async fn rootentry_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.Size" })
}

pub async fn rootentry_mode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.Mode" })
}

pub async fn rootentry_modtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.ModTime" })
}

pub async fn rootentry_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.IsDir" })
}

pub async fn rootentry_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootEntry.Sys" })
}

pub async fn newarozfilewrapper(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "newArozFileWrapper" })
}

pub async fn wrappedarozfile_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.Name" })
}

pub async fn wrappedarozfile_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.Size" })
}

pub async fn wrappedarozfile_mode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.Mode" })
}

pub async fn wrappedarozfile_modtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.ModTime" })
}

pub async fn wrappedarozfile_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.IsDir" })
}

pub async fn wrappedarozfile_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.Sys" })
}

pub async fn wrappedarozfile_readat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.ReadAt" })
}

pub async fn wrappedarozfile_writeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "wrappedArozFile.WriteAt" })
}

pub async fn getnewsftproot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "GetNewSFTPRoot" })
}

pub async fn root_getfshfromid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.getFshFromID" })
}

pub async fn root_fileread(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Fileread" })
}

pub async fn root_filewrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Filewrite" })
}

pub async fn root_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.OpenFile" })
}

pub async fn root_filecmd(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Filecmd" })
}

pub async fn root_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.rename" })
}

pub async fn root_posixrename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.PosixRename" })
}

pub async fn root_statvfs(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.StatVFS" })
}

pub async fn root_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.mkdir" })
}

pub async fn root_rmdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.rmdir" })
}

pub async fn root_link(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.link" })
}

pub async fn root_symlink(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.symlink" })
}

pub async fn root_unlink(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.unlink" })
}

pub async fn listerat_listat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "listerat.ListAt" })
}

pub async fn root_filelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Filelist" })
}

pub async fn root_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.readdir" })
}

pub async fn root_readlink(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.readlink" })
}

pub async fn root_lstat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Lstat" })
}

pub async fn root_realpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.Realpath" })
}

pub async fn root_getfshandsubpathfromsftppathname(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.getFshAndSubpathFromSFTPPathname" })
}

pub async fn root_lfetch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.lfetch" })
}

pub async fn root_fetch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "root.fetch" })
}

pub async fn rootfolder_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.Name" })
}

pub async fn rootfolder_size(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.Size" })
}

pub async fn rootfolder_mode(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.Mode" })
}

pub async fn rootfolder_modtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.ModTime" })
}

pub async fn rootfolder_isdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.IsDir" })
}

pub async fn rootfolder_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.Sys" })
}

pub async fn rootfolder_readat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.ReadAt" })
}

pub async fn rootfolder_writeat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "rootFolder.WriteAt" })
}

pub async fn cleanpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "cleanPath" })
}

pub async fn cleanpathwithbase(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/sftpserver/vroot.go", function: "cleanPathWithBase" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
