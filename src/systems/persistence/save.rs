use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::components::*;

#[cfg(target_arch = "wasm32")]
use gloo_storage::{LocalStorage, Storage};

#[path = "schema.rs"]
pub mod schema;
pub use schema::*;

pub const SAVE_VERSION: u32 = 7;

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
    signal_log: &SignalLog,
    time: &Res<Time>,
    telemetry_consent: &Res<crate::systems::telemetry::TelemetryConsent>,
    telemetry_session_counter: &Res<crate::systems::telemetry::TelemetrySessionCounter>,
) -> SaveData {
    SaveData {
        save_version: SAVE_VERSION,
        save_name: name,
        save_category: category,
        timestamp: current_timestamp(time),
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
        aluminum: station.aluminum_reserves,
        aluminum_ingots: station.aluminum_ingots,
        aluminum_canisters: station.aluminum_canisters,
        hull_plates: station.hull_plate_reserves,
        thruster_reserves: station.thruster_reserves,
        ship_hulls: queue.available_count as f32,
        ai_cores: station.ai_cores,
        repair_progress: station.repair_progress,
        drone_build_progress: station.drone_build_progress,
        max_dispatch: station.max_dispatch,
        power_multiplier: station.power_multiplier,
        signal_fired_ids: signal_log.fired.iter().copied().collect(),
        tab_power: false, // TODO: collect from tabs resource
        tab_cargo: false, // TODO: collect from tabs resource
        tab_refinery: false, // TODO: collect from tabs resource
        tab_foundry: false, // TODO: collect from tabs resource
        tab_hangar: false, // TODO: collect from tabs resource
        drone_count: station.drone_count as u8,
        drones: drone_query.iter().map(|(ship, transform, heading, target)| {
            DroneSaveData {
                assignment_sector: "Unknown".to_string(),
                assignment_pos_x: target.map(|t| t.destination.x).unwrap_or(0.0),
                assignment_pos_y: target.map(|t| t.destination.y).unwrap_or(0.0),
                hull_forge: None,
                core_fabricator: None,
                drone_bay: None,
                pos_x: transform.translation.x,
                pos_y: transform.translation.y,
                heading: heading.0,
            }
        }).collect(),
        sectors_discovered: vec![], // TODO: collect from world
        active_tab: format!("{:?}", active_tab),
        drawer_state: "default".to_string(),
        collected_requests: requests_tab.collected_requests.clone(),
        telemetry_consent: telemetry_consent.opted_in,
        telemetry_sessions: telemetry_session_counter.sessions,
        unlocked_logs: vec![], // Collected from GameState resource in a future task
    }
}

fn current_timestamp(time: &Res<Time>) -> String {
    let secs = time.elapsed().as_secs();
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
    signal_log: Res<SignalLog>,
    time: Res<Time>,
    telemetry_consent: Res<crate::systems::telemetry::TelemetryConsent>,
    telemetry_session_counter: Res<crate::systems::telemetry::TelemetrySessionCounter>,
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
                &signal_log,
                &time,
                &telemetry_consent,
                &telemetry_session_counter,
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
    signal_log: Res<SignalLog>,
    time: Res<Time>,
    telemetry_consent: Res<crate::systems::telemetry::TelemetryConsent>,
    telemetry_session_counter: Res<crate::systems::telemetry::TelemetrySessionCounter>,
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
                &signal_log,
                &time,
                &telemetry_consent,
                &telemetry_session_counter,
            );
            
            if let Err(e) = save_game(&data) {
                error!("Save failed: {e}");
            } else {
                info!("Game saved successfully: {}", event.name);
            }
        }
    }
}
