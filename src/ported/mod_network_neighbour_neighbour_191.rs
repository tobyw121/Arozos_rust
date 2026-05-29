//! Original Go file: `mod/network/neighbour/neighbour.go`
//! Package: `neighbour`; LOC: 185; SHA256: `b3c01c5c0ea181cd71a538f16bf1e9f9795ced076b6bb4d2cf090860f60889b7`

use crate::ported::{LegacyContext, LegacyModuleStatus, LegacyPortError};

pub const STATUS: LegacyModuleStatus = LegacyModuleStatus { original_path: "mod/network/neighbour/neighbour.go", package: "neighbour", go_loc: 185, functions: 8, types: 2, sha256: "b3c01c5c0ea181cd71a538f16bf1e9f9795ced076b6bb4d2cf090860f60889b7" };

pub const GO_IMPORTS: &[&str] = &[
    "encoding/json",
    "imuslab.com/arozos/mod/cluster/wakeonlan",
    "imuslab.com/arozos/mod/database",
    "imuslab.com/arozos/mod/network/mdns",
    "log",
    "time",
];

pub const GO_TYPES: &[(&str, &str, usize)] = &[
    ("Discoverer", "struct", 23),
    ("HostRecord", "struct", 32),
];

pub const GO_FUNCTIONS: &[(&str, &str, usize)] = &[
    ("NewDiscoverer", "", 43),
    ("GetNearbyHosts", "d *Discoverer", 56),
    ("StartScanning", "d *Discoverer", 66),
    ("UpdateScan", "d *Discoverer", 100),
    ("GetOfflineHosts", "d *Discoverer", 124),
    ("SendWakeOnLan", "d *Discoverer", 161),
    ("ScannerRunning", "d *Discoverer", 165),
    ("StopScanning", "d *Discoverer", 173),
];

pub async fn newdiscoverer(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "NewDiscoverer" })
}

pub async fn discoverer_getnearbyhosts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.GetNearbyHosts" })
}

pub async fn discoverer_startscanning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.StartScanning" })
}

pub async fn discoverer_updatescan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.UpdateScan" })
}

pub async fn discoverer_getofflinehosts(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.GetOfflineHosts" })
}

pub async fn discoverer_sendwakeonlan(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.SendWakeOnLan" })
}

pub async fn discoverer_scannerrunning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.ScannerRunning" })
}

pub async fn discoverer_stopscanning(_ctx: &LegacyContext) -> Result<(), LegacyPortError> {
    Err(LegacyPortError::NotYetPorted { file: "mod/network/neighbour/neighbour.go", function: "Discoverer.StopScanning" })
}

pub fn migration_status() -> LegacyModuleStatus { STATUS }
