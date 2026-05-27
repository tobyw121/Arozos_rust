//! Fehlerbehandlung für Rust ArOZ
//! 
//! Dieses Modul definiert alle Fehlerarten, die im System auftreten können.

use std::fmt;
use thiserror::Error;

/// Haupt-Fehler-Enum für alle ArozOS-Fehler
#[derive(Error, Debug)]
pub enum ArozError {
    #[error("Allgemeiner Fehler: {0}")]
    General(String),
    
    #[error("IO-Fehler: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON-Fehler: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Datenbank-Fehler: {0}")]
    Database(String),
    
    #[error("Authentifizierungs-Fehler: {0}")]
    Auth(String),
    
    #[error("Berechtigungs-Fehler: {1}")]
    PermissionDenied(String),
    
    #[error("Modul-Fehler: {0}")]
    Module(String),
    
    #[error("Netzwerk-Fehler: {0}")]
    Network(String),
    
    #[error("Storage-Fehler: {0}")]
    Storage(String),
    
    #[error("Dateisystem-Fehler: {0}")]
    FileSystem(String),
    
    #[error("Ungültige Anfrage: {0}")]
    BadRequest(String),
    
    #[error("Nicht gefunden: {0}")]
    NotFound(String),
    
    #[error("Interner Server-Fehler: {0}")]
    InternalServerError(String),
}

/// Ergebnis-Typ mit ArozError
pub type Result<T> = std::result::Result<T, ArozError>;

impl From<String> for ArozError {
    fn from(err: String) -> Self {
        ArozError::General(err)
    }
}

impl From<&str> for ArozError {
    fn from(err: &str) -> Self {
        ArozError::General(err.to_string())
    }
}

// Implementierung für fmt::Display ist durch Error-Makro automatisch vorhanden
