// Voidrift — Phase 3 Mining System
// ============================================================================
// Scope: Phase 3 ONLY.
// Deliverable: Mining Accumulation, Cargo Bar, Arrival Transitions.
//
// Debug: Reverted Cargo Bar to Mesh2d instead of Sprites to resolve Mali GPU
// gralloc format errors ("Can't acquire next buffer").
// ============================================================================

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::MeshMaterial2d,
    text::JustifyText,
};

// ----------------------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------------------
const SHIP_SPEED: f32 = 120.0;
const ARRIVAL_THRESHOLD: f32 = 8.0;
const MAP_OVERVIEW_SCALE: f32 = 1.5;

const CARGO_CAPACITY: u32 = 100;
const MINING_RATE: f32 = 8.0; // Units per second

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
    mining_log_timer: f32,
}

#[derive(PartialEq, Debug)]
enum ShipState {
    Idle,
    Navigating,
    Mining,
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
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .add_systems(Startup, setup_world)
        // Physics & Follow
        .add_systems(Update, (autopilot_system, camera_follow_system))
        // View Transitions
        .add_systems(OnEnter(GameState::MapView), enter_map_view)
        .add_systems(OnExit(GameState::MapView), exit_map_view)
        // Gameplay systems
        .add_systems(Update, (mining_system, cargo_display_system))
        // Input logic
        .add_systems(Update, handle_input)
        .run();
}

/// Spawns the camera, ship, and markers.
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");

    // 1. CAMERA
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));

    // 2. SHIP (World Space)
    commands.spawn((
        Ship { 
            state: ShipState::Idle, 
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_capacity: CARGO_CAPACITY,
            mining_log_timer: 0.0,
        },
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), // Cyan
        Transform::from_xyz(0.0, 0.0, 1.0),
    ))
    .with_children(|parent| {
        // Cargo Bar Background (Mesh2d)
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(0.0, 24.0, 1.1),
        ));
        // Cargo Bar Fill (Mesh2d)
        parent.spawn((
            CargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, 1.2), // Initial center, will be updated
        ));
    });

    // 3. STATION (Marker)
    spawn_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        font.clone(),
        "Station",
        Vec2::new(-150.0, -200.0),
        Color::srgb(1.0, 1.0, 0.0), // Yellow
        true, // IsStation
    );

    // 4. ASTEROID FIELD (Marker)
    spawn_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        font.clone(),
        "Asteroid Field",
        Vec2::new(150.0, 100.0),
        Color::srgb(0.5, 0.5, 0.5), // Grey
        false, // IsAsteroidField
    );

    // 5. MAP TOGGLE BUTTON (HUD - Child of camera)
    commands.spawn((
        MapToggleButton,
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.2))), // Red button
        Transform::from_xyz(300.0, 500.0, 10.0), 
    ));

    info!("[Voidrift Phase 3] World Initialized with Mesh2d Cargo Bars.");
}

fn spawn_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    font: Handle<Font>,
    label: &str,
    pos: Vec2,
    color: Color,
    is_station: bool,
) {
    let mut entity = commands.spawn((
        MapMarker,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.x, pos.y, 0.5),
    ));

    if is_station {
        entity.insert(Station);
    } else {
        entity.insert(AsteroidField);
    }

    entity.with_children(|parent| {
        parent.spawn((
            Text2d::new(label),
            TextFont { font, font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(JustifyText::Center),
            Transform::from_xyz(0.0, 40.0, 0.1),
        ));
    });
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
                    // Check destination tag
                    if let Some(target_ent) = target.target_entity {
                        if asteroid_query.get(target_ent).is_ok() {
                            ship.state = ShipState::Mining;
                            info!("[Voidrift Phase 3] Arrived at Asteroid Field. Mining started.");
                        } else if station_query.get(target_ent).is_ok() {
                            if ship.cargo > 0.0 {
                                info!("[Voidrift Phase 3] Cargo unloaded at Station. ({:.0} units)", ship.cargo);
                                ship.cargo = 0.0;
                            }
                            ship.state = ShipState::Idle;
                            info!("[Voidrift Phase 3] Arrived at Station.");
                        } else {
                            ship.state = ShipState::Idle;
                        }
                    } else {
                        ship.state = ShipState::Idle;
                    }
                    
                    commands.entity(entity).remove::<AutopilotTarget>();
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
            
            // Logging timer
            ship.mining_log_timer += time.delta_secs();
            if ship.mining_log_timer >= 1.0 {
                info!("[Voidrift Phase 3] Cargo: {:.0}/{}", ship.cargo, ship.cargo_capacity);
                ship.mining_log_timer = 0.0;
            }

            if ship.cargo >= ship.cargo_capacity as f32 {
                ship.state = ShipState::Idle;
                info!("[Voidrift Phase 3] Cargo full — mining complete.");
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
            
            // Manual Left-Anchor Math for Mesh2d:
            // Background is 40.0 wide, centered at 0.0. 
            // Left edge is at X = -20.0.
            // Width of target bar: W = 40.0 * ratio.
            // Center position for Mesh2d: X_center = -20.0 + W/2.0
            
            let fill_width = 40.0 * fill_ratio;
            transform.scale.x = fill_ratio;
            transform.translation.x = -20.0 + (fill_width / 2.0);
        }
    }
}

fn camera_follow_system(
    state: Res<State<GameState>>,
    ship_query: Query<&Transform, (With<Ship>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    mut toggle_query: Query<&mut Transform, (With<MapToggleButton>, Without<MainCamera>, Without<Ship>)>,
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

    if let Ok(mut toggle_transform) = toggle_query.get_single_mut() {
        toggle_transform.translation.x = camera_transform.translation.x + 300.0;
        toggle_transform.translation.y = camera_transform.translation.y + 500.0;
    }
}

fn enter_map_view(mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = camera_query.single_mut();
    projection.scale = MAP_OVERVIEW_SCALE;
    info!("[Voidrift Phase 3] Entered Map View.");
}

fn exit_map_view(mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = camera_query.single_mut();
    projection.scale = 1.0;
    info!("[Voidrift Phase 2] Exited Map View.");
}

fn handle_input(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    toggle_query: Query<(&Transform, Entity), With<MapToggleButton>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>, Without<MapToggleButton>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    
    for touch in touches.iter_just_pressed() {
        let touch_pos = touch.position();
        
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch_pos) {
            
            // 1. Check Toggle Button (HUD)
            let (toggle_transform, _) = toggle_query.single();
            if world_pos.distance(toggle_transform.translation.truncate()) < 40.0 {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
                continue;
            }

            // 2. Check Map Markers (only in MapView)
            if *state.get() == GameState::MapView {
                for (marker_transform, marker_ent) in marker_query.iter() {
                    let marker_pos = marker_transform.translation.truncate();
                    if world_pos.distance(marker_pos) < 50.0 {
                        let (ship_entity, mut ship) = ship_query.single_mut();
                        ship.state = ShipState::Navigating;
                        commands.entity(ship_entity).insert(AutopilotTarget {
                            destination: marker_pos,
                            target_entity: Some(marker_ent),
                        });
                        next_state.set(GameState::SpaceView);
                        info!("[Voidrift Phase 3] Autopilot Engaged: {:?}", marker_pos);
                        break;
                    }
                }
            }
        }
    }
}
