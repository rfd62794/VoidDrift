// Voidrift — Phase 4: Station UI & Refinery (Deploy 1)
// ============================================================================
// Features: power_cells model, ShipState::Docked, Refinery Logic.
// ============================================================================

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
};

// ----------------------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------------------
const SHIP_SPEED: f32 = 120.0;
const ARRIVAL_THRESHOLD: f32 = 8.0;
const MAP_OVERVIEW_SCALE: f32 = 1.5;

const CARGO_CAPACITY: u32 = 100;
const MINING_RATE: f32 = 8.0;

// [PHASE 4] ECONOMIC CONSTANTS
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
    power_cells: u32,       // [PHASE 4]
    mining_log_timer: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum ShipState {
    Idle,
    Navigating,
    Mining,
    Docked,                 // [PHASE 4]
}

#[derive(Component)]
struct AutopilotTarget {
    destination: Vec2,
    target_entity: Option<Entity>,
}

#[derive(Component)]
struct AsteroidField;

#[derive(Component)]
struct Station;

#[derive(Component)]
struct MapMarker;

#[derive(Component)]
struct MapToggleButton;

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct CargoBarFill;

#[derive(Component)]
struct DockingUIPanel;      // [PHASE 4]

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
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .add_systems(Startup, setup_world)
        .add_systems(Update, (autopilot_system, camera_follow_system))
        .add_systems(OnEnter(GameState::MapView), enter_map_view)
        .add_systems(OnExit(GameState::MapView), exit_map_view)
        .add_systems(Update, (mining_system, cargo_display_system, update_docking_ui_visibility))
        .add_systems(Update, handle_input)
        .run();
}

/// Spawns the world objects, ship, and HUD.
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. CAMERA + HUD CHILD
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 999.0),
    ))
    .with_children(|parent| {
        // [HUD] Toggle Button
        parent.spawn((
            MapToggleButton,
            Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
            Transform::from_xyz(100.0, 200.0, -1.0),
        ))
        .with_children(|btn| {
             btn.spawn((
                Text2d::new("MAP"),
                TextFont::from_font_size(24.0),
                Transform::from_xyz(0.0, 0.0, 0.1),
            ));
        });

        // [PHASE 4] DOCKING UI Foundation (Visibility::Hidden)
        // Spawned as child of Camera for fixed HUD positioning.
        parent.spawn((
            DockingUIPanel,
            Mesh2d(meshes.add(Rectangle::new(200.0, 120.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.05, 0.05, 0.1, 0.9))),
            Transform::from_xyz(0.0, 0.0, -0.5),
            Visibility::Hidden,
        ));
    });

    // 2. SHIP
    commands.spawn((
        Ship { 
            state: ShipState::Idle, 
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_capacity: CARGO_CAPACITY,
            power_cells: 0,
            mining_log_timer: 0.0,
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

    // 3. STATION
    commands.spawn((
        MapMarker,
        Station,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_xyz(-150.0, -200.0, 0.5),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text2d::new("STATION"),
            TextFont::from_font_size(18.0),
            Transform::from_xyz(0.0, -40.0, 0.1),
        ));
    });

    // 4. ASTEROID FIELD
    commands.spawn((
        MapMarker,
        AsteroidField,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(150.0, 100.0, 0.5),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text2d::new("ASTEROID FIELD"),
            TextFont::from_font_size(18.0),
            Transform::from_xyz(0.0, -40.0, 0.1),
        ));
    });

    info!("[Voidrift Phase 4] Model extensions & UI Foundation initialized.");
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
                        } else if station_query.get(target_ent).is_ok() {
                            // [PHASE 4] Transition to Docked & Unload
                            ship.state = ShipState::Docked;
                            // Unload cargo (as per current behavior, move to refinery later if needed)
                            // ship.cargo = 0.0; // Keep cargo for refinery!
                            info!("[Voidrift Phase 4] Docked at Station. Ore ready for refinery.");
                        }
                    } else {
                        ship.state = ShipState::Idle;
                    }
                    commands.entity(entity).remove::<AutopilotTarget>();
                    info!("[Voidrift Phase 4] Arrived at destination.");
                } else {
                    let move_dir = direction.normalize();
                    let movement = move_dir * ship.speed * time.delta_secs();
                    transform.translation += movement.extend(0.0);
                }
            }
        }
    }
}

fn mining_system(
    time: Res<Time>,
    mut query: Query<&mut Ship>,
) {
    for mut ship in query.iter_mut() {
        if ship.state == ShipState::Mining {
            let ore_this_tick = MINING_RATE * time.delta_secs();
            ship.cargo = (ship.cargo + ore_this_tick).min(ship.cargo_capacity as f32);
            
            ship.mining_log_timer += time.delta_secs();
            if ship.mining_log_timer >= 1.0 {
                info!("[Voidrift Phase 4] Cargo: {:.1}/{}", ship.cargo, ship.cargo_capacity);
                ship.mining_log_timer = 0.0;
            }

            if ship.cargo >= ship.cargo_capacity as f32 {
                ship.state = ShipState::Idle;
                info!("[Voidrift Phase 4] Mining full.");
            }
        }
    }
}

fn cargo_display_system(
    ship_query: Query<&Ship>,
    mut fill_query: Query<(&mut Transform, &Parent), With<CargoBarFill>>,
) {
    for (mut transform, parent) in fill_query.iter_mut() {
        if let Ok(ship) = ship_query.get(**parent) {
            let fill_ratio = ship.cargo / ship.cargo_capacity as f32;
            let fill_width = 40.0 * fill_ratio;
            transform.scale.x = fill_ratio;
            transform.translation.x = -20.0 + (fill_width / 2.0);
        }
    }
}

fn update_docking_ui_visibility(
    ship_query: Query<&Ship>,
    mut ui_query: Query<&mut Visibility, With<DockingUIPanel>>,
) {
    let ship = ship_query.single();
    let mut ui_visibility = ui_query.single_mut();

    if ship.state == ShipState::Docked {
        *ui_visibility = Visibility::Visible;
    } else {
        *ui_visibility = Visibility::Hidden;
    }
}

fn camera_follow_system(
    state: Res<State<GameState>>,
    ship_query: Query<&Transform, (With<Ship>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let ship_transform = ship_query.single();
    let mut camera_transform = camera_query.single_mut();

    if *state.get() == GameState::SpaceView {
        camera_transform.translation.x = ship_transform.translation.x;
        camera_transform.translation.y = ship_transform.translation.y;
    } else {
        camera_transform.translation.x = 0.0;
        camera_transform.translation.y = 0.0;
    }
}

fn enter_map_view(mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = camera_query.single_mut();
    projection.scale = MAP_OVERVIEW_SCALE;
}

fn exit_map_view(mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = camera_query.single_mut();
    projection.scale = 1.0;
}

fn handle_input(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    toggle_query: Query<(&GlobalTransform, Entity), With<MapToggleButton>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>, Without<MapToggleButton>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    
    for touch in touches.iter_just_pressed() {
        let touch_pos = touch.position();
        
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch_pos) {
            
            // 1. Check HUD Button
            let (toggle_g_transform, _) = toggle_query.single();
            let button_world_pos = toggle_g_transform.translation().truncate();
            
            if world_pos.distance(button_world_pos) < 60.0 {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
                info!("[Voidrift Phase 4] Toggle Button Hit.");
                continue;
            }

            // 2. Check Map Markers
            if *state.get() == GameState::MapView {
                for (marker_transform, marker_ent) in marker_query.iter() {
                    let marker_pos = marker_transform.translation.truncate();
                    if world_pos.distance(marker_pos) < 80.0 {
                        let (ship_entity, mut ship) = ship_query.single_mut();
                        
                        // Departure logic
                        ship.state = ShipState::Navigating;
                        commands.entity(ship_entity).insert(AutopilotTarget {
                            destination: marker_pos,
                            target_entity: Some(marker_ent),
                        });
                        next_state.set(GameState::SpaceView);
                        info!("[Voidrift Phase 4] Departing for destination: {:?}", marker_pos);
                        break;
                    }
                }
            }
        }
    }
}
