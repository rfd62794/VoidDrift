use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

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
    )>,
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

/// Scrolls all star entities at their layer's parallax speed and wraps them at screen edges.
/// Stars track camera movement at (1 - parallax_factor) speed, creating the illusion
/// that far stars (factor=0.05) barely drift while near stars (0.15) move slightly more.
pub fn starfield_scroll_system(
    cam_query: Query<&Transform, With<MainCamera>>,
    mut star_query: Query<(&StarLayer, &mut Transform), Without<MainCamera>>,
    cam_delta: Res<CameraDelta>,
) {
    // DEVICE-CALIBRATED: These bounds are tuned for the Moto G 2025 screen
    // (≈393×851 logical px at scale 1.0). If the game targets other screen sizes,
    // revisit these values — too small causes star pop-in at screen edges,
    // too large wastes update budget on off-screen entities.
    const WRAP_X: f32 = 700.0;
    const WRAP_Y: f32 = 500.0;
    let cam_pos = cam_query.single().translation.truncate();

    for (layer, mut transform) in star_query.iter_mut() {
        // Stars advance by (1 - parallax) of camera delta → they appear to drift
        // backward at parallax-factor speed relative to camera.
        transform.translation.x += cam_delta.0.x * (1.0 - layer.0);
        transform.translation.y += cam_delta.0.y * (1.0 - layer.0);

        // Wrap when the star exits the ±WRAP window around camera.
        let rel_x = transform.translation.x - cam_pos.x;
        let rel_y = transform.translation.y - cam_pos.y;
        if      rel_x >  WRAP_X { transform.translation.x -= WRAP_X * 2.0; }
        else if rel_x < -WRAP_X { transform.translation.x += WRAP_X * 2.0; }
        if      rel_y >  WRAP_Y { transform.translation.y -= WRAP_Y * 2.0; }
        else if rel_y < -WRAP_Y { transform.translation.y += WRAP_Y * 2.0; }
    }
}

pub fn station_rotation_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    mut visual_query: Query<&mut Transform, With<StationVisualsContainer>>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        station.rotation += STATION_ROTATION_SPEED * time.delta_secs();
        if let Ok(mut transform) = visual_query.get_single_mut() {
            transform.rotation = Quat::from_rotation_z(station.rotation);
        }
    }
}
