use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

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
            if opening.timer >= 0.5 {
                opening.phase = OpeningPhase::SignalIdentified;
                opening.timer = 0.0;
                fire_signal(&mut signal_log, 100, "ECHO: WEAK SIGNAL DETECTED. ANALYZING...");
            }
        }
        OpeningPhase::SignalIdentified => {
            if opening.timer >= SIGNAL_PAUSE_S2 {
                opening.phase = OpeningPhase::AutoPiloting;
                opening.timer = 0.0;
                fire_signal(&mut signal_log, 101, "ECHO: SIGNAL IDENTIFIED AS DERELICT STATION. NAVIGATING.");
                
                ship.state = ShipState::Navigating;
                commands.entity(ship_ent).remove::<DockedAt>();
                commands.entity(ship_ent).insert(AutopilotTarget {
                    destination: berth_pos,
                    target_entity: Some(berth_ent),
                });
            }
        }
        OpeningPhase::AutoPiloting => {
            if dist_to_station < 300.0 {
                opening.phase = OpeningPhase::InRange;
                opening.timer = 0.0;
                fire_signal(&mut signal_log, 102, "ECHO: STATION IN RANGE. PREPARING DOCKING SEQUENCE.");
            }
        }
        OpeningPhase::InRange => {
            if ship.state == ShipState::Docked {
                opening.phase = OpeningPhase::Docked;
                opening.timer = 0.0;
                fire_signal(&mut signal_log, 103, "ECHO: DOCKING COMPLETE. SYSTEMS COMING ONLINE.");
            }
        }
        OpeningPhase::Docked => {
            if opening.timer >= SIGNAL_PAUSE_COMPLETE {
                opening.phase = OpeningPhase::Complete;
                opening.timer = 0.0;
                fire_signal(&mut signal_log, 104, "ECHO: ALL SYSTEMS NOMINAL. WELCOME TO VOIDRIFT STATION.");
            }
        }
        OpeningPhase::Complete => {}
    }
}
