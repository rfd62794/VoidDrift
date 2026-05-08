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

pub struct TelemetryPlugin;

impl Plugin for TelemetryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SessionId>()
            .add_systems(OnEnter(crate::components::resources::AppState::InGame), send_session_start);
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
