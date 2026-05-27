//! User-Handler - Benutzerverwaltung
//! 
//! Entspricht mod/user/user.go im Originalprojekt.

use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{Result, ArozError};
use crate::modules_core::database::Database;

/// Benutzer-Informationen
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub is_admin: bool,
    pub groups: Vec<String>,
    pub module_permissions: Vec<String>,
    pub quota_bytes: u64,
    pub used_bytes: u64,
}

impl UserInfo {
    /// Prüft ob der Benutzer Admin ist
    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    /// Prüft Berechtigung für ein Modul
    pub fn get_module_access_permission(&self, module_name: &str) -> bool {
        // Admin hat immer Zugriff
        if self.is_admin {
            return true;
        }
        
        // Prüfe explizite Berechtigungen
        self.module_permissions.iter().any(|m| m == module_name)
    }
}

/// Handler für Benutzerverwaltung
pub struct UserHandler {
    database: Arc<Database>,
    tmp_directory: String,
    universal_modules: Arc<RwLock<Vec<String>>>,
    // session_cache: Arc<RwLock<HashMap<String, UserInfo>>>,
}

impl UserHandler {
    /// Erstellt einen neuen UserHandler
    pub async fn new(database: Arc<Database>, tmp_directory: &str) -> Result<Self> {
        // Erstelle notwendige Datenbank-Tabellen
        database.create_table("users")?;
        database.create_table("groups")?;
        database.create_table("sessions")?;
        
        Ok(UserHandler {
            database,
            tmp_directory: tmp_directory.to_string(),
            universal_modules: Arc::new(RwLock::new(Vec::new())),
        })
    }

    /// Holt Benutzer-Informationen aus einer Anfrage
    pub fn get_user_info_from_request(
        &self,
        _w: &axum::http::Response<axum::body::Body>,
        _r: &axum::http::Request<axum::body::Body>,
    ) -> Result<UserInfo> {
        // TODO: Session-Token aus Request extrahieren und User laden
        
        // Placeholder für jetzt
        Err(ArozError::Auth("Authentication not implemented yet".to_string()))
    }

    /// Holt den Auth-Agent
    pub fn get_auth_agent(&self) -> &Self {
        self
    }

    /// Holt den Benutzernamen aus einem Request
    pub fn get_user_name_from_request(
        &self,
        _w: &axum::http::Response<axum::body::Body>,
        _r: &axum::http::Request<axum::body::Body>,
    ) -> Result<String> {
        // TODO: Implementierung
        Err(ArozError::Auth("Authentication not implemented yet".to_string()))
    }

    /// Holt die Datenbank-Referenz
    pub fn get_database(&self) -> Arc<Database> {
        self.database.clone()
    }

    /// Erstellt einen neuen Benutzer
    pub fn create_user(&self, username: &str, password: &str, email: &str) -> Result<()> {
        // TODO: Passwort hashen und User speichern
        
        let user = UserInfo {
            username: username.to_string(),
            email: email.to_string(),
            is_admin: false,
            groups: Vec::new(),
            module_permissions: Vec::new(),
            quota_bytes: 0,
            used_bytes: 0,
        };
        
        self.database.write("users", username, &user)
    }

    /// Holt Benutzer-Informationen nach Name
    pub fn get_user(&self, username: &str) -> Result<UserInfo> {
        self.database.read("users", username)
    }

    /// Löscht einen Benutzer
    pub fn delete_user(&self, username: &str) -> Result<()> {
        self.database.delete("users", username)
    }

    /// Listet alle Benutzer auf
    pub fn list_users(&self) -> Result<Vec<UserInfo>> {
        let entries = self.database.list_table("users")?;
        let mut users = Vec::new();
        
        for (_, value) in entries {
            if let Ok(user) = serde_json::from_slice::<UserInfo>(&value) {
                users.push(user);
            }
        }
        
        Ok(users)
    }
}

// TODO: Weitere User-Funktionen portieren aus:
// - mod/user/user.go
// - mod/user/permissionHandler.go
// - mod/user/quota.go
// - mod/user/directoryHandler.go
