// Voidrift — Phase 2 Map & Navigation
// ============================================================================
// Scope: Phase 2 ONLY.
// Deliverable: Map View, Autopilot Move, Camera Follow.
// Hard Scope: No mining, no station UI, no space-view touch commands.
//
// Gate 2 behaviours this file must satisfy:
//   TB-P2-01: Map Toggle button opens MapView
//   TB-P2-02/03: Tapping marker sets destination
//   TB-P2-04: Ship navigates and stops at threshold
//   TB-P2-05: Camera follows ship in SpaceView
// ============================================================================

use bevy::{
    prelude::*,
    render::mesh::Mesh2d,
    sprite::{MeshMaterial2d, Anchor},
    text::*,
};

// ----------------------------------------------------------------------------
// CONSTANTS
// ----------------------------------------------------------------------------
const SHIP_SPEED: f32 = 120.0;
const ARRIVAL_THRESHOLD: f32 = 8.0;
/// Manual calculate to fit markers (-150, -200) and (150, 100) + padding.
const MAP_OVERVIEW_SCALE: f32 = 1.5;

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
}

#[derive(PartialEq, Debug)]
enum ShipState {
    Idle,
    Navigating,
}

#[derive(Component)]
struct AutopilotTarget {
    destination: Vec2,
}

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
        Ship { state: ShipState::Idle, speed: SHIP_SPEED },
        Mesh2d(meshes.add(Rectangle::new(32.0, 32.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))), // Cyan
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));

    // 3. STATION (Marker)
    spawn_marker(
        &mut commands,
        &mut meshes,
        &mut materials,
        font.clone(),
        "Station",
        Vec2::new(-150.0, -200.0),
        Color::srgb(1.0, 1.0, 0.0), // Yellow
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
    );

    // 5. MAP TOGGLE BUTTON (HUD - Child of camera)
    commands.spawn((
        MapToggleButton,
        Mesh2d(meshes.add(Rectangle::new(60.0, 60.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.2, 0.2))), // Red button
        Transform::from_xyz(300.0, 500.0, 10.0), 
    ));

    info!("[Voidrift Phase 2] World Initialized with Autopilot.");
}

fn spawn_marker(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    font: Handle<Font>,
    label: &str,
    pos: Vec2,
    color: Color,
) {
    commands.spawn((
        MapMarker,
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(pos.x, pos.y, 0.5),
    ))
    .with_children(|parent| {
        parent.spawn((
            Text2d::new(label),
            TextFont { font, font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            TextLayout::new_with_justify(Justify::Center),
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
    mut commands: Commands,
) {
    for (mut ship, mut transform, entity) in query.iter_mut() {
        if ship.state == ShipState::Navigating {
            if let Ok(target) = target_query.get(entity) {
                let current_pos = transform.translation.truncate();
                let direction = target.destination - current_pos;
                let distance = direction.length();

                if distance < ARRIVAL_THRESHOLD {
                    ship.state = ShipState::Idle;
                    commands.entity(entity).remove::<AutopilotTarget>();
                    info!("[Voidrift Phase 2] Ship Arrived at Target.");
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
    info!("[Voidrift Phase 2] Entered Map View.");
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
    marker_query: Query<(&Transform, &Parent), (With<MapMarker>, Without<Ship>, Without<MapToggleButton>)>,
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
                for (marker_transform, _) in marker_query.iter() {
                    let marker_pos = marker_transform.translation.truncate();
                    if world_pos.distance(marker_pos) < 50.0 {
                        let (ship_entity, mut ship) = ship_query.single_mut();
                        ship.state = ShipState::Navigating;
                        commands.entity(ship_entity).insert(AutopilotTarget {
                            destination: marker_pos,
                        });
                        next_state.set(GameState::SpaceView);
                        info!("[Voidrift Phase 2] Autopilot Engaged: {:?}", marker_pos);
                        break;
                    }
                }
            }
        }
    }
}
