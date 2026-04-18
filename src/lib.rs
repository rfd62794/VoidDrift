// Voidrift — Phase 3 Diagnostic Step 1: Stable Floor (v2)
// ============================================================================
// Goal: Resolve the engine freeze ("No movement") and buffer starvation.
// Changes: Disabled MSAA, removed all child entities, restored Fullscreen.
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

// ----------------------------------------------------------------------------
// APP SETUP
// ----------------------------------------------------------------------------

#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Restoring Fullscreen as it worked for touches in Phase 2
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                present_mode: bevy::window::PresentMode::Fifo,
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        // [DIAGNOSTIC] Disable MSAA to reduce buffer pressure on Mali GPU
        .insert_resource(Msaa::Off)
        .init_state::<GameState>()
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .add_systems(Startup, setup_world)
        .add_systems(Update, (autopilot_system, camera_follow_system))
        .add_systems(OnEnter(GameState::MapView), enter_map_view)
        .add_systems(OnExit(GameState::MapView), exit_map_view)
        .add_systems(Update, handle_input)
        .run();
}

/// Spawns the camera, ship, and markers. NO CHILDREN, NO TEXT.
fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. CAMERA
    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));

    // 2. SHIP (No children)
    commands.spawn((
        Ship { 
            state: ShipState::Idle, 
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_capacity: CARGO_CAPACITY,
        },
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    // 3. STATION
    commands.spawn((
        MapMarker,
        Station,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_xyz(-150.0, -200.0, 0.5),
    ));

    // 4. ASTEROID FIELD
    commands.spawn((
        MapMarker,
        AsteroidField,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
        Transform::from_xyz(150.0, 100.0, 0.5),
    ));

    // 5. MAP TOGGLE BUTTON
    commands.spawn((
        MapToggleButton,
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.2))),
        Transform::from_xyz(300.0, 500.0, 10.0), 
    ));

    info!("[Voidrift Phase 3] DIAGNOSTIC: Stable Floor v2 (Fullscr, Msaa::Off, No children).");
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
                            ship.cargo = 0.0;
                            ship.state = ShipState::Idle;
                        }
                    } else {
                        ship.state = ShipState::Idle;
                    }
                    commands.entity(entity).remove::<AutopilotTarget>();
                    info!("[Voidrift Phase 3] Arrived at destination.");
                } else {
                    let move_dir = direction.normalize();
                    let movement = move_dir * ship.speed * time.delta_secs();
                    transform.translation += movement.extend(0.0);
                }
            }
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
    info!("[Voidrift Phase 3] Exited Map View.");
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
            info!("[Voidrift Phase 3] Touch: {:?} -> World: {:?}", touch_pos, world_pos);
            
            let (toggle_transform, _) = toggle_query.single();
            let button_pos = toggle_transform.translation.truncate();
            
            if world_pos.distance(button_pos) < 80.0 {
                if *state.get() == GameState::SpaceView {
                    next_state.set(GameState::MapView);
                } else {
                    next_state.set(GameState::SpaceView);
                }
                info!("[Voidrift Phase 3] Toggle Button Hit.");
                continue;
            }

            if *state.get() == GameState::MapView {
                for (marker_transform, marker_ent) in marker_query.iter() {
                    let marker_pos = marker_transform.translation.truncate();
                    if world_pos.distance(marker_pos) < 80.0 {
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
