use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use bevy_egui::EguiContexts;
use crate::components::*;

#[derive(SystemParam)]
pub struct AsteroidInputParams<'w, 's> {
    pub contexts: EguiContexts<'w, 's>,
    pub touches: Res<'w, Touches>,
    pub camera_query: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
    pub marker_query: Query<'w, 's, (&'static GlobalTransform, Entity, &'static ActiveAsteroid), With<MapMarker>>,
    pub bottle_query: Query<'w, 's, &'static GlobalTransform, With<ActiveBottle>>,
    pub idle_drone_query: Query<'w, 's, Entity, (With<AutonomousShip>, With<Drone>, Without<AutonomousAssignment>)>,
    pub commands: Commands<'w, 's>,
    pub opening: Res<'w, OpeningSequence>,
    pub state: Res<'w, State<GameState>>,
    pub next_state: ResMut<'w, NextState<GameState>>,
    pub windows: Query<'w, 's, &'static Window>,
    pub mouse_button: Res<'w, ButtonInput<MouseButton>>,
    pub vcfg: Res<'w, crate::config::VisualConfig>,
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

    if p.idle_drone_query.is_empty() {
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
            for (marker_gtransform, _asteroid_ent, active_asteroid) in p.marker_query.iter() {
                let mp = marker_gtransform.translation().truncate();
                if mp.distance(crate::constants::STATION_POS) < 10.0 { continue; }
                if world_pos.distance(mp) < 80.0 {
                    dispatch_pos_opt = Some(mp);
                    if let Some(idle_entity) = p.idle_drone_query.iter().next() {
                        p.commands.entity(idle_entity).insert(AutonomousAssignment {
                            target_pos: mp,
                            ore_type: active_asteroid.ore_type,
                            sector_name: "Inner Ring".to_string(),
                        });
                    }
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
                    for (marker_gtransform, _asteroid_ent, active_asteroid) in p.marker_query.iter() {
                        let mp = marker_gtransform.translation().truncate();
                        if mp.distance(crate::constants::STATION_POS) < 10.0 { continue; }
                        if world_pos.distance(mp) < 80.0 {
                            dispatch_pos_opt = Some(mp);
                            if let Some(idle_entity) = p.idle_drone_query.iter().next() {
                                p.commands.entity(idle_entity).insert(AutonomousAssignment {
                                    target_pos: mp,
                                    ore_type: active_asteroid.ore_type,
                                    sector_name: "Inner Ring".to_string(),
                                });
                            }
                            dispatched = true;
                            break;
                        }
                    }
                }
            }
        }
    }

    if dispatched {
        if let Some(mp) = dispatch_pos_opt {
            info!("[Voidrift] Drone assigned to {:?}.", mp);
        }
        if *p.state.get() == GameState::MapView {
            p.next_state.set(GameState::SpaceView);
        }
    }
}
