//! Original Go file: `mod/auth/oauth2/github.go`
//! Package: `oauth2`; LOC: 86; SHA256: `0f6ff90e8f2ca977ea492073b5507c9308c9bfcea77543350faba7c1a3607751`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/oauth2/github.go", package: "oauth2", go_loc: 86, functions: 3, types: 1, sha256: "0f6ff90e8f2ca977ea492073b5507c9308c9bfcea77543350faba7c1a3607751" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "golang.org/x/oauth2",
    "golang.org/x/oauth2/github",
    "io",
    "net/http",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("GithubField", "struct", 13),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("githubScope", "", 60),
    ("githubEndpoint", "", 64),
    ("githubUserInfo", "", 68),
];

pub async fn githubscope(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/github.go", function: "githubScope" })
}

pub async fn githubendpoint(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/github.go", function: "githubEndpoint" })
}

pub async fn githubuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/oauth2/github.go", function: "githubUserInfo" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
