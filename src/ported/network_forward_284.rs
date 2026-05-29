//! Original Go file: `network.forward.go`
//! Package: `main`; LOC: 172; SHA256: `2d26f604e3221df185bfade02013b75fa80c53450d812f34e7a14f5caac3f7b2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "network.forward.go", package: "main", go_loc: 172, functions: 2, types: 0, sha256: "2d26f604e3221df185bfade02013b75fa80c53450d812f34e7a14f5caac3f7b2" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("portForwardInit", "", 21),
    ("portforward_handleForward", "", 65),
];

pub async fn portforwardinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.forward.go", function: "portForwardInit" })
}

pub async fn portforward_handleforward(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "network.forward.go", function: "portforward_handleForward" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
