use bevy::prelude::*;
use crate::components::*;
use crate::systems::save::*;

#[derive(Resource, Default)]
pub struct MainMenuState {
    pub play_saves: Vec<SaveData>,
    pub stage_saves: Vec<SaveData>,
    pub autosave: Option<SaveData>,
    pub developer_mode: bool,
    pub dev_tap_count: u8,           // counts taps on title
    pub save_name_input: String,     // for new save name entry
    pub show_save_overlay: bool,     // in-game save overlay
    pub pending_load: Option<SaveData>, // save selected for loading
    pub version_mismatch_warning: Option<String>,
    pub dev_mode_signal_fired: bool,
}

pub fn setup_main_menu(
    mut commands: Commands,
    mut menu_state: ResMut<MainMenuState>,
) {
    // Load save lists on menu entry
    menu_state.play_saves = list_saves(&SaveCategory::Play);
    menu_state.stage_saves = list_saves(&SaveCategory::Stage);
    menu_state.autosave = load_game(&autosave_path()).ok();
    menu_state.developer_mode = false;
    menu_state.dev_tap_count = 0;
    menu_state.dev_mode_signal_fired = false;
}
