//! System-Informationen und Hardware-Erkennung
//! 
//! Entspricht den system.go und system.info.go im Originalprojekt.

use serde::{Serialize, Deserialize};

/// System-Informationen
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os: String,
    pub arch: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub available_memory: u64,
    pub disk_space_total: u64,
    pub disk_space_used: u64,
    pub uptime_seconds: u64,
}

impl SystemInfo {
    /// Erstellt neue System-Informationen
    pub fn new() -> Self {
        SystemInfo {
            hostname: String::new(),
            os: String::new(),
            arch: String::new(),
            cpu_cores: 0,
            total_memory: 0,
            available_memory: 0,
            disk_space_total: 0,
            disk_space_used: 0,
            uptime_seconds: 0,
        }
    }

    /// Aktualisiert die System-Informationen
    pub fn refresh(&mut self) {
        // TODO: Echte System-Informationen sammeln mit sysinfo crate
        
        self.hostname = whoami::hostname();
        self.os = std::env::consts::OS.to_string();
        self.arch = std::env::consts::ARCH.to_string();
        
        // CPU Cores
        self.cpu_cores = num_cpus::get();
        
        // Memory und Disk Space würden hier mit sysinfo gesammelt werden
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

// TODO: Weitere System-Funktionen portieren aus:
// - mod/info/hardwareinfo/
// - mod/info/usageinfo/
