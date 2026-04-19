use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ui::add_log_entry;

pub fn autopilot_system(
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform, &mut AutopilotTarget, Entity), (Without<Station>, Without<AsteroidField>, Without<Berth>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    berth_query: Query<&Berth>,
    asteroid_query: Query<&AsteroidField>,
    mut station_query: Query<(Entity, &mut Station, &Transform), (Without<Ship>, Without<AsteroidField>, Without<Berth>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    carbon_field_query: Query<Entity, (With<AsteroidField>, Without<MapMarker>)>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
) {
    for (mut ship, mut transform, mut target, entity) in query.iter_mut() {
        if ship.state == ShipState::Navigating {
            // [PHASE B] Dynamic destination recalculation for Berths
            if let Some(target_ent) = target.target_entity {
                if let Ok(berth) = berth_query.get(target_ent) {
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
                        ship.state = ShipState::Mining; 
                    }
                    else if let Ok(berth) = berth_query.get(target_ent) {
                        if let Ok((_station_ent, mut station, _)) = station_query.get_single_mut() {
                            ship.state = ShipState::Docked; 
                            *active_tab = ActiveStationTab::Reserves;
                            ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                            
                            // [PHASE B] Docking Sequence Trigger
                            station.dock_state = StationDockState::Resuming;
                            station.resume_timer = STATION_RESUME_DELAY;
                            
                            // [PHASE 8b] Reset player power for free if station has power
                            if station.power >= STATION_POWER_FLOOR {
                                ship.power = SHIP_POWER_MAX;
                            }

                            // [PHASE 8b] Automatic deposit of cells to ship (up to 3, cap 5)
                            if station.power_cells > 10 && ship.power_cells < 5 {
                                let needed = 5u32.saturating_sub(ship.power_cells);
                                let transfer = 3u32.min(needed);
                                if station.power_cells >= transfer {
                                    station.power_cells -= transfer;
                                    ship.power_cells += transfer;
                                }
                            }

                            if ship.cargo > 0.0 {
                                match ship.cargo_type {
                                    OreType::Magnetite => {
                                        station.magnetite_reserves += ship.cargo;
                                        let msg = format!("[STATION AI] Magnetite reserves: {}. Power Cells: {}.", station.magnetite_reserves as u32, station.power_cells);
                                        add_log_entry(&mut station, msg);
                                    }
                                    OreType::Carbon => {
                                        station.carbon_reserves += ship.cargo;
                                        let msg = format!("[STATION AI] Carbon reserves: {}. Hull Plates: {}.", station.carbon_reserves as u32, station.hull_plate_reserves);
                                        add_log_entry(&mut station, msg);
                                        if station.hull_plate_reserves == 0 && station.carbon_reserves >= (HULL_REFINERY_RATIO as f32) {
                                            add_log_entry(&mut station, "[STATION AI] Hull synthesis possible. Fabricate AI Cores to expand autonomous fleet.".to_string());
                                        }
                                    }
                                    OreType::Empty => {}
                                }
                                ship.cargo = 0.0;
                                ship.cargo_type = OreType::Empty;
                            }
                            
                            // SECTOR 7 DISCOVERY LOGIC - FALLBACK
                            if station.ai_cores > 0 {
                                if let Ok(s7_ent) = carbon_field_query.get_single() {
                                    commands.entity(s7_ent).insert((MapMarker, Visibility::Visible));
                                }
                            }
                            
                            info!("[Voidrift Phase B] Docking Complete: Berth {}.", berth.arm_index);
                        }
                    } else if let Ok((_ent, mut station, _)) = station_query.get_mut(target_ent) {
                        // NO BERTH? Dock at center (Initial / Opening Sequence)
                        ship.state = ShipState::Docked; 
                        station.dock_state = StationDockState::Resuming;
                        station.resume_timer = STATION_RESUME_DELAY;
                        
                        // [PHASE 8b] Reset player power for free if station has power
                        if station.power >= STATION_POWER_FLOOR {
                            ship.power = SHIP_POWER_MAX;
                        }
                        
                        info!("[Voidrift] Docking Complete: Station Hub.");
                    }
                } else { ship.state = ShipState::Idle; }
                commands.entity(entity).remove::<AutopilotTarget>();
            } else {
                let movement = direction.normalize() * ship.speed * time.delta_secs();
                transform.translation += movement.extend(0.0);
            }
        }
    }
}

/// [PHASE B] Locks docked ship to berth position throughout rotation
pub fn docked_ship_system(
    mut ship_query: Query<(&Ship, &mut Transform, &AutopilotTarget), (Without<Station>, Without<Berth>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    berth_query: Query<&Berth>,
    station_query: Query<(&Station, &Transform), (Without<Ship>, Without<Berth>)>,
) {
    for (ship, mut transform, target) in ship_query.iter_mut() {
        if ship.state == ShipState::Docked {
            if let Some(target_ent) = target.target_entity {
                if let Ok(berth) = berth_query.get(target_ent) {
                    if let Ok((station, s_transform)) = station_query.get_single() {
                        let pos = berth_world_pos(
                            s_transform.translation.truncate(),
                            station.rotation,
                            berth.arm_index,
                        );
                        transform.translation = pos.extend(Z_SHIP);
                        // Optional: Rotate ship to match arm? Direct alignment with arm direction:
                        let arm_angle = station.rotation + (berth.arm_index as f32 * std::f32::consts::TAU / 6.0);
                        transform.rotation = Quat::from_rotation_z(arm_angle - std::f32::consts::FRAC_PI_2);
                    }
                }
            }
        }
    }
}
