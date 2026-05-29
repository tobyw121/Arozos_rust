//! Original Go file: `system.info.go`
//! Package: `main`; LOC: 241; SHA256: `01012711a4101542cf46c2cb473147a8528c09fa19f78545dc200271c3ce0982`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "system.info.go", package: "main", go_loc: 241, functions: 3, types: 0, sha256: "01012711a4101542cf46c2cb473147a8528c09fa19f78545dc200271c3ce0982" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/info/hardwareinfo",
    "imuslab.com/arozos/mod/info/logviewer",
    "imuslab.com/arozos/mod/info/usageinfo",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/updates",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "path/filepath",
    "runtime",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("SystemInfoInit", "", 20),
    ("InfoHandleGetRuntimeInfo", "", 207),
    ("InfoHandleTaskInfo", "", 222),
];

pub async fn systeminfoinit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.info.go", function: "SystemInfoInit" })
}

pub async fn infohandlegetruntimeinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.info.go", function: "InfoHandleGetRuntimeInfo" })
}

pub async fn infohandletaskinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "system.info.go", function: "InfoHandleTaskInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
