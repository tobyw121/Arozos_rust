//! Original Go file: `mod/filesystem/shortcut/shortcut.go`
//! Package: `shortcut`; LOC: 52; SHA256: `1e15d3c46be5392d02bae7cf0630ba9ab3bf87f60a1d1fddd34a4ad1cbf8a535`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/shortcut/shortcut.go", package: "shortcut", go_loc: 52, functions: 2, types: 0, sha256: "1e15d3c46be5392d02bae7cf0630ba9ab3bf87f60a1d1fddd34a4ad1cbf8a535" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/utils",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ReadShortcut", "", 18),
    ("GenerateShortcutBytes", "", 43),
];

pub async fn readshortcut(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/shortcut/shortcut.go", function: "ReadShortcut" })
}

pub async fn generateshortcutbytes(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/shortcut/shortcut.go", function: "GenerateShortcutBytes" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
