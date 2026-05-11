use serde::{Deserialize, Serialize};
use bevy::prelude::Resource;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Resource)]
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
    pub iron: f32,
    #[serde(default)] pub iron_ingots: f32,
    pub tungsten: f32,
    #[serde(default)] pub tungsten_ingots: f32,
    pub nickel: f32,
    #[serde(default)] pub nickel_ingots: f32,
    #[serde(default)] pub aluminum: f32,
    #[serde(default)] pub aluminum_ingots: f32,
    #[serde(default)] pub aluminum_canisters: f32,
    pub hull_plates: f32,
    #[serde(default)] pub thruster_reserves: f32,
    pub ship_hulls: f32,  // kept for save compat — maps to queue count on load
    pub ai_cores: f32,
    pub repair_progress: f32,
    pub drone_build_progress: f32,
    #[serde(default)] pub max_dispatch: u32,
    #[serde(default)] pub power_multiplier: f32,
    #[serde(default)] pub signal_fired_ids: Vec<u32>,

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
    
    // Telemetry consent (None = not yet asked, true = allowed, false = declined)
    #[serde(default)]
    pub telemetry_consent: Option<bool>,
    
    // Telemetry session counter (for re-prompt logic)
    #[serde(default)]
    pub telemetry_sessions: u32,

    // Requests state
    #[serde(default)]
    pub collected_requests: Vec<crate::components::CollectedRequest>,

    // Logs state
    #[serde(default)]
    pub unlocked_logs: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Default)]
pub enum SaveCategory {
    #[default]
    Play,   // player-facing saves
    Stage,  // developer test snapshots
    Auto,   // autosave
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DroneSaveData {
    pub assignment_sector: String,
    pub assignment_pos_x: f32,
    pub assignment_pos_y: f32,
    pub hull_forge: Option<f32>,
    pub core_fabricator: Option<f32>,
    pub drone_bay: Option<f32>,
    pub pos_x: f32,
    pub pos_y: f32,
    pub heading: f32,
}

// Events
#[derive(bevy::prelude::Event)]
pub struct AutosaveEvent;

#[derive(bevy::prelude::Event)]
pub struct SaveRequestEvent {
    pub name: String,
    pub category: SaveCategory,
    pub description: String,
}
