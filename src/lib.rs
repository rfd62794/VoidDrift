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
const SHIP_SPEED: f32 = 180.0;
const ARRIVAL_THRESHOLD: f32 = 8.0;
const MAP_OVERVIEW_SCALE: f32 = 1.5;

// [PHASE 4] EGUI CONFIG (Increased to 3.0 for mobile readability)
const EGUI_SCALE: f32 = 3.0;

const CARGO_CAPACITY: u32 = 100;
const MINING_RATE: f32 = 20.0;

const REFINERY_RATIO: u32 = 10;
const HULL_REFINERY_RATIO: u32 = 5;
const REPAIR_COST: u32 = 25;
const AI_CORE_COST: u32 = 50;

const STATION_POS: Vec2 = Vec2::new(-150.0, -200.0);
const SECTOR_1_POS: Vec2 = Vec2::new(150.0, 100.0);
const SECTOR_7_POS: Vec2 = Vec2::new(350.0, 250.0);
const LOG_MAX_LINES: usize = 10;

// [PHASE 8] POWER COSTS & TIMING
const POWER_COST_CYCLE_TOTAL: u32 = 4;
const POWER_COST_REFINERY: u32 = 1;
const POWER_COST_HULL_FORGE: u32 = 2;
const POWER_WARNING_INTERVAL: f32 = 30.0;

// [PHASE 8b] POWER vs POWER CELLS
const POWER_CELL_RESTORE_VALUE: f32 = 3.0;
const SHIP_POWER_MAX: f32 = 10.0;
const SHIP_POWER_FLOOR: f32 = 3.0;
const SHIP_POWER_COST_TRANSIT: f32 = 1.0;
const SHIP_POWER_COST_MINING: f32 = 2.0;
const STATION_POWER_MAX: f32 = 50.0;
const STATION_POWER_FLOOR: f32 = 10.0;
const STATION_POWER_RESTORE_VALUE: f32 = 5.0;
const EMERGENCY_REFINE_COST: f32 = 10.0;

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
    cargo_type: OreType,
    cargo_capacity: u32,
    power: f32,
    power_cells: u32,
}

#[derive(PartialEq, Debug, Clone, Copy, Default)]
enum OreType {
    #[default]
    Empty,
    Magnetite,
    Carbon,
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
struct AsteroidField {
    ore_type: OreType,
}

#[derive(Component)]
struct Station {
    repair_progress: f32,
    online: bool,
    magnetite_reserves: f32,
    carbon_reserves: f32,
    hull_plate_reserves: u32,
    power_cells: u32,
    power: f32,
    maintenance_timer: Timer,
    last_power_warning_time: f32, // Track reminder timing
    log: std::collections::VecDeque<String>,
}

#[derive(Component)]
struct MapMarker;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CargoBarFill;

#[derive(Component)]
struct AiCore;

#[derive(Component)]
struct AutonomousShip {
    state: AutonomousShipState,
    cargo: f32,
    cargo_type: OreType,
    power: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum AutonomousShipState {
    Holding,    // At station, awaiting sufficient power
    Outbound,   // Traveling to field
    Mining,     // Actively extracting
    Returning,  // Returning to station
    Unloading,  // Depositing cargo
}

#[derive(Component)]
struct AutonomousAssignment {
    target_pos: Vec2,
    ore_type: OreType,
    sector_name: String,
}

#[derive(Component)]
struct ShipCargoBarFill;

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
            hud_ui_system,
            station_visual_system,
            autonomous_ship_system,
            ship_cargo_display_system,
            autonomous_ship_cargo_display_system,
            station_status_system,
            ship_self_preservation_system,
            station_maintenance_system,
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
            cargo_type: OreType::Empty,
            cargo_capacity: CARGO_CAPACITY,
            power: SHIP_POWER_MAX,
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
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, 1.2),
        ));
    });

    // STATION / SECTORS setup
    commands.spawn((
        MapMarker,
        Station { 
            repair_progress: 0.0, 
            online: false,
            magnetite_reserves: 0.0,
            carbon_reserves: 0.0,
            hull_plate_reserves: 0,
            power_cells: 0,
            power: STATION_POWER_MAX,
            maintenance_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            last_power_warning_time: 0.0,
            log: std::collections::VecDeque::from([
                "SYSTEMS INITIALIZED.".to_string(),
            ]),
        },
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, 0.5),
    ));

    // Sector 1: Magnetite (Initial)
    commands.spawn((
        MapMarker,
        AsteroidField { ore_type: OreType::Magnetite },
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.3, 0.3))), // Reddish
        Transform::from_xyz(SECTOR_1_POS.x, SECTOR_1_POS.y, 0.5),
    ));

    // Sector 7: Carbon (Hidden)
    // We spawn it without MapMarker initially
    commands.spawn((
        AsteroidField { ore_type: OreType::Carbon },
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.8, 0.3))), // Greenish
        Transform::from_xyz(SECTOR_7_POS.x, SECTOR_7_POS.y, 0.5),
        Visibility::Hidden,
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
    mut station_query: Query<(Entity, &mut Station)>,
    ai_core_query: Query<&AiCore>,
    carbon_field_query: Query<Entity, (With<AsteroidField>, Without<MapMarker>)>,
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
                        else if let Ok((_station_ent, mut station)) = station_query.get_mut(target_ent) { 
                            ship.state = ShipState::Docked; 
                            ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                            
                            // [PHASE 8b] Reset player power for free if station has power
                            if station.power >= STATION_POWER_FLOOR {
                                ship.power = SHIP_POWER_MAX;
                            }

                            // [PHASE 8b] Automatic deposit of cells to ship (up to 3, cap 5)
                            if station.power_cells > 10 && ship.power_cells < 5 {
                                let transfer = (3 as u32).min(5 - ship.power_cells);
                                if station.power_cells >= transfer {
                                    station.power_cells -= transfer;
                                    ship.power_cells += transfer;
                                }
                            }

                            if ship.cargo > 0.0 {
                                match ship.cargo_type {
                                    OreType::Magnetite => {
                                        station.magnetite_reserves += ship.cargo;
                                        let msg = format!("[STATION AI] Magnetite reserves: {}. Power Cells: {}.", station.magnetite_reserves as u32, station.power_cells);
                                        add_log_entry(&mut station, msg);
                                    }
                                    OreType::Carbon => {
                                        station.carbon_reserves += ship.cargo;
                                        let msg = format!("[STATION AI] Carbon reserves: {}. Hull Plates: {}.", station.carbon_reserves as u32, station.hull_plate_reserves);
                                        add_log_entry(&mut station, msg);
                                        if station.hull_plate_reserves == 0 && station.carbon_reserves >= HULL_REFINERY_RATIO as f32 {
                                            add_log_entry(&mut station, "[STATION AI] Hull synthesis possible. Second AI Core required for autonomous operation.".to_string());
                                        }
                                    }
                                    OreType::Empty => {}
                                }
                                ship.cargo = 0.0;
                                ship.cargo_type = OreType::Empty;
                            }
                            
                            // SECTOR 7 DISCOVERY LOGIC
                            if ai_core_query.get(station_ent).is_ok() {
                                if let Ok(s7_ent) = carbon_field_query.get_single() {
                                    commands.entity(s7_ent).insert((MapMarker, Visibility::Visible));
                                    add_log_entry(&mut station, "[STATION AI] Carbon signature detected. Bearing 047. Hull-grade yield confirmed. Designation: Sector 7.".to_string());
                                }
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

fn mining_system(time: Res<Time>, mut ship_query: Query<(&mut Ship, &Transform)>, field_query: Query<(&AsteroidField, &Transform)>) {
    for (mut ship, ship_transform) in ship_query.iter_mut() {
        if ship.state == ShipState::Mining {
            // Find nearby field to determine ore type
            for (field, field_transform) in field_query.iter() {
                if ship_transform.translation.distance(field_transform.translation) < 50.0 {
                    if ship.cargo_type == OreType::Empty {
                        ship.cargo_type = field.ore_type;
                    }
                    let ore = MINING_RATE * time.delta_secs();
                    ship.cargo = (ship.cargo + ore).min(ship.cargo_capacity as f32);
                    if ship.cargo >= ship.cargo_capacity as f32 { 
                        ship.state = ShipState::Idle; 
                        ship.power = (ship.power - SHIP_POWER_COST_MINING).max(0.0);
                    }
                    break;
                }
            }
        }
    }
}

fn add_log_entry(station: &mut Station, entry: String) {
    // Avoid duplicate consecutive logs if possible (optional but cleaner)
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
    if station.log.len() > LOG_MAX_LINES {
        station.log.pop_front();
    }
}

fn ship_cargo_display_system(ship_query: Query<&Ship>, mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (mut tr, parent, mat_handle) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let r = ship.cargo / ship.cargo_capacity as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -20.0 + (20.0 * r);
            
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = match ship.cargo_type {
                    OreType::Magnetite => Color::srgb(0.8, 0.3, 0.3),
                    OreType::Carbon => Color::srgb(0.3, 0.8, 0.3),
                    OreType::Empty => Color::srgb(0.5, 0.5, 0.5),
                };
            }
        }
    }
}

fn autonomous_ship_cargo_display_system(ship_query: Query<&AutonomousShip>, mut fill_query: Query<(&mut Transform, &Parent, &mut MeshMaterial2d<ColorMaterial>), With<ShipCargoBarFill>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    for (mut tr, parent, mat_handle) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let r = ship.cargo / CARGO_CAPACITY as f32;
            tr.scale.x = r.max(0.001);
            tr.translation.x = -15.0 + (15.0 * r);
            
            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                mat.color = Color::srgb(1.0, 0.5, 0.0); // Autonomous ships are orange
            }
        }
    }
}

fn hud_ui_system(
    mut contexts: EguiContexts,
    mut ship_query: Query<&mut Ship>,
    mut station_query: Query<(Entity, &mut Station)>,
    ai_core_query: Query<&AiCore>,
    auto_ship_query: Query<&AutonomousShip>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

    // 2. REFINERY & LOG UI (Only when docked)
    if ship.state == ShipState::Docked {
        egui::TopBottomPanel::bottom("refinery_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(8.0);
                
                if let Ok((station_ent, mut station)) = station_query.get_single_mut() {
                    ui.vertical_centered(|ui| {
                        // SECTION 1: SYSTEM LOG (Fixed 5-line height)
                        ui.group(|ui| {
                            ui.set_height(60.0); 
                            egui::ScrollArea::vertical()
                                .stick_to_bottom(true)
                                .show(ui, |ui| {
                                    for line in &station.log {
                                        ui.label(egui::RichText::new(line).monospace().size(9.0).color(egui::Color32::LIGHT_GRAY));
                                    }
                                });
                        });
                        ui.add_space(4.0);

                        // SECTION 2: RESOURCE STATUS BAR
                        ui.horizontal(|ui| {
                            ui.add_space(8.0);
                            ui.label(format!("MAG: {:.0}", station.magnetite_reserves));
                            ui.separator();
                            ui.label(format!("CAR: {:.0}", station.carbon_reserves));
                            ui.separator();
                            ui.label(format!("CELLS: {}", station.power_cells));
                            ui.separator();
                            ui.label(format!("HULLS: {}", station.hull_plate_reserves));
                        });
                        ui.add_space(4.0);

                        // SECTION 3: ACTION BUTTONS (Split into 2 rows for touch safety)
                        ui.horizontal(|ui| {
                            // Row 1: Production
                            let can_refine_mag = station.magnetite_reserves >= REFINERY_RATIO as f32;
                            let has_power_mag = station.power_cells >= POWER_COST_REFINERY;
                            let label_mag = if has_power_mag { "REFINE CELLS" } else { "REFINERY OFFLINE" };
                            
                            if ui.add_sized([110.0, 30.0], egui::Button::new(label_mag)).clicked() && can_refine_mag && has_power_mag {
                                let cells = (station.magnetite_reserves as u32) / REFINERY_RATIO;
                                station.magnetite_reserves -= (cells * REFINERY_RATIO) as f32;
                                station.power_cells += cells;
                                station.power_cells -= POWER_COST_REFINERY;
                                add_log_entry(&mut station, format!("[STATION AI] Magnetite refined -> {} cells.", cells));
                            }

                            let can_refine_carb = station.carbon_reserves >= HULL_REFINERY_RATIO as f32;
                            let has_power_hull = station.power_cells >= POWER_COST_HULL_FORGE;
                            let label_hull = if has_power_hull { "REFINE HULL" } else { "FORGE OFFLINE" };

                            if ui.add_sized([110.0, 30.0], egui::Button::new(label_hull)).clicked() && can_refine_carb && has_power_hull {
                                let plates = (station.carbon_reserves as u32) / HULL_REFINERY_RATIO;
                                station.carbon_reserves -= (plates * HULL_REFINERY_RATIO) as f32;
                                station.hull_plate_reserves += plates;
                                station.power_cells -= POWER_COST_HULL_FORGE;
                                add_log_entry(&mut station, format!("[STATION AI] Hull synthesis complete: {} units.", plates));
                            }
                        });

                        ui.add_space(4.0);

                        ui.horizontal(|ui| {
                            // Row 2: Construction & Repair
                            if ai_core_query.get(station_ent).is_err() {
                                let can_build_core = station.power_cells >= AI_CORE_COST;
                                let core_label = if can_build_core { format!("AI CORE ({})", AI_CORE_COST) } else { "CORE LACKS POWER".to_string() };
                                if ui.add_sized([120.0, 30.0], egui::Button::new(core_label)).clicked() && can_build_core {
                                    station.power_cells -= AI_CORE_COST;
                                    commands.entity(station_ent).insert(AiCore);
                                    add_log_entry(&mut station, "[STATION AI] AI Core nominal. Awaiting directive.".to_string());
                                }
                            } else if auto_ship_query.iter().count() < 2 {
                                let can_assemble = station.hull_plate_reserves >= 1;
                                if ui.add_sized([120.0, 30.0], egui::Button::new("ASSEMBLE SHIP")).clicked() && can_assemble {
                                    station.hull_plate_reserves -= 1;
                                    commands.entity(station_ent).remove::<AiCore>();
                                    
                                    let ship_count = auto_ship_query.iter().count();
                                    let (target_pos, ore, name) = if ship_count == 0 {
                                        (SECTOR_1_POS, OreType::Magnetite, "Sector 1".to_string())
                                    } else {
                                        (SECTOR_7_POS, OreType::Carbon, "Sector 7".to_string())
                                    };

                                    commands.spawn((
                                        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: ore },
                                        AutonomousAssignment { target_pos, ore_type: ore, sector_name: name.clone() },
                                        Mesh2d(meshes.add(Rectangle::new(24.0, 24.0))),
                                        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                        Transform::from_xyz(STATION_POS.x, STATION_POS.y, 0.5),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
                                            Transform::from_xyz(0.0, 24.0, 1.1),
                                        ));
                                        parent.spawn((
                                            ShipCargoBarFill,
                                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
                                            Transform::from_xyz(0.0, 24.0, 1.2),
                                        ));
                                    });
                                    add_log_entry(&mut station, format!("[STATION AI] Autonomous unit assigned. {}. {} extraction.", name, if ore == OreType::Magnetite { "Magnetite" } else { "Carbon" }));
                                }
                            }

                            if !station.online {
                                let can_repair = station.power_cells >= REPAIR_COST;
                                if ui.add_sized([80.0, 30.0], egui::Button::new("REPAIR")).clicked() && can_repair {
                                    station.power_cells -= REPAIR_COST;
                                    station.repair_progress = 1.0;
                                    station.online = true;
                                    add_log_entry(&mut station, "[STATION AI] Repair complete. Power grid online.".to_string());
                                }
                            }
                        });
                    });
                }
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
            for (mt, me) in marker_query.iter() {
                let mp = mt.translation.truncate();
                if world_pos.distance(mp) < 80.0 {
                    let (ship_entity, mut ship) = ship_query.single_mut();
                    
                    // Avoid docking redundancy
                    if ship.state == ShipState::Docked && mp.distance(STATION_POS) < 10.0 { 
                        continue; 
                    }

                    ship.state = ShipState::Navigating;
                    ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    commands.entity(ship_entity).insert(AutopilotTarget { 
                        destination: mp, 
                        target_entity: Some(me) 
                    });

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}

fn autonomous_ship_system(
    time: Res<Time>,
    mut ship_query: Query<(&mut AutonomousShip, &mut Transform, &AutonomousAssignment)>,
    mut station_query: Query<&mut Station>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        for (mut ship, mut transform, assignment) in ship_query.iter_mut() {
            match ship.state {
                AutonomousShipState::Holding => {
                    if station.power_cells >= POWER_COST_CYCLE_TOTAL {
                        station.power_cells -= POWER_COST_CYCLE_TOTAL;
                        ship.state = AutonomousShipState::Outbound;
                        add_log_entry(&mut station, "[STATION AI] Power confirmed. Dispatching autonomous unit.".to_string());
                    }
                }
                AutonomousShipState::Outbound => {
                    let direction = assignment.target_pos - transform.translation.truncate();
                    let distance = direction.length();
                    if distance < ARRIVAL_THRESHOLD {
                        ship.state = AutonomousShipState::Mining;
                        ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Mining => {
                    ship.cargo = (ship.cargo + MINING_RATE * time.delta_secs()).min(CARGO_CAPACITY as f32);
                    if ship.cargo >= CARGO_CAPACITY as f32 {
                        ship.state = AutonomousShipState::Returning;
                        ship.power = (ship.power - SHIP_POWER_COST_MINING).max(0.0);
                    }
                }
                AutonomousShipState::Returning => {
                    let direction = STATION_POS - transform.translation.truncate();
                    let distance = direction.length();
                    if distance < ARRIVAL_THRESHOLD {
                        ship.state = AutonomousShipState::Unloading;
                        ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Unloading => {
                    let ore_name = if assignment.ore_type == OreType::Magnetite { "Magnetite" } else { "Carbon" };
                    match assignment.ore_type {
                        OreType::Magnetite => station.magnetite_reserves += ship.cargo,
                        OreType::Carbon => station.carbon_reserves += ship.cargo,
                        _ => {}
                    }
                    // [PHASE 8b] Recharge autonomous ship using station cells
                    if station.power_cells > 0 {
                        station.power_cells -= 1;
                        ship.power = SHIP_POWER_MAX;
                    }

                    let msg = format!("[STATION AI] Cargo deposited: {}. {} recovered.", assignment.sector_name, ore_name);
                    add_log_entry(&mut station, msg);
                    ship.cargo = 0.0;
                    
                    // Return to holding or critical return
                    if ship.power < 2.0 {
                         add_log_entry(&mut station, "[STATION AI] Autonomous unit returned. Low power. Recharging.".to_string());
                    }
                    ship.state = AutonomousShipState::Holding;
                }
            }
        }
    }
}

fn station_status_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    ship_query: Query<&AutonomousShip>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        let now = time.elapsed_secs();
        let power = station.power_cells;
        
        let should_warn = now - station.last_power_warning_time > POWER_WARNING_INTERVAL || station.last_power_warning_time == 0.0;
        
        // 1. Critical Power Warning
        if power < POWER_COST_CYCLE_TOTAL && should_warn {
            add_log_entry(&mut station, format!("[STATION AI] Power reserves critical. Reserve: {} cells.", power));
            station.last_power_warning_time = now;
        }
        
        // 2. Ships Holding
        let any_holding = ship_query.iter().any(|s| s.state == AutonomousShipState::Holding);
        if any_holding && power < POWER_COST_CYCLE_TOTAL && should_warn {
             add_log_entry(&mut station, "[STATION AI] Insufficient power. Autonomous unit holding.".to_string());
             station.last_power_warning_time = now;
        }

        // 3. Automation Suspension Notice (Log once on state change)
        if station.power < STATION_POWER_FLOOR && station.online {
             // Already handled by maintenance for now, but good to have a dedicated check if needed
        }
    }
}
