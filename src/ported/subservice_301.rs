//! Original Go file: `subservice.go`
//! Package: `main`; LOC: 83; SHA256: `c98d789cadf8958f1c01800839be564ff3c4f1bad9447e856c4e517fce3ef2b6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "subservice.go", package: "main", go_loc: 83, functions: 2, types: 0, sha256: "c98d789cadf8958f1c01800839be564ff3c4f1bad9447e856c4e517fce3ef2b6" };

pub const GO_IMPORTS: &[&str] = &[
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/subservice",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "os",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SubserviceInit", "", 34),
    ("SubserviceHandleShutdown", "", 79),
];

pub async fn subserviceinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "subservice.go", function: "SubserviceInit" })
}

pub async fn subservicehandleshutdown(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "subservice.go", function: "SubserviceHandleShutdown" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
