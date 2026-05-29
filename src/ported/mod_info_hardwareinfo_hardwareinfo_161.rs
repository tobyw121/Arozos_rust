//! Original Go file: `mod/info/hardwareinfo/hardwareinfo.go`
//! Package: `hardwareinfo`; LOC: 132; SHA256: `7131ebd6894b0fc9b763da4c2f29f0cc30df8eff26db75e6dc3ad47919ba562f`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/info/hardwareinfo/hardwareinfo.go", package: "hardwareinfo", go_loc: 132, functions: 5, types: 4, sha256: "7131ebd6894b0fc9b763da4c2f29f0cc30df8eff26db75e6dc3ad47919ba562f" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os/exec",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("CPUInfo", "struct", 21),
    ("LogicalDisk", "struct", 29),
    ("ArOZInfo", "struct", 35),
    ("Server", "struct", 46),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewInfoServer", "", 50),
    ("PrintSystemHardwareDebugMessage", "", 60),
    ("GetArOZInfo", "s *Server", 70),
    ("wmicGetinfo", "", 89),
    ("filterGrepResults", "", 125),
];

pub async fn newinfoserver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/hardwareinfo.go", function: "NewInfoServer" })
}

pub async fn printsystemhardwaredebugmessage(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/hardwareinfo.go", function: "PrintSystemHardwareDebugMessage" })
}

pub async fn server_getarozinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/hardwareinfo.go", function: "Server.GetArOZInfo" })
}

pub async fn wmicgetinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/hardwareinfo.go", function: "wmicGetinfo" })
}

pub async fn filtergrepresults(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/info/hardwareinfo/hardwareinfo.go", function: "filterGrepResults" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
