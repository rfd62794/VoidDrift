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
    
    // Additional game state
    pub ship_cargo: f32,
    pub signal_entries: Vec<String>,
}

impl SaveData {
    pub fn default(name: String, category: SaveCategory, description: String) -> Self {
        Self {
            save_version: SAVE_VERSION,
            save_name: name,
            save_category: category,
            timestamp: "0".to_string(),
            description,
            opening_complete: false,
            opening_phase: "Adrift".to_string(),
            station_online: false,
            station_power: 0.0,
            power_cells: 0,
            magnetite: 0.0,
            carbon: 0.0,
            hull_plates: 0,
            ship_hulls: 0,
            ai_cores: 0,
            repair_progress: 0.0,
            tab_power: false,
            tab_cargo: false,
            tab_refinery: false,
            tab_foundry: false,
            tab_hangar: false,
            drone_count: 0,
            drones: vec![],
            sectors_discovered: vec![],
            active_tab: "Reserves".to_string(),
            drawer_state: "default".to_string(),
            ship_cargo: 0.0,
            signal_entries: vec![],
        }
    }
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

// Save collection and application
pub fn collect_save_data(
    name: String,
    category: SaveCategory,
    description: String,
    params: &SaveSystemParams,
) -> SaveData {
    let Ok(station) = params.station_query.get_single() else {
        return SaveData::default(name, category, description);
    };
    
    // Collect drone data
    let mut drones = vec![];
    let drone_count = params.drone_query.iter().count() as u8;
    
    for drone in params.drone_query.iter() {
        drones.push(DroneSaveData {
            assignment_sector: "Unknown".to_string(), // TODO: get from AutonomousAssignment
            assignment_pos_x: 0.0, // TODO: get from AutonomousAssignment
            assignment_pos_y: 0.0, // TODO: get from AutonomousAssignment
            ore_type: format!("{:?}", drone.cargo_type),
            state: format!("{:?}", drone.state),
            cargo: drone.cargo,
            is_echo_primary: drone.is_echo_primary,
        });
    }
    
    // Collect ship data
    let ship_cargo = params.ship_query.iter().map(|s| s.cargo).next().unwrap_or(0.0);
    
    // Collect discovered sectors (simplified for now)
    let sectors_discovered = vec!["Starting Sector".to_string()];
    
    SaveData {
        save_version: SAVE_VERSION,
        save_name: name,
        save_category: category,
        timestamp: current_timestamp(),
        description,
        opening_complete: params.opening.phase == OpeningPhase::Complete,
        opening_phase: format!("{:?}", params.opening.phase),
        station_online: station.online,
        station_power: station.power,
        power_cells: station.power_cells,
        magnetite: station.magnetite_reserves,
        carbon: station.carbon_reserves,
        hull_plates: station.hull_plate_reserves,
        ship_hulls: station.ship_hulls,
        ai_cores: station.ai_cores,
        repair_progress: station.repair_progress,
        tab_power: true, // Simplified - assume tabs are unlocked based on progress
        tab_cargo: true,
        tab_refinery: station.repair_progress > 0.25,
        tab_foundry: station.repair_progress > 0.50,
        tab_hangar: station.repair_progress > 0.75,
        drone_count,
        drones,
        sectors_discovered,
        active_tab: format!("{:?}", params.active_tab),
        drawer_state: "default".to_string(),
        ship_cargo,
        signal_entries: params.signal_log.entries.iter().cloned().collect(),
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
    params: SaveSystemParams,
) {
    info!("Save request system running, {} events received", events.len());
    
    for event in events.read() {
        info!("Processing save request: name={}, category={:?}", event.name, event.category);
        
        let data = collect_save_data(
            event.name.clone(),
            event.category.clone(),
            event.description.clone(),
            &params,
        );
        
        info!("Save data collected, attempting to save...");
        if let Err(e) = save_game(&data) {
            error!("Save failed: {e}");
        } else {
            info!("Game saved successfully: {}", event.name);
        }
    }
}

// Load restoration system
pub fn restore_save_data(
    data: &SaveData,
    mut commands: Commands,
    mut station: ResMut<Station>,
    mut opening: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut active_tab: ResMut<ActiveStationTab>,
) -> Result<(), String> {
    info!("Restoring save data: {}", data.save_name);
    
    // Restore station state
    station.online = data.station_online;
    station.power = data.station_power;
    station.power_cells = data.power_cells;
    station.magnetite_reserves = data.magnetite;
    station.carbon_reserves = data.carbon;
    station.hull_plate_reserves = data.hull_plates;
    station.ship_hulls = data.ship_hulls;
    station.ai_cores = data.ai_cores;
    station.repair_progress = data.repair_progress;
    
    // Restore opening sequence
    if data.opening_complete {
        opening.phase = OpeningPhase::Complete;
        opening.timer = 0.0;
    } else {
        // Try to parse the phase back
        if let Ok(phase_str) = data.opening_phase.parse::<String>() {
            match phase_str.as_str() {
                "Adrift" => opening.phase = OpeningPhase::Adrift,
                "SignalIdentified" => opening.phase = OpeningPhase::SignalIdentified,
                "AutoPiloting" => opening.phase = OpeningPhase::AutoPiloting,
                "Docking" => opening.phase = OpeningPhase::Docking,
                "Complete" => opening.phase = OpeningPhase::Complete,
                _ => opening.phase = OpeningPhase::Adrift,
            }
        }
        opening.timer = 0.0;
    }
    
    // Restore active tab
    if let Ok(tab_str) = data.active_tab.parse::<String>() {
        match tab_str.as_str() {
            "Reserves" => *active_tab = ActiveStationTab::Reserves,
            "Power" => *active_tab = ActiveStationTab::Power,
            "Cargo" => *active_tab = ActiveStationTab::Cargo,
            "Refinery" => *active_tab = ActiveStationTab::Refinery,
            "Foundry" => *active_tab = ActiveStationTab::Foundry,
            "Hangar" => *active_tab = ActiveStationTab::Hangar,
            _ => *active_tab = ActiveStationTab::default(),
        }
    }
    
    // Restore signal log
    signal_log.entries.clear();
    for entry in data.signal_entries {
        signal_log.entries.push_back(entry);
    }
    
    // Add restoration message
    signal_log.entries.push_back(
        format!("ECHO: SAVE '{}' RESTORED.", data.save_name.to_uppercase())
    );
    
    info!("Save data restored successfully");
    Ok(())
}

// Load request system
pub fn load_request_system(
    mut events: EventReader<LoadRequestEvent>,
    mut commands: Commands,
    mut station: ResMut<Station>,
    mut opening: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut active_tab: ResMut<ActiveStationTab>,
) {
    for event in events.read() {
        info!("Processing load request: {:?}", event.path);
        
        match load_game(&event.path) {
            Ok(save_data) => {
                info!("Save file loaded successfully, restoring game state...");
                if let Err(e) = restore_save_data(
                    &save_data,
                    &mut commands,
                    &mut station,
                    &mut opening,
                    &mut signal_log,
                    &mut active_tab,
                ) {
                    error!("Failed to restore save data: {e}");
                } else {
                    info!("Game state restored successfully");
                }
            }
            Err(e) => {
                error!("Failed to load save file: {e}");
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

#[derive(Event)]
pub struct LoadRequestEvent {
    pub path: PathBuf,
}
