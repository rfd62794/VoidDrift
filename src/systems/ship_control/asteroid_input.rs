use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ship_control::ship_spawn::spawn_drone_ship;

pub fn asteroid_input_system(
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&GlobalTransform, Entity, &ActiveAsteroid), With<MapMarker>>,
    queue: Res<ShipQueue>,
    mut commands: Commands,
    opening: Res<OpeningSequence>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
    mut dispatch_events: EventWriter<DroneDispatched>,
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
                    let spawn_pos = if let Ok((_, s_transform)) = station_query.get_single() {
                        s_transform.translation.truncate()
                    } else {
                        STATION_POS
                    };

                    spawn_drone_ship(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        spawn_pos,
                        AutopilotTarget { destination: mp, target_entity: Some(asteroid_ent) },
                        active_asteroid.ore_type,
                    );

                    dispatch_events.send(DroneDispatched);
                    info!("[Voidrift] Ship dispatched to {:?}.", mp);

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}
