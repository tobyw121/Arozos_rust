//! Original Go file: `mod/fileservers/servers/webdavserv/webdavserv.go`
//! Package: `webdavserv`; LOC: 120; SHA256: `7f4926bbe17b1eae782997249cc482b80f0bc2207d4cf22e6d5818f4091970f2`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/fileservers/servers/webdavserv/webdavserv.go", package: "webdavserv", go_loc: 120, functions: 9, types: 2, sha256: "7f4926bbe17b1eae782997249cc482b80f0bc2207d4cf22e6d5818f4091970f2" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/fileservers",
    "imuslab.com/arozos/mod/storage/webdav",
    "imuslab.com/arozos/mod/user",
    "imuslab.com/arozos/mod/utils",
    "net/http",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("ManagerOption", "struct", 17),
    ("Manager", "struct", 25),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewWebDAVManager", "", 31),
    ("HandleStatusChange", "m *Manager", 53),
    ("WebDavToogle", "m *Manager", 80),
    ("WebDavGetEndpoints", "m *Manager", 86),
    ("GetWebDavEnabled", "m *Manager", 102),
    ("HandleConnectionList", "m *Manager", 107),
    ("HandlePermissionEdit", "m *Manager", 111),
    ("HandleClearAllPending", "m *Manager", 115),
    ("HandleRequest", "m *Manager", 118),
];

pub async fn newwebdavmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "NewWebDAVManager" })
}

pub async fn manager_handlestatuschange(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.HandleStatusChange" })
}

pub async fn manager_webdavtoogle(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.WebDavToogle" })
}

pub async fn manager_webdavgetendpoints(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.WebDavGetEndpoints" })
}

pub async fn manager_getwebdavenabled(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.GetWebDavEnabled" })
}

pub async fn manager_handleconnectionlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.HandleConnectionList" })
}

pub async fn manager_handlepermissionedit(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.HandlePermissionEdit" })
}

pub async fn manager_handleclearallpending(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.HandleClearAllPending" })
}

pub async fn manager_handlerequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/fileservers/servers/webdavserv/webdavserv.go", function: "Manager.HandleRequest" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
