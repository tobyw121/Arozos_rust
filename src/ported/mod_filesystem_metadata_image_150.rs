//! Original Go file: `mod/filesystem/metadata/image.go`
//! Package: `metadata`; LOC: 115; SHA256: `470897ebe6006e065a7f1c57dc1a1bb01e3ef4676304a8ee4b9eed9d435c1aa1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/image.go", package: "metadata", go_loc: 115, functions: 2, types: 0, sha256: "470897ebe6006e065a7f1c57dc1a1bb01e3ef4676304a8ee4b9eed9d435c1aa1" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "errors",
    "github.com/nfnt/resize",
    "github.com/oliamb/cutter",
    "golang.org/x/image/webp",
    "image",
    "image/jpeg",
    "imuslab.com/arozos/mod/filesystem",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForImage", "", 18),
    ("handleWebpDecoding", "", 112),
];

pub async fn generatethumbnailforimage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/image.go", function: "generateThumbnailForImage" })
}

pub async fn handlewebpdecoding(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/image.go", function: "handleWebpDecoding" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
