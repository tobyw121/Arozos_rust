//! Module Handler - Modulverwaltung und Registrierung
//! 
//! Entspricht mod/modules/module.go im Originalprojekt.

use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use crate::error::{Result, ArozError};

/// Informationen über ein Modul
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Name des Moduls, z.B. "Audio"
    pub name: String,
    /// Beschreibung des Moduls
    pub desc: String,
    /// Gruppe des Moduls, z.B. "system", "media"
    pub group: String,
    /// Pfad zum Modul-Icon
    pub icon_path: String,
    /// Version des Moduls
    pub version: String,
    /// Standard-Startverzeichnis, z.B. "Audio/index.html"
    pub start_dir: String,
    /// Unterstützt FloatWindow
    pub support_fw: bool,
    /// Launch-Verzeichnis für FloatWindow-Modus
    pub launch_fw_dir: String,
    /// Unterstützt Embedded-Modus
    pub support_emb: bool,
    /// Launch-Link für Embedded-Modus
    pub launch_emb: String,
    /// Initiale FloatWindow-Größe [Breite, Höhe]
    pub init_fw_size: Vec<i32>,
    /// Initiale Embedded-Größe [Breite, Höhe]
    pub init_emb_size: Vec<i32>,
    /// Unterstützte Datei-Erweiterungen
    pub supported_ext: Vec<String>,
    
    // Interne Eigenschaften (nicht serialisiert)
    #[serde(skip)]
    pub allow_reload: bool,
}

/// Handler für Modul-Verwaltung
pub struct ModuleHandler {
    loaded_modules: Arc<RwLock<Vec<ModuleInfo>>>,
    user_handler: Arc<crate::modules_core::user::UserHandler>,
    tmp_directory: String,
}

impl ModuleHandler {
    /// Erstellt einen neuen ModuleHandler
    pub fn new(
        user_handler: Arc<crate::modules_core::user::UserHandler>,
        tmp_directory: &str,
    ) -> Self {
        ModuleHandler {
            loaded_modules: Arc::new(RwLock::new(Vec::new())),
            user_handler,
            tmp_directory: tmp_directory.to_string(),
        }
    }

    /// Registriert ein neues Modul
    pub fn register_module(&self, module: ModuleInfo) -> Result<()> {
        let mut modules = self.loaded_modules.write()
            .map_err(|_| ArozError::Module("Failed to acquire write lock".to_string()))?;
        
        modules.push(module);
        Ok(())
    }

    /// Registriert ein Modul aus JSON-String
    pub fn register_module_from_json(&self, json_string: &str, allow_reload: bool) -> Result<()> {
        let mut module: ModuleInfo = serde_json::from_str(json_string)
            .map_err(|e| ArozError::Module(format!("Invalid JSON: {}", e)))?;
        
        module.allow_reload = allow_reload;
        self.register_module(module)
    }

    /// Entfernt ein Modul nach Namen
    pub fn deregister_module(&self, module_name: &str) -> Result<()> {
        let mut modules = self.loaded_modules.write()
            .map_err(|_| ArozError::Module("Failed to acquire write lock".to_string()))?;
        
        modules.retain(|m| m.name != module_name);
        Ok(())
    }

    /// Gibt eine Liste aller Modul-Namen zurück
    pub fn get_module_name_list(&self) -> Result<Vec<String>> {
        let modules = self.loaded_modules.read()
            .map_err(|_| ArozError::Module("Failed to acquire read lock".to_string()))?;
        
        Ok(modules.iter().map(|m| m.name.clone()).collect())
    }

    /// Listet alle geladenen Module auf (für API)
    pub fn list_loaded_modules(&self) -> Vec<ModuleInfo> {
        match self.loaded_modules.read() {
            Ok(modules) => modules.clone(),
            Err(_) => Vec::new(),
        }
    }

    /// Holt Modul-Informationen nach ID/Name
    pub fn get_module_info_by_id(&self, module_id: &str) -> Option<ModuleInfo> {
        match self.loaded_modules.read() {
            Ok(modules) => modules.iter()
                .find(|m| m.name == module_id)
                .cloned(),
            Err(_) => None,
        }
    }

    /// Sortiert die Modul-Liste alphabetisch
    pub fn module_sort_list(&self) -> Result<()> {
        let mut modules = self.loaded_modules.write()
            .map_err(|_| ArozError::Module("Failed to acquire write lock".to_string()))?;
        
        modules.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(())
    }
}

// TODO: Globale Initialisierungsfunktion muss noch implementiert werden
// pub fn module_service_init(state: &crate::startup::AppState) -> Result<()> {
//     // Datenbank-Tabelle für Module erstellen
//     state.database.create_table("module")?;
//     
//     Ok(())
// }

// TODO: Weitere Modul-Funktionen portieren:
// - HandleDefaultLauncher
// - GetLaunchParameter  
// - InstallViaZip
// - InstallModuleViaGit
// - UninstallModule
