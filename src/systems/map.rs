use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn camera_follow_system(
    state: Res<State<GameState>>,
    ship: Query<&Transform, (With<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<AsteroidField>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut cam: Query<&mut Transform, (With<MainCamera>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<AsteroidField>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut cam_delta: ResMut<CameraDelta>,
    pan_state: Res<MapPanState>,
) {
    let st = ship.single();
    let mut ct = cam.single_mut();
    let old_pos = ct.translation.truncate();
    
    let target_pos = if *state.get() == GameState::MapView {
        STATION_POS + pan_state.cumulative_offset
    } else {
        // SpaceView
        if pan_state.is_focused {
            st.translation.truncate()
        } else {
            pan_state.cumulative_offset
        }
    };

    ct.translation.x = target_pos.x;
    ct.translation.y = target_pos.y;

    // Write camera delta so starfield_scroll_system can parallax-scroll each layer.
    cam_delta.0 = ct.translation.truncate() - old_pos;
}

pub fn show_map_elements(mut query: Query<&mut Visibility, With<MapElement>>) {
    for mut vis in query.iter_mut() {
        *vis = Visibility::Inherited;
    }
}

pub fn hide_map_elements(mut query: Query<&mut Visibility, With<MapElement>>) {
    for mut vis in query.iter_mut() {
        *vis = Visibility::Hidden;
    }
}

pub fn map_highlight_system(
    auto_target: Query<(&AutopilotTarget, &Ship)>,
    mut highlight: Query<(&mut Transform, &mut Visibility), (With<DestinationHighlight>, Without<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<AsteroidField>, Without<Berth>, Without<StarLayer>)>,
) {
    if let Ok((target, ship)) = auto_target.get_single() {
        if let Ok((mut h_transform, mut h_vis)) = highlight.get_single_mut() {
            if ship.state == ShipState::Navigating {
                *h_vis = Visibility::Inherited; 
                h_transform.translation.x = target.destination.x;
                h_transform.translation.y = target.destination.y;
            } else {
                *h_vis = Visibility::Hidden;
            }
        }
    }
}

pub fn enter_map_view() { 
    // Zoom persistence handled by pinch_zoom_system + removal of exit resets
}

pub fn exit_map_view() { 
    // Zoom persistence handled by pinch_zoom_system
}

pub fn map_input_system(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>, Without<MainCamera>, Without<AutonomousShip>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut ship_query: Query<(Entity, &mut Ship), (With<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<AsteroidField>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    berth_query: Query<(Entity, &Berth), (Without<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<AsteroidField>, Without<DestinationHighlight>, Without<StarLayer>)>,
    opening: Res<OpeningSequence>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    // Suppress single-touch input if multi-touch is active
    if touches.iter().count() >= 2 {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (mt, me) in marker_query.iter() {
                let mp = mt.translation.truncate();
                if world_pos.distance(mp) < 80.0 {
                    let (ship_entity, mut ship) = ship_query.single_mut();
                    
                    if ship.state == ShipState::Docked && mp.distance(STATION_POS) < 10.0 { 
                        continue; 
                    }

                    let mut target_ent = me;
                    let mut destination = mp;

                    if mp.distance(STATION_POS) < 10.0 {
                        if let Some((b_ent, _)) = berth_query.iter().find(|(_, b)| b.berth_type == BerthType::Player) {
                            target_ent = b_ent;
                            destination = STATION_POS; 
                        }
                    }

                    ship.state = ShipState::Navigating;
                    *active_tab = ActiveStationTab::Reserves;
                    ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    commands.entity(ship_entity).remove::<DockedAt>();
                    commands.entity(ship_entity).insert(AutopilotTarget { 
                        destination, 
                        target_entity: Some(target_ent) 
                    });

                    if *state.get() == GameState::MapView {
                        next_state.set(GameState::SpaceView);
                    }
                    break;
                }
            }
        }
    }
}

pub fn pinch_zoom_system(
    touches: Res<Touches>,
    mut query: Query<&mut OrthographicProjection, (With<MainCamera>, Without<Ship>, Without<Station>, Without<AutonomousShip>, Without<AsteroidField>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut last_dist: Local<Option<f32>>,
) {
    let Ok(mut projection) = query.get_single_mut() else { return; };
    
    // We only care about the distance between the first two touch points
    let touch_points: Vec<Vec2> = touches.iter().map(|t| t.position()).take(2).collect();

    if touch_points.len() == 2 {
        let dist = touch_points[0].distance(touch_points[1]);
        
        if let Some(prev) = *last_dist {
            let delta = prev - dist; // Positive if pinching (getting closer), negative if pulling (getting further)
            
            // Apply scale delta
            let new_scale = (projection.scale + delta * ZOOM_SPEED).clamp(ZOOM_MIN, ZOOM_MAX);
            projection.scale = new_scale;
        }
        *last_dist = Some(dist);
    } else {
        *last_dist = None;
    }
}

pub fn map_pan_system(
    touches: Res<Touches>,
    mut pan_state: ResMut<MapPanState>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    state: Res<State<GameState>>,
    ship_query: Query<&Transform, With<Ship>>,
    opening: Res<OpeningSequence>,
) {
    // Guard against panning during cinematic opening
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    // Suppress panning if multi-touch (zooming) is active
    if touches.iter().count() >= 2 {
        pan_state.last_position = None;
        return;
    }

    if let Some(touch) = touches.iter().next() {
        if let Some(last_pos) = pan_state.last_position {
            let delta = touch.position() - last_pos;
            let projection = projection_query.single();
            
            // If dragging in SpaceView while focused, break focus and initialize offset to current ship position
            if *state.get() == GameState::SpaceView && pan_state.is_focused {
                pan_state.is_focused = false;
                if let Ok(st) = ship_query.get_single() {
                    pan_state.cumulative_offset = st.translation.truncate();
                }
            }

            // Adjust pan speed by current zoom level so it feels consistent
            pan_state.cumulative_offset.x -= delta.x * MAP_PAN_SPEED * projection.scale;
            pan_state.cumulative_offset.y += delta.y * MAP_PAN_SPEED * projection.scale;
        }
        pan_state.last_position = Some(touch.position());
    } else {
        pan_state.last_position = None;
    }
}
