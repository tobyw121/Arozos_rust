//! Original Go file: `mod/filesystem/fspermission/fspermission.go`
//! Package: `fspermission`; LOC: 88; SHA256: `815ce643cd7789f8b12445a5c4c72d4feaea41d259f677abec86b8894667aba0`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/filesystem/fspermission/fspermission.go", package: "fspermission", go_loc: 88, functions: 2, types: 0, sha256: "815ce643cd7789f8b12445a5c4c72d4feaea41d259f677abec86b8894667aba0" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "fmt",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "log",
    "os",
    "strconv",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("GetFilePermissions", "", 20),
    ("SetFilePermisson", "", 32),
];

pub async fn getfilepermissions(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fspermission/fspermission.go", function: "GetFilePermissions" })
}

pub async fn setfilepermisson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/filesystem/fspermission/fspermission.go", function: "SetFilePermisson" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
