use bevy::prelude::*;
use crate::components::*;

/// Updates real-time quest progress bars.
/// Follows the Universal Disjointness pattern for mobile stability.
pub fn quest_update_system(
    mut quest_log: ResMut<QuestLog>,
    // Universal Disjointness filters applied to all Transform-accessing (or potentially panicking) queries
    station_query: Query<&Station, (With<Station>, Without<InOpeningSequence>, Without<AutonomousShip>, Without<MainCamera>, Without<StarLayer>, Without<StationVisualsContainer>, Without<DestinationHighlight>, Without<ShipCargoBarFill>, Without<AsteroidField>, Without<Berth>)>,
) {
    let Ok(station) = station_query.get_single() else { return };

    for obj in quest_log.objectives.iter_mut() {
        if obj.id == 3 && obj.state == ObjectiveState::Active {
            // Repair the station (Objective 3) tracks iron_reserves
            obj.progress_current = Some(station.iron_reserves as u32);
        }
    }
}
