use bevy::prelude::*;
use crate::components::*;
use crate::config::{BalanceConfig, VisualConfig};

/// Moves ships with AutopilotTarget toward their destination.
/// On arrival:
///   - Asteroid  → transitions to Mining
///   - Berth     → fires ShipDockedWithCargo or ShipDockedWithBottle event
///   - Station   → opening sequence hub dock (fallback only)
pub fn autopilot_system(
    time: Res<Time>,
    cfg: Res<BalanceConfig>,
    vcfg: Res<VisualConfig>,
    mut query: Query<(&mut Ship, &mut Transform, &mut AutopilotTarget, Entity), BaseShipFilter>,
    berth_query: Query<(Entity, &Berth)>,
    asteroid_query: Query<&ActiveAsteroid>,
    station_query: Query<(Entity, &Station, &Transform), BaseStationFilter>,
    mut commands: Commands,
    bottle_query: Query<&ActiveBottle>,
    carrying_query: Query<&CarryingBottle>,
    mut cargo_docked_events: EventWriter<ShipDockedWithCargo>,
    mut bottle_docked_events: EventWriter<ShipDockedWithBottle>,
) {
    for (mut ship, mut transform, mut target, entity) in query.iter_mut() {
        if ship.state == ShipState::Navigating {
            // Recalculate berth destination dynamically (station rotates)
            if let Some(target_ent) = target.target_entity {
                if let Ok((_, berth)) = berth_query.get(target_ent) {
                    if let Ok((_s_ent, station, s_transform)) = station_query.get_single() {
                        target.destination = berth_world_pos(
                            s_transform.translation.truncate(),
                            station.rotation,
                            berth.arm_index,
                            &vcfg,
                        );
                    }
                }
            }

            let current_pos = transform.translation.truncate();
            let direction = target.destination - current_pos;
            let distance = direction.length();
            let threshold = if let Some(target_ent) = target.target_entity {
                if asteroid_query.get(target_ent).is_ok() { cfg.mining.arrival_threshold_mining } else { cfg.mining.arrival_threshold }
            } else { cfg.mining.arrival_threshold };

            if distance < threshold {
                if let Some(target_ent) = target.target_entity {
                    if asteroid_query.get(target_ent).is_ok() {
                        // Arrived at asteroid → start mining
                        ship.state = ShipState::Mining;
                    } else if berth_query.get(target_ent).is_ok() {
                        // Arrived at berth → fire arrival event, economy/narrative handled downstream
                        if carrying_query.get(entity).is_ok() {
                            bottle_docked_events.send(ShipDockedWithBottle { ship_entity: entity });
                        } else {
                            cargo_docked_events.send(ShipDockedWithCargo {
                                ship_entity: entity,
                                ore_type: ship.cargo_type,
                                amount: ship.cargo,
                                despawn: true,
                            });
                        }
                        // Entity will be despawned by economy system this frame
                        continue;

                    } else if let Ok((station_ent, _, _)) = station_query.get(target_ent) {
                        // Hub dock (opening sequence cinematic only — opening drone carries zero cargo)
                        ship.cargo = 0.0;
                        ship.state = ShipState::Docked;
                        cargo_docked_events.send(ShipDockedWithCargo {
                            ship_entity: entity,
                            ore_type: ship.cargo_type,
                            amount: 0.0,
                            despawn: false, // opening_sequence_system despawns this entity at t>=10.5
                        });
                        info!("[Voidrift] Hub dock complete (opening sequence).");
                        commands.entity(entity).remove::<AutopilotTarget>().insert(DockedAt(station_ent));
                    } else if bottle_query.get(target_ent).is_ok() {
                        // Arrived at bottle -> collect and return
                        commands.entity(target_ent).despawn_recursive();
                        ship.state = ShipState::Navigating;
                        commands.entity(entity).insert(CarryingBottle(target_ent));
                        
                        // Target nearest available berth for unloading
                        if let Ok((_ent, station, s_transform)) = station_query.get_single() {
                            if let Some((berth_ent, berth)) = berth_query.iter().next() {
                                let berth_pos = crate::components::berth_world_pos(
                                    s_transform.translation.truncate(),
                                    station.rotation,
                                    berth.arm_index,
                                    &vcfg,
                                );
                                commands.entity(entity).insert(AutopilotTarget {
                                    destination: berth_pos,
                                    target_entity: Some(berth_ent),
                                });
                            } else {
                                bevy::log::warn!(
                                    "autopilot: CarryingBottle ship {:?} found no berth — falling back to station hub",
                                    entity
                                );
                                commands.entity(entity).insert(AutopilotTarget {
                                    destination: s_transform.translation.truncate(),
                                    target_entity: None,
                                });
                            }
                        }
                        continue;
                    } else {
                        // Target entity no longer exists (asteroid despawned before arrival).
                        // Transition to mining anyway so the mining system can retarget or send it home.
                        bevy::log::warn!(
                            "autopilot: target entity {:?} matched no query arm — defaulting to Mining",
                            target_ent
                        );
                        ship.state = ShipState::Mining;
                    }
                } else {
                    bevy::log::warn!(
                        "autopilot: ship {:?} has no target entity — setting Idle",
                        entity
                    );
                    ship.state = ShipState::Idle;
                }
            } else {
                let movement = direction.normalize() * ship.speed * time.delta_secs();
                transform.translation += movement.extend(0.0);
            }
        }
    }
}

/// Locks the opening-sequence drone to the hub while Docked.
/// Only used during the intro cinematic — regular mission ships despawn on arrival.
pub fn docked_ship_system(
    vcfg: Res<VisualConfig>,
    mut ship_query: Query<(&Ship, &mut Transform, &DockedAt), (With<Ship>, With<InOpeningSequence>, Without<Station>, Without<Berth>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    station_query: Query<(&Station, &Transform), (With<Station>, Without<Ship>)>,
) {
    for (ship, mut transform, docked_at) in ship_query.iter_mut() {
        if ship.state == ShipState::Docked {
            if let Ok((_, s_transform)) = station_query.get(docked_at.0) {
                transform.translation = s_transform.translation.truncate().extend(vcfg.z_layer.z_ship);
            }
        }
    }
}
