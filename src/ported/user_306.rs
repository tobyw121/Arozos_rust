//! Original Go file: `user.go`
//! Package: `main`; LOC: 565; SHA256: `1bb4f4c385bddacf85cacce9e320a87484c52b9a2f8f55632ae6b61bebce79ec`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "user.go", package: "main", go_loc: 565, functions: 9, types: 0, sha256: "1bb4f4c385bddacf85cacce9e320a87484c52b9a2f8f55632ae6b61bebce79ec" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/base64",
    "encoding/json",
    "fmt",
    "github.com/satori/go.uuid",
    "image",
    "image/gif",
    "image/jpeg",
    "image/png",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/modules",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/prouter",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
    "strconv",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("UserSystemInit", "", 31),
    ("user_handleUserRemove", "", 104),
    ("user_handleUserEdit", "", 192),
    ("user_getInterfaceInfo", "", 368),
    ("user_getProfilePic", "", 388),
    ("user_handleUserInfo", "", 444),
    ("user_handleList", "", 517),
    ("getUserIcon", "", 556),
    ("setUserIcon", "", 562),
];

pub async fn usersysteminit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "UserSystemInit" })
}

pub async fn user_handleuserremove(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_handleUserRemove" })
}

pub async fn user_handleuseredit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_handleUserEdit" })
}

pub async fn user_getinterfaceinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_getInterfaceInfo" })
}

pub async fn user_getprofilepic(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_getProfilePic" })
}

pub async fn user_handleuserinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_handleUserInfo" })
}

pub async fn user_handlelist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "user_handleList" })
}

pub async fn getusericon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "getUserIcon" })
}

pub async fn setusericon(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "user.go", function: "setUserIcon" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
