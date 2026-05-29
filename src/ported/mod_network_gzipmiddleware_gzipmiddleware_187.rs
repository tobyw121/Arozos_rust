//! Original Go file: `mod/network/gzipmiddleware/gzipmiddleware.go`
//! Package: `gzipmiddleware`; LOC: 107; SHA256: `e837b0c2f7f8b13693d9ebfbc69f0b439cb39ab49bc3d9decb247cf2b005febf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/gzipmiddleware/gzipmiddleware.go", package: "gzipmiddleware", go_loc: 107, functions: 5, types: 2, sha256: "e837b0c2f7f8b13693d9ebfbc69f0b439cb39ab49bc3d9decb247cf2b005febf" };

pub const GO_IMPORTS: &[&str] = &[
    "compress/gzip",
    "io",
    "net/http",
    "strings",
    "sync",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("gzipResponseWriter", "struct", 25),
    ("gzipFuncResponseWriter", "struct", 83),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("WriteHeader", "w *gzipResponseWriter", 30),
    ("Write", "w *gzipResponseWriter", 35),
    ("Compress", "", 42),
    ("Write", "w gzipFuncResponseWriter", 88),
    ("CompressFunc", "", 95),
];

pub async fn gzipresponsewriter_writeheader(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/gzipmiddleware/gzipmiddleware.go", function: "gzipResponseWriter.WriteHeader" })
}

pub async fn gzipresponsewriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/gzipmiddleware/gzipmiddleware.go", function: "gzipResponseWriter.Write" })
}

pub async fn compress(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/gzipmiddleware/gzipmiddleware.go", function: "Compress" })
}

pub async fn gzipfuncresponsewriter_write(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/gzipmiddleware/gzipmiddleware.go", function: "gzipFuncResponseWriter.Write" })
}

pub async fn compressfunc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/gzipmiddleware/gzipmiddleware.go", function: "CompressFunc" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
