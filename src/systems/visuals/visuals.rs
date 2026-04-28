use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use rand::Rng;

pub fn thruster_glow_system(
    mut query: Query<(&Parent, &mut Visibility), With<ThrusterGlow>>,
    ship_query: Query<&Ship>,
    auto_ship_query: Query<&AutonomousShip>,
) {
    for (parent, mut visibility) in query.iter_mut() {
        let is_moving = if let Ok(ship) = ship_query.get(**parent) {
            ship.state == ShipState::Navigating || ship.state == ShipState::Mining
        } else if let Ok(auto_ship) = auto_ship_query.get(**parent) {
            auto_ship.state == AutonomousShipState::Outbound 
                || auto_ship.state == AutonomousShipState::Returning 
                || auto_ship.state == AutonomousShipState::Mining
        } else {
            false
        };

        if is_moving && *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else if !is_moving && *visibility == Visibility::Visible {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn ship_rotation_system(
    mut query: Query<(
        &mut Transform, 
        &mut LastHeading, 
        Option<&AutopilotTarget>, 
        Option<&AutonomousAssignment>, 
        Option<&Ship>, 
        Option<&AutonomousShip>
    ), (Without<Station>, Without<StarLayer>, Without<StationVisualsContainer>, Without<ActiveAsteroid>, Without<Berth>)>,
) {
    use std::f32::consts::PI;
    for (mut transform, mut last_heading, target_opt, assign_opt, ship_opt, auto_ship_opt) in query.iter_mut() {
        let is_navigating = if let Some(ship) = ship_opt {
            ship.state == ShipState::Navigating
        } else if let Some(auto_ship) = auto_ship_opt {
            auto_ship.state == AutonomousShipState::Outbound || auto_ship.state == AutonomousShipState::Returning
        } else {
            false
        };

        if is_navigating {
            let destination = if let Some(target) = target_opt {
                Some(target.destination)
            } else if let Some(assign) = assign_opt {
                Some(assign.target_pos)
            } else {
                None
            };
            
            if let Some(dest) = destination {
                let current_pos = transform.translation.truncate();
                let dir = dest - current_pos;
                if dir.length_squared() > 1.0 {
                    let heading = dir.y.atan2(dir.x) - PI / 2.0;
                    last_heading.0 = heading;
                }
            }
        }
        
        transform.rotation = Quat::from_rotation_z(last_heading.0);
    }
}

/// Scrolls star entities with parallax and wraps them around the camera.
/// On wrap, stars are redistributed randomly within bounds — no hard edges at any zoom.
pub fn starfield_scroll_system(
    cam_query: Query<(&Transform, &OrthographicProjection), VisualsCameraFilter>,
    mut star_query: Query<(&StarLayer, &mut Transform), VisualsStarFilter>,
    cam_delta: Res<CameraDelta>,
) {
    let Ok((cam_transform, proj)) = cam_query.get_single() else { return; };
    let cam_pos = cam_transform.translation.truncate();
    let mut rng = rand::thread_rng();

    // Half-extents scale with zoom so stars always fill the view.
    let wrap_x = 900.0_f32 * proj.scale.max(1.0);
    let wrap_y = 1000.0_f32 * proj.scale.max(1.0);

    for (layer, mut transform) in star_query.iter_mut() {
        transform.translation.x += cam_delta.0.x * (1.0 - layer.0);
        transform.translation.y += cam_delta.0.y * (1.0 - layer.0);

        let rel_x = transform.translation.x - cam_pos.x;
        let rel_y = transform.translation.y - cam_pos.y;

        // On wrap: scatter to a random position on the opposite side
        // so there's never a visible edge or empty region.
        if rel_x > wrap_x {
            transform.translation.x = cam_pos.x + rng.gen_range(-wrap_x..-wrap_x * 0.5);
            transform.translation.y = cam_pos.y + rng.gen_range(-wrap_y..wrap_y);
        } else if rel_x < -wrap_x {
            transform.translation.x = cam_pos.x + rng.gen_range(wrap_x * 0.5..wrap_x);
            transform.translation.y = cam_pos.y + rng.gen_range(-wrap_y..wrap_y);
        }
        if rel_y > wrap_y {
            transform.translation.y = cam_pos.y + rng.gen_range(-wrap_y..-wrap_y * 0.5);
            transform.translation.x = cam_pos.x + rng.gen_range(-wrap_x..wrap_x);
        } else if rel_y < -wrap_y {
            transform.translation.y = cam_pos.y + rng.gen_range(wrap_y * 0.5..wrap_y);
            transform.translation.x = cam_pos.x + rng.gen_range(-wrap_x..wrap_x);
        }
    }
}

pub fn station_rotation_system(
    time: Res<Time>,
    mut station_query: Query<(&mut Station, &Transform), VisualsStationFilter>,
    mut visual_query: Query<&mut Transform, VisualsContainerFilter>,
    ship_query: Query<(&Ship, &Transform), VisualsShipFilter>,
    autonomous_query: Query<(&AutonomousShip, &Transform), VisualsAutoShipFilter>,
) {
    if let Ok((mut station, station_transform)) = station_query.get_single_mut() {
        let station_pos = station_transform.translation.truncate();
        
        // [PHASE B] Approach detection
        // Include both mission ships and the cinematic opening drone
        let ship_approaching = ship_query.iter().any(|(ship, ship_transform)| {
            (ship.state == ShipState::Navigating || ship.state == ShipState::Docked) &&
                ship_transform.translation.truncate().distance(station_pos) < STATION_DOCK_SLOWDOWN_DISTANCE
        });
        
        let drone_approaching = autonomous_query.iter().any(|(drone, drone_transform)| {
            drone.state == AutonomousShipState::Returning &&
                drone_transform.translation.truncate().distance(station_pos) < STATION_DOCK_SLOWDOWN_DISTANCE
        });
        
        let incoming = ship_approaching || drone_approaching;
        
        match station.dock_state {
            StationDockState::Rotating => {
                if incoming {
                    station.dock_state = StationDockState::Slowing;
                }
                station.rotation += station.rotation_speed * time.delta_secs();
            }
            StationDockState::Slowing => {
                if !incoming {
                    station.dock_state = StationDockState::Rotating;
                    station.rotation_speed = STATION_ROTATION_SPEED;
                } else {
                    station.rotation_speed = (station.rotation_speed - STATION_SLOWDOWN_RATE * time.delta_secs())
                        .max(0.0);
                    if station.rotation_speed == 0.0 {
                        station.dock_state = StationDockState::Paused;
                    }
                }
                station.rotation += station.rotation_speed * time.delta_secs();
            }
            StationDockState::Paused => {
                // Rotation suspended. Waiting for dock event (handled in autopilot/autonomous)
            }
            StationDockState::Resuming => {
                if station.resume_timer > 0.0 {
                    station.resume_timer -= time.delta_secs();
                } else {
                    station.rotation_speed = (station.rotation_speed + STATION_RESUME_RATE * time.delta_secs())
                        .min(STATION_ROTATION_SPEED);
                    if station.rotation_speed >= STATION_ROTATION_SPEED {
                        station.dock_state = StationDockState::Rotating;
                    }
                }
                station.rotation += station.rotation_speed * time.delta_secs();
            }
        }

        if let Ok(mut transform) = visual_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_z(station.rotation);
        }
    }
}

/// [PHASE B] Updates berth circle colors based on occupancy
pub fn berth_occupancy_system(
    berth_query: Query<&Berth>,
    ship_query: Query<(&Ship, &AutopilotTarget)>,
    drone_query: Query<(&AutonomousShip, &AutonomousAssignment)>,
    berth_visual_query: Query<(&BerthVisual, &MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 1. Determine occupancy per arm index
    let mut occupancy = [BerthType::Open; 6];
    
    // Check player
    for (ship, target) in ship_query.iter() {
        if ship.state == ShipState::Docked {
            if let Some(target_ent) = target.target_entity {
                if let Ok(berth) = berth_query.get(target_ent) {
                    occupancy[berth.arm_index as usize] = BerthType::Player;
                }
            }
        }
    }
    
    // Check drones
    for (drone, _assignment) in drone_query.iter() {
        if drone.state == AutonomousShipState::Unloading || drone.state == AutonomousShipState::Holding {
            occupancy[BERTH_2_ARM_INDEX as usize] = BerthType::Drone;
        }
    }

    // 2. Update visuals
    for (visual, material_handle) in berth_visual_query.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = match occupancy[visual.0 as usize] {
                crate::components::BerthType::Player => Color::srgb(0.0, 0.67, 1.0),   // Cyan
                crate::components::BerthType::Drone  => Color::srgb(1.0, 0.53, 0.0),   // Orange
                _ => Color::srgb(0.4, 0.4, 0.4),                    // Grey
            };
        }
    }
}
