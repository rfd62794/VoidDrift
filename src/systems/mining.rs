use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn mining_system(
    time: Res<Time>, 
    mut ship_query: Query<(&mut Ship, &Transform, &Children), (Without<MiningBeam>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>, 
    mut field_query: Query<(&mut AsteroidField, &Transform, &MeshMaterial2d<ColorMaterial>), (Without<Ship>, Without<MiningBeam>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut beam_query: Query<(&mut Transform, &mut Visibility), (With<MiningBeam>, Without<Ship>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    for (mut ship, ship_transform, children) in ship_query.iter_mut() {
        let is_mining = ship.state == ShipState::Mining;
        let mut target_dist = None;

        if is_mining {
            // Find nearby field to determine ore type
            for (mut field, field_transform, mat_handle) in field_query.iter_mut() {
                let dist = ship_transform.translation.distance(field_transform.translation);
                if dist < 50.0 {
                    target_dist = Some(dist);
                    if ship.cargo_type == OreType::Empty {
                        ship.cargo_type = field.ore_type;
                    } else if ship.cargo_type != field.ore_type {
                        // Mismatched field - ship cannot mine this ore type into existing cargo
                        continue;
                    }
                    let ore = MINING_RATE * time.delta_secs();
                    ship.cargo = (ship.cargo + ore).min(ship.cargo_capacity as f32);
                    if ship.cargo >= ship.cargo_capacity as f32 { 
                        ship.state = ShipState::Idle; 
                        ship.power = (ship.power - SHIP_POWER_COST_MINING).max(0.0);
                        target_dist = None; // Disable beam upon finish
                        
                        // [POLISH] Visual depletion
                        if !field.depleted {
                            field.depleted = true;
                            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                                mat.color = Color::srgb(0.2, 0.2, 0.2); // Dark grey #333333
                            }
                        }
                    } else {
                        // Restore color if mining resumes
                        if field.depleted {
                            field.depleted = false;
                            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                                mat.color = match field.ore_type {
                                    OreType::Magnetite => Color::srgb(0.8, 0.3, 0.3),
                                    OreType::Carbon => Color::srgb(0.3, 0.8, 0.3),
                                    OreType::Empty => Color::srgb(0.5, 0.5, 0.5),
                                };
                            }
                        }
                    }
                    break;
                }
            }
        }
        
        // Handle beam visibility and scaling for player ship
        for &child in children.iter() {
            if let Ok((mut b_transform, mut b_vis)) = beam_query.get_mut(child) {
                if let Some(dist) = target_dist {
                    *b_vis = Visibility::Visible;
                    b_transform.scale.y = dist;
                    b_transform.translation.y = dist / 2.0; // Extend forward from ship center
                } else {
                    *b_vis = Visibility::Hidden;
                }
            }
        }
    }
}
