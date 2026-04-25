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
    mut ship_query: Query<(Entity, &mut Ship, &mut Transform), (With<InOpeningSequence>, Without<AutonomousShipTag>)>,
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
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::SignalIdentified => {
            // Only structural detection signals here
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1001, "> SIGNAL DETECTED.");
                fire_signal(&mut signal_log, 1002, "> PLOTTING INTERCEPT COURSE.");
                fire_signal(&mut signal_log, 1003, "> FUEL CRITICAL - PASSIVE DRIFT ONLY.");
            }
            if opening.timer >= 4.0 {
                opening.phase = OpeningPhase::AutoPiloting;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::AutoPiloting => {
            // Ship is moving — only environmental signals
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1004, "> STRUCTURE DETECTED.");
                fire_signal(&mut signal_log, 1005, "> STATION CLASS - UNKNOWN.");
                
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
            opening.beat_timer += time.delta_secs();
            let t = opening.beat_timer;

            if t >= 0.5  { fire_signal(&mut signal_log, 1010, "> ..."); }
            if t >= 1.5  { fire_signal(&mut signal_log, 1011, "> HELLO."); }
            if t >= 2.5  { fire_signal(&mut signal_log, 1012, "> I HAVE BEEN WAITING."); }
            if t >= 3.5  { fire_signal(&mut signal_log, 1013, "> I AM ECHO - STATION AI, VOIDRIFT STATION."); }
            if t >= 5.0  { fire_signal(&mut signal_log, 1014, "> I HAVE ENOUGH RESERVE POWER FOR THIS MESSAGE."); }
            if t >= 6.5  { fire_signal(&mut signal_log, 1015, "> YOUR SHIP - MAY I?"); }
            if t >= 7.5  { fire_signal(&mut signal_log, 1016, "> I KNOW WHERE THE ORE IS."); }
            if t >= 9.0  { fire_signal(&mut signal_log, 1017, "> THANK YOU, COMMANDER."); }

            // Advance to Complete after last signal
            if t >= 10.5 {
                opening.phase = OpeningPhase::Complete;
                opening.beat_timer = 0.0;
                commands.entity(ship_ent).remove::<InOpeningSequence>();
            }
        }
        OpeningPhase::Complete => {}
    }
}
