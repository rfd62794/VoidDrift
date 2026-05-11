use std::path::PathBuf;
use serde_json;
use bevy::prelude::*;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

use super::schema::{SaveData, SaveCategory};
use super::SAVE_VERSION;

// File path functions
pub fn get_save_base_dir() -> PathBuf {
    #[cfg(target_os = "android")]
    {
        // Android app data directory
        PathBuf::from("/data/data/com.rfditservices.voidrift/files")
    }
    #[cfg(not(target_os = "android"))]
    {
        // Desktop
        std::env::current_dir().unwrap_or_default()
    }
}

pub fn save_dir(category: &SaveCategory) -> PathBuf {
    let base = get_save_base_dir().join("saves");
    match category {
        SaveCategory::Play => base.join("play"),
        SaveCategory::Stage => base.join("stage"),
        SaveCategory::Auto => base,
    }
}

pub fn autosave_path() -> PathBuf {
    get_save_base_dir().join("saves/autosave.json")
}

// Save and load functions
pub fn save_game(data: &SaveData) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let key = if data.save_category == SaveCategory::Auto {
            "voidrift_autosave".to_string()
        } else {
            let filename = sanitize_filename(&data.save_name);
            format!("voidrift_save_{filename}")
        };

        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Serialization failed: {e}"))?;

        LocalStorage::set(&key, json)
            .map_err(|e| format!("LocalStorage write failed: {e}"))?;

        // Update save index
        let index_key = "voidrift_save_index";
        let mut keys: Vec<String> = LocalStorage::get(index_key).unwrap_or_default();
        if !keys.contains(&key) {
            keys.push(key);
            LocalStorage::set(index_key, keys)
                .map_err(|e| format!("Failed to update save index: {e}"))?;
        }

        Ok(())
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let path = if data.save_category == SaveCategory::Auto {
            let p = autosave_path();
            if let Some(parent) = p.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create autosave dir: {e}"))?;
            }
            p
        } else {
            let dir = save_dir(&data.save_category);
            std::fs::create_dir_all(&dir)
                .map_err(|e| format!("Failed to create save dir: {e}"))?;
            let filename = sanitize_filename(&data.save_name);
            dir.join(format!("{filename}.json"))
        };

        let json = serde_json::to_string_pretty(data)
            .map_err(|e| format!("Serialization failed: {e}"))?;

        std::fs::write(&path, json)
            .map_err(|e| format!("Write failed: {e}"))?;

        Ok(())
    }
}

pub fn load_game(path: &PathBuf) -> Result<SaveData, String> {
    #[cfg(target_arch = "wasm32")]
    {
        // For WASM, path is actually a key name in localStorage
        let key = path.to_str().unwrap_or("");
        let json: String = LocalStorage::get(key)
            .map_err(|e| format!("LocalStorage read failed: {e}"))?;

        let mut data: SaveData = serde_json::from_str(&json)
            .map_err(|e| format!("Deserialization failed: {e}"))?;

        // Migration: add telemetry_consent for old saves
        if data.save_version < 5 && data.telemetry_consent.is_none() {
            data.telemetry_consent = None; // Explicitly set to None for new field
        }

        // Migration: add telemetry_sessions for old saves
        if data.save_version < 6 {
            data.telemetry_sessions = 0;
        }

        // Migration: add unlocked_logs for old saves
        if data.save_version < 7 {
            data.unlocked_logs = Vec::new();
        }

        if data.save_version != SAVE_VERSION {
            return Ok(data);
        }

        Ok(data)
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let json = std::fs::read_to_string(path)
            .map_err(|e| format!("Read failed: {e}"))?;

        let mut data: SaveData = serde_json::from_str(&json)
            .map_err(|e| format!("Deserialization failed: {e}"))?;

        // Migration: add telemetry_consent for old saves
        if data.save_version < 5 && data.telemetry_consent.is_none() {
            data.telemetry_consent = None; // Explicitly set to None for new field
        }

        // Migration: add telemetry_sessions for old saves
        if data.save_version < 6 {
            data.telemetry_sessions = 0;
        }

        // Migration: add unlocked_logs for old saves
        if data.save_version < 7 {
            data.unlocked_logs = Vec::new();
        }

        if data.save_version != SAVE_VERSION {
            // Return data anyway but caller can show version warning
            return Ok(data);
        }

        Ok(data)
    }
}

pub fn list_saves(category: &SaveCategory) -> Vec<SaveData> {
    #[cfg(target_arch = "wasm32")]
    {
        // gloo-storage 0.4 doesn't provide a keys() method
        // For WASM, we maintain an index of save keys
        let index_key = "voidrift_save_index";
        let keys: Vec<String> = LocalStorage::get(index_key).unwrap_or_default();

        let prefix = if *category == SaveCategory::Auto {
            "voidrift_autosave".to_string()
        } else {
            "voidrift_save_".to_string()
        };

        let mut saves: Vec<SaveData> = keys
            .iter()
            .filter(|k| k.starts_with(&prefix))
            .filter_map(|key| {
                let path = PathBuf::from(key);
                load_game(&path).ok()
            })
            .collect();

        // Sort by timestamp descending - newest first
        saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        saves
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let dir = save_dir(category);
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return vec![];
        };

        let mut saves: Vec<SaveData> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            .filter_map(|e| load_game(&e.path()).ok())
            .collect();

        // Sort by timestamp descending - newest first
        saves.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        saves
    }
}

pub fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect()
}

pub fn current_timestamp(time: &Res<Time>) -> String {
    let secs = time.elapsed().as_secs();
    format!("{secs}")
}
