use bevy::prelude::*;
use bevy_egui::EguiContexts;
use crate::components::*;
use crate::constants::*;
use crate::systems::ship_control::ship_spawn::spawn_drone_ship;

pub fn asteroid_input_system(
    mut contexts: EguiContexts,
    touches: Res<Touches>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&GlobalTransform, Entity, &ActiveAsteroid), With<MapMarker>>,
    bottle_query: Query<&GlobalTransform, With<ActiveBottle>>,
    queue: Res<ShipQueue>,
    mut commands: Commands,
    opening: Res<OpeningSequence>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
    mut dispatch_events: EventWriter<DroneDispatched>,
    windows: Query<&Window>,
    mouse_button: Res<ButtonInput<MouseButton>>,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    if contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if touches.iter().count() >= 2 {
        return;
    }

    if queue.available_count == 0 {
        return;
    }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    // Dispatch logic shared by touch and mouse
    let mut handle_dispatch = |world_pos: Vec2| {
        // Skip if click is within bottle range (bottle takes priority)
        if let Ok(bottle_transform) = bottle_query.get_single() {
            let bottle_pos = bottle_transform.translation().truncate();
            if world_pos.distance(bottle_pos) < 60.0 {
                return false;
            }
        }

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
                return true;
            }
        }
        false
    };

    // Touch input (Android)
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            if handle_dispatch(world_pos) {
                break;
            }
        }
    }

    // Mouse click fallback (WASM + desktop)
    if let Some(cursor_pos) = windows.get_single().ok()
        .and_then(|w| w.cursor_position())
    {
        if mouse_button.just_pressed(MouseButton::Left) {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                handle_dispatch(world_pos);
            }
        }
    }
}
