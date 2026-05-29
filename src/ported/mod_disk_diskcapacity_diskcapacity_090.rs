//! Original Go file: `mod/disk/diskcapacity/diskcapacity.go`
//! Package: `diskcapacity`; LOC: 134; SHA256: `0fc5318c2884f7e90ae0e77f78e3f471ad9c22d5c868af4249261afbeec476bb`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/disk/diskcapacity/diskcapacity.go", package: "diskcapacity", go_loc: 134, functions: 4, types: 2, sha256: "0fc5318c2884f7e90ae0e77f78e3f471ad9c22d5c868af4249261afbeec476bb" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/disk/diskcapacity/dftool",
    "imuslab.com/arozos/mod/filesystem/arozfs",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "path/filepath",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Resolver", "struct", 22),
    ("CapacityInfo", "struct", 26),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewCapacityResolver", "", 36),
    ("HandleCapacityResolving", "cr *Resolver", 42),
    ("HandleTmpCapacityResolving", "cr *Resolver", 77),
    ("ResolveCapacityInfo", "cr *Resolver", 90),
];

pub async fn newcapacityresolver(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskcapacity/diskcapacity.go", function: "NewCapacityResolver" })
}

pub async fn resolver_handlecapacityresolving(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskcapacity/diskcapacity.go", function: "Resolver.HandleCapacityResolving" })
}

pub async fn resolver_handletmpcapacityresolving(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskcapacity/diskcapacity.go", function: "Resolver.HandleTmpCapacityResolving" })
}

pub async fn resolver_resolvecapacityinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/disk/diskcapacity/diskcapacity.go", function: "Resolver.ResolveCapacityInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
