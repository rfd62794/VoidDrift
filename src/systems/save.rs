use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::components::*;

pub const SAVE_VERSION: u32 = 3;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SaveData {
    // Meta
    pub save_version: u32,
    pub save_name: String,
    pub save_category: SaveCategory,
    pub timestamp: String,
    pub description: String,          // developer note for stage saves

    // Opening state
    pub opening_complete: bool,
    pub opening_phase: String,        // serialized phase name

    // Station state
    pub station_online: bool,
    pub station_power: f32,
    pub power_cells: u32,
    pub magnetite: f32,
    pub carbon: f32,
    pub hull_plates: u32,
    pub ship_hulls: u32,
    pub ai_cores: u32,
    pub repair_progress: f32,

    // Tabs unlocked
    pub tab_power: bool,
    pub tab_cargo: bool,
    pub tab_refinery: bool,
    pub tab_foundry: bool,
    pub tab_hangar: bool,

    // Fleet state
    pub drone_count: u8,
    pub drones: Vec<DroneSaveData>,

    // World state
    pub sectors_discovered: Vec<String>,

    // UI state
    pub active_tab: String,
    pub drawer_state: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum SaveCategory {
    Play,   // player-facing saves
    Stage,  // developer test snapshots
    Auto,   // autosave
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DroneSaveData {
    pub assignment_sector: String,
    pub assignment_pos_x: f32,
    pub assignment_pos_y: f32,
    pub ore_type: String,
    pub state: String,
    pub cargo: f32,
    pub is_echo_primary: bool,
}

// File path functions
pub fn save_dir(category: &SaveCategory) -> PathBuf {
    #[cfg(target_os = "android")]
    let base = PathBuf::from("/sdcard/Download/voidrift/saves");
    #[cfg(not(target_os = "android"))]
    let base = PathBuf::from("./saves");

    match category {
        SaveCategory::Play => base.join("play"),
        SaveCategory::Stage => base.join("stage"),
        SaveCategory::Auto => base.join(".."),
    }
}

pub fn autosave_path() -> PathBuf {
    #[cfg(target_os = "android")]
    return PathBuf::from("/sdcard/Download/voidrift/autosave.json");
    #[cfg(not(target_os = "android"))]
    return PathBuf::from("./autosave.json");
}

// Save and load functions
pub fn save_game(data: &SaveData) -> Result<(), String> {
    let path = if data.save_category == SaveCategory::Auto {
        autosave_path()
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

pub fn load_game(path: &PathBuf) -> Result<SaveData, String> {
    let json = std::fs::read_to_string(path)
        .map_err(|e| format!("Read failed: {e}"))?;

    let data: SaveData = serde_json::from_str(&json)
        .map_err(|e| format!("Deserialization failed: {e}"))?;

    if data.save_version != SAVE_VERSION {
        // Return data anyway but caller can show version warning
        return Ok(data);
    }

    Ok(data)
}

pub fn list_saves(category: &SaveCategory) -> Vec<SaveData> {
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

fn sanitize_filename(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
        .collect()
}

// Events
#[derive(Event)]
pub struct AutosaveEvent;

#[derive(Event)]
pub struct SaveRequestEvent {
    pub name: String,
    pub category: SaveCategory,
    pub description: String,
}
