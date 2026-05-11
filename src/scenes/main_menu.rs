use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::components::*;
use crate::components::resources::{ShipQueue, MaxDispatch};
use crate::config::VisualConfig;
use crate::systems::persistence::save::{list_saves, load_game, autosave_path, SaveCategory, SaveData, SAVE_VERSION};

#[path = "restore.rs"]
pub mod restore;
#[path = "save_overlay.rs"]
pub mod save_overlay;
#[path = "menu_starfield.rs"]
pub mod menu_starfield;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    vcfg: Res<VisualConfig>,
) {
    spawn_menu_camera(&mut commands);
    spawn_menu_starfield(&mut commands, &mut meshes, &mut materials, &vcfg);
    
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
    time: Res<Time>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::NONE
            .fill(egui::Color32::TRANSPARENT))
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

                ui.add_space(24.0);

                // PLAY SAVES
                render_save_list(
                    ui,
                    "PLAY SAVES",
                    &menu_state.play_saves.clone(),
                    &mut menu_state,
                    btn_width,
                    false,
                    &mut next_state,
                    &time,
                );

                // STAGE SAVES - developer mode only
                if menu_state.developer_mode {
                    ui.add_space(16.0);
                    render_save_list(
                        ui,
                        "STAGE SAVES",
                        &menu_state.stage_saves.clone(),
                        &mut menu_state,
                        btn_width,
                        true,
                        &mut next_state,
                        &time,
                    );
                }
            });
        });
}

fn render_save_list(
    ui: &mut egui::Ui,
    heading: &str,
    saves: &[SaveData],
    menu_state: &mut MainMenuState,
    btn_width: f32,
    is_stage: bool,
    next_state: &mut NextState<AppState>,
    time: &Res<Time>,
) {
    ui.label(
        egui::RichText::new(heading)
            .size(12.0)
            .color(egui::Color32::from_rgb(80, 100, 80))
    );

    ui.add_space(4.0);

    if saves.is_empty() {
        ui.label(
            egui::RichText::new("  no saves found")
                .size(11.0)
                .color(egui::Color32::from_rgb(50, 60, 50))
        );
    }

    for save in saves.iter().take(20) {
        let version_ok = save.save_version == SAVE_VERSION;
        let label = if version_ok {
            format!(
                "{}    {}",
                save.save_name,
                format_timestamp(&save.timestamp, time),
            )
        } else {
            format!("{}  [VERSION MISMATCH]", save.save_name)
        };

        let color = if !version_ok {
            egui::Color32::from_rgb(180, 60, 60)
        } else if is_stage {
            egui::Color32::from_rgb(180, 140, 0)
        } else {
            egui::Color32::from_rgb(0, 180, 90)
        };

        let btn = ui.add_sized(
            egui::vec2(btn_width, 40.0),
            egui::Button::new(
                egui::RichText::new(&label).size(13.0).color(color)
            )
        );

        if btn.clicked() {
            menu_state.pending_load = Some(save.clone());
            next_state.set(AppState::InGame);
        }
    }
}

fn format_timestamp(ts: &str, time: &Res<Time>) -> String {
    ts.parse::<u64>().map(|secs| {
        let now = time.elapsed().as_secs();

        let diff = now.saturating_sub(secs);
        if diff < 60 {
            format!("{}s ago", diff)
        } else if diff < 3600 {
            let mins = diff / 60;
            format!("{}m ago", mins)
        } else {
            let hours = diff / 3600;
            format!("{}h ago", hours)
        }
    }).unwrap_or_else(|_| ts.to_string())
}

fn spawn_menu_camera(commands: &mut Commands) {
    commands.spawn((
        Camera2d::default(),
        Camera {
            order: -1, // Below MainCamera (order 0) — no ambiguity when transitioning
            ..default()
        },
        OrthographicProjection {
            far: 1200.0,
            ..OrthographicProjection::default_2d()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
        MenuCamera,
    ));
}

pub fn cleanup_menu(
    mut commands: Commands,
    cam_query: Query<Entity, With<MenuCamera>>,
    star_query: Query<Entity, With<MenuStar>>,
) {
    for e in cam_query.iter() { commands.entity(e).despawn_recursive(); }
    for e in star_query.iter() { commands.entity(e).despawn_recursive(); }
}

pub use restore::ingame_startup_system;
pub use save_overlay::save_overlay_system;
pub use menu_starfield::{spawn_menu_starfield, menu_star_drift_system};
