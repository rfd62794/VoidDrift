use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn mining_system(
    time: Res<Time>, 
    mut signal_log: ResMut<SignalLog>,
    mut ship_query: Query<(&mut Ship, &Transform, &Children), (Without<MiningBeam>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>, 
    mut field_query: Query<(&mut AsteroidField, &Transform, &MeshMaterial2d<ColorMaterial>), (Without<Ship>, Without<MiningBeam>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut beam_query: Query<(Entity, &mut Transform, &mut Visibility), (With<MiningBeam>, Without<Ship>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
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
                    // [PHASE EXPANSION] LASER TIER GATE
                    let req = ore_laser_required(&field.ore_deposit);
                    let ship_tier = ship.laser_tier;
                    
                    let can_mine = match (req, ship_tier) {
                        (LaserTier::Basic, _) => true,
                        (LaserTier::Tungsten, LaserTier::Tungsten) | (LaserTier::Tungsten, LaserTier::Composite) => true,
                        (LaserTier::Composite, LaserTier::Composite) => true,
                        _ => false,
                    };

                    if !can_mine {
                        signal_log.entries.push_front("> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.".to_string());
                        ship.state = ShipState::Idle;
                        break;
                    }

                    target_dist = Some(dist);
                    if ship.cargo_type == OreType::Empty {
                        ship.cargo_type = field.ore_type;
                    } else if ship.cargo_type != field.ore_type {
                        continue;
                    }
                    let ore_amount = MINING_RATE * time.delta_secs();
                    ship.cargo = (ship.cargo + ore_amount).min(ship.cargo_capacity as f32);
                    if ship.cargo >= ship.cargo_capacity as f32 { 
                        ship.state = ShipState::Idle; 
                        ship.power = (ship.power - SHIP_POWER_COST_MINING).max(0.0);
                        target_dist = None;
                        
                        if !field.depleted {
                            field.depleted = true;
                            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                                mat.color = COLOR_DEPLETED;
                            }
                        }
                    } else {
                        if field.depleted {
                            field.depleted = false;
                            if let Some(mat) = materials.get_mut(&mat_handle.0) {
                                mat.color = match field.ore_deposit {
                                    OreDeposit::Magnetite => COLOR_MAGNETITE,
                                    OreDeposit::Iron => COLOR_IRON,
                                    OreDeposit::Carbon => COLOR_CARBON,
                                    OreDeposit::Tungsten => COLOR_TUNGSTEN,
                                    OreDeposit::Titanite => COLOR_TITANITE,
                                    OreDeposit::CrystalCore => COLOR_CRYSTAL,
                                };
                            }
                        }
                    }
                    break;
                }
            }
        }
        
        // Handle beam visibility and scaling
        for &child in children.iter() {
            if let Ok((_, mut b_transform, mut b_vis)) = beam_query.get_mut(child) {
                if let Some(dist) = target_dist {
                    *b_vis = Visibility::Visible;
                    b_transform.scale.y = dist;
                    b_transform.translation.y = dist / 2.0;
                } else {
                    *b_vis = Visibility::Hidden;
                }
            }
        }
    }
}
