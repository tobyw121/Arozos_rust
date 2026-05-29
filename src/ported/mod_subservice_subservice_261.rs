//! Original Go file: `mod/subservice/subservice.go`
//! Package: `subservice`; LOC: 610; SHA256: `ee7c4618c3786ff0ffd5a0eb7beef5da8754878b2e5248906785069cfa0f4fbf`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/subservice/subservice.go", package: "subservice", go_loc: 610, functions: 16, types: 2, sha256: "ee7c4618c3786ff0ffd5a0eb7beef5da8754878b2e5248906785069cfa0f4fbf" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/info/logger",
    "imuslab.com/arozos/mod/modules",
    "imuslab.com/arozos/mod/network/reverseproxy",
    "imuslab.com/arozos/mod/network/websocketproxy",
    "imuslab.com/arozos/mod/user",
    "log",
    "net/http",
    "net/url",
    "os",
    "os/exec",
    "path/filepath",
    "runtime",
    "sort",
    "strconv",
    "strings",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("SubService", "struct", 33),
    ("SubServiceRouter", "struct", 43),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSubServiceRouter", "", 54),
    ("LoadSubservicesFromRootPath", "sr *SubServiceRouter", 71),
    ("Launch", "sr *SubServiceRouter", 88),
    ("HandleListing", "sr *SubServiceRouter", 316),
    ("HandleKillSubService", "sr *SubServiceRouter", 371),
    ("HandleStartSubService", "sr *SubServiceRouter", 392),
    ("CheckUserPermissionOnSubservice", "sr *SubServiceRouter", 413),
    ("CheckIfReverseProxyPath", "sr *SubServiceRouter", 419),
    ("Close", "sr *SubServiceRouter", 447),
    ("KillSubService", "sr *SubServiceRouter", 466),
    ("StartSubService", "sr *SubServiceRouter", 516),
    ("GetSubserviceRoot", "sr *SubServiceRouter", 539),
    ("GetNextUsablePort", "sr *SubServiceRouter", 549),
    ("CheckIfPortInUse", "sr *SubServiceRouter", 557),
    ("HandleRoutingRequest", "sr *SubServiceRouter", 566),
    ("RestartSubService", "sr *SubServiceRouter", 601),
];

pub async fn newsubservicerouter(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "NewSubServiceRouter" })
}

pub async fn subservicerouter_loadsubservicesfromrootpath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.LoadSubservicesFromRootPath" })
}

pub async fn subservicerouter_launch(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.Launch" })
}

pub async fn subservicerouter_handlelisting(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.HandleListing" })
}

pub async fn subservicerouter_handlekillsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.HandleKillSubService" })
}

pub async fn subservicerouter_handlestartsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.HandleStartSubService" })
}

pub async fn subservicerouter_checkuserpermissiononsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.CheckUserPermissionOnSubservice" })
}

pub async fn subservicerouter_checkifreverseproxypath(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.CheckIfReverseProxyPath" })
}

pub async fn subservicerouter_close(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.Close" })
}

pub async fn subservicerouter_killsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.KillSubService" })
}

pub async fn subservicerouter_startsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.StartSubService" })
}

pub async fn subservicerouter_getsubserviceroot(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.GetSubserviceRoot" })
}

pub async fn subservicerouter_getnextusableport(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.GetNextUsablePort" })
}

pub async fn subservicerouter_checkifportinuse(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.CheckIfPortInUse" })
}

pub async fn subservicerouter_handleroutingrequest(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.HandleRoutingRequest" })
}

pub async fn subservicerouter_restartsubservice(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/subservice/subservice.go", function: "SubServiceRouter.RestartSubService" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
