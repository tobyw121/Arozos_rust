//! Original Go file: `main.router.go`
//! Package: `main`; LOC: 201; SHA256: `5e85737c813b4ab7358eb68417ac7df001feb8b58b3fd80f15470a7ec89db2d4`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "main.router.go", package: "main", go_loc: 201, functions: 2, types: 0, sha256: "5e85737c813b4ab7358eb68417ac7df001feb8b58b3fd80f15470a7ec89db2d4" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/network/gzipmiddleware",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "path/filepath",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("mrouter", "", 21),
    ("routerStaticContentServer", "", 195),
];

pub async fn mrouter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "main.router.go", function: "mrouter" })
}

pub async fn routerstaticcontentserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "main.router.go", function: "routerStaticContentServer" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
