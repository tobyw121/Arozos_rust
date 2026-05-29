//! Original Go file: `startup.go`
//! Package: `main`; LOC: 132; SHA256: `287099fc07780456a8dd2f47f3be76f3dfdc953f3ac0d7388005c8b118fb6cc1`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "startup.go", package: "main", go_loc: 132, functions: 1, types: 0, sha256: "287099fc07780456a8dd2f47f3be76f3dfdc953f3ac0d7388005c8b118fb6cc1" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/info/logger",
    "log",
    "os",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("RunStartup", "", 19),
];

pub async fn runstartup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "startup.go", function: "RunStartup" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
