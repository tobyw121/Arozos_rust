//! Original Go file: `mod/storage/ftp/aofs.go`
//! Package: `ftp`; LOC: 276; SHA256: `1d6cbb5a2fbb7a34cdbe6d431f3e5e990e257271f7802f5d8f42ff3df68eeb63`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/ftp/aofs.go", package: "ftp", go_loc: 276, functions: 16, types: 1, sha256: "1d6cbb5a2fbb7a34cdbe6d431f3e5e990e257271f7802f5d8f42ff3df68eeb63" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "github.com/spf13/afero",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/user",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("aofs", "struct", 23),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Create", "a aofs", 28),
    ("Chown", "a aofs", 39),
    ("Mkdir", "a aofs", 50),
    ("MkdirAll", "a aofs", 61),
    ("Open", "a aofs", 72),
    ("Stat", "a aofs", 85),
    ("OpenFile", "a aofs", 97),
    ("AllocateSpace", "a aofs", 109),
    ("Remove", "a aofs", 116),
    ("RemoveAll", "a aofs", 128),
    ("Rename", "a aofs", 139),
    ("Name", "a aofs", 184),
    ("Chmod", "a aofs", 188),
    ("Chtimes", "a aofs", 199),
    ("pathRewrite", "a aofs", 212),
    ("checkAllowAccess", "a aofs", 263),
];

pub async fn aofs_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Create" })
}

pub async fn aofs_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Chown" })
}

pub async fn aofs_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Mkdir" })
}

pub async fn aofs_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.MkdirAll" })
}

pub async fn aofs_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Open" })
}

pub async fn aofs_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Stat" })
}

pub async fn aofs_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.OpenFile" })
}

pub async fn aofs_allocatespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.AllocateSpace" })
}

pub async fn aofs_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Remove" })
}

pub async fn aofs_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.RemoveAll" })
}

pub async fn aofs_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Rename" })
}

pub async fn aofs_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Name" })
}

pub async fn aofs_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Chmod" })
}

pub async fn aofs_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.Chtimes" })
}

pub async fn aofs_pathrewrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.pathRewrite" })
}

pub async fn aofs_checkallowaccess(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/ftp/aofs.go", function: "aofs.checkAllowAccess" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
