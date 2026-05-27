//! Rust ArOZ - AlpNAS Desktop
//! 
//! Eine vollständige Portierung des arozOS Projekts von Go nach Rust.
//! 
//! # Module
//! 
//! Dieses Projekt ist in Module unterteilt, die der originalen Go-Struktur entsprechen:
//! 
//! - `modules_core::agi` - AGI (Aroz Gateway Interface) für Skripting
//! - `modules_core::auth` - Authentifizierung und Benutzerverwaltung
//! - `modules_core::database` - Key-Value Datenbank-Abstraktion
//! - `modules_core::filesystem` - Dateisystem-Operationen
//! - `modules_core::network` - Netzwerk-Dienste (HTTP, FTP, WebDAV, etc.)
//! - `modules_core::storage` - Storage-Pool Management
//! - `modules_core::user` - Benutzer- und Gruppenverwaltung
//! - `modules_core::modules` - Modul-Loader und Registry

pub mod error;
pub mod network;
pub mod permission;
pub mod system;

// Core modules (ehemals "mod" - umbenannt da "mod" ein reserviertes Keyword ist)
pub mod modules_core {
    pub mod agi;
    pub mod database;
    pub mod modules;
    pub mod user;
    
    // Placeholder für weitere Module (können bei Bedarf implementiert werden)
    // pub mod apt;
    // pub mod auth;
    // pub mod cluster;
    // pub mod compatibility;
    // pub mod console;
    // pub mod disk;
    // pub mod fileservers;
    // pub mod filesystem;
    // pub mod info;
    // pub mod iot;
    // pub mod media;
    // pub mod network;
    // pub mod notification;
    // pub mod permission;
    // pub mod prouter;
    // pub mod quota;
    // pub mod security;
    // pub mod share;
    // pub mod storage;
    // pub mod subservice;
    // pub mod time;
    // pub mod updates;
    // pub mod utils;
    // pub mod www;
}

// Re-exports wichtiger Typen
pub use error::{ArozError, Result};
pub use system::SystemInfo;
