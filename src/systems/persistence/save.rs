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
    pub iron: f32,
    #[serde(default)] pub iron_ingots: f32,
    pub tungsten: f32,
    #[serde(default)] pub tungsten_ingots: f32,
    pub nickel: f32,
    #[serde(default)] pub nickel_ingots: f32,
    pub hull_plates: f32,
    #[serde(default)] pub thruster_reserves: f32,
    pub ship_hulls: f32,  // kept for save compat — maps to queue count on load
    pub ai_cores: f32,
    pub repair_progress: f32,
    pub drone_build_progress: f32,

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
    
    // Requests state
    #[serde(default)]
    pub collected_requests: Vec<CollectedRequest>,
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
    pub pos_x: f32,
    pub pos_y: f32,
    pub heading: f32,
}

// File path functions
fn get_save_base_dir() -> PathBuf {
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

// Save collection and application
pub fn collect_save_data(
    name: String,
    category: SaveCategory,
    description: String,
    station: &Station,
    opening: &OpeningSequence,
    active_tab: &ActiveStationTab,
    queue: &ShipQueue,
    drone_query: &Query<(&Ship, &Transform, &LastHeading, Option<&AutopilotTarget>), With<AutonomousShipTag>>,
    requests_tab: &RequestsTabState,
) -> SaveData {
    SaveData {
        save_version: SAVE_VERSION,
        save_name: name,
        save_category: category,
        timestamp: current_timestamp(),
        description,
        opening_complete: opening.phase == OpeningPhase::Complete,
        opening_phase: format!("{:?}", opening.phase),
        station_online: station.online,
        iron: station.iron_reserves,
        iron_ingots: station.iron_ingots,
        tungsten: station.tungsten_reserves,
        tungsten_ingots: station.tungsten_ingots,
        nickel: station.nickel_reserves,
        nickel_ingots: station.nickel_ingots,
        hull_plates: station.hull_plate_reserves,
        thruster_reserves: station.thruster_reserves,
        ship_hulls: queue.available_count as f32,
        ai_cores: station.ai_cores,
        repair_progress: station.repair_progress,
        drone_build_progress: station.drone_build_progress,
        tab_power: false, // TODO: collect from tabs resource
        tab_cargo: false, // TODO: collect from tabs resource
        tab_refinery: false, // TODO: collect from tabs resource
        tab_foundry: false, // TODO: collect from tabs resource
        tab_hangar: false, // TODO: collect from tabs resource
        drone_count: drone_query.iter().count() as u8,
        drones: drone_query.iter().map(|(ship, transform, heading, target)| {
            DroneSaveData {
                assignment_sector: "Unknown".to_string(), // Could be derived from pos if needed
                assignment_pos_x: target.map(|t| t.destination.x).unwrap_or(0.0),
                assignment_pos_y: target.map(|t| t.destination.y).unwrap_or(0.0),
                ore_type: format!("{:?}", ship.cargo_type),
                state: format!("{:?}", ship.state),
                cargo: ship.cargo,
                is_echo_primary: false,
                pos_x: transform.translation.x,
                pos_y: transform.translation.y,
                heading: heading.0,
            }
        }).collect(),
        sectors_discovered: vec![], // TODO: collect from world
        active_tab: format!("{:?}", active_tab),
        drawer_state: "default".to_string(),
        collected_requests: requests_tab.collected_requests.clone(),
    }
}

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

// Autosave system
pub fn autosave_system(
    mut events: EventReader<AutosaveEvent>,
    station_query: Query<&Station, (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    opening: Res<OpeningSequence>,
    active_tab: Res<ActiveStationTab>,
    queue: Res<ShipQueue>,
    drone_query: Query<(&Ship, &Transform, &LastHeading, Option<&AutopilotTarget>), With<AutonomousShipTag>>,
    requests_tab: Res<RequestsTabState>,
) {
    if let Ok(station) = station_query.get_single() {
        for _ in events.read() {
            let data = collect_save_data(
                "autosave".to_string(),
                SaveCategory::Auto,
                String::new(),
                station,
                &opening,
                &active_tab,
                &queue,
                &drone_query,
                &requests_tab,
            );
            if let Err(e) = save_game(&data) {
                warn!("Autosave failed: {e}");
            }
        }
    }
}

// Save request system
pub fn save_request_system(
    mut events: EventReader<SaveRequestEvent>,
    station_query: Query<&Station, (With<Station>, Without<Ship>, Without<AutonomousShipTag>)>,
    opening: Res<OpeningSequence>,
    active_tab: Res<ActiveStationTab>,
    queue: Res<ShipQueue>,
    drone_query: Query<(&Ship, &Transform, &LastHeading, Option<&AutopilotTarget>), With<AutonomousShipTag>>,
    requests_tab: Res<RequestsTabState>,
) {
    if let Ok(station) = station_query.get_single() {
        for event in events.read() {
            let data = collect_save_data(
                event.name.clone(),
                event.category.clone(),
                event.description.clone(),
                station,
                &opening,
                &active_tab,
                &queue,
                &drone_query,
                &requests_tab,
            );
            
            if let Err(e) = save_game(&data) {
                error!("Save failed: {e}");
            } else {
                info!("Game saved successfully: {}", event.name);
            }
        }
    }
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
