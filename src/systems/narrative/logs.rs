use bevy::prelude::*;
use crate::config::content::LogsConfig;
use crate::systems::persistence::save::SaveData;
use crate::components::ViewState;

pub fn check_log_unlocks(
    logs_config: Res<LogsConfig>,
    mut save_data: ResMut<SaveData>,
    view_state: Res<ViewState>,
    station_query: Query<(&crate::game_state::Station, &crate::game_state::StationQueues)>,
) {
    let (station, queues) = match station_query.get_single() {
        Ok(s) => s,
        Err(_) => return,
    };

    for log in &logs_config.logs {
        if save_data.unlocked_logs.contains(&log.id) {
            continue;
        }

        let unlocked = match log.unlock_trigger.as_str() {
            "game_start" => true,
            "first_drone_built" => save_data.drone_count >= 1,
            "drone_count_3" => save_data.drone_count >= 3,
            "drone_count_5" => save_data.drone_count >= 5,
            "drone_count_10" => save_data.drone_count >= 10,
            "first_ingot_produced" => station.iron_ingots > 0.0
                || station.tungsten_ingots > 0.0
                || station.nickel_ingots > 0.0
                || station.aluminum_ingots > 0.0,
            "pipeline_opened" => view_state.production_tree_ever_opened,
            "first_component_built" => station.hull_plate_reserves > 0.0
                || station.thruster_reserves > 0.0
                || station.ai_cores > 0.0
                || station.aluminum_canisters > 0.0,
            "outer_ring_unlocked" => {
                // This trigger is not yet implemented in the game
                // For now, return false
                false
            }
            "canister_built" => station.aluminum_canisters > 0.0,
            _ => false,
        };

        if unlocked {
            save_data.unlocked_logs.push(log.id.clone());
        }
    }
}
