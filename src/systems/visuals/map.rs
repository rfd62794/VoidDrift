use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn camera_follow_system(
    state: Res<State<GameState>>,
    ship: Query<&Transform, (With<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<ActiveAsteroid>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut cam: Query<&mut Transform, (With<MainCamera>, Without<Ship>, Without<AutonomousShip>, Without<Station>, Without<ActiveAsteroid>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut cam_delta: ResMut<CameraDelta>,
    pan_state: Res<MapPanState>,
) {
    // Write camera delta so starfield_scroll_system can parallax-scroll each layer.
    cam_delta.0 = Vec2::ZERO;

    let Ok(mut ct) = cam.get_single_mut() else { return; };
    let old_pos = ct.translation.truncate();
    
    // Determine the target position
    let target_pos = if *state.get() == GameState::MapView {
        STATION_POS + pan_state.cumulative_offset
    } else {
        // SpaceView
        let ship_pos = ship.get_single().map(|t| t.translation.truncate()).ok();
        
        if pan_state.is_focused {
            if let Some(pos) = ship_pos {
                pos
            } else {
                // Default to Station if focus is active but no ship exists
                STATION_POS
            }
        } else {
            pan_state.cumulative_offset
        }
    };

    ct.translation.x = target_pos.x;
    ct.translation.y = target_pos.y;

    // Calculate delta for starfield parallax
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
    mut highlight: Query<(&mut Transform, &mut Visibility), (With<DestinationHighlight>, Without<Ship>, Without<MainCamera>, Without<Station>, Without<AutonomousShip>, Without<ActiveAsteroid>, Without<Berth>, Without<StarLayer>)>,
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
    windows: Query<&Window>,
    device_type: Res<DeviceType>,
    mut query: Query<&mut OrthographicProjection, (With<MainCamera>, Without<Ship>, Without<Station>, Without<AutonomousShip>, Without<ActiveAsteroid>, Without<Berth>, Without<DestinationHighlight>, Without<StarLayer>)>,
    mut last_dist: Local<Option<f32>>,
    mut scroll_events: EventReader<bevy::input::mouse::MouseWheel>,
) {
    let Ok(mut projection) = query.get_single_mut() else { return; };
    
    // Use different zoom sensitivity based on device type
    let zoom_speed = match *device_type {
        DeviceType::Mobile => 0.04, // Much faster zoom for touch
        DeviceType::Desktop => 0.02, // Faster zoom for mouse
    };
    
    // We only care about the distance between the first two touch points
    let touch_points: Vec<Vec2> = touches.iter().map(|t| t.position()).take(2).collect();

    if touch_points.len() == 2 {
        let dist = touch_points[0].distance(touch_points[1]);
        
        if let Some(prev) = *last_dist {
            let delta = prev - dist; // Positive if pinching (getting closer), negative if pulling (getting further)
            
            // Apply scale delta
            let new_scale = (projection.scale + delta * zoom_speed).clamp(ZOOM_MIN, ZOOM_MAX);
            projection.scale = new_scale;
        }
        *last_dist = Some(dist);
    } else {
        *last_dist = None;
    }

    // Scroll wheel zoom (WASM + desktop) - only when cursor is over the window
    let cursor_in_window = windows.iter().any(|window| {
        window.cursor_position().is_some()
    });

    for event in scroll_events.read() {
        if !cursor_in_window {
            continue; // Don't zoom if cursor is outside the window
        }
        let zoom_delta = match event.unit {
            bevy::input::mouse::MouseScrollUnit::Line => event.y * 0.1,
            bevy::input::mouse::MouseScrollUnit::Pixel => event.y * 0.001,
        };
        // Apply same zoom logic as pinch — adjust OrthographicProjection.scale
        // Clamp to same min/max zoom bounds as pinch zoom
        projection.scale = (projection.scale - zoom_delta).clamp(ZOOM_MIN, ZOOM_MAX);
    }
}

pub fn map_pan_system(
    touches: Res<Touches>,
    mut pan_state: ResMut<MapPanState>,
    projection_query: Query<&OrthographicProjection, With<MainCamera>>,
    state: Res<State<GameState>>,
    ship_query: Query<&Transform, With<Ship>>,
    opening: Res<OpeningSequence>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    mut cursor_moved: EventReader<CursorMoved>,
    mut last_cursor_pos: Local<Option<Vec2>>,
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
                } else {
                    // Default offset to station if focus is broken while no ship exists
                    pan_state.cumulative_offset = STATION_POS;
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

    // Click-drag pan (WASM + desktop)
    if mouse_button.pressed(MouseButton::Left) {
        for event in cursor_moved.read() {
            if let Some(last) = *last_cursor_pos {
                let delta = event.position - last;
                let Ok(projection) = projection_query.get_single() else { continue; };

                // If dragging in SpaceView while focused, break focus and initialize offset to current ship position
                if *state.get() == GameState::SpaceView && pan_state.is_focused {
                    pan_state.is_focused = false;
                    if let Ok(st) = ship_query.get_single() {
                        pan_state.cumulative_offset = st.translation.truncate();
                    } else {
                        // Default offset to station if focus is broken while no ship exists
                        pan_state.cumulative_offset = STATION_POS;
                    }
                }

                // Apply same pan logic as touch drag
                // Invert delta direction to match touch drag feel
                pan_state.cumulative_offset.x -= delta.x * MAP_PAN_SPEED * projection.scale;
                pan_state.cumulative_offset.y += delta.y * MAP_PAN_SPEED * projection.scale;
            }
            *last_cursor_pos = Some(event.position);
        }
    } else {
        *last_cursor_pos = None;
        cursor_moved.clear();
    }
}
