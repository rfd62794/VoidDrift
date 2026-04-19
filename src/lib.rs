// Voidrift — Phase 4: Station UI & Refinery (Final Gate 4 Build)
// ============================================================================
// Goal: Final Phase 4 closure. Opt-C: Logic verified, text deferred.
// ============================================================================

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
};
use bevy_egui::{egui, EguiPlugin, EguiContextSettings, EguiContexts};

// ----------------------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------------------
const SHIP_SPEED: f32 = 120.0;
const ARRIVAL_THRESHOLD: f32 = 8.0;
const MAP_OVERVIEW_SCALE: f32 = 1.5;

// [PHASE 4] EGUI CONFIG (Increased to 3.0 for mobile readability)
const EGUI_SCALE: f32 = 3.0;

const CARGO_CAPACITY: u32 = 100;
const MINING_RATE: f32 = 8.0;

const REFINERY_RATIO: u32 = 10;
const REPAIR_COST: u32 = 25;

// ----------------------------------------------------------------------------
// STATES & COMPONENTS
// ----------------------------------------------------------------------------

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum GameState {
    #[default]
    SpaceView,
    MapView,
}

#[derive(Component)]
struct Ship {
    state: ShipState,
    speed: f32,
    cargo: f32,
    cargo_capacity: u32,
    power_cells: u32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ShipState {
    Idle,
    Navigating,
    Mining,
    Docked,
}

#[derive(Component)]
struct AutopilotTarget {
    destination: Vec2,
    target_entity: Option<Entity>,
}

#[derive(Component)]
struct AsteroidField;

#[derive(Component)]
struct Station {
    repair_progress: f32, // 0.0 = derelict, 1.0 = online
    online: bool,
    ore_reserves: f32,
}

#[derive(Component)]
struct MapMarker;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CargoBarFill;

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
        .add_systems(Startup, setup_world)
        .add_systems(Update, (autopilot_system, camera_follow_system))
        .add_systems(OnEnter(GameState::MapView), enter_map_view)
        .add_systems(OnExit(GameState::MapView), exit_map_view)
        .add_systems(Update, (
            mining_system, 
            cargo_display_system, 
            hud_ui_system,
            station_visual_system,
            slice_completion_system,
        ))
        .add_systems(Update, handle_input)
        .run();
}

/// Spawns the world objects, ship, and HUD.
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("[Voidrift Phase 4] Final Production Build. PresentMode: Fifo.");

    // 1. CAMERA
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 999.0),
        EguiContextSettings {
            scale_factor: EGUI_SCALE,
            ..default()
        },
    ));

    // 2. SHIP
    commands.spawn((
        Ship { 
            state: ShipState::Idle, 
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_capacity: CARGO_CAPACITY,
            power_cells: 0,
        },
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(0.0, 24.0, 1.1),
        ));
        parent.spawn((
            CargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, 1.2),
        ));
    });

    // STATION / ASTEROIDS setup
    commands.spawn((
        MapMarker,
        Station { 
            repair_progress: 0.0, 
            online: false,
            ore_reserves: 0.0,
        },
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_xyz(-150.0, -200.0, 0.5),
    ));

    commands.spawn((
        MapMarker,
        AsteroidField,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(150.0, 100.0, 0.5),
    ));
}

// ----------------------------------------------------------------------------
// SYSTEMS
// ----------------------------------------------------------------------------

fn autopilot_system(
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform, Entity)>,
    target_query: Query<&AutopilotTarget>,
    asteroid_query: Query<&AsteroidField>,
    station_query: Query<&Station>,
    mut commands: Commands,
) {
    for (mut ship, mut transform, entity) in query.iter_mut() {
        if ship.state == ShipState::Navigating {
            if let Ok(target) = target_query.get(entity) {
                let current_pos = transform.translation.truncate();
                let direction = target.destination - current_pos;
                let distance = direction.length();
                if distance < ARRIVAL_THRESHOLD {
                    if let Some(target_ent) = target.target_entity {
                        if asteroid_query.get(target_ent).is_ok() { 
                            ship.state = ShipState::Mining; 
                        }
                        else if let Ok(mut station) = station_query.get_mut(target_ent) { 
                            ship.state = ShipState::Docked; 
                            if ship.cargo > 0.0 {
                                station.ore_reserves += ship.cargo;
                                info!("[Voidrift Phase 6] Unloaded {:.1} ore into reserves. Total: {:.1}", ship.cargo, station.ore_reserves);
                                ship.cargo = 0.0;
                            }
                            info!("[Voidrift Phase 4] Gate Certification: Docked.");
                        }
                    } else { ship.state = ShipState::Idle; }
                    commands.entity(entity).remove::<AutopilotTarget>();
                } else {
                    let movement = direction.normalize() * ship.speed * time.delta_secs();
                    transform.translation += movement.extend(0.0);
                }
            }
        }
    }
}

fn mining_system(time: Res<Time>, mut query: Query<&mut Ship>) {
    for mut ship in query.iter_mut() {
        if ship.state == ShipState::Mining {
            let ore = MINING_RATE * time.delta_secs();
            ship.cargo = (ship.cargo + ore).min(ship.cargo_capacity as f32);
            if ship.cargo >= ship.cargo_capacity as f32 { ship.state = ShipState::Idle; }
        }
    }
}

fn cargo_display_system(ship: Query<&Ship>, mut fill: Query<(&mut Transform, &Parent), With<CargoBarFill>>) {
    for (mut tr, parent) in fill.iter_mut() {
        if let Ok(ship) = ship.get(**parent) {
            let r = ship.cargo / ship.cargo_capacity as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -20.0 + (20.0 * r);
        }
    }
}

fn hud_ui_system(
    mut contexts: EguiContexts,
    mut ship_query: Query<&mut Ship>,
    mut station_query: Query<&mut Station>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut ship = ship_query.single_mut();
    let ctx = contexts.ctx_mut();

    // 1. MAP TOGGLE (Always available)
    egui::SidePanel::left("navigation_panel")
        .frame(egui::Frame::NONE.fill(egui::Color32::from_black_alpha(0)))
        .show(ctx, |ui| {
            ui.add_space(16.0);
            let label = if *state.get() == GameState::SpaceView { "MAP" } else { "EXIT MAP" };
            if ui.add(egui::Button::new(label).min_size(egui::vec2(80.0, 40.0))).clicked() {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
            }
        });

    // 2. REFINERY UI (Only when docked)
    if ship.state == ShipState::Docked {
        egui::TopBottomPanel::bottom("refinery_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_space(16.0);
                    
                    // DATA COLUMN
                    ui.vertical(|ui| {
                        if let Ok(station) = station_query.get_single() {
                            ui.label(format!("ORE RESERVES: {:.1}", station.ore_reserves));
                        }
                        ui.label(format!("POWER CELLS: {}", ship.power_cells));
                    });

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(16.0);
                        
                        // REFINE BUTTON
                        if let Ok(mut station) = station_query.get_single_mut() {
                            let can_refine = station.ore_reserves >= REFINERY_RATIO as f32;
                            ui.add_enabled_ui(can_refine, |ui| {
                                if ui.button("REFINE").clicked() {
                                    let cells = (station.ore_reserves as u32) / REFINERY_RATIO;
                                    if cells > 0 {
                                        station.ore_reserves -= (cells * REFINERY_RATIO) as f32;
                                        ship.power_cells += cells;
                                        info!("[Voidrift Phase 4] Refined {} ore -> {} cells. Total: {}", 
                                            (cells * REFINERY_RATIO), cells, ship.power_cells);
                                    }
                                }
                            });
                        }

                        // REPAIR BUTTON
                        if let Ok(mut station) = station_query.get_single_mut() {
                            if !station.online {
                                ui.add_space(8.0);
                                let can_repair = ship.power_cells >= REPAIR_COST;
                                let repair_label = if can_repair { "REPAIR".to_string() } else { format!("REPAIR ({} cells)", REPAIR_COST) };
                                
                                ui.add_enabled_ui(can_repair, |ui| {
                                    if ui.button(repair_label).clicked() {
                                        ship.power_cells -= REPAIR_COST;
                                        station.repair_progress = 1.0;
                                        station.online = true;
                                        info!("[Voidrift Phase 5] Station repair complete. Slice done.");
                                    }
                                });
                            }
                        }
                    });
                });
                ui.add_space(8.0);
            });
    }
}

fn station_visual_system(
    station_query: Query<(&Station, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (station, material_handle) in &station_query {
        if station.online {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                if material.color != Color::srgb(0.0, 0.8, 1.0) {
                    material.color = Color::srgb(0.0, 0.8, 1.0);
                    info!("[Voidrift Phase 5] Station visual: online state activated.");
                }
            }
        }
    }
}

fn slice_completion_system(
    mut contexts: EguiContexts,
    station_query: Query<&Station>,
) {
    for station in &station_query {
        if station.online {
            egui::Window::new("slice_complete")
                .title_bar(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(contexts.ctx_mut(), |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("STATION ONLINE");
                        ui.label("Slice Complete.");
                    });
                });
        }
    }
}

fn camera_follow_system(state: Res<State<GameState>>, ship: Query<&Transform, (With<Ship>, Without<MainCamera>)>, mut cam: Query<&mut Transform, With<MainCamera>>) {
    let st = ship.single();
    let mut ct = cam.single_mut();
    if *state.get() == GameState::SpaceView {
        ct.translation.x = st.translation.x;
        ct.translation.y = st.translation.y;
    } else {
        ct.translation.x = 0.0;
        ct.translation.y = 0.0;
    }
}

fn enter_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { cam.single_mut().scale = MAP_OVERVIEW_SCALE; }
fn exit_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { cam.single_mut().scale = 1.0; }

fn handle_input(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            // Map interaction logic (only in MapView)
            if *state.get() == GameState::MapView {
                for (mt, me) in marker_query.iter() {
                    let mp = mt.translation.truncate();
                    if world_pos.distance(mp) < 80.0 {
                        let (ship_entity, mut ship) = ship_query.single_mut();
                        
                        // Avoid docking redundancy
                        if ship.state == ShipState::Docked && mp.distance(Vec2::new(-150.0, -200.0)) < 10.0 { 
                            continue; 
                        }

                        ship.state = ShipState::Navigating;
                        commands.entity(ship_entity).insert(AutopilotTarget { 
                            destination: mp, 
                            target_entity: Some(me) 
                        });
                        next_state.set(GameState::SpaceView);
                        break;
                    }
                }
            }
        }
    }
}
