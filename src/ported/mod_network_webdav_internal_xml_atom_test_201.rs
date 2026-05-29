//! Original Go file: `mod/network/webdav/internal/xml/atom_test.go`
//! Package: `xml`; LOC: 56; SHA256: `73e80ce60046bbcb61ae206610e935abe76094a79a48d7bcada3a4423531dd04`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/webdav/internal/xml/atom_test.go", package: "xml", go_loc: 56, functions: 2, types: 0, sha256: "73e80ce60046bbcb61ae206610e935abe76094a79a48d7bcada3a4423531dd04" };

pub const GO_IMPORTS: &[&str] = &[
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ParseTime", "", 44),
    ("NewText", "", 52),
];

pub async fn parsetime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/atom_test.go", function: "ParseTime" })
}

pub async fn newtext(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/webdav/internal/xml/atom_test.go", function: "NewText" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
