//! Original Go file: `mod/prouter/prouter.go`
//! Package: `prouter`; LOC: 114; SHA256: `0514f7df55f0bef3bbb865af3a9886c9ac89b2c5f16ff81f7e35720ba57e49f5`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/prouter/prouter.go", package: "prouter", go_loc: 114, functions: 2, types: 2, sha256: "0514f7df55f0bef3bbb865af3a9886c9ac89b2c5f16ff81f7e35720ba57e49f5" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/security/csrf",
    "imuslab.com/arozos/mod/user",
    "log",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("RouterOption", "struct", 22),
    ("RouterDef", "struct", 32),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewModuleRouter", "", 41),
    ("HandleFunc", "router *RouterDef", 52),
];

pub async fn newmodulerouter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/prouter/prouter.go", function: "NewModuleRouter" })
}

pub async fn routerdef_handlefunc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/prouter/prouter.go", function: "RouterDef.HandleFunc" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
