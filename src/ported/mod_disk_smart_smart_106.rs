//! Original Go file: `mod/disk/smart/smart.go`
//! Package: `smart`; LOC: 153; SHA256: `1c108d94b6be15cfdcb5146a0dfc4851e32d5b192df8120297f260f93ecd9819`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/smart/smart.go", package: "smart", go_loc: 153, functions: 7, types: 1, sha256: "1c108d94b6be15cfdcb5146a0dfc4851e32d5b192df8120297f260f93ecd9819" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "os",
    "os/exec",
    "runtime",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SMARTListener", "struct", 29),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSmartListener", "", 35),
    ("scanAvailableDevices", "", 65),
    ("readSMARTDevices", "", 81),
    ("fillHealthyStatus", "", 91),
    ("fillCapacity", "", 114),
    ("GetSMART", "s *SMARTListener", 133),
    ("getBinary", "", 138),
];

pub async fn newsmartlistener(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "NewSmartListener" })
}

pub async fn scanavailabledevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "scanAvailableDevices" })
}

pub async fn readsmartdevices(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "readSMARTDevices" })
}

pub async fn fillhealthystatus(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "fillHealthyStatus" })
}

pub async fn fillcapacity(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "fillCapacity" })
}

pub async fn smartlistener_getsmart(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "SMARTListener.GetSMART" })
}

pub async fn getbinary(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/smart/smart.go", function: "getBinary" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
