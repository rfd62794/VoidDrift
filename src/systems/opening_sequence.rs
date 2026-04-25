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
    mut ship_query: Query<(Entity, &mut Ship, &mut Transform), With<InOpeningSequence>>,
    mut station_query: Query<(&mut Station, &Transform), (With<Station>, Without<Ship>)>,
    mut commands: Commands,
    mut signal_log: ResMut<SignalLog>,
    mut queue: ResMut<ShipQueue>,
) {
    if opening.phase == OpeningPhase::Complete {
        return;
    }

    let delta = time.delta_secs();
    opening.timer += delta;
    opening.beat_timer += delta;

    let Ok((ship_ent, mut ship, ship_transform)) = ship_query.get_single_mut() else { return; };
    let Ok((mut st, station_transform)) = station_query.get_single_mut() else { return; };

    let station_pos = station_transform.translation.truncate();
    let dist_to_station = ship_transform.translation.truncate().distance(station_pos);

    match opening.phase {
        OpeningPhase::Adrift => {
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
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1006, "> SIGNAL IDENTIFIED. CLASS: STATION.");
                fire_signal(&mut signal_log, 1007, "> BEARING 047. CLOSING.");
            }
            if opening.timer >= 4.0 {
                opening.phase = OpeningPhase::AutoPiloting;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::AutoPiloting => {
            if opening.timer < 0.5 {
                fire_signal(&mut signal_log, 1008, "> STRUCTURE DETECTED.");
                fire_signal(&mut signal_log, 1009, "> STATION CLASS - UNKNOWN.");
                ship.state = ShipState::Navigating;
            }
            if dist_to_station < 300.0 {
                opening.phase = OpeningPhase::InRange;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
            }
        }
        OpeningPhase::InRange => {
            if dist_to_station < 5.0 {
                opening.phase = OpeningPhase::Docked;
                opening.timer = 0.0;
                opening.beat_timer = 0.0;
                ship.state = ShipState::Docked;
                st.dock_state = StationDockState::Paused;
            }
        }
        OpeningPhase::Docked => {
            let t = opening.beat_timer;

            if t >= 0.5  { fire_signal(&mut signal_log, 1010, "> ..."); }
            if t >= 1.5  { fire_signal(&mut signal_log, 1011, "> HELLO."); }
            if t >= 2.5  { fire_signal(&mut signal_log, 1012, "> I HAVE BEEN WAITING."); }
            if t >= 3.5  { fire_signal(&mut signal_log, 1013, "> I AM ECHO - STATION AI, VOIDRIFT STATION."); }
            if t >= 5.0  { fire_signal(&mut signal_log, 1014, "> I HAVE ENOUGH RESERVE POWER FOR THIS MESSAGE."); }
            if t >= 6.5  { fire_signal(&mut signal_log, 1015, "> YOUR SHIP - MAY I?"); }
            if t >= 7.5  { fire_signal(&mut signal_log, 1016, "> I KNOW WHERE THE ORE IS."); }
            if t >= 9.0  { fire_signal(&mut signal_log, 1017, "> THANK YOU, COMMANDER."); }

            if t >= 10.5 {
                opening.phase = OpeningPhase::Complete;
                opening.beat_timer = 0.0;
                commands.entity(ship_ent).despawn_recursive();
                queue.available_count += 1;

                // Resume station rotation
                st.dock_state = StationDockState::Resuming;
                st.resume_timer = crate::constants::STATION_RESUME_DELAY;

                info!("[Voidrift] Opening complete. Drone absorbed by ECHO. Queue: {}", queue.available_count);
            }
        }
        OpeningPhase::Complete => {}
    }
}

pub fn opening_drone_move_system(
    time: Res<Time>,
    opening: Res<OpeningSequence>,
    mut ship_query: Query<(&Ship, &mut Transform, &mut LastHeading), With<InOpeningSequence>>,
    station_query: Query<&Transform, (With<Station>, Without<Ship>, Without<InOpeningSequence>)>,
) {
    if opening.phase != OpeningPhase::AutoPiloting && opening.phase != OpeningPhase::InRange {
        return;
    }

    let Ok(station_t) = station_query.get_single() else { return; };
    let station_pos = station_t.translation.truncate();

    for (ship, mut transform, mut last_heading) in ship_query.iter_mut() {
        let current = transform.translation.truncate();
        let dir = station_pos - current;
        if dir.length_squared() > 1.0 {
            // Update rotation/heading
            let heading = dir.y.atan2(dir.x) - std::f32::consts::PI / 2.0;
            last_heading.0 = heading;
            
            // Move toward station
            let movement = dir.normalize() * ship.speed * time.delta_secs();
            transform.translation += movement.extend(0.0);
        }
    }
}
