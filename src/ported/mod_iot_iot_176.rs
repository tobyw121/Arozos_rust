//! Original Go file: `mod/iot/iot.go`
//! Package: `iot`; LOC: 73; SHA256: `ed61b217e810b24db6832237d1d6548a5e88bf3c76fffdcdb0b3d535f516f290`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/iot/iot.go", package: "iot", go_loc: 73, functions: 0, types: 5, sha256: "ed61b217e810b24db6832237d1d6548a5e88bf3c76fffdcdb0b3d535f516f290" };

pub const GO_IMPORTS: &[&str] = &[];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Endpoint", "struct", 11),
    ("Device", "struct", 27),
    ("AuthInfo", "struct", 43),
    ("Stats", "struct", 49),
    ("ProtocolHandler", "interface", 64),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[];

pub fn migration_status() -> LegacyModuleStatus { STATUS }
