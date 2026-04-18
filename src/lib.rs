// Voidrift — Phase 4: Station UI & Refinery (Final Gate 4 Build)
// ============================================================================
// Goal: Final Phase 4 closure. Opt-C: Logic verified, text deferred.
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
struct DockingUIPanel;

#[derive(Component)]
struct RefineButton;

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
        .add_systems(Update, (
            mining_system, 
            cargo_display_system, 
            update_docking_ui_visibility,
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
            MeshMaterial2d(materials.add(Color::srgba(0.8, 0.2, 0.2, 0.8))),
            Transform::from_xyz(120.0, 220.0, -1.0),
        ));

        // [PHASE 4] DOCKING UI (Bottom Bar Layout - Text Deferred per Option C)
        parent.spawn((
            DockingUIPanel,
            Mesh2d(meshes.add(Rectangle::new(400.0, 90.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.02, 0.02, 0.1, 0.9))),
            Transform::from_xyz(0.0, -210.0, -10.0),
            Visibility::Hidden,
        ))
        .with_children(|panel| {
            // Accent line
            panel.spawn((
                Mesh2d(meshes.add(Rectangle::new(400.0, 2.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.8, 0.8))),
                Transform::from_xyz(0.0, 44.0, 0.1),
            ));

            // REFINE BUTTON (Actionable Cyan Rectangle)
            panel.spawn((
                RefineButton,
                Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.0, 0.8, 0.8))),
                Transform::from_xyz(130.0, 0.0, 0.1),
            ));
        });
    });

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
        Station,
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
                        if asteroid_query.get(target_ent).is_ok() { ship.state = ShipState::Mining; }
                        else if station_query.get(target_ent).is_ok() { 
                            ship.state = ShipState::Docked; 
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

fn update_docking_ui_visibility(ship: Query<&Ship>, mut ui: Query<&mut Visibility, With<DockingUIPanel>>) {
    let ship = ship.single();
    let mut vis = ui.single_mut();
    if ship.state == ShipState::Docked { *vis = Visibility::Visible; }
    else { *vis = Visibility::Hidden; }
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
    toggle_query: Query<(&GlobalTransform, Entity), With<MapToggleButton>>,
    btn_query: Query<(&GlobalTransform, Entity), With<RefineButton>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>, Without<MapToggleButton>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    mut commands: Commands,
) {
    let (camera, camera_transform) = camera_query.single();
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            let (toggle_gt, _) = toggle_query.single();
            if world_pos.distance(toggle_gt.translation().truncate()) < 60.0 {
                if *state.get() == GameState::SpaceView { next_state.set(GameState::MapView); }
                else { next_state.set(GameState::SpaceView); }
                continue;
            }

            let mut handled = false;
            {
                let (_, mut ship) = ship_query.single_mut();
                if ship.state == ShipState::Docked {
                    let (btn_gt, _) = btn_query.single();
                    let bp = btn_gt.translation().truncate();
                    if world_pos.x > bp.x - 50.0 && world_pos.x < bp.x + 50.0 && world_pos.y > bp.y - 25.0 && world_pos.y < bp.y + 25.0 {
                        let cells = (ship.cargo as u32) / REFINERY_RATIO;
                        if cells > 0 {
                            ship.cargo -= (cells * REFINERY_RATIO) as f32;
                            ship.power_cells += cells;
                            info!("[Voidrift Phase 4] Refined {} ore -> {} cells. Total: {}", (cells * REFINERY_RATIO), cells, ship.power_cells);
                        }
                        handled = true;
                    }
                }
            }

            if !handled && *state.get() == GameState::MapView {
                for (mt, me) in marker_query.iter() {
                    let mp = mt.translation.truncate();
                    if world_pos.distance(mp) < 80.0 {
                        let (ship_entity, mut ship) = ship_query.single_mut();
                        if ship.state == ShipState::Docked && mp.distance(Vec2::new(-150.0, -200.0)) < 10.0 { continue; }
                        ship.state = ShipState::Navigating;
                        commands.entity(ship_entity).insert(AutopilotTarget { destination: mp, target_entity: Some(me) });
                        next_state.set(GameState::SpaceView);
                        break;
                    }
                }
            }
        }
    }
}
