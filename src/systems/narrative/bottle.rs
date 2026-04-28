use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

#[derive(Resource)]
pub struct BottleSpawnTimer {
    pub timer: Timer,
    pub spawned: bool,
}

impl Default for BottleSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(BOTTLE_SPAWN_DELAY, TimerMode::Once),
            spawned: false,
        }
    }
}

pub fn bottle_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bottle_timer: ResMut<BottleSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if bottle_timer.spawned {
        return;
    }

    bottle_timer.timer.tick(time.delta());

    if bottle_timer.timer.finished() {
        bottle_timer.spawned = true;
        
        let spawn_pos = Vec2::new(150.0, 200.0); // Drift somewhere
        
        commands.spawn((
            ActiveBottle,
            MapMarker,
            MapElement,
            Mesh2d(meshes.add(Rectangle::new(16.0, 32.0))), // Big enough to tap
            MeshMaterial2d(materials.add(COLOR_BOTTLE)),
            Transform::from_xyz(spawn_pos.x, spawn_pos.y, Z_ENVIRONMENT),
        ));
        
        info!("[Voiddrift] Bottle spawned at {:?}", spawn_pos);
    }
}

pub fn bottle_input_system(
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    bottle_query: Query<(Entity, &GlobalTransform), With<ActiveBottle>>,
    mut queue: ResMut<ShipQueue>,
    mut commands: Commands,
    opening: Res<OpeningSequence>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
) {
    if opening.phase != OpeningPhase::Complete { return; }
    if touches.iter().count() >= 2 { return; }
    if queue.available_count == 0 { return; }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (bottle_ent, bottle_gtransform) in bottle_query.iter() {
                let bp = bottle_gtransform.translation().truncate();
                
                if world_pos.distance(bp) < 60.0 { // Mobile touch target size
                    let spawn_pos = if let Ok((_, s_transform)) = station_query.get_single() {
                        s_transform.translation.truncate()
                    } else {
                        STATION_POS
                    };

                    let ship_ent = commands.spawn((
                        Ship {
                            state: ShipState::Navigating,
                            speed: SHIP_SPEED,
                            cargo: 0.0,
                            cargo_type: OreDeposit::Iron, // Dummy
                            cargo_capacity: CARGO_CAPACITY,
                            laser_tier: LaserTier::Basic,
                            current_mining_target: None,
                        },
                        AutonomousShipTag,
                        LastHeading(0.0),
                        AutopilotTarget {
                            destination: bp,
                            target_entity: Some(bottle_ent),
                        },
                        Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(20.0, 28.0))),
                        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.6, 1.0))),
                        Transform::from_xyz(spawn_pos.x, spawn_pos.y, Z_SHIP),
                    )).id();

                    commands.entity(ship_ent).with_children(|parent| {
                        parent.spawn((
                            ThrusterGlow,
                            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.0))),
                            Transform::from_xyz(0.0, -18.0, 0.1),
                            Visibility::Hidden,
                        ));
                    });

                    queue.available_count -= 1;
                    info!("[Voiddrift] Drone dispatched to collect Bottle.");

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}
