use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ui::add_log_entry;

pub fn autopilot_system(
    time: Res<Time>,
    mut query: Query<(&mut Ship, &mut Transform, Entity)>,
    target_query: Query<&AutopilotTarget>,
    asteroid_query: Query<&AsteroidField>,
    mut station_query: Query<(Entity, &mut Station)>,
    carbon_field_query: Query<Entity, (With<AsteroidField>, Without<MapMarker>)>,
    mut active_tab: ResMut<ActiveStationTab>,
    mut commands: Commands,
) {
    for (mut ship, mut transform, entity) in query.iter_mut() {
        if ship.state == ShipState::Navigating {
            if let Ok(target) = target_query.get(entity) {
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
                        else if let Ok((_station_ent, mut station)) = station_query.get_mut(target_ent) { 
                            ship.state = ShipState::Docked; 
                            *active_tab = ActiveStationTab::Reserves;
                            ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                            
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
                            
                            info!("[Voidrift Phase 4] Gate Certification: Docked.");
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
}
