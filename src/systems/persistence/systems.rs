use bevy::prelude::*;
use crate::components::*;

use super::schema::{SaveData, SaveCategory, DroneSaveData, AutosaveEvent, SaveRequestEvent};
use super::io::{save_game, current_timestamp};
use super::SAVE_VERSION;

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
    scout_enabled: &crate::components::resources::ScoutEnabled,
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
        drones: drone_query.iter().map(|(_ship, transform, heading, target)| {
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
        scout_enabled: scout_enabled.active,
    }
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
    scout_enabled: Res<crate::components::resources::ScoutEnabled>,
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
                &scout_enabled,
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
    scout_enabled: Res<crate::components::resources::ScoutEnabled>,
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
                &scout_enabled,
            );
            
            if let Err(e) = save_game(&data) {
                error!("Save failed: {e}");
            } else {
                info!("Game saved successfully: {}", event.name);
            }
        }
    }
}
