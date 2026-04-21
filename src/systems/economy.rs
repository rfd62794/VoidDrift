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
    mut drawer_state: ResMut<DrawerState>,
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
                *drawer_state = DrawerState::Collapsed;  // Auto-collapse on undock
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

pub fn processing_queue_system(
    time: Res<Time>,
    mut station_query: Query<(&mut Station, &mut StationQueues), Without<Ship>>,
) {
    if let Ok((mut station, mut queues)) = station_query.get_single_mut() {
        let delta = time.delta_secs();

        // 1. Magnetite Refinery
        if let Some(mut job) = queues.magnetite_refinery.take() {
            process_job(&mut job, &mut station, delta, REFINERY_MAGNETITE_TIME, ProcessingOperation::MagnetiteRefinery);
            if job.batches > 0 {
                queues.magnetite_refinery = Some(job);
            }
        }

        // 2. Carbon Refinery
        if let Some(mut job) = queues.carbon_refinery.take() {
            process_job(&mut job, &mut station, delta, REFINERY_CARBON_TIME, ProcessingOperation::CarbonRefinery);
            if job.batches > 0 {
                queues.carbon_refinery = Some(job);
            }
        }

        // 3. Hull Forge
        if let Some(mut job) = queues.hull_forge.take() {
            process_job(&mut job, &mut station, delta, FORGE_HULL_TIME, ProcessingOperation::HullForge);
            if job.batches > 0 {
                queues.hull_forge = Some(job);
            }
        }

        // 4. Core Fabricator
        if let Some(mut job) = queues.core_fabricator.take() {
            process_job(&mut job, &mut station, delta, FORGE_CORE_TIME, ProcessingOperation::CoreFabricator);
            if job.batches > 0 {
                queues.core_fabricator = Some(job);
            }
        }
    }
}

fn process_job(job: &mut ProcessingJob, station: &mut Station, delta: f32, batch_time: f32, op: ProcessingOperation) {
    if job.batches == 0 { return; }

    job.timer -= delta;
    if job.timer <= 0.0 {
        // Batch Complete
        match op {
            ProcessingOperation::MagnetiteRefinery => station.power_cells += 1,
            ProcessingOperation::CarbonRefinery => station.hull_plate_reserves += 1,
            ProcessingOperation::HullForge => station.ship_hulls += 1,
            ProcessingOperation::CoreFabricator => station.ai_cores += 1,
        }
        job.completed += 1;
        job.batches -= 1;

        if job.batches > 0 && !job.clearing {
            job.timer = batch_time;
        } else {
            // Queue empty or clearing finished
            job.batches = 0; 
            add_log_entry(station, format!("> {:?} COMPLETE.", op).to_uppercase());
        }
    }
}

pub fn auto_dock_system(
    mut removed_autopilot: RemovedComponents<AutopilotTarget>,
    mut ship_query: Query<&mut Ship, (With<PlayerShip>, Without<Station>)>,
    mut station_query: Query<(&mut Station, &mut StationQueues), (With<Station>, Without<Ship>)>,
    settings: Res<AutoDockSettings>,
) {
    for _ in removed_autopilot.read() {
        if let Ok(mut ship) = ship_query.get_single_mut() {
            if ship.state == ShipState::Docked {
                if let Ok((mut station, mut queues)) = station_query.get_single_mut() {
                    // 1. AUTO UNLOAD
                    if settings.auto_unload && ship.cargo > 0.0 {
                        match ship.cargo_type {
                            OreType::Magnetite => station.magnetite_reserves += ship.cargo,
                            OreType::Carbon => station.carbon_reserves += ship.cargo,
                            _ => {}
                        }
                        add_log_entry(&mut station, "> CARGO UNLOADED AUTOMATICALLY.".to_string());
                        ship.cargo = 0.0;
                        ship.cargo_type = OreType::Empty;
                    }

                    // 2. AUTO SMELT MAGNETITE
                    if settings.auto_smelt_magnetite {
                        let batches = (station.magnetite_reserves / REFINERY_RATIO as f32).floor() as u32;
                        if batches > 0 {
                            queue_job(&mut station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, batches);
                            add_log_entry(&mut station, "> MAGNETITE QUEUED FOR REFINING.".to_string());
                        }
                    }

                    // 3. AUTO SMELT CARBON
                    if settings.auto_smelt_carbon {
                        let batches = (station.carbon_reserves / HULL_PLATE_COST_CARBON as f32).floor() as u32;
                        if batches > 0 {
                            queue_job(&mut station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, batches);
                            add_log_entry(&mut station, "> CARBON QUEUED FOR REFINING.".to_string());
                        }
                    }
                }
            }
        }
    }
}

pub fn queue_job(station: &mut Station, queue: &mut Option<ProcessingJob>, op: ProcessingOperation, batches: u32) {
    if batches == 0 { return; }
    
    // Calculate cost
    let (cost_mag, cost_carb, cost_plates, cost_cells, pwr, time) = match op {
        ProcessingOperation::MagnetiteRefinery => (batches as f32 * REFINERY_RATIO as f32, 0.0, 0, 0, batches as f32 * POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME),
        ProcessingOperation::CarbonRefinery => (0.0, batches as f32 * HULL_PLATE_COST_CARBON as f32, 0, 0, batches as f32 * POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME),
        ProcessingOperation::HullForge => (0.0, 0.0, batches * SHIP_HULL_COST_PLATES, 0, batches as f32 * POWER_COST_SHIP_FORGE as f32, FORGE_HULL_TIME),
        ProcessingOperation::CoreFabricator => (0.0, 0.0, 0, batches * AI_CORE_COST_CELLS, batches as f32 * POWER_COST_AI_FABRICATE as f32, FORGE_CORE_TIME),
    };

    // Deduct resources
    station.magnetite_reserves -= cost_mag;
    station.carbon_reserves -= cost_carb;
    station.hull_plate_reserves = station.hull_plate_reserves.saturating_sub(cost_plates);
    station.power_cells = station.power_cells.saturating_sub(cost_cells);
    station.power -= pwr;

    // Update queue
    if let Some(ref mut job) = queue {
        job.batches += batches;
        job.clearing = false; // Resume queuing if it was clearing
    } else {
        *queue = Some(ProcessingJob {
            operation: op,
            batches,
            timer: time,
            completed: 0,
            clearing: false,
        });
    }
}
