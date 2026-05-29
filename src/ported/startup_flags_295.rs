//! Original Go file: `startup.flags.go`
//! Package: `main`; LOC: 104; SHA256: `6328c30e1822ee6d1a834e7e49ccbc939bafa5be47ba2a1c076c695a1187fcd6`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "startup.flags.go", package: "main", go_loc: 104, functions: 2, types: 0, sha256: "6328c30e1822ee6d1a834e7e49ccbc939bafa5be47ba2a1c076c695a1187fcd6" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("StartupFlagsInit", "", 19),
    ("handleBootFlagsFunction", "", 42),
];

pub async fn startupflagsinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "startup.flags.go", function: "StartupFlagsInit" })
}

pub async fn handlebootflagsfunction(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "startup.flags.go", function: "handleBootFlagsFunction" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
