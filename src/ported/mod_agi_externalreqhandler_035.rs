//! Original Go file: `mod/agi/externalReqHandler.go`
//! Package: `agi`; LOC: 227; SHA256: `d03aebdd459b2fac30e4fed57f4ba128dce8c92007afbe4e7a9e1fdcc3622822`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/agi/externalReqHandler.go", package: "agi", go_loc: 227, functions: 5, types: 1, sha256: "d03aebdd459b2fac30e4fed57f4ba128dce8c92007afbe4e7a9e1fdcc3622822" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/agi/static",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "path/filepath",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("endpointFormat", "struct", 15),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("ExtAPIHandler", "g *Gateway", 21),
    ("AddExternalEndPoint", "g *Gateway", 89),
    ("RemoveExternalEndPoint", "g *Gateway", 126),
    ("ListExternalEndpoint", "g *Gateway", 164),
    ("checkIfExternalEndpointExist", "g *Gateway", 208),
];

pub async fn gateway_extapihandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/externalReqHandler.go", function: "Gateway.ExtAPIHandler" })
}

pub async fn gateway_addexternalendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/externalReqHandler.go", function: "Gateway.AddExternalEndPoint" })
}

pub async fn gateway_removeexternalendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/externalReqHandler.go", function: "Gateway.RemoveExternalEndPoint" })
}

pub async fn gateway_listexternalendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/externalReqHandler.go", function: "Gateway.ListExternalEndpoint" })
}

pub async fn gateway_checkifexternalendpointexist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/agi/externalReqHandler.go", function: "Gateway.checkIfExternalEndpointExist" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
