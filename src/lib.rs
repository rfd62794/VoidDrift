// Voidrift — Phase 10: Documentation & Final Refactor
// ============================================================================

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod constants;
pub use constants::*;

mod components;
pub use components::*;

pub mod systems;

// ----------------------------------------------------------------------------
// APP SETUP
// ----------------------------------------------------------------------------

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                present_mode: bevy::window::PresentMode::Fifo,
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin) // UiPlugin automatically included via bevy_ui feature
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .insert_resource(CameraDelta::default())
        .insert_resource(SignalLog::default())
        .insert_resource(SignalStripExpanded(false))
        .insert_resource(OpeningSequence { phase: OpeningPhase::Adrift, timer: 0.0 })
        .insert_resource(ActiveStationTab::default())
        .insert_resource(ForgeSettings::default())
        .insert_resource(AutoDockSettings::default())
        .insert_resource(QuestLog::default())
        .insert_resource(TutorialState::default())
        .insert_resource(MapPanState::default())
        .add_systems(Startup, (
            systems::setup::setup_world,
            systems::bevy_ui_signal::setup_signal_strip,
            systems::debug_log::setup_debug_log_system,
        ))
        .add_systems(Update, (
            systems::autopilot::autopilot_system, 
            systems::map::camera_follow_system,                
            systems::visuals::starfield_scroll_system,
            systems::visuals::station_rotation_system,
            systems::autopilot::docked_ship_system,
            systems::autonomous::docked_autonomous_ship_system, // Chain after rotation to avoid jitter
            systems::visuals::berth_occupancy_system,
            systems::ui::station_visual_system,
            systems::visuals::ship_rotation_system,
            systems::visuals::thruster_glow_system,
        ).chain())
        .add_systems(OnEnter(GameState::MapView), (
            systems::map::enter_map_view,
            systems::map::show_map_elements,
        ))
        .add_systems(OnExit(GameState::MapView), (
            systems::map::exit_map_view,
            systems::map::hide_map_elements,
        ))
        .add_systems(Update, (
            // --- Gameplay & Logistics ---
            systems::mining::mining_system, 
            systems::autonomous::autonomous_ship_system,
            systems::autonomous::autonomous_beam_system.after(systems::autonomous::autonomous_ship_system),
            systems::ui::ship_cargo_display_system,
            systems::ui::autonomous_ship_cargo_display_system,
            systems::ui::cargo_label_system,
        ))
        .add_systems(Update, (
            // --- Station, Narrative & UI ---
            systems::bevy_ui_signal::signal_strip_interaction,
            systems::bevy_ui_signal::signal_strip_system,
            systems::ui::hud_ui_system,
            // --- Debug Logging ---
            systems::debug_log::log_bevy_ui_state,
            systems::debug_log::log_camera_ui_state,
            systems::debug_log::periodic_flush_debug_log_system,
            systems::ui::station_visual_system,
            systems::economy::station_status_system,
            systems::economy::station_maintenance_system,
            systems::economy::ship_self_preservation_system,
            systems::economy::processing_queue_system,
            systems::economy::auto_dock_system,
            systems::map::map_highlight_system,
            systems::map::map_input_system,
            systems::map::pinch_zoom_system,
            systems::map::map_pan_system,
            systems::narrative::opening_sequence_system,
            systems::narrative::signal_system,
            systems::narrative::tutorial_system,
            systems::quest::quest_update_system,
        ))
        .run();
}
