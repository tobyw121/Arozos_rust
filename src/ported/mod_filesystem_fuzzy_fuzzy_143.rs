//! Original Go file: `mod/filesystem/fuzzy/fuzzy.go`
//! Package: `fuzzy`; LOC: 113; SHA256: `c820b5adca5454d18504d4c0796805188ebef878d39a61ceeeb1883cde547422`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/fuzzy/fuzzy.go", package: "fuzzy", go_loc: 113, functions: 3, types: 1, sha256: "c820b5adca5454d18504d4c0796805188ebef878d39a61ceeeb1883cde547422" };

pub const GO_IMPORTS: &[&str] = &[
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Matcher", "struct", 23),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewFuzzyMatcher", "", 29),
    ("Match", "m *Matcher", 38),
    ("buildFuzzyChunks", "", 60),
];

pub async fn newfuzzymatcher(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fuzzy/fuzzy.go", function: "NewFuzzyMatcher" })
}

pub async fn matcher_match(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fuzzy/fuzzy.go", function: "Matcher.Match" })
}

pub async fn buildfuzzychunks(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fuzzy/fuzzy.go", function: "buildFuzzyChunks" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
