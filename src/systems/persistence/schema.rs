use serde::{Deserialize, Serialize};
use bevy::prelude::Resource;

#[derive(Serialize, Deserialize, Clone, Debug, Default, Resource)]
pub struct SaveData {
    // Meta
    #[serde(default)] pub save_version: u32,
    #[serde(default)] pub save_name: String,
    #[serde(default)] pub save_category: SaveCategory,
    #[serde(default)] pub timestamp: String,
    #[serde(default)] pub description: String,          // developer note for stage saves

    // Opening state
    #[serde(default)] pub opening_complete: bool,
    #[serde(default)] pub opening_phase: String,        // serialized phase name

    // Station state
    #[serde(default)] pub station_online: bool,
    #[serde(default)] pub iron: f32,
    #[serde(default)] pub iron_ingots: f32,
    #[serde(default)] pub tungsten: f32,
    #[serde(default)] pub tungsten_ingots: f32,
    #[serde(default)] pub nickel: f32,
    #[serde(default)] pub nickel_ingots: f32,
    #[serde(default)] pub aluminum: f32,
    #[serde(default)] pub aluminum_ingots: f32,
    #[serde(default)] pub aluminum_canisters: f32,
    #[serde(default)] pub hull_plates: f32,
    #[serde(default)] pub thruster_reserves: f32,
    #[serde(default)] pub ship_hulls: f32,  // kept for save compat — maps to queue count on load
    #[serde(default)] pub ai_cores: f32,
    #[serde(default)] pub repair_progress: f32,
    #[serde(default)] pub drone_build_progress: f32,
    #[serde(default)] pub max_dispatch: u32,
    #[serde(default)] pub power_multiplier: f32,
    #[serde(default)] pub signal_fired_ids: Vec<u32>,

    // Tabs unlocked
    #[serde(default)] pub tab_power: bool,
    #[serde(default)] pub tab_cargo: bool,
    #[serde(default)] pub tab_refinery: bool,
    #[serde(default)] pub tab_foundry: bool,
    #[serde(default)] pub tab_hangar: bool,

    // Fleet state
    #[serde(default)] pub drone_count: u8,
    #[serde(default)] pub drones: Vec<DroneSaveData>,

    // World state
    #[serde(default)] pub sectors_discovered: Vec<String>,

    // UI state
    #[serde(default)] pub active_tab: String,
    #[serde(default)] pub drawer_state: String,
    
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

    // Scout Mk I automation state
    #[serde(default)]
    pub scout_enabled: bool,
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

#[cfg(test)]
mod tests {
    use super::*;

    // Test anchor for Issue #56: test_save_deserializes_with_missing_fields
    #[test]
    fn test_save_deserializes_with_missing_fields() {
        // Construct a JSON save string with one field omitted
        let json_with_missing_field = r#"{
            "save_version": 1,
            "save_name": "Test",
            "timestamp": "2024-01-01",
            "description": "Test save"
        }"#;

        // Deserialize - should not panic and should use default values for missing fields
        let result: Result<SaveData, _> = serde_json::from_str(json_with_missing_field);
        
        // Confirm no panic and default values used
        assert!(result.is_ok());
        let save_data = result.unwrap();
        
        // Verify default values were used for missing fields
        assert_eq!(save_data.save_version, 1);
        assert_eq!(save_data.save_name, "Test");
        assert_eq!(save_data.timestamp, "2024-01-01");
        assert_eq!(save_data.description, "Test save");
        
        // These should have default values since they were omitted
        assert_eq!(save_data.iron, 0.0);
        assert_eq!(save_data.drone_count, 0);
        assert_eq!(save_data.tab_cargo, false);
    }
}
