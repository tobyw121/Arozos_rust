//! Datenbank-Modul
//! 
//! Key-Value Datenbank-Abstraktion (ersetzt BoltDB aus dem Go-Projekt)
//! Verwendet sled als Embedded Database.

use std::sync::Arc;
use sled::{Db, Tree};
use serde::{Serialize, de::DeserializeOwned};
use crate::error::{Result, ArozError};

/// Datenbank-Wrapper für sled
pub struct Database {
    db: Db,
}

impl Database {
    /// Erstellt eine neue Datenbank oder öffnet eine bestehende
    pub async fn new(path: &str) -> Result<Self> {
        let db = sled::open(path)
            .map_err(|e| ArozError::Database(format!("Failed to open database: {}", e)))?;
        
        Ok(Database { db })
    }

    /// Erstellt eine neue Tabelle (Tree in sled)
    pub fn create_table(&self, table_name: &str) -> Result<()> {
        self.db.open_tree(table_name)
            .map_err(|e| ArozError::Database(format!("Failed to create table: {}", e)))?;
        Ok(())
    }

    /// Schreibt einen Wert in die Datenbank
    pub fn write<T: Serialize>(&self, table: &str, key: &str, value: &T) -> Result<()> {
        let tree = self.get_tree(table)?;
        let json_value = serde_json::to_vec(value)
            .map_err(|e| ArozError::Database(format!("Serialization error: {}", e)))?;
        
        tree.insert(key, json_value)
            .map_err(|e| ArozError::Database(format!("Write error: {}", e)))?;
        
        tree.flush()
            .map_err(|e| ArozError::Database(format!("Flush error: {}", e)))?;
        
        Ok(())
    }

    /// Liest einen Wert aus der Datenbank
    pub fn read<T: DeserializeOwned>(&self, table: &str, key: &str) -> Result<T> {
        let tree = self.get_tree(table)?;
        let value = tree.get(key)
            .map_err(|e| ArozError::Database(format!("Read error: {}", e)))?;
        
        match value {
            Some(data) => {
                let result: T = serde_json::from_slice(&data)
                    .map_err(|e| ArozError::Database(format!("Deserialization error: {}", e)))?;
                Ok(result)
            }
            None => Err(ArozError::NotFound(format!("Key '{}' not found in table '{}'", key, table))),
        }
    }

    /// Löscht einen Eintrag aus der Datenbank
    pub fn delete(&self, table: &str, key: &str) -> Result<()> {
        let tree = self.get_tree(table)?;
        tree.remove(key)
            .map_err(|e| ArozError::Database(format!("Delete error: {}", e)))?;
        Ok(())
    }

    /// Listet alle Einträge einer Tabelle auf
    pub fn list_table(&self, table: &str) -> Result<Vec<(String, Vec<u8>)>> {
        let tree = self.get_tree(table)?;
        let mut results = Vec::new();
        
        for item in tree.iter() {
            let (key, value) = item
                .map_err(|e| ArozError::Database(format!("Iteration error: {}", e)))?;
            
            let key_str = String::from_utf8_lossy(&key).to_string();
            results.push((key_str, value.to_vec()));
        }
        
        Ok(results)
    }

    /// Schließt die Datenbank
    pub fn close(&self) -> Result<()> {
        self.db.flush()
            .map_err(|e| ArozError::Database(format!("Flush on close error: {}", e)))?;
        drop(self.db.clone());
        Ok(())
    }

    /// Hilfsfunktion zum Holen eines Tree
    fn get_tree(&self, table: &str) -> Result<Tree> {
        self.db.open_tree(table)
            .map_err(|e| ArozError::Database(format!("Failed to open tree: {}", e)))
    }
}

impl Clone for Database {
    fn clone(&self) -> Self {
        Database {
            db: self.db.clone(),
        }
    }
}

// TODO: Weitere Datenbank-Funktionen portieren aus:
// - mod/database/database.go
// - mod/database/database_core.go
