//! Original Go file: `mod/console/console.go`
//! Package: `console`; LOC: 32; SHA256: `033596eadc947677de22c1b5178ac425d4ca86d6eac2b434525733b754a9e83b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/console/console.go", package: "console", go_loc: 32, functions: 2, types: 1, sha256: "033596eadc947677de22c1b5178ac425d4ca86d6eac2b434525733b754a9e83b" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "fmt",
    "os",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Console", "struct", 10),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewConsole", "", 14),
    ("ListenAndHandle", "c *Console", 20),
];

pub async fn newconsole(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/console/console.go", function: "NewConsole" })
}

pub async fn console_listenandhandle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/console/console.go", function: "Console.ListenAndHandle" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
