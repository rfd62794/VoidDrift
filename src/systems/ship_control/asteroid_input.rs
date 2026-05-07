use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::EguiContexts;
use crate::components::*;
use crate::constants::*;
use crate::config::VisualConfig;
use crate::systems::ship_control::ship_spawn::spawn_drone_ship;

#[derive(SystemParam)]
pub struct AsteroidInputParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
    pub touches: Res<'w, Touches>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
    pub marker_query: Query<'w, 's, (&'static GlobalTransform, Entity, &'static ActiveAsteroid), With<MapMarker>>,
    pub bottle_query: Query<'w, 's, &'static GlobalTransform, With<ActiveBottle>>,
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
    pub tutorial: Res<'w, TutorialState>,
}

pub fn asteroid_input_system(mut p: AsteroidInputParams) {
    if p.opening.phase != OpeningPhase::Complete {
        return;
    }

    if p.view_state.show_production_tree {
        return;
    }

    if p.tutorial.active.is_some() {
        return;
    }

    if p.contexts.ctx_mut().wants_pointer_input() {
        return;
    }

    if p.touches.iter().count() >= 2 {
        return;
    }

    if p.queue.available_count == 0 {
        return;
    }

    let Ok((camera, camera_transform)) = p.camera_query.get_single() else { return; };

    let bottle_pos_opt = p.bottle_query.get_single()
        .ok()
        .map(|t| t.translation().truncate());

    let mut dispatched = false;
    let mut dispatch_pos_opt: Option<Vec2> = None;

    // Touch input (Android)
    'touch: for touch in p.touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            if let Some(bp) = bottle_pos_opt {
                if world_pos.distance(bp) < p.vcfg.bottle.hit_radius { continue 'touch; }
            }
            for (marker_gtransform, asteroid_ent, active_asteroid) in p.marker_query.iter() {
                let mp = marker_gtransform.translation().truncate();
                if mp.distance(STATION_POS) < 10.0 { continue; }
                if world_pos.distance(mp) < 80.0 {
                    dispatch_pos_opt = Some(mp);
                    let spawn_pos = p.station_query.get_single()
                        .map(|(_, t)| t.translation.truncate()).unwrap_or(STATION_POS);
                    spawn_drone_ship(&mut p.commands, &mut p.meshes, &mut p.materials,
                        spawn_pos, AutopilotTarget { destination: mp, target_entity: Some(asteroid_ent) },
                        active_asteroid.ore_type, &p.vcfg);
                    dispatched = true;
                    break 'touch;
                }
            }
        }
    }

    // Mouse click fallback (WASM + desktop)
    if !dispatched {
        if let Some(cursor_pos) = p.windows.get_single().ok().and_then(|w| w.cursor_position()) {
            if p.mouse_button.just_pressed(MouseButton::Left) {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                    if let Some(bp) = bottle_pos_opt {
                        if world_pos.distance(bp) < p.vcfg.bottle.hit_radius { return; }
                    }
                    for (marker_gtransform, asteroid_ent, active_asteroid) in p.marker_query.iter() {
                        let mp = marker_gtransform.translation().truncate();
                        if mp.distance(STATION_POS) < 10.0 { continue; }
                        if world_pos.distance(mp) < 80.0 {
                            dispatch_pos_opt = Some(mp);
                            let spawn_pos = p.station_query.get_single()
                                .map(|(_, t)| t.translation.truncate()).unwrap_or(STATION_POS);
                            spawn_drone_ship(&mut p.commands, &mut p.meshes, &mut p.materials,
                                spawn_pos, AutopilotTarget { destination: mp, target_entity: Some(asteroid_ent) },
                                active_asteroid.ore_type, &p.vcfg);
                            dispatched = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    if dispatched {
        p.dispatch_events.send(DroneDispatched);
        if let Some(mp) = dispatch_pos_opt {
            info!("[Voidrift] Ship dispatched to {:?}.", mp);
        }
        if *p.state.get() == GameState::MapView {
            p.next_state.set(GameState::SpaceView);
        }
    }
}
