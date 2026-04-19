use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ui::add_log_entry;

pub fn autonomous_ship_system(
    time: Res<Time>,
    mut ship_query: Query<(&mut AutonomousShip, &mut Transform, &mut AutonomousAssignment, Option<&Children>)>,
    mut station_query: Query<(&mut Station, &Transform)>,
    mut beam_query: Query<(&mut Transform, &mut Visibility), (With<MiningBeam>, Without<AsteroidField>, Without<AutonomousShip>)>,
) {
    if let Ok((mut station, s_transform)) = station_query.get_single_mut() {
        for (mut ship, mut transform, mut assignment, children_opt) in ship_query.iter_mut() {
            match ship.state {
                AutonomousShipState::Holding => {
                    if station.power_cells >= POWER_COST_CYCLE_TOTAL {
                        station.power_cells -= POWER_COST_CYCLE_TOTAL;
                        ship.state = AutonomousShipState::Outbound;
                        add_log_entry(&mut station, "[STATION AI] Power confirmed. Dispatching autonomous unit.".to_string());
                    }
                }
                AutonomousShipState::Outbound => {
                    let direction = assignment.target_pos - transform.translation.truncate();
                    let distance = direction.length();
                    if distance < ARRIVAL_THRESHOLD_MINING {
                        ship.state = AutonomousShipState::Mining;
                        ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Mining => {
                    ship.cargo = (ship.cargo + MINING_RATE * time.delta_secs()).min(CARGO_CAPACITY as f32);
                    if ship.cargo >= CARGO_CAPACITY as f32 {
                        ship.state = AutonomousShipState::Returning;
                        ship.power = (ship.power - SHIP_POWER_COST_MINING).max(0.0);
                    }
                }
                AutonomousShipState::Returning => {
                    // [PHASE B] Dynamic destination tracking for Berth 2
                    // We use the station and transform already matched at the top of the system
                    let berth_pos = berth_world_pos(
                        s_transform.translation.truncate(),
                        station.rotation,
                        BERTH_2_ARM_INDEX,
                    );
                    assignment.target_pos = berth_pos;
                    
                    let direction = berth_pos - transform.translation.truncate();
                    let distance = direction.length();
                    if distance < ARRIVAL_THRESHOLD {
                        ship.state = AutonomousShipState::Unloading;
                        ship.power = (ship.power - SHIP_POWER_COST_TRANSIT).max(0.0);
                        
                        // [PHASE B] Trigger station resume if not already resuming
                        station.dock_state = StationDockState::Resuming;
                        station.resume_timer = STATION_RESUME_DELAY;
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Unloading => {
                    let ore_name = if assignment.ore_type == OreType::Magnetite { "Magnetite" } else { "Carbon" };
                    match assignment.ore_type {
                        OreType::Magnetite => station.magnetite_reserves += ship.cargo,
                        OreType::Carbon => station.carbon_reserves += ship.cargo,
                        _ => {}
                    }
                    // [PHASE 8b] Recharge autonomous ship using station cells
                    if station.power_cells > 0 {
                        station.power_cells -= 1;
                        ship.power = SHIP_POWER_MAX;
                    }

                    let msg = format!("[STATION AI] Cargo deposited: {}. {} recovered.", assignment.sector_name, ore_name);
                    add_log_entry(&mut station, msg);
                    ship.cargo = 0.0;
                    
                    // Return to holding or critical return
                    if ship.power < 2.0 {
                         add_log_entry(&mut station, "[STATION AI] Autonomous unit returned. Low power. Recharging.".to_string());
                    }
                    ship.state = AutonomousShipState::Holding;
                }
            }

            if let Some(children) = children_opt {
                for &child in children.iter() {
                    if let Ok((mut b_transform, mut b_vis)) = beam_query.get_mut(child) {
                        if ship.state == AutonomousShipState::Mining {
                            let dist = transform.translation.truncate().distance(assignment.target_pos);
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
    }
}
