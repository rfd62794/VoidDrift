use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::EguiContexts;
use crate::components::*;
use crate::constants::*;
use crate::config::VisualConfig;
use crate::config::visual::rgb;
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

#[derive(SystemParam)]
pub struct BottleInputParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
    pub touches: Res<'w, Touches>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
    pub bottle_query: Query<'w, 's, (Entity, &'static GlobalTransform), With<ActiveBottle>>,
    pub queue: Res<'w, ShipQueue>,
    pub commands: Commands<'w, 's>,
    pub opening: Res<'w, OpeningSequence>,
    pub state: Res<'w, State<GameState>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<ColorMaterial>>,
    pub station_query: Query<'w, 's, (&'static Station, &'static Transform), With<Station>>,
    pub dispatch_events: EventWriter<'w, DroneDispatched>,
    pub windows: Query<'w, 's, &'static Window>,
    pub mouse_button: Res<'w, ButtonInput<MouseButton>>,
    pub vcfg: Res<'w, VisualConfig>,
    pub view_state: Res<'w, ViewState>,
}

pub fn bottle_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bottle_timer: ResMut<BottleSpawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    requests_tab: Res<RequestsTabState>,
    vcfg: Res<VisualConfig>,
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
        
        let b = &vcfg.bottle;
        let spawn_pos = Vec2::new(b.spawn_x, b.spawn_y);

        commands.spawn((
            ActiveBottle,
            MapMarker,
            MapElement,
            Mesh2d(meshes.add(Rectangle::new(b.width, b.height))),
            MeshMaterial2d(materials.add(rgb(b.color))),
            Transform::from_xyz(spawn_pos.x, spawn_pos.y, Z_ENVIRONMENT),
        ));
        
        info!("[Voiddrift] Bottle spawned at {:?}", spawn_pos);
    }
}

pub fn bottle_input_system(mut p: BottleInputParams) {
    if p.opening.phase != OpeningPhase::Complete { return; }
    if p.view_state.show_production_tree { return; }
    if p.contexts.ctx_mut().wants_pointer_input() { return; }
    if p.touches.iter().count() >= 2 { return; }
    if p.queue.available_count == 0 { return; }

    let Ok((camera, camera_transform)) = p.camera_query.get_single() else { return; };

    // Dispatch logic shared by touch and mouse
    let mut handle_dispatch = |world_pos: Vec2| {
        for (bottle_ent, bottle_gtransform) in p.bottle_query.iter() {
            let bp = bottle_gtransform.translation().truncate();

            if world_pos.distance(bp) < p.vcfg.bottle.hit_radius {
                let spawn_pos = if let Ok((_, s_transform)) = p.station_query.get_single() {
                    s_transform.translation.truncate()
                } else {
                    STATION_POS
                };

                spawn_bottle_drone(
                    &mut p.commands,
                    &mut p.meshes,
                    &mut p.materials,
                    spawn_pos,
                    AutopilotTarget { destination: bp, target_entity: Some(bottle_ent) },
                    &p.vcfg,
                );

                p.dispatch_events.send(DroneDispatched);
                info!("[Voidrift] Drone dispatched to collect Bottle.");

                if *p.state.get() == GameState::MapView {
                    p.next_state.set(GameState::SpaceView);
                }
                return true;
            }
        }
        false
    };

    // Touch input (Android)
    for touch in p.touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            if handle_dispatch(world_pos) {
                break;
            }
        }
    }

    // Mouse click fallback (WASM + desktop)
    if let Some(cursor_pos) = p.windows.get_single().ok()
        .and_then(|w| w.cursor_position())
    {
        if p.mouse_button.just_pressed(MouseButton::Left) {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                handle_dispatch(world_pos);
            }
        }
    }
}
