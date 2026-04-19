// Voidrift — Phase 4: Station UI & Refinery (Final Gate 4 Build)
// ============================================================================
// Goal: Final Phase 4 closure. Opt-C: Logic verified, text deferred.
// ============================================================================

use bevy::{
    prelude::*,
};
use bevy_egui::EguiPlugin;
use rand::{Rng, SeedableRng};

mod constants;
pub use constants::*;

mod components;
pub use components::*;

pub mod systems;
use crate::systems::setup::*;
use crate::systems::visuals::*;
use crate::systems::autopilot::*;
use crate::systems::mining::*;
use crate::systems::autonomous::*;
use crate::systems::economy::*;
use crate::systems::ui::*;
use crate::systems::map::*;

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
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .insert_resource(CameraDelta::default())
        .add_systems(Startup, systems::setup::setup_world)
        .add_systems(Update, (
            systems::autopilot::autopilot_system, 
            systems::map::camera_follow_system, 
            systems::visuals::starfield_scroll_system,
            systems::visuals::ship_rotation_system,
            systems::visuals::thruster_glow_system,
        ).chain())
        .add_systems(OnEnter(GameState::MapView), systems::map::enter_map_view)
        .add_systems(OnExit(GameState::MapView), systems::map::exit_map_view)
        .add_systems(Update, (
            systems::mining::mining_system, 
            systems::ui::hud_ui_system,
            systems::ui::station_visual_system,
            systems::autonomous::autonomous_ship_system,
            systems::ui::ship_cargo_display_system,
            systems::ui::autonomous_ship_cargo_display_system,
            systems::economy::station_status_system,
            systems::economy::ship_self_preservation_system,
            systems::economy::station_maintenance_system,
        ))
        .add_systems(Update, systems::map::handle_input)
        .run();
}


