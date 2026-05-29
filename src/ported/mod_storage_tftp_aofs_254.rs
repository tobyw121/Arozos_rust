//! Original Go file: `mod/storage/tftp/aofs.go`
//! Package: `tftp`; LOC: 257; SHA256: `9c0d8326c8462f77f21c5b8ed4889f2426146be755cb46ef647749ffa615e9e6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/storage/tftp/aofs.go", package: "tftp", go_loc: 257, functions: 16, types: 1, sha256: "9c0d8326c8462f77f21c5b8ed4889f2426146be755cb46ef647749ffa615e9e6" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "github.com/spf13/afero",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/user",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("aofs", "struct", 24),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("Create", "a aofs", 29),
    ("Chown", "a aofs", 40),
    ("Mkdir", "a aofs", 51),
    ("MkdirAll", "a aofs", 62),
    ("Open", "a aofs", 73),
    ("Stat", "a aofs", 85),
    ("OpenFile", "a aofs", 96),
    ("AllocateSpace", "a aofs", 107),
    ("Remove", "a aofs", 114),
    ("RemoveAll", "a aofs", 126),
    ("Rename", "a aofs", 137),
    ("Name", "a aofs", 182),
    ("Chmod", "a aofs", 186),
    ("Chtimes", "a aofs", 197),
    ("pathRewrite", "a aofs", 210),
    ("checkAllowAccess", "a aofs", 244),
];

pub async fn aofs_create(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Create" })
}

pub async fn aofs_chown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Chown" })
}

pub async fn aofs_mkdir(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Mkdir" })
}

pub async fn aofs_mkdirall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.MkdirAll" })
}

pub async fn aofs_open(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Open" })
}

pub async fn aofs_stat(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Stat" })
}

pub async fn aofs_openfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.OpenFile" })
}

pub async fn aofs_allocatespace(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.AllocateSpace" })
}

pub async fn aofs_remove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Remove" })
}

pub async fn aofs_removeall(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.RemoveAll" })
}

pub async fn aofs_rename(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Rename" })
}

pub async fn aofs_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Name" })
}

pub async fn aofs_chmod(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Chmod" })
}

pub async fn aofs_chtimes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.Chtimes" })
}

pub async fn aofs_pathrewrite(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.pathRewrite" })
}

pub async fn aofs_checkallowaccess(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/storage/tftp/aofs.go", function: "aofs.checkAllowAccess" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
