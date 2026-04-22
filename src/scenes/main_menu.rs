use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
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
    _commands: Commands,
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

pub fn main_menu_system(
    mut contexts: EguiContexts,
    mut menu_state: ResMut<MainMenuState>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE
            .fill(egui::Color32::from_rgb(4, 6, 12)))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(120.0);

                // Station title - 7-tap developer unlock
                let title_response = ui.add(
                    egui::Label::new(
                        egui::RichText::new("VOIDRIFT STATION")
                            .size(28.0)
                            .color(egui::Color32::from_rgb(0, 204, 102))
                            .strong()
                    )
                    .sense(egui::Sense::click())
                );

                if title_response.clicked() {
                    menu_state.dev_tap_count += 1;
                    if menu_state.dev_tap_count >= 7 {
                        menu_state.developer_mode = true;
                    }
                }

                ui.label(
                    egui::RichText::new("COMMAND INTERFACE")
                        .size(14.0)
                        .color(egui::Color32::from_rgb(60, 80, 60))
                );

                if menu_state.developer_mode {
                    ui.label(
                        egui::RichText::new("[ DEVELOPER MODE ]")
                            .size(11.0)
                            .color(egui::Color32::from_rgb(180, 120, 0))
                    );
                }

                ui.add_space(48.0);

                // ECHO ambient line
                ui.label(
                    egui::RichText::new("> ECHO: AWAITING AUTHORIZATION.")
                        .size(12.0)
                        .color(egui::Color32::from_rgb(0, 140, 70))
                );

                ui.add_space(32.0);

                let btn_width = 320.0;
                let btn_height = 52.0;
                let btn_size = egui::vec2(btn_width, btn_height);

                // NEW GAME
                if ui.add_sized(btn_size, egui::Button::new(
                    egui::RichText::new("NEW GAME")
                        .size(16.0)
                )).clicked() {
                    menu_state.pending_load = None;
                    next_state.set(AppState::InGame);
                }

                ui.add_space(8.0);

                // CONTINUE
                let continue_label = if menu_state.autosave.is_some() {
                    "CONTINUE"
                } else {
                    "CONTINUE  (no autosave)"
                };
                let continue_btn = ui.add_sized(btn_size, egui::Button::new(
                    egui::RichText::new(continue_label).size(16.0)
                ));
                if continue_btn.clicked() {
                    if let Some(save) = &menu_state.autosave {
                        menu_state.pending_load = Some(save.clone());
                        next_state.set(AppState::InGame);
                    }
                }
            });
        });
}

pub fn ingame_startup_system(
    menu_state: Res<MainMenuState>,
    mut opening: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
) {
    if let Some(save_data) = &menu_state.pending_load {
        // LOAD PATH - apply save data, skip opening sequence
        opening.phase = OpeningPhase::Complete;
        opening.timer = 0.0;
        
        // TODO: Apply other save data to resources
        
        signal_log.entries.push_back(
            "ECHO: SAVE LOADED SUCCESSFULLY.".to_string()
        );
        signal_log.entries.push_back(
            format!("ECHO: {} RESTORED.", save_data.save_name.to_uppercase())
        );
    } else {
        // NEW GAME PATH - opening sequence runs normally
        // Opening sequence already starts at Adrift phase by default
    }
}
