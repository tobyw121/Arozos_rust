//! Original Go file: `console.go`
//! Package: `main`; LOC: 299; SHA256: `66be354e7cd9f5a7f498b8135a0e8c04ef4f8fb6939ecd1cfbbbead07ce70df3`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "console.go", package: "main", go_loc: 299, functions: 3, types: 0, sha256: "66be354e7cd9f5a7f498b8135a0e8c04ef4f8fb6939ecd1cfbbbead07ce70df3" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/utils",
    "os",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("consoleCommandHandler", "", 14),
    ("matchSubfix", "", 215),
    ("parseCommandLine", "", 236),
];

pub async fn consolecommandhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "console.go", function: "consoleCommandHandler" })
}

pub async fn matchsubfix(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "console.go", function: "matchSubfix" })
}

pub async fn parsecommandline(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "console.go", function: "parseCommandLine" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
