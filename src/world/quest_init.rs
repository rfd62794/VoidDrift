use bevy::prelude::*;
use crate::components::*;
use crate::config::QuestConfig;

pub fn init_quest_log(commands: &mut Commands, quest_cfg: &QuestConfig) {
    let objectives: Vec<QuestObjective> = quest_cfg.quest_objectives.iter().map(|def| {
        let initial_state = match def.initial_state.as_str() {
            "Active" => ObjectiveState::Active,
            "Locked" => ObjectiveState::Locked,
            "Complete" => ObjectiveState::Complete,
            _ => ObjectiveState::Locked,
        };

        QuestObjective {
            id: def.id,
            description: def.description.clone(),
            progress_current: if def.progress_target.is_some() { Some(0) } else { None },
            progress_target: def.progress_target,
            state: initial_state,
        }
    }).collect();

    commands.insert_resource(QuestLog {
        panel_open: false,
        objectives,
    });
}
