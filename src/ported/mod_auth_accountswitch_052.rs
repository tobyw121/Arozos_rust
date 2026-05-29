//! Original Go file: `mod/auth/accountSwitch.go`
//! Package: `auth`; LOC: 546; SHA256: `35e483475237217571efdf78280c8f3ea3c91c5b383a8be61c28596f9c4a4f5b`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/auth/accountSwitch.go", package: "auth", go_loc: 546, functions: 25, types: 3, sha256: "35e483475237217571efdf78280c8f3ea3c91c5b383a8be61c28596f9c4a4f5b" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "fmt",
    "github.com/gorilla/sessions",
    "github.com/satori/go.uuid",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/http",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SwitchableAccount", "struct", 32),
    ("SwitchableAccountsPool", "struct", 37),
    ("SwitchableAccountPoolManager", "struct", 44),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSwitchableAccountPoolManager", "", 53),
    ("RunNightlyCleanup", "m *SwitchableAccountPoolManager", 76),
    ("HandleSwitchableAccountListing", "m *SwitchableAccountPoolManager", 89),
    ("GetUnauthedSwitchableAccountCreatorList", "m *SwitchableAccountPoolManager", 142),
    ("HandleLogoutforUser", "m *SwitchableAccountPoolManager", 164),
    ("HandleLogoutAllAccounts", "m *SwitchableAccountPoolManager", 200),
    ("HandleAccountSwitch", "m *SwitchableAccountPoolManager", 236),
    ("GetAllPools", "m *SwitchableAccountPoolManager", 335),
    ("MatchPoolCreatorOrResetPoolID", "m *SwitchableAccountPoolManager", 359),
    ("GetPoolByID", "m *SwitchableAccountPoolManager", 386),
    ("RemoveUserFromAllSwitchableAccountPool", "p *SwitchableAccountPoolManager", 397),
    ("ExpireUserFromAllSwitchableAccountPool", "p *SwitchableAccountPoolManager", 411),
    ("IsAccessibleByRequest", "p *SwitchableAccountsPool", 431),
    ("IsAccessibleBy", "p *SwitchableAccountsPool", 440),
    ("UserAlreadyInPool", "p *SwitchableAccountsPool", 449),
    ("UpdateUserLastSwitchTime", "p *SwitchableAccountsPool", 458),
    ("GetLastSwitchTimeFromUsername", "p *SwitchableAccountsPool", 467),
    ("UpdateUserPoolAccountInfo", "p *SwitchableAccountsPool", 478),
    ("ExpireUser", "p *SwitchableAccountsPool", 490),
    ("RemoveUser", "p *SwitchableAccountsPool", 500),
    ("DeletePoolIfAllUserSessionExpired", "p *SwitchableAccountsPool", 513),
    ("Save", "p *SwitchableAccountsPool", 528),
    ("Delete", "p *SwitchableAccountsPool", 533),
    ("IsAccountExpired", "p *SwitchableAccountsPool", 538),
    ("unsetPoolidFromSession", "", 542),
];

pub async fn newswitchableaccountpoolmanager(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "NewSwitchableAccountPoolManager" })
}

pub async fn switchableaccountpoolmanager_runnightlycleanup(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.RunNightlyCleanup" })
}

pub async fn switchableaccountpoolmanager_handleswitchableaccountlisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.HandleSwitchableAccountListing" })
}

pub async fn switchableaccountpoolmanager_getunauthedswitchableaccountcreatorlist(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.GetUnauthedSwitchableAccountCreatorList" })
}

pub async fn switchableaccountpoolmanager_handlelogoutforuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.HandleLogoutforUser" })
}

pub async fn switchableaccountpoolmanager_handlelogoutallaccounts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.HandleLogoutAllAccounts" })
}

pub async fn switchableaccountpoolmanager_handleaccountswitch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.HandleAccountSwitch" })
}

pub async fn switchableaccountpoolmanager_getallpools(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.GetAllPools" })
}

pub async fn switchableaccountpoolmanager_matchpoolcreatororresetpoolid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.MatchPoolCreatorOrResetPoolID" })
}

pub async fn switchableaccountpoolmanager_getpoolbyid(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.GetPoolByID" })
}

pub async fn switchableaccountpoolmanager_removeuserfromallswitchableaccountpool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.RemoveUserFromAllSwitchableAccountPool" })
}

pub async fn switchableaccountpoolmanager_expireuserfromallswitchableaccountpool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountPoolManager.ExpireUserFromAllSwitchableAccountPool" })
}

pub async fn switchableaccountspool_isaccessiblebyrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.IsAccessibleByRequest" })
}

pub async fn switchableaccountspool_isaccessibleby(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.IsAccessibleBy" })
}

pub async fn switchableaccountspool_useralreadyinpool(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.UserAlreadyInPool" })
}

pub async fn switchableaccountspool_updateuserlastswitchtime(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.UpdateUserLastSwitchTime" })
}

pub async fn switchableaccountspool_getlastswitchtimefromusername(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.GetLastSwitchTimeFromUsername" })
}

pub async fn switchableaccountspool_updateuserpoolaccountinfo(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.UpdateUserPoolAccountInfo" })
}

pub async fn switchableaccountspool_expireuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.ExpireUser" })
}

pub async fn switchableaccountspool_removeuser(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.RemoveUser" })
}

pub async fn switchableaccountspool_deletepoolifallusersessionexpired(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.DeletePoolIfAllUserSessionExpired" })
}

pub async fn switchableaccountspool_save(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.Save" })
}

pub async fn switchableaccountspool_delete(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.Delete" })
}

pub async fn switchableaccountspool_isaccountexpired(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "SwitchableAccountsPool.IsAccountExpired" })
}

pub async fn unsetpoolidfromsession(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/auth/accountSwitch.go", function: "unsetPoolidFromSession" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
