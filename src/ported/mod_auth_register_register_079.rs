//! Original Go file: `mod/auth/register/register.go`
//! Package: `register`; LOC: 315; SHA256: `2c34335d3b7601f60f3c882db03a0ee55ac2499609d75ecbc72cb38f7eab519e`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/register/register.go", package: "register", go_loc: 315, functions: 14, types: 2, sha256: "2c34335d3b7601f60f3c882db03a0ee55ac2499609d75ecbc72cb38f7eab519e" };

pub const GO_IMPORTS: &[&str] = &[
    "bufio",
    "encoding/base64",
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/auth",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/permission",
    "imuslab.com/arozos/mod/utils",
    "io",
    "log",
    "net/http",
    "net/mail",
    "os",
    "strings",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("RegisterOptions", "struct", 28),
    ("RegisterHandler", "struct", 33),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewRegisterHandler", "", 42),
    ("createDefaultGroup", "", 80),
    ("HandleRegisterCheck", "h *RegisterHandler", 85),
    ("HandleRegisterInterface", "h *RegisterHandler", 94),
    ("readImageFileAsBase64", "", 116),
    ("GetDefaultUserGroup", "h *RegisterHandler", 132),
    ("SetDefaultUserGroup", "h *RegisterHandler", 137),
    ("SetAllowRegistry", "h *RegisterHandler", 152),
    ("CleanRegisters", "h *RegisterHandler", 157),
    ("ListAllUserEmails", "h *RegisterHandler", 174),
    ("HandleRegisterRequest", "h *RegisterHandler", 196),
    ("HandleEmailChange", "h *RegisterHandler", 274),
    ("GetUserEmail", "h *RegisterHandler", 302),
    ("isValidEmail", "", 312),
];

pub async fn newregisterhandler(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "NewRegisterHandler" })
}

pub async fn createdefaultgroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "createDefaultGroup" })
}

pub async fn registerhandler_handleregistercheck(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.HandleRegisterCheck" })
}

pub async fn registerhandler_handleregisterinterface(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.HandleRegisterInterface" })
}

pub async fn readimagefileasbase64(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "readImageFileAsBase64" })
}

pub async fn registerhandler_getdefaultusergroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.GetDefaultUserGroup" })
}

pub async fn registerhandler_setdefaultusergroup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.SetDefaultUserGroup" })
}

pub async fn registerhandler_setallowregistry(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.SetAllowRegistry" })
}

pub async fn registerhandler_cleanregisters(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.CleanRegisters" })
}

pub async fn registerhandler_listalluseremails(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.ListAllUserEmails" })
}

pub async fn registerhandler_handleregisterrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.HandleRegisterRequest" })
}

pub async fn registerhandler_handleemailchange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.HandleEmailChange" })
}

pub async fn registerhandler_getuseremail(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "RegisterHandler.GetUserEmail" })
}

pub async fn isvalidemail(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/register/register.go", function: "isValidEmail" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
