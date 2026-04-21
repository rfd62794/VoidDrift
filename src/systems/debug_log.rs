use bevy::prelude::*;
use bevy::ui::IsDefaultUiCamera;
use std::collections::VecDeque;
use std::sync::Mutex;
use crate::components::*;
use crate::systems::bevy_ui_signal::{SignalStripRoot, SignalEntryContainer, SignalEntry};

// Global debug log collector
static DEBUG_LOG: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

#[derive(Resource)]
pub struct DebugLogSystem {
    max_entries: usize,
}

impl Default for DebugLogSystem {
    fn default() -> Self {
        Self {
            max_entries: 1000,
        }
    }
}

pub fn setup_debug_log_system(mut commands: Commands) {
    commands.insert_resource(DebugLogSystem::default());
    println!("[DEBUG LOG] Debug logging system initialized");
}

pub fn log_debug_info(message: String) {
    if let Ok(mut log) = DEBUG_LOG.lock() {
        if log.len() >= 1000 {
            log.pop_front();
        }
        log.push_back(format!("[{:?}] {}", std::thread::current().id(), message));
    }
}

pub fn flush_debug_log_system(
    _debug_log: Res<DebugLogSystem>,
    // Add any other queries you want to monitor
) {
    if let Ok(log) = DEBUG_LOG.lock() {
        println!("[DEBUG LOG] === FLUSHING {} ENTRIES ===", log.len());
        for entry in log.iter() {
            println!("{}", entry);
        }
        println!("[DEBUG LOG] === END FLUSH ===");
    }
}

// Bevy UI specific debug logging
pub fn log_bevy_ui_state(
    strip_query: Query<(Entity, &Node), With<SignalStripRoot>>,
    container_query: Query<Entity, With<SignalEntryContainer>>,
    entry_query: Query<Entity, With<SignalEntry>>,
    signal_log: Res<SignalLog>,
) {
    let strip_count = strip_query.iter().count();
    let container_count = container_query.iter().count();
    let entry_count = entry_query.iter().count();
    
    log_debug_info(format!("Bevy UI State - Strip: {}, Container: {}, Entries: {}, SignalLog: {}", 
                         strip_count, container_count, entry_count, signal_log.entries.len()));
    
    // Log detailed strip node info
    for (entity, node) in strip_query.iter() {
        log_debug_info(format!("Strip Node {} - pos: {:?}, width: {:?}, height: {:?}", 
                             entity, node.position_type, node.width, node.height));
    }
}

// Camera and UI system debug logging
pub fn log_camera_ui_state(
    camera_query: Query<(Entity, &Camera, Option<&IsDefaultUiCamera>)>,
    ui_camera_query: Query<(Entity, &Camera), With<IsDefaultUiCamera>>,
) {
    let main_cameras = camera_query.iter().count();
    let ui_cameras = ui_camera_query.iter().count();
    
    log_debug_info(format!("Camera State - Main cameras: {}, UI cameras: {}", main_cameras, ui_cameras));
    
    for (entity, camera, is_ui_camera) in camera_query.iter() {
        log_debug_info(format!("Camera {} - order: {}, is_ui_camera: {:?}", entity, camera.order, is_ui_camera.is_some()));
    }
    
    for (entity, camera) in ui_camera_query.iter() {
        log_debug_info(format!("UI Camera {} - order: {}", entity, camera.order));
    }
}

// System registration debug logging
pub fn log_system_registration() {
    log_debug_info("Bevy UI signal strip system registered".to_string());
    log_debug_info("Camera debug system registered".to_string());
    log_debug_info("Debug log flush system registered".to_string());
}
