//! Original Go file: `mod/filesystem/metadata/folder.go`
//! Package: `metadata`; LOC: 189; SHA256: `b8a5632fe9f9ee8ce97faaf30c4c253cb69b466b41167e62d9880ea6471909c8`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/folder.go", package: "metadata", go_loc: 189, functions: 2, types: 0, sha256: "b8a5632fe9f9ee8ce97faaf30c4c253cb69b466b41167e62d9880ea6471909c8" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/base64",
    "errors",
    "github.com/nfnt/resize",
    "image",
    "image/draw",
    "image/jpeg",
    "image/png",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "log",
    "os",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForFolder", "", 25),
    ("generateLayeredThumbnailFolder", "", 133),
];

pub async fn generatethumbnailforfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/folder.go", function: "generateThumbnailForFolder" })
}

pub async fn generatelayeredthumbnailfolder(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/folder.go", function: "generateLayeredThumbnailFolder" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
