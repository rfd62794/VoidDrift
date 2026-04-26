// Voidrift — Phase 10: Documentation & Final Refactor
// ============================================================================

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod constants;
pub use constants::*;

mod components;
pub use components::*;

pub mod systems;
pub mod scenes;
use scenes::main_menu::MainMenuState;
use systems::setup::cleanup_world_entities;

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
        .init_state::<AppState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .insert_resource(CameraDelta::default())
        .insert_resource(SignalLog::default())
        .insert_resource(SignalStripExpanded(false))
        .insert_resource(OpeningSequence { phase: OpeningPhase::Adrift, timer: 0.0, beat_timer: 0.0 })
        .insert_resource(ActiveStationTab::default())
        .insert_resource(DrawerState::default())
        .insert_resource(UiLayout::default())
        .insert_resource(WorldViewRect::default())
        .insert_resource(ForgeSettings::default())
        .insert_resource(ProductionToggles::default())
        .insert_resource(QuestLog::default())
        .insert_resource(TutorialState::default())
        .insert_resource(MapPanState::default())
        .insert_resource(MainMenuState::default())
        .insert_resource(ShipQueue::default())
        .init_resource::<AsteroidRespawnTimer>()
        .add_systems(Startup, (
            systems::debug_log::setup_debug_log_system,
        ))
        .add_systems(OnEnter(AppState::MainMenu), (
            scenes::main_menu::setup_main_menu,
        ))
        .add_systems(OnExit(AppState::MainMenu), (
            scenes::main_menu::cleanup_menu,
        ))
        .add_systems(Update, (
            scenes::main_menu::main_menu_system,
        ).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, (
            scenes::main_menu::save_overlay_system,
        ).run_if(in_state(AppState::InGame)))
        .add_event::<systems::save::AutosaveEvent>()
        .add_event::<systems::save::SaveRequestEvent>()
        .add_systems(Update, (
            systems::save::autosave_system,
            systems::save::save_request_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(AppState::InGame), (
            cleanup_world_entities,
            systems::setup::setup_world,
            systems::asteroid_spawn::spawn_initial_asteroids,
            systems::debug_log::setup_debug_log_system,
            scenes::main_menu::ingame_startup_system,
        ).chain())
        .add_systems(Update, (
            systems::autopilot::autopilot_system, 
            systems::map::camera_follow_system,                
            systems::visuals::starfield_scroll_system,
            systems::visuals::station_rotation_system,
            systems::autopilot::docked_ship_system,
            systems::visuals::berth_occupancy_system,
            systems::ui::station_visual_system,
            systems::visuals::ship_rotation_system,
            systems::visuals::thruster_glow_system,
        ).chain().run_if(in_state(AppState::InGame)))
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
            systems::asteroid_spawn::asteroid_respawn_system,
            systems::asteroid_lifecycle::asteroid_lifecycle_system,
            systems::mining::mining_system, 
            systems::auto_process::auto_refine_system,
            systems::auto_process::auto_forge_system,
            systems::auto_process::auto_build_drones_system,
            systems::ui::ship_cargo_display_system,
            systems::ui::cargo_label_system,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            // --- Station, Narrative & UI ---
            systems::ui::hud_ui_system,
            systems::ui::station_visual_system,
            systems::map::map_highlight_system,
            systems::asteroid_input::asteroid_input_system,
            systems::map::pinch_zoom_system,
            systems::map::map_pan_system,
            systems::narrative::opening_sequence_system,
            systems::narrative::opening_drone_move_system,
            systems::narrative::signal_system,
            systems::narrative::tutorial_system,
            systems::quest::quest_update_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(PostUpdate, (
            systems::viewport::ui_layout_system,
            systems::viewport::drawer_viewport_system
                .after(systems::viewport::ui_layout_system),
        ).run_if(in_state(AppState::InGame)))
        .run();
}
