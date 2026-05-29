//! Original Go file: `mod/prouter/lanCheck.go`
//! Package: `prouter`; LOC: 110; SHA256: `9e760aeb49b5c01e6fbe17fd31e950ca58825323be93a056704ff22492eede6b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/prouter/lanCheck.go", package: "prouter", go_loc: 110, functions: 3, types: 1, sha256: "9e760aeb49b5c01e6fbe17fd31e950ca58825323be93a056704ff22492eede6b" };

pub const GO_IMPORTS: &[&str] = &[
    "bytes",
    "net",
    "net/http",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ipRange", "struct", 10),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("checkIfLAN", "", 46),
    ("isPrivateSubnet", "", 90),
    ("inRange", "", 104),
];

pub async fn checkiflan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/prouter/lanCheck.go", function: "checkIfLAN" })
}

pub async fn isprivatesubnet(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/prouter/lanCheck.go", function: "isPrivateSubnet" })
}

pub async fn inrange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/prouter/lanCheck.go", function: "inRange" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
