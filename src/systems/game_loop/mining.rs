use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn mining_system(
    time: Res<Time>,
    mut ship_query: Query<(Entity, &mut Ship, &Transform, &Children), (Without<MiningBeam>, Without<ActiveAsteroid>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut insufficient_laser_events: EventWriter<InsufficientLaserEvent>, 
    mut asteroid_query: Query<(Entity, &mut ActiveAsteroid, &Transform, Option<&MeshMaterial2d<ColorMaterial>>), (Without<Ship>, Without<MiningBeam>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut beam_query: Query<(Entity, &mut Transform, &mut Visibility), (With<MiningBeam>, Without<Ship>, Without<ActiveAsteroid>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    station_query: Query<(&Station, &Transform), With<Station>>,
    berth_query: Query<(Entity, &Berth)>,
    mut commands: Commands,
) {
    for (ship_ent, mut ship, ship_transform, children) in ship_query.iter_mut() {
        let is_mining = ship.state == ShipState::Mining;
        let mut target_dist = None;

        if is_mining {
            // If we have a current target, try to mine it
            if let Some(target_entity) = ship.current_mining_target {
                if let Ok((_, mut asteroid, a_transform, mat_handle)) = asteroid_query.get_mut(target_entity) {
                    let dist = ship_transform.translation.distance(a_transform.translation);
                    if dist < 80.0 {
                        // Check laser tier
                        let req = ore_laser_required(&asteroid.ore_type);
                        let ship_tier = ship.laser_tier;
                        
                        let can_mine = match (req, ship_tier) {
                            (LaserTier::Basic, _) => true,
                            (LaserTier::Tungsten, LaserTier::Tungsten) | (LaserTier::Tungsten, LaserTier::Composite) => true,
                            (LaserTier::Composite, LaserTier::Composite) => true,
                            _ => false,
                        };

                        if !can_mine {
                            insufficient_laser_events.send(InsufficientLaserEvent { ship_entity: ship_ent });
                            ship.state = ShipState::Idle;
                            continue;
                        }

                        target_dist = Some(dist);
                        if ship.cargo == 0.0 {
                            ship.cargo_type = asteroid.ore_type;
                        } else if ship.cargo_type != asteroid.ore_type {
                            continue; // Should not happen, but safeguard
                        }

                        let power_multiplier = if let Ok((station, _)) = station_query.get_single() {
                            station.power_multiplier
                        } else {
                            1.0
                        };
                        let effective_mining_rate = MINING_RATE * power_multiplier;
                        let ore_amount = effective_mining_rate * time.delta_secs();
                        ship.cargo = (ship.cargo + ore_amount).min(ship.cargo_capacity as f32);
                        asteroid.ore_remaining -= ore_amount;

                        // Visual feedback if depleted (asteroid lifecycle handles despawn)
                        if asteroid.ore_remaining <= 0.0 {
                            if let Some(mat_h) = mat_handle {
                                if let Some(mat) = materials.get_mut(&mat_h.0) {
                                    mat.color = COLOR_DEPLETED;
                                }
                            }

                            // Try to retarget nearest
                            let mut nearest_dist = f32::MAX;
                            let mut nearest_ent = None;
                            for (other_ent, other_ast, other_transform, _) in asteroid_query.iter() {
                                if other_ent != target_entity && other_ast.ore_type == ship.cargo_type && other_ast.ore_remaining > 0.0 {
                                    let d = ship_transform.translation.distance(other_transform.translation);
                                    if d < nearest_dist && d < 100.0 {
                                        nearest_dist = d;
                                        nearest_ent = Some(other_ent);
                                    }
                                }
                            }
                            if let Some(new_target) = nearest_ent {
                                ship.current_mining_target = Some(new_target);
                            } else {
                                ship.current_mining_target = None;
                            }
                        }
                    } else {
                        // Too far, lose target
                        bevy::log::warn!(
                            "mining: ship {:?} too far from target {:?} — clearing mining target",
                            ship_ent,
                            ship.current_mining_target
                        );
                        ship.current_mining_target = None;
                    }
                } else {
                    // Target despawned (lifespan expired or fully depleted)
                    let mut nearest_dist = f32::MAX;
                    let mut nearest_ent = None;
                    for (other_ent, other_ast, other_transform, _) in asteroid_query.iter() {
                        if other_ast.ore_type == ship.cargo_type && other_ast.ore_remaining > 0.0 {
                            let d = ship_transform.translation.distance(other_transform.translation);
                            if d < nearest_dist && d < 100.0 {
                                nearest_dist = d;
                                nearest_ent = Some(other_ent);
                            }
                        }
                    }
                    if let Some(new_target) = nearest_ent {
                        ship.current_mining_target = Some(new_target);
                    } else {
                        ship.current_mining_target = None;
                    }
                }
            } else {
                // No current target, try to find one nearby
                let mut nearest_dist = f32::MAX;
                let mut nearest_ent = None;
                for (other_ent, other_ast, other_transform, _) in asteroid_query.iter() {
                    let d = ship_transform.translation.distance(other_transform.translation);
                    if d < nearest_dist && d < 80.0 {
                        if ship.cargo == 0.0 || ship.cargo_type == other_ast.ore_type {
                            if other_ast.ore_remaining > 0.0 {
                                nearest_dist = d;
                                nearest_ent = Some(other_ent);
                            }
                        }
                    }
                }
                if let Some(new_target) = nearest_ent {
                    ship.current_mining_target = Some(new_target);
                }
            }

            let return_to_station = ship.cargo >= ship.cargo_capacity as f32 || ship.current_mining_target.is_none();

            if return_to_station {
                ship.state = ShipState::Navigating;
                target_dist = None;
                ship.current_mining_target = None;
                
                // Target nearest available berth for unloading
                if let Ok((station, s_transform)) = station_query.get_single() {
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
                        bevy::log::warn!(
                            "mining: ship {:?} returning to station but no berth found — targeting hub position",
                            ship_ent
                        );
                        commands.entity(ship_ent).insert(AutopilotTarget {
                            destination: s_transform.translation.truncate(),
                            target_entity: None,
                        });
                    }
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
