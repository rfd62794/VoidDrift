use bevy::prelude::*;
use crate::components::*;

pub fn signal_system(
    time: Res<Time>,
    mut signal: ResMut<SignalLog>,
    opening: Res<OpeningSequence>,
    station_query: Query<(&Station, &StationQueues), (With<Station>, Without<Ship>, Without<AutonomousShip>)>,
    auto_ships: Query<&AutonomousShip, With<AutonomousShipTag>>,
    ship_query: Query<(&Ship, &Transform), (With<PlayerShip>, Without<Station>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<AsteroidField>, Without<Berth>)>,
    mut quest_log: ResMut<QuestLog>,
) {
    let now = time.elapsed_secs();
    let station_res = station_query.get_single();

    // ID 1: Game Start
    fire_signal(&mut signal, 1, "> SIGNAL RECEIVED.");

    // [Step 4] Quest Sync Logic (Side-effects of signals)
    // We update quest objectives whenever the corresponding signal ID has been fired.
    for obj in quest_log.objectives.iter_mut() {
        match obj.id {
            1 => { // Locate signal source
                if signal.fired.contains(&4) { // STRUCTURE DETECTED
                    obj.state = ObjectiveState::Complete;
                }
            }
            2 => { // Dock at derelict station
                if signal.fired.contains(&4) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&5) { // DOCKING COMPLETE
                    obj.state = ObjectiveState::Complete;
                }
            }
            3 => { // Repair station
                if signal.fired.contains(&5) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&11) { // STATION ONLINE
                    obj.state = ObjectiveState::Complete;
                }
            }
            4 => { // Build AI Core
                if signal.fired.contains(&11) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&13) { // AI CORE NOMINAL
                    obj.state = ObjectiveState::Complete;
                }
            }
            5 => { // Discover Sector 7
                if signal.fired.contains(&13) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&14) { // CARBON SIGNATURE / S7 DETECTED
                    obj.state = ObjectiveState::Complete;
                }
            }
            6 => { // Mine Carbon
                if signal.fired.contains(&14) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&16) { // SHIP HULL COMPLETE (implies Carbon found/refined)
                    obj.state = ObjectiveState::Complete;
                }
            }
            7 => { // Assemble autonomous ship
                if signal.fired.contains(&16) && obj.state == ObjectiveState::Locked {
                    obj.state = ObjectiveState::Active;
                }
                if signal.fired.contains(&17) { // AUTONOMOUS UNIT LAUNCHED
                    obj.state = ObjectiveState::Complete;
                }
            }
            _ => {}
        }
    }

    // ID 2: 2s after start
    if opening.timer >= crate::constants::SIGNAL_PAUSE_S2 && opening.phase == OpeningPhase::SignalIdentified {
        fire_signal(&mut signal, 2, "> SOURCE IDENTIFIED. BEARING 047.");
    }

    // ID 3: Autopiloting
    if opening.phase == OpeningPhase::AutoPiloting {
        fire_signal(&mut signal, 3, "> MOVING TO INVESTIGATE.");
    }

    // ID 4: Station in range
    if opening.phase == OpeningPhase::InRange {
        fire_signal(&mut signal, 4, "> STRUCTURE DETECTED. DERELICT CLASS.");
    }

    // ID 5: Docked
    if opening.phase == OpeningPhase::Docked || opening.phase == OpeningPhase::Complete {
        fire_signal(&mut signal, 5, "> DOCKING COMPLETE.");
    }

    // ID 6: 1s after dock
    if opening.phase == OpeningPhase::Docked && opening.timer >= crate::constants::SIGNAL_PAUSE_DOCK_REPORT {
        fire_signal(&mut signal, 6, "> POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.");
    }

    // After opening completes, check world triggers
    if opening.phase == OpeningPhase::Complete {
        if let Ok((st, _)) = station_res {
            // ID 7: Reserves unlocked
            fire_signal(&mut signal, 7, "> REPAIRS POSSIBLE. MATERIALS REQUIRED.");

            // ID 8: First Magnetite
            if st.iron_reserves > 0.0 {
                fire_signal(&mut signal, 8, "> IRON ACQUIRED. REFINERY READY.");
            }

            // ID 9: First Cells
            if st.tungsten_reserves > 0.0 {
                fire_signal(&mut signal, 9, "> TUNGSTEN PRODUCED. REPAIR THRESHOLD: 25.");
            }

            // ID 10: Threshold 25
            if st.iron_reserves >= 25.0 {
                fire_signal(&mut signal, 10, "> REPAIR THRESHOLD MET. INITIATE WHEN READY.");
            }

            // ID 11: Station Online
            if st.online {
                if fire_signal(&mut signal, 11, "> POWER RESTORED. STATION ONLINE.") {
                    signal.last_fired_at.insert(11, now);
                }
                fire_signal(&mut signal, 27, "> AUTOMATED SYSTEMS ONLINE.");
            }

            // ID 13: AI Core
            if st.ai_cores > 0 {
                if fire_signal(&mut signal, 13, "> AI CORE NOMINAL. SECTOR 7 SCAN INITIATED.") {
                    signal.last_fired_at.insert(13, now);
                }
            }

            // ID 15: Hull Plate
            if st.hull_plate_reserves > 0 {
                fire_signal(&mut signal, 15, "> HULL PLATE FABRICATED. FORGE AVAILABLE.");
            }

            // ID 16: Ship Hull
            if st.ship_hulls > 0 {
                fire_signal(&mut signal, 16, "> SHIP HULL COMPLETE. ASSEMBLY POSSIBLE.");
            }

            // ID 25, 26: First dock pre-online
            if !st.online {
                if fire_signal(&mut signal, 25, "> SMELTER OPERATIONAL. MANUAL MODE.") {
                    signal.last_fired_at.insert(25, now);
                }
                
                if signal.fired.contains(&25) {
                    if let Some(t25) = signal.last_fired_at.get(&25) {
                        if now - *t25 >= 1.0 {
                            fire_signal(&mut signal, 26, "> FORGE OPERATIONAL. MANUAL MODE.");
                        }
                    }
                }
            }
        }

        // Periodic/Stateful Triggers
        if let Ok((st, _)) = station_res {
            // ID 12: 2s after ID 11 (Online)
            if st.online && signal.fired.contains(&11) {
                if let Some(t11) = signal.last_fired_at.get(&11) {
                    if now - *t11 >= 2.0 {
                        fire_signal(&mut signal, 12, "> AI CORE FABRICATION NOW AVAILABLE.");
                    }
                }
            }

            // ID 14: 3s after ID 13 (Core)
            if st.ai_cores > 0 && signal.fired.contains(&13) {
                if let Some(t13) = signal.last_fired_at.get(&13) {
                    if now - *t13 >= 3.0 {
                        fire_signal(&mut signal, 14, "> CARBON SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 7.");
                    }
                }
            }
            
            // ID 19: Critical Power (Refirable)
            fire_refirable(&mut signal, 19, "> MATERIAL RESERVES CRITICAL. MINING RUN REQUIRED.", 
                now,
                st.iron_reserves < 5.0, 
                st.iron_reserves >= 15.0
            );

            // [PHASE B] Docking Sequence Signals
            fire_refirable(&mut signal, 28, "> INCOMING VESSEL DETECTED. DOCKING SEQUENCE INITIATED.",
                now,
                st.dock_state == StationDockState::Slowing,
                st.dock_state == StationDockState::Rotating
            );
            fire_refirable(&mut signal, 29, "> ROTATION SUSPENDED. BERTH ALIGNED.",
                now,
                st.dock_state == StationDockState::Paused,
                st.dock_state == StationDockState::Slowing
            );
            fire_refirable(&mut signal, 30, "> DOCKING COMPLETE. ROTATION RESUMING.",
                now,
                st.dock_state == StationDockState::Resuming,
                st.dock_state == StationDockState::Rotating
            );

            let any_docked = ship_query.iter().any(|(s, _)| s.state == ShipState::Docked);
            let s30_fired = signal.fired.contains(&30);
            fire_refirable(&mut signal, 31, "> VESSEL DEPARTED. BERTH CLEAR.",
                now,
                st.dock_state == StationDockState::Rotating && !any_docked && s30_fired,
                st.dock_state == StationDockState::Resuming
            );
        }

        if let Ok((_st, q)) = station_res {
            let processing_active = q.iron_refinery.is_some() || q.tungsten_refinery.is_some() || q.nickel_refinery.is_some() || q.hull_forge.is_some() || q.core_fabricator.is_some();
            
            fire_refirable(&mut signal, 32, "> INDUSTRIAL PROCESSING ACTIVE. PARALLEL QUEUES COMMENCED.",
                now,
                processing_active,
                !processing_active
            );

            let s32_fired = signal.fired.contains(&32);
            fire_refirable(&mut signal, 33, "> PROCESSING QUEUES EMPTY. PRODUCTION HALTED.",
                now,
                !processing_active && s32_fired,
                processing_active
            );
        }

        // ID 17, 18: Fleet expansion
        let auto_count = auto_ships.iter().count();
        if auto_count >= 1 {
            fire_signal(&mut signal, 17, "> AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.");
        }
        if auto_count >= 2 {
            fire_signal(&mut signal, 18, "> AUTONOMOUS UNIT LAUNCHED. SECTOR 7 ASSIGNED.");
        }

        // ID 20, 21: Auto ship holding/dispatched
        let any_holding = auto_ships.iter().any(|s| s.state == AutonomousShipState::Holding);
        let any_active = auto_ships.iter().any(|s| s.state != AutonomousShipState::Holding);
        let was_holding = signal.fired.contains(&20);

        fire_refirable(&mut signal, 20, "> AUTONOMOUS UNIT HOLDING. POWER INSUFFICIENT.",
            now,
            any_holding,
            !any_holding
        );
        fire_refirable(&mut signal, 21, "> AUTONOMOUS UNIT DISPATCHED.",
            now,
            any_active && was_holding,
            !any_active || !was_holding
        );
    }
}

fn fire_signal(log: &mut SignalLog, id: u32, line: &str) -> bool {
    if !log.fired.contains(&id) {
        log.entries.push_back(line.to_string());
        log.fired.insert(id);
        if log.entries.len() > 10 {
            log.entries.pop_front();
        }
        return true;
    }
    false
}

fn fire_refirable(log: &mut SignalLog, id: u32, line: &str, now: f32, condition: bool, reset_condition: bool) {
    let last_fired = log.last_fired_at.get(&id).cloned().unwrap_or(-100.0);
    
    if condition && !log.fired.contains(&id) && now - last_fired > 2.0 {
        if fire_signal(log, id, line) {
            log.last_fired_at.insert(id, now);
        }
    } else if reset_condition {
        log.fired.remove(&id);
    }
}
