use bevy::prelude::*;
use bevy::ui::IsDefaultUiCamera;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::components::*;
use crate::systems::bevy_ui_signal::{SignalStripRoot, SignalEntryContainer, SignalEntry};

// Global debug log collector
static DEBUG_LOG: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

#[derive(Resource)]
pub struct DebugLogSystem {
    max_entries: usize,
    log_file_path: String,
}

impl Default for DebugLogSystem {
    fn default() -> Self {
        // Create timestamped filename in Android accessible directory
        let now = std::time::SystemTime::now();
        let timestamp = now.duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        // Use /sdcard/Download which is accessible on Android
        let filename = format!("/sdcard/Download/voidrift_debug_{}.log", timestamp);
        
        Self {
            max_entries: 1000,
            log_file_path: filename,
        }
    }
}

pub fn setup_debug_log_system(mut commands: Commands) {
    let debug_system = DebugLogSystem::default();
    let log_path = debug_system.log_file_path.clone();
    
    commands.insert_resource(debug_system);
    
    // Create initial log file
    if let Ok(mut file) = File::create(&log_path) {
        let _ = writeln!(file, "=== VOIDRIFT DEBUG LOG STARTED ===");
        let _ = writeln!(file, "Timestamp: {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs());
        let _ = writeln!(file, "Log file: {}", log_path);
        let _ = writeln!(file, "=== LOG ENTRIES ===");
        let _ = file.flush();
        println!("[DEBUG LOG] Debug logging system initialized - writing to: {}", log_path);
    } else {
        println!("[DEBUG LOG] WARNING: Could not create log file: {}", log_path);
    }
}

pub fn log_debug_info(message: String) {
    if let Ok(mut log) = DEBUG_LOG.lock() {
        if log.len() >= 1000 {
            log.pop_front();
        }
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        log.push_back(format!("[{}] {}", timestamp, message));
    }
}

fn write_log_to_file(log_path: &str, entries: &[String]) -> Result<(), std::io::Error> {
    let mut file = File::create(log_path)?;
    
    // Write header
    writeln!(file, "=== VOIDRIFT DEBUG LOG ===")?;
    writeln!(file, "Generated: {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_secs())?;
    writeln!(file, "Total entries: {}", entries.len())?;
    writeln!(file, "=== LOG ENTRIES ===")?;
    
    // Write all entries
    for entry in entries {
        writeln!(file, "{}", entry)?;
    }
    
    writeln!(file, "=== END LOG ===")?;
    file.flush()?;
    Ok(())
}

pub fn flush_debug_log_system(
    debug_log: Res<DebugLogSystem>,
    // Add any other queries you want to monitor
) {
    if let Ok(log) = DEBUG_LOG.lock() {
        // Write to console (for immediate feedback)
        println!("[DEBUG LOG] === FLUSHING {} ENTRIES TO FILE ===", log.len());
        
        // Write to file
        let log_entries: Vec<String> = log.iter().cloned().collect();
        match write_log_to_file(&debug_log.log_file_path, &log_entries) {
            Ok(()) => {
                println!("[DEBUG LOG] Successfully wrote {} entries to: {}", log.len(), debug_log.log_file_path);
            }
            Err(e) => {
                println!("[DEBUG LOG] ERROR writing to file {}: {}", debug_log.log_file_path, e);
                // Fallback to console output
                for entry in log.iter() {
                    println!("{}", entry);
                }
            }
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

// Periodic flush system - runs every 5 seconds
pub fn periodic_flush_debug_log_system(debug_log: Res<DebugLogSystem>) {
    static mut LAST_FLUSH: u64 = 0;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    unsafe {
        if now - LAST_FLUSH >= 5 {
            LAST_FLUSH = now;
            log_debug_info("=== PERIODIC FLUSH TRIGGERED ===".to_string());
            flush_debug_log_system(debug_log);
        }
    }
}

// System registration debug logging
pub fn log_system_registration() {
    log_debug_info("Bevy UI signal strip system registered".to_string());
    log_debug_info("Camera debug system registered".to_string());
    log_debug_info("Debug log flush system registered".to_string());
    log_debug_info("Periodic debug log flush system registered".to_string());
}
