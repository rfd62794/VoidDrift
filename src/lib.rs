// Voidrift — Phase 10: Documentation & Final Refactor
// ============================================================================

use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiContexts};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn configure_egui_scale(
    windows: Query<&Window>,
    mut egui_contexts: EguiContexts,
) {
    if let Ok(window) = windows.get_single() {
        let device_scale = window.scale_factor() as f32;
        // Clamp to reasonable range across all targets
        // Android Moto G: scale_factor ~3.0 → egui_scale ~3.0
        // Tablet: scale_factor ~2.0 → egui_scale ~2.0
        // WASM: scale_factor ~1.0–2.0 → egui_scale ~2.0 minimum
        let egui_scale = (device_scale).clamp(2.0, 4.0);
        if let Some(ctx) = egui_contexts.try_ctx_mut() {
            ctx.set_pixels_per_point(egui_scale);
        }
    }
}

fn update_ui_layout_from_window(
    windows: Query<&Window>,
    mut ui_layout: ResMut<UiLayout>,
) {
    if let Ok(window) = windows.get_single() {
        ui_layout.screen_width = window.physical_width() as f32;
        ui_layout.screen_height = window.physical_height() as f32;
    }
}

#[cfg(target_arch = "wasm32")]
fn detect_device_type(mut device_type: ResMut<DeviceType>) {
    let window = web_sys::window().unwrap();
    let is_touch = js_sys::Reflect::get(&window, &"ontouchstart".into())
        .map(|v| !v.is_undefined())
        .unwrap_or(false);
    *device_type = if is_touch {
        DeviceType::Mobile
    } else {
        DeviceType::Desktop
    };
}

#[cfg(target_arch = "wasm32")]
fn setup_fullscreen_resize_handler() {
    use wasm_bindgen::prelude::*;
    use web_sys::{window, EventTarget};
    
    let window = window().unwrap();
    let closure = Closure::wrap(Box::new(move || {
        let win = web_sys::window().unwrap();
        let w = win.inner_width().unwrap().as_f64().unwrap() as u32;
        let h = win.inner_height().unwrap().as_f64().unwrap() as u32;
        
        // Resize the bevy canvas to match browser window
        if let Some(document) = win.document() {
            if let Some(canvas) = document.get_element_by_id("bevy-canvas") {
                let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();
                canvas.set_width(w);
                canvas.set_height(h);
            }
        }
    }) as Box<dyn Fn()>);
    
    window
        .add_event_listener_with_callback("fullscreenchange", closure.as_ref().unchecked_ref())
        .unwrap();
    
    window
        .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
        .unwrap();
    
    closure.forget();
}

mod constants;
pub use constants::*;

pub mod config;
use config::{BalanceConfig, VisualConfig, ContentConfig, TutorialConfig, QuestConfig, RequestConfig, LogsConfig};

mod components;
pub use crate::components::*;

pub mod systems;
pub mod scenes;
use systems::telemetry::TelemetryPlugin;
use scenes::main_menu::MainMenuState;
use systems::setup::cleanup_world_entities;

// ----------------------------------------------------------------------------
// APP SETUP
// ----------------------------------------------------------------------------

#[cfg(not(target_arch = "wasm32"))]
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
        .add_plugins(TelemetryPlugin)
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
        .insert_resource(MaxDispatch::default())
        .insert_resource(RequestsTabState::default())
        .insert_resource(ProductionTabState::default())
        .insert_resource(DeviceType::default())
        .insert_resource(systems::narrative::bottle::BottleSpawnTimer::default())
        .insert_resource(systems::persistence::save::SaveData::default())
        .insert_resource(BalanceConfig::load())
        .insert_resource(VisualConfig::load())
        .insert_resource(ContentConfig::load())
        .insert_resource(TutorialConfig::load())
        .insert_resource(QuestConfig::load())
        .insert_resource(RequestConfig::load())
        .insert_resource(LogsConfig::load())
        .insert_resource(ContentState::default())
        .insert_resource(ViewState::default())
        .init_resource::<AsteroidRespawnTimer>()
        .add_systems(Startup, (
            configure_egui_scale,
            systems::visuals::debug_log::setup_debug_log_system,
        ))
        .add_systems(Update, (
            update_ui_layout_from_window,
        ))
        .add_systems(OnEnter(AppState::MainMenu), (
            scenes::main_menu::setup_main_menu,
        ))
        .add_systems(OnExit(AppState::MainMenu), (
            scenes::main_menu::cleanup_menu,
        ))
        .add_systems(OnExit(AppState::InGame), (
            cleanup_world_entities,
        ))
        .add_systems(Update, (
            scenes::main_menu::main_menu_system,
            scenes::main_menu::menu_star_drift_system,
        ).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, (
            scenes::main_menu::save_overlay_system,
        ).run_if(in_state(AppState::InGame)))
        .add_event::<systems::persistence::save::AutosaveEvent>()
        .add_event::<systems::persistence::save::SaveRequestEvent>()
        .add_event::<ShipDockedWithCargo>()
        .add_event::<ShipDockedWithBottle>()
        .add_event::<FulfillRequestEvent>()
        .add_event::<RepairStationEvent>()
        .add_event::<OpeningCompleteEvent>()
        .add_event::<DroneDispatched>()
        .add_event::<InsufficientLaserEvent>()
        .add_event::<SignalFired>()
        .add_systems(Update, (
            systems::persistence::save::autosave_system,
            systems::persistence::save::save_request_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(AppState::InGame), (
            cleanup_world_entities,
            systems::setup::reset_game_resources,
            systems::setup::setup_world,
            systems::asteroid::spawn::spawn_initial_asteroids,
            systems::visuals::debug_log::setup_debug_log_system,
            scenes::main_menu::ingame_startup_system,
        ).chain())
        .add_systems(Update, (
            systems::ship_control::autopilot::autopilot_system,
            systems::game_loop::economy::ship_docked_economy_system,
            systems::narrative::narrative_events::narrative_event_system,
            systems::visuals::map::camera_follow_system,
            systems::visuals::visuals::starfield_scroll_system,
            systems::visuals::visuals::station_rotation_system,
            systems::ship_control::autopilot::docked_ship_system,
            systems::visuals::visuals::berth_occupancy_system,
            systems::ui::hud::station_visual_system,
            systems::visuals::visuals::ship_rotation_system,
            systems::visuals::visuals::thruster_glow_system,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(GameState::MapView), (
            systems::visuals::map::enter_map_view,
            systems::visuals::map::show_map_elements,
        ))
        .add_systems(OnExit(GameState::MapView), (
            systems::visuals::map::exit_map_view,
            systems::visuals::map::hide_map_elements,
        ))
        .add_systems(Update, (
            systems::asteroid::spawn::asteroid_respawn_system,
            systems::asteroid::lifecycle::asteroid_lifecycle_system,
            systems::game_loop::mining::mining_system,
            systems::game_loop::auto_process::auto_refine_system,
            systems::game_loop::auto_process::auto_forge_system,
            systems::game_loop::auto_process::auto_build_drones_system,
            systems::ui::hud::ship_cargo_display_system,
            systems::ui::hud::cargo_label_system,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::ui::hud::sync_max_drones_system,
            systems::ui::tutorial::tutorial_system,
            systems::ui::hud::hud_ui_system,
            systems::visuals::map::map_highlight_system,
            systems::ship_control::asteroid_input::asteroid_input_system,
            systems::visuals::map::pinch_zoom_system,
            systems::visuals::map::map_pan_system,
            systems::narrative::opening_sequence::opening_sequence_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::narrative::opening_sequence::opening_drone_move_system,
            systems::narrative::signal::signal_system,
            systems::narrative::quest::quest_signal_system,
            systems::narrative::quest::quest_update_system,
            systems::narrative::bottle::bottle_spawn_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(Update, systems::narrative::bottle::bottle_input_system
            .run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::narrative::content_router::content_event_system,
            systems::narrative::content_router::content_ambient_system,
            systems::narrative::logs::check_log_unlocks,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(PostUpdate, (
            systems::visuals::viewport::ui_layout_system,
            systems::visuals::viewport::drawer_viewport_system
                .after(systems::visuals::viewport::ui_layout_system),
        ).run_if(in_state(AppState::InGame)))
        .run();
}

// WASM-specific entry point
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#bevy-canvas".to_string()),
                    #[cfg(not(target_arch = "wasm32"))]
                    resolution: bevy::window::WindowResolution::new(720.0, 1280.0),
                    present_mode: bevy::window::PresentMode::Fifo,
                    title: "Voidrift".to_string(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: true,
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                file_path: "assets".to_string(),
                ..default()
            })
        )
        .add_plugins(EguiPlugin)
        .add_plugins(TelemetryPlugin)
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
        .insert_resource(MaxDispatch::default())
        .insert_resource(RequestsTabState::default())
        .insert_resource(ProductionTabState::default())
        .insert_resource(DeviceType::default())
        .insert_resource(systems::narrative::bottle::BottleSpawnTimer::default())
        .insert_resource(systems::persistence::save::SaveData::default())
        .insert_resource(BalanceConfig::load())
        .insert_resource(VisualConfig::load())
        .insert_resource(ContentConfig::load())
        .insert_resource(TutorialConfig::load())
        .insert_resource(QuestConfig::load())
        .insert_resource(RequestConfig::load())
        .insert_resource(LogsConfig::load())
        .insert_resource(ContentState::default())
        .insert_resource(ViewState::default())
        .init_resource::<AsteroidRespawnTimer>()
        .add_systems(Startup, (
            configure_egui_scale,
            detect_device_type,
            setup_fullscreen_resize_handler,
            systems::visuals::debug_log::setup_debug_log_system,
        ))
        .add_systems(Update, (
            update_ui_layout_from_window,
        ))
        .add_systems(OnEnter(AppState::MainMenu), (
            scenes::main_menu::setup_main_menu,
        ))
        .add_systems(OnExit(AppState::MainMenu), (
            scenes::main_menu::cleanup_menu,
        ))
        .add_systems(OnExit(AppState::InGame), (
            cleanup_world_entities,
        ))
        .add_systems(Update, (
            scenes::main_menu::main_menu_system,
            scenes::main_menu::menu_star_drift_system,
        ).run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, (
            scenes::main_menu::save_overlay_system,
        ).run_if(in_state(AppState::InGame)))
        .add_event::<systems::persistence::save::AutosaveEvent>()
        .add_event::<systems::persistence::save::SaveRequestEvent>()
        .add_event::<ShipDockedWithCargo>()
        .add_event::<ShipDockedWithBottle>()
        .add_event::<FulfillRequestEvent>()
        .add_event::<RepairStationEvent>()
        .add_event::<OpeningCompleteEvent>()
        .add_event::<DroneDispatched>()
        .add_event::<InsufficientLaserEvent>()
        .add_event::<SignalFired>()
        .add_systems(Update, (
            systems::persistence::save::autosave_system,
            systems::persistence::save::save_request_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(AppState::InGame), (
            cleanup_world_entities,
            systems::setup::reset_game_resources,
            systems::setup::setup_world,
            systems::asteroid::spawn::spawn_initial_asteroids,
            systems::visuals::debug_log::setup_debug_log_system,
            scenes::main_menu::ingame_startup_system,
        ).chain())
        .add_systems(Update, (
            systems::ship_control::autopilot::autopilot_system,
            systems::game_loop::economy::ship_docked_economy_system,
            systems::narrative::narrative_events::narrative_event_system,
            systems::visuals::map::camera_follow_system,
            systems::visuals::visuals::starfield_scroll_system,
            systems::visuals::visuals::station_rotation_system,
            systems::ship_control::autopilot::docked_ship_system,
            systems::visuals::visuals::berth_occupancy_system,
            systems::ui::hud::station_visual_system,
            systems::visuals::visuals::ship_rotation_system,
            systems::visuals::visuals::thruster_glow_system,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(OnEnter(GameState::MapView), (
            systems::visuals::map::enter_map_view,
            systems::visuals::map::show_map_elements,
        ))
        .add_systems(OnExit(GameState::MapView), (
            systems::visuals::map::exit_map_view,
            systems::visuals::map::hide_map_elements,
        ))
        .add_systems(Update, (
            systems::asteroid::spawn::asteroid_respawn_system,
            systems::asteroid::lifecycle::asteroid_lifecycle_system,
            systems::game_loop::mining::mining_system,
            systems::game_loop::auto_process::auto_refine_system,
            systems::game_loop::auto_process::auto_forge_system,
            systems::game_loop::auto_process::auto_build_drones_system,
            systems::ui::hud::ship_cargo_display_system,
            systems::ui::hud::cargo_label_system,
        ).chain().run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::ui::hud::sync_max_drones_system,
            systems::ui::tutorial::tutorial_system,
            systems::ui::hud::hud_ui_system,
            systems::visuals::map::map_highlight_system,
            systems::ship_control::asteroid_input::asteroid_input_system,
            systems::visuals::map::pinch_zoom_system,
            systems::visuals::map::map_pan_system,
            systems::narrative::opening_sequence::opening_sequence_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::narrative::opening_sequence::opening_drone_move_system,
            systems::narrative::signal::signal_system,
            systems::narrative::quest::quest_signal_system,
            systems::narrative::quest::quest_update_system,
            systems::narrative::bottle::bottle_spawn_system,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(Update, systems::narrative::bottle::bottle_input_system
            .run_if(in_state(AppState::InGame)))
        .add_systems(Update, (
            systems::narrative::content_router::content_event_system,
            systems::narrative::content_router::content_ambient_system,
            systems::narrative::logs::check_log_unlocks,
        ).run_if(in_state(AppState::InGame)))
        .add_systems(PostUpdate, (
            systems::visuals::viewport::ui_layout_system,
            systems::visuals::viewport::drawer_viewport_system
                .after(systems::visuals::viewport::ui_layout_system),
        ).run_if(in_state(AppState::InGame)))
        .run();
}
