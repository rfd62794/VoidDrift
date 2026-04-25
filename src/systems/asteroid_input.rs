use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn asteroid_input_system(
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>, Without<MainCamera>, Without<AutonomousShip>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut queue: ResMut<ShipQueue>,
    mut commands: Commands,
    opening: Res<OpeningSequence>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    if touches.iter().count() >= 2 {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (mt, me) in marker_query.iter() {
                let mp = mt.translation.truncate();
                if world_pos.distance(mp) < 80.0 {
                    // Ignore clicks on station
                    if mp.distance(STATION_POS) < 10.0 {
                        continue;
                    }

                    // Assign the next available ship
                    if let Some(ship_entity) = queue.available_ships.pop() {
                        commands.entity(ship_entity).remove::<DockedAt>();
                        commands.entity(ship_entity).insert(AutopilotTarget {
                            destination: mp,
                            target_entity: Some(me),
                        });
                        queue.active_ships.push(ship_entity);
                    }
                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}
