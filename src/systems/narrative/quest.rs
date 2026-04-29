use bevy::prelude::*;
use crate::components::*;

/// Reacts to SignalFired events and updates quest objective states.
/// Owns all objective state transitions — signal.rs only fires signals.
pub fn quest_signal_system(
    mut signal_events: EventReader<SignalFired>,
    mut quest_log: ResMut<QuestLog>,
) {
    for event in signal_events.read() {
        for obj in quest_log.objectives.iter_mut() {
            match obj.id {
                1 => { // Locate signal source
                    if event.signal_id == 4 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                2 => { // Dock at derelict station
                    if event.signal_id == 4 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 5 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                3 => { // Repair station
                    if event.signal_id == 5 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 11 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                4 => { // Build AI Core
                    if event.signal_id == 11 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 13 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                5 => { // Discover Sector 7
                    if event.signal_id == 13 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 14 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                6 => { // Mine Carbon
                    if event.signal_id == 14 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 16 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                7 => { // Assemble autonomous ship
                    if event.signal_id == 16 && obj.state == ObjectiveState::Locked {
                        obj.state = ObjectiveState::Active;
                    }
                    if event.signal_id == 17 {
                        obj.state = ObjectiveState::Complete;
                    }
                }
                _ => {}
            }
        }
    }
}

/// Updates real-time quest progress bars (numeric current/target values).
pub fn quest_update_system(
    mut quest_log: ResMut<QuestLog>,
    station_query: Query<&Station, (With<Station>, Without<InOpeningSequence>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<ActiveAsteroid>, Without<Berth>)>,
) {
    let Ok(station) = station_query.get_single() else { return };

    for obj in quest_log.objectives.iter_mut() {
        if obj.id == 3 && obj.state == ObjectiveState::Active {
            // Repair the station (Objective 3) tracks iron_reserves
            obj.progress_current = Some(station.iron_reserves as u32);
        }
    }
}
