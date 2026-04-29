use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::models::{Area, Density, Task, ThemeMode};

const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
    #[serde(default = "default_version")]
    pub schema_version: u32,
    pub tasks: Vec<Task>,
    pub areas: Vec<Area>,
    pub theme_mode: ThemeMode,
    pub density: Density,
    pub collapsed_areas: Vec<String>,
}

fn default_version() -> u32 {
    SCHEMA_VERSION
}

pub fn store_path() -> Option<PathBuf> {
    dirs::config_dir().map(|p| p.join("icedgtd").join("store.json"))
}

pub async fn load() -> Result<Store, String> {
    let path = store_path().ok_or_else(|| "no config dir".to_string())?;
    let bytes = tokio::fs::read(&path)
        .await
        .map_err(|e| format!("read {}: {}", path.display(), e))?;
    let store: Store = serde_json::from_slice(&bytes).map_err(|e| format!("parse: {}", e))?;
    Ok(store)
}

pub async fn save(snapshot: Store) -> Result<(), String> {
    let path = store_path().ok_or_else(|| "no config dir".to_string())?;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|e| format!("mkdir {}: {}", parent.display(), e))?;
    }
    let bytes = serde_json::to_vec_pretty(&snapshot).map_err(|e| format!("serialize: {}", e))?;
    tokio::fs::write(&path, bytes)
        .await
        .map_err(|e| format!("write {}: {}", path.display(), e))?;
    Ok(())
}
