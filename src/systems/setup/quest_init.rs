use bevy::prelude::*;
use crate::components::*;

pub fn init_quest_log(commands: &mut Commands) {
    commands.insert_resource(QuestLog {
        panel_open: false,
        objectives: vec![
            QuestObjective {
                id: 1,
                description: "Locate the signal source".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Active,
            },
            QuestObjective {
                id: 2,
                description: "Dock at the derelict station".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 3,
                description: "Repair the station".to_string(),
                progress_current: Some(0),
                progress_target: Some(25),
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 4,
                description: "Build an AI Core".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 5,
                description: "Discover Sector 3".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Active, // Start active for expansion
            },
            QuestObjective {
                id: 6,
                description: "Mine Carbon 3".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 7,
                description: "Assemble autonomous ship".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
        ],
    });
}
