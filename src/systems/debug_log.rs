use std::collections::VecDeque;
use std::sync::Mutex;

// Global debug log buffer
static DEBUG_LOG: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());

#[derive(Resource)]
pub struct DebugLogSystem {
    pub max_entries: usize,
}

impl Default for DebugLogSystem {
    fn default() -> Self {
        Self {
            max_entries: 1000,
        }
    }
}

pub fn log_debug_info(message: String) {
    if let Ok(mut log) = DEBUG_LOG.lock() {
        log.push_back(message);
        // Keep only the most recent entries
        while log.len() > 1000 {
            log.pop_front();
        }
    }
}

pub fn setup_debug_log_system(mut commands: Commands) {
    commands.insert_resource(DebugLogSystem::default());
    log_debug_info("Debug logging system initialized".to_string());
}

pub fn flush_debug_log_system(_debug_log: Res<DebugLogSystem>) {
    if let Ok(log) = DEBUG_LOG.lock() {
        for entry in log.iter() {
            println!("{}", entry);
        }
    }
}
