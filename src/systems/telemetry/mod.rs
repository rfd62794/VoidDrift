use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

const TELEMETRY_URL: &str = "http://localhost:8000/v1/event";
const CLIENT_VERSION: &str = "3.3.0";

#[derive(Debug, Serialize, Deserialize)]
struct TelemetryEvent {
    event_type: String,
    timestamp: String,
    session_id: String,
    client_version: String,
    platform: String,
    meta: serde_json::Value,
}

#[derive(Resource)]
pub struct SessionId(pub String);

impl Default for SessionId {
    fn default() -> Self {
        Self(Uuid::new_v4().to_string())
    }
}

#[derive(Resource, Default)]
pub struct LoopStallTimer {
    pub elapsed_seconds: f32,
    pub has_fired: bool,
}

#[derive(Resource, Clone)]
pub struct LoopStallConfig {
    pub threshold_seconds: f32,
}

impl Default for LoopStallConfig {
    fn default() -> Self {
        Self {
            threshold_seconds: 120.0,
        }
    }
}

pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SessionId>()
            .init_resource::<LoopStallTimer>()
            .init_resource::<LoopStallConfig>()
            .add_systems(OnEnter(crate::components::resources::AppState::InGame), send_session_start)
            .add_systems(
                Update,
                (
                    track_loop_stall,
                    reset_loop_stall_timer_on_pipeline_open,
                ).run_if(in_state(crate::components::resources::AppState::InGame))
            );
    }
}

fn get_platform() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        "wasm".to_string()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        "native".to_string()
    }
}

fn send_session_start(session_id: Res<SessionId>) {
    let event = TelemetryEvent {
        event_type: "session_start".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        session_id: session_id.0.clone(),
        client_version: CLIENT_VERSION.to_string(),
        platform: get_platform(),
        meta: serde_json::json!({}),
    };

    #[cfg(target_arch = "wasm32")]
    {
        send_event_wasm(event);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        send_event_native(event);
    }
}

fn track_loop_stall(
    time: Res<Time>,
    config: Res<LoopStallConfig>,
    mut timer: ResMut<LoopStallTimer>,
    session_id: Res<SessionId>,
    station_query: Query<&crate::components::game_state::Station>,
    view_state: Res<crate::components::resources::ViewState>,
    balance_config: Res<crate::config::balance::BalanceConfig>,
    app_state: Res<State<crate::components::resources::AppState>>,
) {
    if !app_state.get().eq(&crate::components::resources::AppState::InGame) {
        return;
    }

    // Check if production tree is open
    if view_state.show_production_tree {
        return;
    }

    // Check if player has resources for at least one upgrade
    let can_afford_upgrade = if let Ok(station) = station_query.get_single() {
        let hull_cost = balance_config.forge.hull_plate_cost_iron as f32;
        let thruster_cost = balance_config.forge.thruster_cost_tungsten as f32;
        let core_cost = balance_config.forge.ai_core_cost_nickel as f32;
        let canister_cost = balance_config.forge.aluminum_canister_cost_aluminum as f32;

        station.iron_reserves >= hull_cost
            || station.tungsten_reserves >= thruster_cost
            || station.nickel_reserves >= core_cost
            || station.aluminum_reserves >= canister_cost
    } else {
        false
    };

    if can_afford_upgrade {
        timer.elapsed_seconds += time.delta_secs();

        if timer.elapsed_seconds >= config.threshold_seconds && !timer.has_fired {
            timer.has_fired = true;
            // Clone the timer value to pass as Res
            let elapsed = timer.elapsed_seconds;
            let timer_snapshot = LoopStallTimer {
                elapsed_seconds: elapsed,
                has_fired: timer.has_fired,
            };
            send_loop_stall_internal(session_id, timer_snapshot);
        }
    } else {
        // Reset timer if condition not met
        timer.elapsed_seconds = 0.0;
    }
}

fn send_loop_stall_internal(session_id: Res<SessionId>, timer_snapshot: LoopStallTimer) {
    let meta = serde_json::json!({
        "time_since_last_upgrade_seconds": timer_snapshot.elapsed_seconds as u32,
        "pipeline_opened": false,
        "current_ring_unlocked": 1
    });

    let event = TelemetryEvent {
        event_type: "loop_stall".to_string(),
        timestamp: Utc::now().to_rfc3339(),
        session_id: session_id.0.clone(),
        client_version: CLIENT_VERSION.to_string(),
        platform: get_platform(),
        meta,
    };

    #[cfg(target_arch = "wasm32")]
    {
        send_event_wasm(event);
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        send_event_native(event);
    }
}

fn reset_loop_stall_timer_on_pipeline_open(
    mut timer: ResMut<LoopStallTimer>,
    view_state: Res<crate::components::resources::ViewState>,
) {
    if view_state.show_production_tree {
        timer.elapsed_seconds = 0.0;
        timer.has_fired = false;
    }
}

fn reset_loop_stall_timer_on_upgrade(
    mut timer: ResMut<LoopStallTimer>,
) {
    // This system is called when upgrades are purchased
    // For now, we'll reset the timer when processing completes
    // In a future task, we can hook this to specific upgrade purchase events
    timer.elapsed_seconds = 0.0;
    timer.has_fired = false;
}

#[cfg(target_arch = "wasm32")]
fn send_event_wasm(event: TelemetryEvent) {
    // WASM implementation using wasm-bindgen fetch
    // This will be implemented in a future Sprint 8 task
    // For now, log the event to show it would be sent
    info!("Telemetry event (WASM): {:?}", event);
}

#[cfg(not(target_arch = "wasm32"))]
fn send_event_native(event: TelemetryEvent) {
    use std::thread;
    use std::time::Duration;

    let url = TELEMETRY_URL.to_string();
    
    thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build();
        
        match client {
            Ok(client) => {
                match client.post(&url).json(&event).send() {
                    Ok(_) => info!("Telemetry event sent successfully"),
                    Err(e) => warn!("Failed to send telemetry event: {}", e),
                }
            }
            Err(e) => warn!("Failed to create HTTP client: {}", e),
        }
    });
}
