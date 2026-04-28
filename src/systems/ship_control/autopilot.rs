use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::persistence::save::AutosaveEvent;

/// Moves ships with AutopilotTarget toward their destination.
/// On arrival:
///   - Asteroid  → transitions to Mining
///   - Berth     → unloads cargo, despawns ship, increments ShipQueue
///   - Station   → opening sequence hub dock (fallback only)
pub fn autopilot_system(
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform, &mut AutopilotTarget, Entity), BaseShipFilter>,
    berth_query: Query<(Entity, &Berth)>,
    asteroid_query: Query<&ActiveAsteroid>,
    mut station_query: Query<(Entity, &mut Station, &Transform), BaseStationFilter>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
    mut autosave_events: EventWriter<AutosaveEvent>,
    mut queue: ResMut<ShipQueue>,
    bottle_query: Query<&ActiveBottle>,
    carrying_query: Query<&CarryingBottle>,
    mut requests_tab: ResMut<RequestsTabState>,
    mut signal_log: ResMut<SignalLog>,
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
                        );
                    }
                }
            }

            let current_pos = transform.translation.truncate();
            let direction = target.destination - current_pos;
            let distance = direction.length();
            let threshold = if let Some(target_ent) = target.target_entity {
                if asteroid_query.get(target_ent).is_ok() { ARRIVAL_THRESHOLD_MINING } else { ARRIVAL_THRESHOLD }
            } else { ARRIVAL_THRESHOLD };

            if distance < threshold {
                if let Some(target_ent) = target.target_entity {
                    if asteroid_query.get(target_ent).is_ok() {
                        // Arrived at asteroid → start mining
                        ship.state = ShipState::Mining;
                    } else if berth_query.get(target_ent).is_ok() {
                        // Arrived at berth → unload, despawn, return to queue
                        if let Ok((_station_ent, mut station, _)) = station_query.get_single_mut() {
                            match ship.cargo_type {
                                OreDeposit::Iron     => station.iron_reserves     += ship.cargo,
                                OreDeposit::Tungsten => station.tungsten_reserves += ship.cargo,
                                OreDeposit::Nickel   => station.nickel_reserves   += ship.cargo,
                                OreDeposit::Aluminum => station.aluminum_reserves += ship.cargo,
                            }
                            *active_tab = ActiveStationTab::Cargo;
                            station.dock_state = StationDockState::Resuming;
                            station.resume_timer = STATION_RESUME_DELAY;
                            
                            // Check if carrying bottle
                            if carrying_query.get(entity).is_ok() {
                                // Narrative Event!
                                signal_log.entries.push_back("SIGNAL RECEIVED — ORIGIN UNKNOWN\nFrequency matched. You were expected.\nWe have observed your work. It is... acceptable.\nA proposal follows.".to_string());
                                if signal_log.entries.len() > 10 {
                                    signal_log.entries.pop_front();
                                }
                                requests_tab.collected_requests.push(CollectedRequest {
                                    id: RequestId::FirstLight,
                                    faction: FactionId::Signal,
                                    fulfilled: false,
                                });
                            }
                        }

                        queue.available_count += 1;
                        info!("[Voiddrift] Ship docked & unloaded. Queue: {}", queue.available_count);
                        autosave_events.send(AutosaveEvent);
                        commands.entity(entity).despawn_recursive();
                        // Entity is gone — stop processing this ship
                        continue;

                    } else if let Ok((station_ent, mut station, _)) = station_query.get_mut(target_ent) {
                        // Hub dock (opening sequence cinematic only)
                        match ship.cargo_type {
                            OreDeposit::Iron     => station.iron_reserves     += ship.cargo,
                            OreDeposit::Tungsten => station.tungsten_reserves += ship.cargo,
                            OreDeposit::Nickel   => station.nickel_reserves   += ship.cargo,
                            OreDeposit::Aluminum => station.aluminum_reserves += ship.cargo,
                        }
                        ship.cargo = 0.0;
                        ship.state = ShipState::Docked;
                        station.dock_state = StationDockState::Resuming;
                        station.resume_timer = STATION_RESUME_DELAY;
                        info!("[Voidrift] Hub dock complete (opening sequence).");
                        commands.entity(entity).remove::<AutopilotTarget>().insert(DockedAt(station_ent));
                    } else {
                        // Target entity no longer exists (asteroid despawned before arrival).
                        // Transition to mining anyway so the mining system can retarget or send it home.
                        ship.state = ShipState::Mining;
                    }
                } else {
                    if let Some(target_ent) = target.target_entity {
                        if bottle_query.get(target_ent).is_ok() {
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
                                    );
                                    commands.entity(entity).insert(AutopilotTarget {
                                        destination: berth_pos,
                                        target_entity: Some(berth_ent),
                                    });
                                } else {
                                    commands.entity(entity).insert(AutopilotTarget {
                                        destination: s_transform.translation.truncate(),
                                        target_entity: None,
                                    });
                                }
                            }
                            continue;
                        }
                    }
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
    mut ship_query: Query<(&Ship, &mut Transform, &DockedAt), (With<Ship>, With<InOpeningSequence>, Without<Station>, Without<Berth>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    station_query: Query<(&Station, &Transform), (With<Station>, Without<Ship>)>,
) {
    for (ship, mut transform, docked_at) in ship_query.iter_mut() {
        if ship.state == ShipState::Docked {
            if let Ok((_, s_transform)) = station_query.get(docked_at.0) {
                transform.translation = s_transform.translation.truncate().extend(Z_SHIP);
            }
        }
    }
}
