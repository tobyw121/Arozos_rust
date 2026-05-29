//! Original Go file: `mod/notification/agents/smtpn/smtpn.go`
//! Package: `smtpn`; LOC: 138; SHA256: `8b3e478d2ea06a54fe2a5bd02312d71f3ced1366d81bbd809f947b6c1a9a2e0d`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/notification/agents/smtpn/smtpn.go", package: "smtpn", go_loc: 138, functions: 8, types: 1, sha256: "8b3e478d2ea06a54fe2a5bd02312d71f3ced1366d81bbd809f947b6c1a9a2e0d" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "errors",
    "imuslab.com/arozos/mod/notification",
    "imuslab.com/arozos/mod/utils",
    "log",
    "net/smtp",
    "os",
    "strconv",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Agent", "struct", 24),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewSMTPNotificationAgent", "", 34),
    ("GenerateEmptyConfigFile", "", 55),
    ("Name", "a Agent", 69),
    ("Desc", "a Agent", 73),
    ("IsConsumer", "a Agent", 77),
    ("IsProducer", "a Agent", 81),
    ("ConsumerNotification", "a Agent", 85),
    ("ProduceNotification", "a Agent", 136),
];

pub async fn newsmtpnotificationagent(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "NewSMTPNotificationAgent" })
}

pub async fn generateemptyconfigfile(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "GenerateEmptyConfigFile" })
}

pub async fn agent_name(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.Name" })
}

pub async fn agent_desc(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.Desc" })
}

pub async fn agent_isconsumer(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.IsConsumer" })
}

pub async fn agent_isproducer(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.IsProducer" })
}

pub async fn agent_consumernotification(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.ConsumerNotification" })
}

pub async fn agent_producenotification(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/notification/agents/smtpn/smtpn.go", function: "Agent.ProduceNotification" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
