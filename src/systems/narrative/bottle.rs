use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ship_control::ship_spawn::spawn_bottle_drone;

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
    requests_tab: Res<RequestsTabState>,
) {
    // On load: if FirstLight card already exists, bottle was already collected.
    // Set spawned=true so the timer never fires again.
    if requests_tab.collected_requests.iter().any(|r| r.id == RequestId::FirstLight) {
        bottle_timer.spawned = true;
    }

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
    if opening.phase != OpeningPhase::Complete { return; }
    if touches.iter().count() >= 2 { return; }
    if queue.available_count == 0 { return; }

    let Ok((camera, camera_transform)) = camera_query.get_single() else { return; };

    // Dispatch logic shared by touch and mouse
    let mut handle_dispatch = |world_pos: Vec2| {
        for (bottle_ent, bottle_gtransform) in bottle_query.iter() {
            let bp = bottle_gtransform.translation().truncate();

            if world_pos.distance(bp) < 60.0 {
                let spawn_pos = if let Ok((_, s_transform)) = station_query.get_single() {
                    s_transform.translation.truncate()
                } else {
                    STATION_POS
                };

                spawn_bottle_drone(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    spawn_pos,
                    AutopilotTarget { destination: bp, target_entity: Some(bottle_ent) },
                );

                dispatch_events.send(DroneDispatched);
                info!("[Voidrift] Drone dispatched to collect Bottle.");

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
