//! Original Go file: `mod/filesystem/metadata/video.go`
//! Package: `metadata`; LOC: 84; SHA256: `93620edb3426c608bbb18202439dfdbd3a05ad96a16b0e6bbd2416e3ff692cd4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/metadata/video.go", package: "metadata", go_loc: 84, functions: 2, types: 0, sha256: "93620edb3426c608bbb18202439dfdbd3a05ad96a16b0e6bbd2416e3ff692cd4" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "errors",
    "github.com/oliamb/cutter",
    "image",
    "image/jpeg",
    "imuslab.com/arozos/mod/apt",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/utils",
    "os/exec",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("generateThumbnailForVideo", "", 17),
    ("pkg_exists", "", 81),
];

pub async fn generatethumbnailforvideo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/video.go", function: "generateThumbnailForVideo" })
}

pub async fn pkg_exists(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/metadata/video.go", function: "pkg_exists" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
