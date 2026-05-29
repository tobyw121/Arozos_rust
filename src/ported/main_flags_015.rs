//! Original Go file: `main.flags.go`
//! Package: `main`; LOC: 114; SHA256: `40cd632f6c1dba5f94d1edff13f8abe8b810e2c0080e0198fa578255202af19a`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "main.flags.go", package: "main", go_loc: 114, functions: 0, types: 0, sha256: "40cd632f6c1dba5f94d1edff13f8abe8b810e2c0080e0198fa578255202af19a" };

pub const GO_IMPORTS: &[&str] = &[
    "flag",
    "imuslab.com/arozos/mod/apt",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/disk/raid",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/media/mediaserver",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/www",
    "os",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
