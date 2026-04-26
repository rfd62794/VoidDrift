use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn asteroid_input_system(
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&GlobalTransform, Entity, &ActiveAsteroid), With<MapMarker>>,
    mut queue: ResMut<ShipQueue>,
    mut commands: Commands,
    opening: Res<OpeningSequence>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    if touches.iter().count() >= 2 {
        return;
    }

    if queue.available_count == 0 {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (marker_gtransform, asteroid_ent, active_asteroid) in marker_query.iter() {
                let mp = marker_gtransform.translation().truncate();

                // Ignore clicks on station (near origin)
                if mp.distance(STATION_POS) < 10.0 {
                    continue;
                }

                if world_pos.distance(mp) < 80.0 {
                    let ore_type = active_asteroid.ore_type;

                    // Spawn a fresh ship at the station hub
                    let spawn_pos = if let Ok((_, s_transform)) = station_query.get_single() {
                        s_transform.translation.truncate()
                    } else {
                        STATION_POS
                    };

                    // Spawn ship with all required child visuals
                    let ship_ent = commands.spawn((
                        Ship {
                            state: ShipState::Navigating,
                            speed: SHIP_SPEED,
                            cargo: 0.0,
                            cargo_type: ore_type,
                            cargo_capacity: CARGO_CAPACITY,
                            laser_tier: LaserTier::Basic,
                            current_mining_target: None,
                        },
                        AutonomousShipTag,
                        LastHeading(0.0),
                        AutopilotTarget {
                            destination: mp,
                            target_entity: Some(asteroid_ent),
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
                        parent.spawn((
                            MiningBeam,
                            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
                            MeshMaterial2d(materials.add(Color::srgba(1.0, 0.5, 0.0, 0.6))),
                            Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP),
                            Visibility::Hidden,
                        ));
                        parent.spawn((
                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
                            Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP),
                        ));
                        parent.spawn((
                            ShipCargoBarFill,
                            Mesh2d(meshes.add(Rectangle::new(30.0, 4.0))),
                            MeshMaterial2d(materials.add(Color::srgb(0.0, 0.6, 1.0))),
                            Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05),
                        ));
                    });

                    queue.available_count -= 1;
                    info!("[Voidrift] Ship dispatched to {:?}. Queue remaining: {}", mp, queue.available_count);

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}
