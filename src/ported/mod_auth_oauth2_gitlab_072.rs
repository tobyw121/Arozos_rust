//! Original Go file: `mod/auth/oauth2/gitlab.go`
//! Package: `oauth2`; LOC: 88; SHA256: `101f7887486c37496a9e0bd3ff101c738036375d6c7f2503fd574b10d1c72a85`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/gitlab.go", package: "oauth2", go_loc: 88, functions: 3, types: 1, sha256: "101f7887486c37496a9e0bd3ff101c738036375d6c7f2503fd574b10d1c72a85" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "golang.org/x/oauth2",
    "io",
    "net/http",
    "net/url",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("GitlabField", "struct", 13),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("gitlabScope", "", 58),
    ("gitlabEndpoint", "", 62),
    ("gitlabUserInfo", "", 70),
];

pub async fn gitlabscope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/gitlab.go", function: "gitlabScope" })
}

pub async fn gitlabendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/gitlab.go", function: "gitlabEndpoint" })
}

pub async fn gitlabuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/gitlab.go", function: "gitlabUserInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
