use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn camera_follow_system(
    state: Res<State<GameState>>,
    ship: Query<&Transform, (With<Ship>, Without<MainCamera>)>,
    mut cam: Query<&mut Transform, (With<MainCamera>, Without<Ship>)>,
    mut cam_delta: ResMut<CameraDelta>,
) {
    let st = ship.single();
    let mut ct = cam.single_mut();
    let old_pos = ct.translation.truncate();
    if *state.get() == GameState::SpaceView {
        ct.translation.x = st.translation.x;
        ct.translation.y = st.translation.y;
    } else {
        ct.translation.x = STATION_POS.x;
        ct.translation.y = STATION_POS.y;
    }
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
    mut highlight: Query<(&mut Transform, &mut Visibility), With<DestinationHighlight>>,
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

pub fn enter_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { 
    cam.single_mut().scale = MAP_STRATEGIC_SCALE; 
}

pub fn exit_map_view(mut cam: Query<&mut OrthographicProjection, With<MainCamera>>) { 
    cam.single_mut().scale = 1.0; 
}

pub fn map_input_system(
    touches: Res<Touches>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    marker_query: Query<(&Transform, Entity), (With<MapMarker>, Without<Ship>)>,
    mut ship_query: Query<(Entity, &mut Ship), With<Ship>>,
    berth_query: Query<(Entity, &Berth)>,
    opening: Res<OpeningSequence>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    let (camera, camera_transform) = camera_query.single();
    for touch in touches.iter_just_pressed() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, touch.position()) {
            for (mt, me) in marker_query.iter() {
                let mp = mt.translation.truncate();
                if world_pos.distance(mp) < 80.0 {
                    let (ship_entity, mut ship) = ship_query.single_mut();
                    
                    // Avoid docking redundancy
                    if ship.state == ShipState::Docked && mp.distance(STATION_POS) < 10.0 { 
                        continue; 
                    }

                    // If it's a station marker, target Berth 1
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
