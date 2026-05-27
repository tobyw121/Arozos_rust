//! Rust ArOZ - AlpNAS Desktop
//! 
//! Eine vollständige Portierung des arozOS Projekts von Go nach Rust.
//! 
//! # Module
//! 
//! Dieses Projekt ist in Module unterteilt, die der originalen Go-Struktur entsprechen:
//! 
//! - `mod::agi` - AGI (Aroz Gateway Interface) für Skripting
//! - `mod::auth` - Authentifizierung und Benutzerverwaltung
//! - `mod::database` - Key-Value Datenbank-Abstraktion
//! - `mod::filesystem` - Dateisystem-Operationen
//! - `mod::network` - Netzwerk-Dienste (HTTP, FTP, WebDAV, etc.)
//! - `mod::storage` - Storage-Pool Management
//! - `mod::user` - Benutzer- und Gruppenverwaltung
//! - `mod::modules` - Modul-Loader und Registry

pub mod error;
pub mod network;
pub mod permission;
pub mod system;

pub mod mod {
    pub mod agi;
    pub mod apt;
    pub mod auth;
    pub mod cluster;
    pub mod compatibility;
    pub mod console;
    pub mod database;
    pub mod disk;
    pub mod fileservers;
    pub mod filesystem;
    pub mod info;
    pub mod iot;
    pub mod media;
    pub mod modules;
    pub mod network;
    pub mod notification;
    pub mod permission;
    pub mod prouter;
    pub mod quota;
    pub mod security;
    pub mod share;
    pub mod storage;
    pub mod subservice;
    pub mod time;
    pub mod updates;
    pub mod user;
    pub mod utils;
    pub mod www;
}

// Re-exports wichtiger Typen
pub use error::{ArozError, Result};
pub use system::SystemInfo;
