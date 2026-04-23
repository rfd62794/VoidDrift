use bevy::prelude::*;
use crate::components::*;

fn fire_signal(signal_log: &mut SignalLog, id: u32, message: &str) {
    if !signal_log.fired.contains(&id) {
        signal_log.fired.insert(id);
        signal_log.entries.push_back(message.to_string());
        info!("Signal fired: {} - {}", id, message);
    }
}

pub fn opening_sequence_system(
    time: Res<Time>,
    mut opening: ResMut<OpeningSequence>,
    mut ship_query: Query<(Entity, &mut Ship, &mut Transform), (With<PlayerShip>, Without<AutonomousShipTag>)>,
    station_query: Query<(&Station, &Transform), (With<Station>, Without<Ship>)>,
    berth_query: Query<(Entity, &Berth), Without<Ship>>,
    mut commands: Commands,
    mut signal_log: ResMut<SignalLog>,
) {
    if opening.phase == OpeningPhase::Complete {
        return;
    }

    let delta = time.delta_secs();
    opening.timer += delta;

    let Ok((ship_ent, mut ship, ship_transform)) = ship_query.get_single_mut() else { return; };
    let Ok((st, station_transform)) = station_query.get_single() else { return; };
    
    // Find the player berth specifically (there are multiple berths now)
    let Some((berth_ent, berth)) = berth_query.iter().find(|(_, b)| b.berth_type == BerthType::Player) else { return; };

    // Calculate world pos from station rotation
    let berth_pos = berth_world_pos(
        station_transform.translation.truncate(),
        st.rotation,
        berth.arm_index
    );

    let dist_to_station = ship_transform.translation.truncate().distance(berth_pos);

    match opening.phase {
        OpeningPhase::Adrift => {
            // Beat 1 - Adrift narrative
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1000, "> ...");
                fire_signal(&mut signal_log, 1001, "> ...");
                fire_signal(&mut signal_log, 1002, "> SIGNAL DETECTED.");
                fire_signal(&mut signal_log, 1003, "> PLOTTING INTERCEPT COURSE.");
                fire_signal(&mut signal_log, 1004, "> FUEL CRITICAL - PASSIVE DRIFT ONLY.");
                fire_signal(&mut signal_log, 1005, "> ETA: UNKNOWN.");
            }
            if opening.timer >= 3.0 {
                opening.phase = OpeningPhase::SignalIdentified;
                opening.timer = 0.0;
            }
        }
        OpeningPhase::SignalIdentified => {
            // Beat 2 - Arrival narrative
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1006, "> STRUCTURE DETECTED.");
                fire_signal(&mut signal_log, 1007, "> STATION CLASS - UNKNOWN.");
                fire_signal(&mut signal_log, 1008, "> DOCKING CLAMPS ENGAGED.");
                fire_signal(&mut signal_log, 1009, "> SHIP SECURE.");
                fire_signal(&mut signal_log, 1010, "> HULL INTEGRITY: CRITICAL.");
                fire_signal(&mut signal_log, 1011, "> POWER: ZERO.");
                fire_signal(&mut signal_log, 1012, "> ...");
                fire_signal(&mut signal_log, 1013, "> ...");
                fire_signal(&mut signal_log, 1014, "> HELLO.");
            }
            if opening.timer >= 4.0 {
                opening.phase = OpeningPhase::AutoPiloting;
                opening.timer = 0.0;
            }
        }
        OpeningPhase::AutoPiloting => {
            // Beat 3 - ECHO speaks narrative
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1015, "> HELLO.");
                fire_signal(&mut signal_log, 1016, "> I HAVE BEEN WAITING.");
                fire_signal(&mut signal_log, 1017, "> I AM ECHO - STATION AI, VOIDRIFT STATION.");
                fire_signal(&mut signal_log, 1018, "> I HAVE ENOUGH RESERVE POWER FOR THIS MESSAGE.");
                fire_signal(&mut signal_log, 1019, "> AND ONE MORE THING.");
                fire_signal(&mut signal_log, 1020, "> YOUR SHIP - MAY I?");
                fire_signal(&mut signal_log, 1021, "> I KNOW WHERE THE ORE IS.");
                fire_signal(&mut signal_log, 1022, "> I CAN BRING BACK ENOUGH TO START THE REACTOR.");
                fire_signal(&mut signal_log, 1023, "> YOU WILL BE SAFE HERE WHILE I AM GONE.");
                fire_signal(&mut signal_log, 1024, "> THE STATION WILL HOLD.");
                fire_signal(&mut signal_log, 1025, "> ...");
                fire_signal(&mut signal_log, 1026, "> THANK YOU, COMMANDER.");
                
                // ECHO takes the ship
                ship.state = ShipState::Navigating;
                commands.entity(ship_ent).remove::<DockedAt>();
                commands.entity(ship_ent).insert(AutopilotTarget {
                    destination: berth_pos,
                    target_entity: Some(berth_ent),
                });
            }
            if dist_to_station < 300.0 {
                opening.phase = OpeningPhase::InRange;
                opening.timer = 0.0;
            }
        }
        OpeningPhase::InRange => {
            if ship.state == ShipState::Docked {
                opening.phase = OpeningPhase::Docked;
                opening.timer = 0.0;
            }
        }
        OpeningPhase::Docked => {
            // Beat 4 - Waiting in the Dark narrative
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1027, "> ECHO: ARRIVAL AT S1 MAGNETITE FIELDS.");
                fire_signal(&mut signal_log, 1028, "> ECHO: MINING COMMENCING.");
                fire_signal(&mut signal_log, 1029, "> ECHO: CARGO 20/100...");
                fire_signal(&mut signal_log, 1030, "> ECHO: CARGO 60/100...");
                fire_signal(&mut signal_log, 1031, "> ECHO: CARGO 100/100. RETURNING.");
                fire_signal(&mut signal_log, 1032, "> ECHO: DOCKING IN 12 SECONDS.");
                fire_signal(&mut signal_log, 1033, "> ECHO: PROCESSING FIRST POWER CELL.");
                fire_signal(&mut signal_log, 1034, "> ECHO: REACTOR ONLINE.");
            }
            if opening.timer >= 3.0 {
                opening.phase = OpeningPhase::Complete;
                opening.timer = 0.0;
            }
        }
        OpeningPhase::Complete => {}
    }
}
