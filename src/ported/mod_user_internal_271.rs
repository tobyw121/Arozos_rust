//! Original Go file: `mod/user/internal.go`
//! Package: `user`; LOC: 38; SHA256: `a9bb207c9a4a0932c98f6c00d9d8d7a2bd8c94e3bc7a897bd9be906e56288095`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/user/internal.go", package: "user", go_loc: 38, functions: 3, types: 0, sha256: "a9bb207c9a4a0932c98f6c00d9d8d7a2bd8c94e3bc7a897bd9be906e56288095" };

pub const GO_IMPORTS: &[&str] = &[
    "errors",
    "imuslab.com/arozos/mod/filesystem",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("getHandlerFromVirtualPath", "", 14),
    ("getHandlerFromID", "", 24),
    ("getIDFromVirtualPath", "", 36),
];

pub async fn gethandlerfromvirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/internal.go", function: "getHandlerFromVirtualPath" })
}

pub async fn gethandlerfromid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/internal.go", function: "getHandlerFromID" })
}

pub async fn getidfromvirtualpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/user/internal.go", function: "getIDFromVirtualPath" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
