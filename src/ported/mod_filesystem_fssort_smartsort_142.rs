//! Original Go file: `mod/filesystem/fssort/smartsort.go`
//! Package: `fssort`; LOC: 59; SHA256: `04cf9382a54b335f93b42284d25cb17c356d4560a46856307abefba248d3a163`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/fssort/smartsort.go", package: "fssort", go_loc: 59, functions: 2, types: 0, sha256: "04cf9382a54b335f93b42284d25cb17c356d4560a46856307abefba248d3a163" };

pub const GO_IMPORTS: &[&str] = &[
    "fmt",
    "regexp",
    "sort",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SortNaturalFilelist", "", 18),
    ("sortNaturalStrings", "", 35),
];

pub async fn sortnaturalfilelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/smartsort.go", function: "SortNaturalFilelist" })
}

pub async fn sortnaturalstrings(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fssort/smartsort.go", function: "sortNaturalStrings" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
