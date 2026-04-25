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
    // Always zero-out delta first — only set it if we actually move the camera
    // this prevents stale velocity from scrolling the starfield after a ship despawns
    cam_delta.0 = Vec2::ZERO;

    let Ok(st) = ship.get_single() else { return; };
    let Ok(mut ct) = cam.get_single_mut() else { return; };
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
            let Ok(projection) = projection_query.get_single() else { return; };
            
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
