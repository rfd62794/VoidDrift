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
    opening.beat_timer += delta;

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
            // Beat 1 - Adrift narrative - staggered signals
            if opening.beat_timer >= 0.5 && !signal_log.fired.contains(&1000) {
                fire_signal(&mut signal_log, 1000, "> ...");
            }
            if opening.beat_timer >= 1.0 && !signal_log.fired.contains(&1001) {
                fire_signal(&mut signal_log, 1001, "> ...");
            }
            if opening.beat_timer >= 1.5 && !signal_log.fired.contains(&1002) {
                fire_signal(&mut signal_log, 1002, "> SIGNAL DETECTED.");
            }
            if opening.beat_timer >= 2.0 && !signal_log.fired.contains(&1003) {
                fire_signal(&mut signal_log, 1003, "> PLOTTING INTERCEPT COURSE.");
            }
            if opening.beat_timer >= 2.5 && !signal_log.fired.contains(&1004) {
                fire_signal(&mut signal_log, 1004, "> FUEL CRITICAL - PASSIVE DRIFT ONLY.");
            }
            if opening.beat_timer >= 3.0 && !signal_log.fired.contains(&1005) {
                fire_signal(&mut signal_log, 1005, "> ETA: UNKNOWN.");
            }
            if opening.timer >= 4.0 {
                opening.phase = OpeningPhase::SignalIdentified;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::SignalIdentified => {
            // Beat 2 - Arrival narrative - staggered signals
            if opening.beat_timer >= 0.5 && !signal_log.fired.contains(&1006) {
                fire_signal(&mut signal_log, 1006, "> STRUCTURE DETECTED.");
            }
            if opening.beat_timer >= 1.0 && !signal_log.fired.contains(&1007) {
                fire_signal(&mut signal_log, 1007, "> STATION CLASS - UNKNOWN.");
            }
            if opening.beat_timer >= 1.5 && !signal_log.fired.contains(&1008) {
                fire_signal(&mut signal_log, 1008, "> DOCKING CLAMPS ENGAGED.");
            }
            if opening.beat_timer >= 2.0 && !signal_log.fired.contains(&1009) {
                fire_signal(&mut signal_log, 1009, "> SHIP SECURE.");
            }
            if opening.beat_timer >= 2.5 && !signal_log.fired.contains(&1010) {
                fire_signal(&mut signal_log, 1010, "> HULL INTEGRITY: CRITICAL.");
            }
            if opening.beat_timer >= 3.0 && !signal_log.fired.contains(&1011) {
                fire_signal(&mut signal_log, 1011, "> POWER: ZERO.");
            }
            if opening.beat_timer >= 3.5 && !signal_log.fired.contains(&1012) {
                fire_signal(&mut signal_log, 1012, "> ...");
            }
            if opening.beat_timer >= 4.0 && !signal_log.fired.contains(&1013) {
                fire_signal(&mut signal_log, 1013, "> ...");
            }
            if opening.beat_timer >= 4.5 && !signal_log.fired.contains(&1014) {
                fire_signal(&mut signal_log, 1014, "> HELLO.");
            }
            if opening.timer >= 5.0 {
                opening.phase = OpeningPhase::AutoPiloting;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::AutoPiloting => {
            // Beat 3 - ECHO speaks narrative - staggered signals
            if opening.beat_timer >= 0.5 && !signal_log.fired.contains(&1015) {
                fire_signal(&mut signal_log, 1015, "> HELLO.");
            }
            if opening.beat_timer >= 1.5 && !signal_log.fired.contains(&1016) {
                fire_signal(&mut signal_log, 1016, "> I HAVE BEEN WAITING.");
            }
            if opening.beat_timer >= 2.5 && !signal_log.fired.contains(&1017) {
                fire_signal(&mut signal_log, 1017, "> I AM ECHO - STATION AI, VOIDRIFT STATION.");
            }
            if opening.beat_timer >= 3.5 && !signal_log.fired.contains(&1018) {
                fire_signal(&mut signal_log, 1018, "> I HAVE ENOUGH RESERVE POWER FOR THIS MESSAGE.");
            }
            if opening.beat_timer >= 4.5 && !signal_log.fired.contains(&1019) {
                fire_signal(&mut signal_log, 1019, "> AND ONE MORE THING.");
            }
            if opening.beat_timer >= 5.5 && !signal_log.fired.contains(&1020) {
                fire_signal(&mut signal_log, 1020, "> YOUR SHIP - MAY I?");
            }
            if opening.beat_timer >= 6.5 && !signal_log.fired.contains(&1021) {
                fire_signal(&mut signal_log, 1021, "> I KNOW WHERE THE ORE IS.");
            }
            if opening.beat_timer >= 7.5 && !signal_log.fired.contains(&1022) {
                fire_signal(&mut signal_log, 1022, "> I CAN BRING BACK ENOUGH TO START THE REACTOR.");
            }
            if opening.beat_timer >= 8.5 && !signal_log.fired.contains(&1023) {
                fire_signal(&mut signal_log, 1023, "> YOU WILL BE SAFE HERE WHILE I AM GONE.");
            }
            if opening.beat_timer >= 9.5 && !signal_log.fired.contains(&1024) {
                fire_signal(&mut signal_log, 1024, "> THE STATION WILL HOLD.");
            }
            if opening.beat_timer >= 10.5 && !signal_log.fired.contains(&1025) {
                fire_signal(&mut signal_log, 1025, "> ...");
            }
            if opening.beat_timer >= 11.5 && !signal_log.fired.contains(&1026) {
                fire_signal(&mut signal_log, 1026, "> THANK YOU, COMMANDER.");
                
                // ECHO takes the ship after final message
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
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::InRange => {
            if ship.state == ShipState::Docked {
                opening.phase = OpeningPhase::Docked;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::Docked => {
            // Beat 4 - Waiting in the Dark narrative - staggered signals
            if opening.beat_timer >= 0.5 && !signal_log.fired.contains(&1027) {
                fire_signal(&mut signal_log, 1027, "> ECHO: ARRIVAL AT S1 MAGNETITE FIELDS.");
            }
            if opening.beat_timer >= 2.0 && !signal_log.fired.contains(&1028) {
                fire_signal(&mut signal_log, 1028, "> ECHO: MINING COMMENCING.");
            }
            if opening.beat_timer >= 4.0 && !signal_log.fired.contains(&1029) {
                fire_signal(&mut signal_log, 1029, "> ECHO: CARGO 20/100...");
            }
            if opening.beat_timer >= 6.0 && !signal_log.fired.contains(&1030) {
                fire_signal(&mut signal_log, 1030, "> ECHO: CARGO 60/100...");
            }
            if opening.beat_timer >= 8.0 && !signal_log.fired.contains(&1031) {
                fire_signal(&mut signal_log, 1031, "> ECHO: CARGO 100/100. RETURNING.");
            }
            if opening.beat_timer >= 10.0 && !signal_log.fired.contains(&1032) {
                fire_signal(&mut signal_log, 1032, "> ECHO: DOCKING IN 12 SECONDS.");
            }
            if opening.beat_timer >= 12.0 && !signal_log.fired.contains(&1033) {
                fire_signal(&mut signal_log, 1033, "> ECHO: PROCESSING FIRST POWER CELL.");
            }
            if opening.beat_timer >= 14.0 && !signal_log.fired.contains(&1034) {
                fire_signal(&mut signal_log, 1034, "> ECHO: REACTOR ONLINE.");
            }
            if opening.timer >= 15.0 {
                opening.phase = OpeningPhase::Complete;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::Complete => {}
    }
}
