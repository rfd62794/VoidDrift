use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ui::add_log_entry;

pub fn station_status_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
    ship_query: Query<&AutonomousShip>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        let now = time.elapsed_secs();
        let power = station.power_cells;
        
        let should_warn = now - station.last_power_warning_time > POWER_WARNING_INTERVAL || station.last_power_warning_time == 0.0;
        
        // 1. Critical Power Warning
        if power < POWER_COST_CYCLE_TOTAL && should_warn {
            add_log_entry(&mut station, format!("[STATION AI] Power reserves critical. Reserve: {} cells.", power));
            station.last_power_warning_time = now;
        }
        
        // 2. Ships Holding
        let any_holding = ship_query.iter().any(|s| s.state == AutonomousShipState::Holding);
        if any_holding && power < POWER_COST_CYCLE_TOTAL && should_warn {
             add_log_entry(&mut station, "[STATION AI] Insufficient power. Autonomous unit holding.".to_string());
             station.last_power_warning_time = now;
        }

        // 3. Automation Suspension Notice (Log once on state change)
        if station.power < STATION_POWER_FLOOR && station.online {
             // Already handled by maintenance for now, but good to have a dedicated check if needed
        }
    }
}

pub fn ship_self_preservation_system(
    mut ship_query: Query<(Entity, &mut Ship)>,
    mut station_query: Query<&mut Station>,
    mut commands: Commands,
) {
    if let Ok((ship_entity, mut ship)) = ship_query.get_single_mut() {
        if ship.power < SHIP_POWER_FLOOR && ship.state != ShipState::Docked {
            // 1. Consume onboard cell
            if ship.power_cells > 0 {
                ship.power_cells -= 1;
                ship.power = (ship.power + POWER_CELL_RESTORE_VALUE).min(SHIP_POWER_MAX);
                if let Ok(mut station) = station_query.get_single_mut() {
                    add_log_entry(&mut station, format!("[SHIP] Power Cell consumed. Power: {:.1}", ship.power));
                }
            } 
            // 2. Emergency Refine (10 Magnetite -> Power Boost)
            else if ship.cargo_type == OreType::Magnetite && ship.cargo >= EMERGENCY_REFINE_COST {
                ship.cargo -= EMERGENCY_REFINE_COST;
                ship.power = (ship.power + POWER_CELL_RESTORE_VALUE).min(SHIP_POWER_MAX);
                if let Ok(mut station) = station_query.get_single_mut() {
                    add_log_entry(&mut station, "[SHIP] Emergency refine initiated. Power restored.".to_string());
                }
            }
            // 3. Force Return
            else if ship.state != ShipState::Navigating {
                ship.state = ShipState::Navigating;
                commands.entity(ship_entity).remove::<DockedAt>();
                commands.spawn(AutopilotTarget {
                    destination: STATION_POS,
                    target_entity: None,
                });
                if let Ok(mut station) = station_query.get_single_mut() {
                    add_log_entry(&mut station, "[SHIP] Power critical. Returning to station.".to_string());
                }
            }
        }
    }
}

pub fn station_maintenance_system(
    time: Res<Time>,
    mut station_query: Query<&mut Station>,
) {
    if let Ok(mut station) = station_query.get_single_mut() {
        station.maintenance_timer.tick(time.delta());
        if station.maintenance_timer.just_finished() {
            if station.power < STATION_POWER_FLOOR {
                if station.power_cells > 0 {
                    station.power_cells -= 1;
                    station.power = (station.power + STATION_POWER_RESTORE_VALUE).min(STATION_POWER_MAX);
                    add_log_entry(&mut station, "[STATION AI] Power Cell consumed. Base power restored.".to_string());
                } else if station.power < 2.0 {
                    add_log_entry(&mut station, "[STATION AI] Base power critical. Suspending automation.".to_string());
                }
            }
        }
    }
}
