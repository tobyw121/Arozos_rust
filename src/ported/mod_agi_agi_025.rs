//! Original Go file: `mod/agi/agi.go`
//! Package: `agi`; LOC: 447; SHA256: `17629d96e4569d5287a0f60a80326c7285bc5793f499a88270808d7e3c16d62c`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/agi.go", package: "agi", go_loc: 447, functions: 12, types: 3, sha256: "17629d96e4569d5287a0f60a80326c7285bc5793f499a88270808d7e3c16d62c" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "github.com/robertkrimen/otto",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/apt",
    "imuslab.com/arozos/mod/filesystem",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/filesystem/metadata",
    "imuslab.com/arozos/mod/iot",
    "imuslab.com/arozos/mod/share",
    "imuslab.com/arozos/mod/time/nightly",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "io",
    "log",
    "net/http",
    "os",
    "path/filepath",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("AgiPackage", "struct", 45),
    ("AgiSysInfo", "struct", 49),
    ("Gateway", "struct", 71),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewGateway", "", 79),
    ("RegisterNightlyOperations", "g *Gateway", 99),
    ("InitiateAllWebAppModules", "g *Gateway", 125),
    ("RunScript", "g *Gateway", 148),
    ("RaiseError", "g *Gateway", 164),
    ("filterDBTable", "g *Gateway", 171),
    ("APIHandler", "g *Gateway", 188),
    ("InterfaceHandler", "g *Gateway", 199),
    ("ExecuteAGIScript", "g *Gateway", 278),
    ("ExecuteAGIScriptAsUser", "g *Gateway", 339),
    ("getUserSpecificTempFilePath", "g *Gateway", 419),
    ("bufferRemoteResourcesToLocal", "g *Gateway", 431),
];

pub async fn newgateway(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "NewGateway" })
}

pub async fn gateway_registernightlyoperations(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.RegisterNightlyOperations" })
}

pub async fn gateway_initiateallwebappmodules(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.InitiateAllWebAppModules" })
}

pub async fn gateway_runscript(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.RunScript" })
}

pub async fn gateway_raiseerror(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.RaiseError" })
}

pub async fn gateway_filterdbtable(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.filterDBTable" })
}

pub async fn gateway_apihandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.APIHandler" })
}

pub async fn gateway_interfacehandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.InterfaceHandler" })
}

pub async fn gateway_executeagiscript(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.ExecuteAGIScript" })
}

pub async fn gateway_executeagiscriptasuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.ExecuteAGIScriptAsUser" })
}

pub async fn gateway_getuserspecifictempfilepath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.getUserSpecificTempFilePath" })
}

pub async fn gateway_bufferremoteresourcestolocal(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/agi.go", function: "Gateway.bufferRemoteResourcesToLocal" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
