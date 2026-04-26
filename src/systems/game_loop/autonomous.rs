use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::systems::ui::add_log_entry;
use crate::systems::persistence::save::AutosaveEvent;

pub fn autonomous_ship_system(
    time: Res<Time>,
    mut ship_query: Query<(Entity, &mut AutonomousShip, &mut Transform, &mut AutonomousAssignment), (Without<Station>, Without<MiningBeam>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
    mut station_query: Query<(&mut Station, &Transform), (Without<AutonomousShip>, Without<MiningBeam>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
    berth_query: Query<(Entity, &Berth)>,
    mut commands: Commands,
    mut autosave_events: EventWriter<AutosaveEvent>,
) {
    if let Ok((mut station, s_transform)) = station_query.get_single_mut() {
        for (ship_entity, mut ship, mut transform, mut assignment) in ship_query.iter_mut() {
            match ship.state {
                AutonomousShipState::Holding => {
                    ship.state = AutonomousShipState::Outbound;
                    commands.entity(ship_entity).remove::<DockedAt>();
                    add_log_entry(&mut station, "[STATION AI] Dispatching autonomous unit.".to_string());
                }
                AutonomousShipState::Outbound => {
                    let direction = assignment.target_pos - transform.translation.truncate();
                    let distance = direction.length();
                    if distance < ARRIVAL_THRESHOLD_MINING {
                        ship.state = AutonomousShipState::Mining;
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Mining => {
                    ship.cargo = (ship.cargo + MINING_RATE * time.delta_secs()).min(CARGO_CAPACITY as f32);
                    if ship.cargo >= CARGO_CAPACITY as f32 {
                        ship.state = AutonomousShipState::Returning;
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
                        
                        // Find Drone Berth entity to dock
                        if let Some((b_ent, _)) = berth_query.iter().find(|(_, b)| b.berth_type == BerthType::Drone) {
                            commands.entity(ship_entity).insert(DockedAt(b_ent));
                        }
                        
                        // Trigger autosave when drone docks
                        autosave_events.send(AutosaveEvent);

                        // [PHASE B] Trigger station resume if not already resuming
                        station.dock_state = StationDockState::Resuming;
                        station.resume_timer = STATION_RESUME_DELAY;
                    } else {
                        let movement = direction.normalize() * SHIP_SPEED * time.delta_secs();
                        transform.translation += movement.extend(0.0);
                    }
                }
                AutonomousShipState::Unloading => {
                    let ore_name = match assignment.ore_type {
                        OreDeposit::Iron => "Iron",
                        OreDeposit::Tungsten => "Tungsten",
                        OreDeposit::Nickel => "Nickel",
                    };
                    match assignment.ore_type {
                        OreDeposit::Iron => station.iron_reserves += ship.cargo,
                        OreDeposit::Tungsten => station.tungsten_reserves += ship.cargo,
                        OreDeposit::Nickel => station.nickel_reserves += ship.cargo,
                    }

                    let msg = format!("[STATION AI] Cargo deposited: {}. {} recovered.", assignment.sector_name, ore_name);
                    add_log_entry(&mut station, msg);
                    ship.cargo = 0.0;
                    
                    ship.state = AutonomousShipState::Holding;
                }
            }
        }
    }
}

pub fn autonomous_beam_system(
    ship_query: Query<(&AutonomousShip, &Transform, &AutonomousAssignment, &Children), (Without<MiningBeam>, Without<Station>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
    mut beam_query: Query<(&mut Transform, &mut Visibility), (With<MiningBeam>, Without<AutonomousShip>, Without<Station>, Without<ActiveAsteroid>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<Berth>)>,
) {
    for (ship, transform, assignment, children) in ship_query.iter() {
        for &child in children.iter() {
            if let Ok((mut b_transform, mut b_vis)) = beam_query.get_mut(child) {
                if ship.state == AutonomousShipState::Mining {
                    let dist = transform.translation.truncate().distance(assignment.target_pos);
                    *b_vis = Visibility::Visible;
                    
                    b_transform.translation = Vec3::new(0.0, dist / 2.0, -0.1);
                    b_transform.scale = Vec3::new(1.0, dist / 8.0, 1.0);
                } else {
                    *b_vis = Visibility::Hidden;
                }
            }
        }
    }
}

/// [PHASE B] Locks autonomous ships to berth position throughout rotation
pub fn docked_autonomous_ship_system(
    mut ship_query: Query<(&AutonomousShip, &mut Transform, &DockedAt), (With<AutonomousShip>, Without<Ship>, Without<Station>, Without<Berth>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>)>,
    berth_query: Query<&Berth>,
    station_query: Query<(&Station, &Transform), (With<Station>, Without<Ship>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
) {
    for (ship, mut transform, docked_at) in ship_query.iter_mut() {
        if ship.state == AutonomousShipState::Unloading || ship.state == AutonomousShipState::Holding {
            let target_ent = docked_at.0;
            if let Ok(berth) = berth_query.get(target_ent) {
                if let Ok((station, s_transform)) = station_query.get_single() {
                    let pos = berth_world_pos(
                        s_transform.translation.truncate(),
                        station.rotation,
                        berth.arm_index,
                    );
                    transform.translation = pos.extend(Z_SHIP);
                    
                    // Rotate drone to match arm direction
                    let arm_angle = station.rotation + (berth.arm_index as f32 * std::f32::consts::TAU / 6.0);
                    transform.rotation = Quat::from_rotation_z(arm_angle - std::f32::consts::FRAC_PI_2);
                }
            }
        }
    }
}
