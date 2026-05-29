//! Original Go file: `mod/iot/hds/utils.go`
//! Package: `hds`; LOC: 92; SHA256: `93a22543e00c9862c3cb0b327e97e25833e7f9081911e7969fd7a873a8543b36`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/hds/utils.go", package: "hds", go_loc: 92, functions: 6, types: 0, sha256: "93a22543e00c9862c3cb0b327e97e25833e7f9081911e7969fd7a873a8543b36" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "io",
    "log",
    "net",
    "net/http",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("isJSON", "", 15),
    ("tryGet", "", 20),
    ("tryGetHDSUUID", "", 43),
    ("tryGetHDSInfo", "", 54),
    ("getHDSStatus", "", 69),
    ("getLocalIP", "", 78),
];

pub async fn isjson(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "isJSON" })
}

pub async fn tryget(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "tryGet" })
}

pub async fn trygethdsuuid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "tryGetHDSUUID" })
}

pub async fn trygethdsinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "tryGetHDSInfo" })
}

pub async fn gethdsstatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "getHDSStatus" })
}

pub async fn getlocalip(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/iot/hds/utils.go", function: "getLocalIP" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
