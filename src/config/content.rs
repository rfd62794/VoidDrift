use serde::Deserialize;

fn read_yaml(filename: &'static str) -> &'static str {
    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    {
        // include_str! requires literal, so we match on filename
        match filename {
            "echo.yaml" => include_str!("../../assets/content/echo.yaml"),
            "tutorial.yaml" => include_str!("../../assets/content/tutorial.yaml"),
            "objectives.yaml" => include_str!("../../assets/content/objectives.yaml"),
            "requests.yaml" => include_str!("../../assets/content/requests.yaml"),
            "logs.yaml" => include_str!("../../assets/content/logs.yaml"),
            _ => panic!("Unknown config file: {}", filename),
        }
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    {
        Box::leak(
            std::fs::read_to_string(format!("assets/content/{}", filename))
                .expect(&format!("Failed to read assets/content/{}", filename))
                .into_boxed_str(),
        )
    }
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct ContentConfig {
    pub one_shots: Vec<OneShotLine>,
    pub ambient: Vec<AmbientLine>,
    pub event_pools: Vec<EventPool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct OneShotLine {
    pub id: String,
    pub trigger: String,
    pub text: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AmbientLine {
    pub id: String,
    pub text: String,
    pub weight: u32,
    pub eligible_after: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EventPool {
    pub trigger: String,
    pub chance: f32,
    pub lines: Vec<String>,
}

impl ContentConfig {
    pub fn load() -> Self {
        let src = read_yaml("echo.yaml");
        serde_yaml::from_str(src).expect("Failed to parse assets/content/echo.yaml")
    }
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct TutorialConfig {
    pub steps: Vec<TutorialStep>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TutorialStep {
    pub id: u32,
    pub trigger: String,
    pub highlight: String,
    pub requires: Vec<u32>,
    pub popup: TutorialPopup,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TutorialPopup {
    pub title: String,
    pub body: String,
    pub button: String,
}

impl TutorialConfig {
    pub fn load() -> Self {
        let src = read_yaml("tutorial.yaml");
        serde_yaml::from_str(src).expect("Failed to parse assets/content/tutorial.yaml")
    }
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct QuestConfig {
    pub quest_objectives: Vec<QuestObjectiveDef>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct QuestObjectiveDef {
    pub id: u32,
    pub description: String,
    pub progress_target: Option<u32>,
    pub initial_state: String,
    pub triggers: ObjectiveTriggers,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ObjectiveTriggers {
    pub activate_on_signal: Option<u32>,
    pub complete_on_signal: Option<u32>,
}

impl QuestConfig {
    pub fn load() -> Self {
        let src = read_yaml("objectives.yaml");
        serde_yaml::from_str(src).expect("Failed to parse assets/content/objectives.yaml")
    }
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct RequestConfig {
    pub faction_requests: Vec<RequestDef>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RequestDef {
    pub id: String,
    pub title: String,
    pub flavor: String,
    pub faction: String,
    pub repeatable: bool,
    pub repeat_cooldown_seconds: Option<f32>,
    pub reputation_reward: u32,
    pub min_reputation: u32,
    pub requirements: Vec<ResourceRequirement>,
    pub rewards: Vec<RewardDef>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ResourceRequirement {
    pub resource: String,
    pub amount: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RewardDef {
    pub r#type: String,
    pub value: f32,
}

impl RequestConfig {
    pub fn load() -> Self {
        let src = read_yaml("requests.yaml");
        serde_yaml::from_str(src).expect("Failed to parse assets/content/requests.yaml")
    }
}

#[derive(Deserialize, Clone, Debug, bevy::prelude::Resource)]
pub struct LogsConfig {
    pub logs: Vec<LogEntry>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LogEntry {
    pub id: String,
    pub unlock_trigger: String,
    pub title: String,
    pub body: String,
}

impl LogsConfig {
    pub fn load() -> Self {
        let src = read_yaml("logs.yaml");
        serde_yaml::from_str(src).expect("Failed to parse assets/content/logs.yaml")
    }
}
