use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn mining_system(
    time: Res<Time>, 
    mut signal_log: ResMut<SignalLog>,
    mut ship_query: Query<(Entity, &mut Ship, &Transform, &Children), (Without<MiningBeam>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>, 
    mut field_query: Query<(&mut AsteroidField, &Transform, &MeshMaterial2d<ColorMaterial>), (Without<Ship>, Without<MiningBeam>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut beam_query: Query<(Entity, &mut Transform, &mut Visibility), (With<MiningBeam>, Without<Ship>, Without<AsteroidField>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
    berth_query: Query<(Entity, &Berth)>,
    mut commands: Commands,
) {
    for (ship_ent, mut ship, ship_transform, children) in ship_query.iter_mut() {
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
                    if ship.cargo == 0.0 {
                        ship.cargo_type = field.ore_deposit;
                    } else if ship.cargo_type != field.ore_deposit {
                        continue;
                    }
                    let ore_amount = MINING_RATE * time.delta_secs();
                    ship.cargo = (ship.cargo + ore_amount).min(ship.cargo_capacity as f32);
                    if ship.cargo >= ship.cargo_capacity as f32 { 
                        ship.state = ShipState::Navigating; 
                        target_dist = None;
                        
                        // Target nearest available berth for unloading
                        if let Ok((station, s_transform)) = station_query.get_single() {
                            // Find first available berth (no occupied check needed — ships despawn on dock)
                            if let Some((berth_ent, berth)) = berth_query.iter().next() {
                                let berth_pos = crate::components::berth_world_pos(
                                    s_transform.translation.truncate(),
                                    station.rotation,
                                    berth.arm_index,
                                );
                                commands.entity(ship_ent).insert(AutopilotTarget {
                                    destination: berth_pos,
                                    target_entity: Some(berth_ent),
                                });
                            } else {
                                // Fallback: dock at station hub if no berths exist
                                commands.entity(ship_ent).insert(AutopilotTarget {
                                    destination: s_transform.translation.truncate(),
                                    target_entity: None,
                                });
                            }
                        }
                        
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
                                    OreDeposit::Iron => COLOR_IRON,
                                    OreDeposit::Tungsten => COLOR_TUNGSTEN,
                                    OreDeposit::Nickel => COLOR_NICKEL,
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
