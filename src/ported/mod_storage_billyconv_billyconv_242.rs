//! Original Go file: `mod/storage/billyconv/billyconv.go`
//! Package: `billyconv`; LOC: 211; SHA256: `fc5bdc30bfed3a7480ca8b9395cf1370bc43929492b652fe07bf9f345822b6b3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/billyconv/billyconv.go", package: "billyconv", go_loc: 211, functions: 30, types: 3, sha256: "fc5bdc30bfed3a7480ca8b9395cf1370bc43929492b652fe07bf9f345822b6b3" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/go-git/go-billy/v5",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "io/fs",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ArozFsToBillyFileSytemAdapter", "struct", 22),
    ("ArozFSFileAdapter", "struct", 135),
    ("fileInfoWrapper", "struct", 186),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewArozFsToBillyFsAdapter", "", 26),
    ("Create", "adp *ArozFsToBillyFileSytemAdapter", 32),
    ("Open", "adp *ArozFsToBillyFileSytemAdapter", 41),
    ("OpenFile", "adp *ArozFsToBillyFileSytemAdapter", 50),
    ("Stat", "adp *ArozFsToBillyFileSytemAdapter", 59),
    ("Rename", "adp *ArozFsToBillyFileSytemAdapter", 68),
    ("Remove", "adp *ArozFsToBillyFileSytemAdapter", 74),
    ("Join", "adp *ArozFsToBillyFileSytemAdapter", 79),
    ("TempFile", "adp *ArozFsToBillyFileSytemAdapter", 83),
    ("ReadDir", "adp *ArozFsToBillyFileSytemAdapter", 87),
    ("MkdirAll", "adp *ArozFsToBillyFileSytemAdapter", 102),
    ("Lstat", "adp *ArozFsToBillyFileSytemAdapter", 107),
    ("Symlink", "adp *ArozFsToBillyFileSytemAdapter", 111),
    ("Readlink", "adp *ArozFsToBillyFileSytemAdapter", 115),
    ("Chroot", "adp *ArozFsToBillyFileSytemAdapter", 119),
    ("Root", "adp *ArozFsToBillyFileSytemAdapter", 123),
    ("CleanAndFilterFilename", "adp *ArozFsToBillyFileSytemAdapter", 128),
    ("ArozfsFileToBillyFile", "", 139),
    ("Name", "afa *ArozFSFileAdapter", 145),
    ("Read", "afa *ArozFSFileAdapter", 149),
    ("ReadAt", "afa *ArozFSFileAdapter", 153),
    ("Seek", "afa *ArozFSFileAdapter", 157),
    ("Write", "afa *ArozFSFileAdapter", 161),
    ("Lock", "afa *ArozFSFileAdapter", 165),
    ("Unlock", "afa *ArozFSFileAdapter", 169),
    ("Truncate", "afa *ArozFSFileAdapter", 173),
    ("Close", "afa *ArozFSFileAdapter", 177),
    ("Sys", "f fileInfoWrapper", 191),
    ("ConvertToOsFileInfo", "", 196),
    ("ConvertDirEntriesToFileInfos", "", 201),
];

pub async fn newarozfstobillyfsadapter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "NewArozFsToBillyFsAdapter" })
}

pub async fn arozfstobillyfilesytemadapter_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Create" })
}

pub async fn arozfstobillyfilesytemadapter_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Open" })
}

pub async fn arozfstobillyfilesytemadapter_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.OpenFile" })
}

pub async fn arozfstobillyfilesytemadapter_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Stat" })
}

pub async fn arozfstobillyfilesytemadapter_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Rename" })
}

pub async fn arozfstobillyfilesytemadapter_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Remove" })
}

pub async fn arozfstobillyfilesytemadapter_join(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Join" })
}

pub async fn arozfstobillyfilesytemadapter_tempfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.TempFile" })
}

pub async fn arozfstobillyfilesytemadapter_readdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.ReadDir" })
}

pub async fn arozfstobillyfilesytemadapter_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.MkdirAll" })
}

pub async fn arozfstobillyfilesytemadapter_lstat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Lstat" })
}

pub async fn arozfstobillyfilesytemadapter_symlink(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Symlink" })
}

pub async fn arozfstobillyfilesytemadapter_readlink(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Readlink" })
}

pub async fn arozfstobillyfilesytemadapter_chroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Chroot" })
}

pub async fn arozfstobillyfilesytemadapter_root(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.Root" })
}

pub async fn arozfstobillyfilesytemadapter_cleanandfilterfilename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFsToBillyFileSytemAdapter.CleanAndFilterFilename" })
}

pub async fn arozfsfiletobillyfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozfsFileToBillyFile" })
}

pub async fn arozfsfileadapter_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Name" })
}

pub async fn arozfsfileadapter_read(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Read" })
}

pub async fn arozfsfileadapter_readat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.ReadAt" })
}

pub async fn arozfsfileadapter_seek(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Seek" })
}

pub async fn arozfsfileadapter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Write" })
}

pub async fn arozfsfileadapter_lock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Lock" })
}

pub async fn arozfsfileadapter_unlock(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Unlock" })
}

pub async fn arozfsfileadapter_truncate(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Truncate" })
}

pub async fn arozfsfileadapter_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ArozFSFileAdapter.Close" })
}

pub async fn fileinfowrapper_sys(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "fileInfoWrapper.Sys" })
}

pub async fn converttoosfileinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ConvertToOsFileInfo" })
}

pub async fn convertdirentriestofileinfos(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/billyconv/billyconv.go", function: "ConvertDirEntriesToFileInfos" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
