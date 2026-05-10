use bevy::prelude::*;
use crate::components::*;
use crate::config::BalanceConfig;

pub fn signal_system(
    time: Res<Time>,
    mut signal: ResMut<SignalLog>,
    opening: Res<OpeningSequence>,
    station_query: Query<(&Station, &StationQueues), (With<Station>, Without<Ship>)>,
    queue: Res<ShipQueue>,
    ship_query: Query<(&Ship, &Transform), (With<InOpeningSequence>, Without<Station>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
    mut signal_fired_events: EventWriter<SignalFired>,
    cfg: Res<BalanceConfig>,
) {
    let now = time.elapsed_secs();
    let station_res = station_query.get_single();

    // ID 1: Game Start
    emit(&mut signal, &mut signal_fired_events, 1, "> SIGNAL RECEIVED.");

    // ID 2: 2s after start
    if opening.timer >= cfg.narrative.signal_pause_s2 && opening.phase == OpeningPhase::SignalIdentified {
        emit(&mut signal, &mut signal_fired_events, 2, "> SOURCE IDENTIFIED. BEARING 047.");
    }

    // ID 3: Autopiloting
    if opening.phase == OpeningPhase::AutoPiloting {
        emit(&mut signal, &mut signal_fired_events, 3, "> MOVING TO INVESTIGATE.");
    }

    // ID 4: Station in range
    if opening.phase == OpeningPhase::InRange {
        emit(&mut signal, &mut signal_fired_events, 4, "> STRUCTURE DETECTED. DERELICT CLASS.");
    }

    // ID 5: Docked
    if opening.phase == OpeningPhase::Docked || opening.phase == OpeningPhase::Complete {
        emit(&mut signal, &mut signal_fired_events, 5, "> DOCKING COMPLETE.");
    }

    // ID 6: 1s after dock
    if opening.phase == OpeningPhase::Docked && opening.timer >= cfg.narrative.signal_pause_dock_report {
        emit(&mut signal, &mut signal_fired_events, 6, "> POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.");
    }

    // After opening completes, check world triggers
    if opening.phase == OpeningPhase::Complete {
        if let Ok((st, _)) = station_res {
            // ID 7: Reserves unlocked
            emit(&mut signal, &mut signal_fired_events, 7, "> REPAIRS POSSIBLE. MATERIALS REQUIRED.");

            // ID 8: First Magnetite
            if st.iron_reserves > 0.0 {
                emit(&mut signal, &mut signal_fired_events, 8, "> IRON ACQUIRED. REFINERY READY.");
            }

            // ID 9: First Cells
            if st.tungsten_reserves > 0.0 {
                emit(&mut signal, &mut signal_fired_events, 9, "> TUNGSTEN PRODUCED. REPAIR THRESHOLD: 25.");
            }

            // ID 10: Threshold 25
            if st.iron_reserves >= 25.0 {
                emit(&mut signal, &mut signal_fired_events, 10, "> REPAIR THRESHOLD MET. INITIATE WHEN READY.");
            }

            // ID 11: Station Online
            if st.online {
                if emit(&mut signal, &mut signal_fired_events, 11, "> POWER RESTORED. STATION ONLINE.") {
                    signal.last_fired_at.insert(11, now);
                }
                emit(&mut signal, &mut signal_fired_events, 27, "> AUTOMATED SYSTEMS ONLINE.");
            }

            // ID 13: AI Core
            if st.ai_cores > 0.0 {
                if emit(&mut signal, &mut signal_fired_events, 13, "> AI CORE NOMINAL. SECTOR 7 SCAN INITIATED.") {
                    signal.last_fired_at.insert(13, now);
                }
            }

            // ID 15: Hull Plate
            if st.hull_plate_reserves > 0.0 {
                emit(&mut signal, &mut signal_fired_events, 15, "> HULL PLATE FABRICATED. FORGE AVAILABLE.");
            }

            // ID 16: Drone queue has a ship ready
            if queue.available_count > 0 {
                emit(&mut signal, &mut signal_fired_events, 16, "> SHIP HULL COMPLETE. ASSEMBLY POSSIBLE.");
            }

            // ID 25, 26: First dock pre-online
            if !st.online {
                if emit(&mut signal, &mut signal_fired_events, 25, "> SMELTER OPERATIONAL. MANUAL MODE.") {
                    signal.last_fired_at.insert(25, now);
                }

                if signal.fired.contains(&25) {
                    if let Some(t25) = signal.last_fired_at.get(&25) {
                        if now - *t25 >= 1.0 {
                            emit(&mut signal, &mut signal_fired_events, 26, "> FORGE OPERATIONAL. MANUAL MODE.");
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
                    if now - *t11 >= cfg.narrative.signal_pause_complete {
                        emit(&mut signal, &mut signal_fired_events, 12, "> AI CORE FABRICATION NOW AVAILABLE.");
                    }
                }
            }

            // ID 14: 3s after ID 13 (Core)
            if st.ai_cores > 0.0 && signal.fired.contains(&13) {
                if let Some(t13) = signal.last_fired_at.get(&13) {
                    if now - *t13 >= 3.0 {
                        emit(&mut signal, &mut signal_fired_events, 14, "> CARBON SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 7.");
                    }
                }
            }

            // ID 19: Critical Power (Refirable)
            emit_refirable(&mut signal, &mut signal_fired_events, 19, "> MATERIAL RESERVES CRITICAL. MINING RUN REQUIRED.",
                now,
                st.iron_reserves < 5.0,
                st.iron_reserves >= 15.0
            );

            // [PHASE B] Docking Sequence Signals
            emit_refirable(&mut signal, &mut signal_fired_events, 28, "> INCOMING VESSEL DETECTED. DOCKING SEQUENCE INITIATED.",
                now,
                st.dock_state == StationDockState::Slowing,
                st.dock_state == StationDockState::Rotating
            );
            emit_refirable(&mut signal, &mut signal_fired_events, 29, "> ROTATION SUSPENDED. BERTH ALIGNED.",
                now,
                st.dock_state == StationDockState::Paused,
                st.dock_state == StationDockState::Slowing
            );
            emit_refirable(&mut signal, &mut signal_fired_events, 30, "> DOCKING COMPLETE. ROTATION RESUMING.",
                now,
                st.dock_state == StationDockState::Resuming,
                st.dock_state == StationDockState::Rotating
            );

            let any_docked = ship_query.iter().any(|(s, _)| s.state == ShipState::Docked);
            let s30_fired = signal.fired.contains(&30);
            emit_refirable(&mut signal, &mut signal_fired_events, 31, "> VESSEL DEPARTED. BERTH CLEAR.",
                now,
                st.dock_state == StationDockState::Rotating && !any_docked && s30_fired,
                st.dock_state == StationDockState::Resuming
            );
        }

        if let Ok((_st, q)) = station_res {
            let processing_active = q.iron_refinery.is_some() || q.tungsten_refinery.is_some() || q.nickel_refinery.is_some() || q.hull_forge.is_some() || q.core_fabricator.is_some();

            emit_refirable(&mut signal, &mut signal_fired_events, 32, "> INDUSTRIAL PROCESSING ACTIVE. PARALLEL QUEUES COMMENCED.",
                now,
                processing_active,
                !processing_active
            );

            let s32_fired = signal.fired.contains(&32);
            emit_refirable(&mut signal, &mut signal_fired_events, 33, "> PROCESSING QUEUES EMPTY. PRODUCTION HALTED.",
                now,
                !processing_active && s32_fired,
                processing_active
            );
        }

        // ID 17, 18: Fleet expansion — based on queue count
        if queue.available_count >= 1 {
            emit(&mut signal, &mut signal_fired_events, 17, "> AUTONOMOUS UNIT READY. AWAITING ASSIGNMENT.");
        }
        if queue.available_count >= 2 {
            emit(&mut signal, &mut signal_fired_events, 18, "> FLEET STRENGTH GROWING.");
        }
    }
}

/// Commits a one-shot signal. Returns true if newly fired (false if already seen).
fn emit(log: &mut SignalLog, events: &mut EventWriter<SignalFired>, id: u32, line: &str) -> bool {
    if !log.fired.contains(&id) {
        log.entries.push_back(line.to_string());
        log.fired.insert(id);
        if log.entries.len() > 10 {
            log.entries.pop_front();
        }
        events.send(SignalFired { signal_id: id });
        return true;
    }
    false
}

/// Commits a refirable signal (resets when reset_condition is true).
fn emit_refirable(log: &mut SignalLog, events: &mut EventWriter<SignalFired>, id: u32, line: &str, now: f32, condition: bool, reset_condition: bool) {
    let last_fired = log.last_fired_at.get(&id).cloned().unwrap_or(-100.0);

    if condition && !log.fired.contains(&id) && now - last_fired > 2.0 {
        if emit(log, events, id, line) {
            log.last_fired_at.insert(id, now);
        }
    } else if reset_condition {
        log.fired.remove(&id);
    }
}
