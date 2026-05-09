use serde::Deserialize;

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
        let src = Self::read_yaml();
        serde_yaml::from_str(src).expect("Failed to parse assets/content/echo.yaml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_yaml() -> &'static str {
        include_str!("../../assets/content/echo.yaml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_yaml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/content/echo.yaml")
                .expect("Failed to read assets/content/echo.yaml")
                .into_boxed_str(),
        )
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
        let src = Self::read_yaml();
        serde_yaml::from_str(src).expect("Failed to parse assets/content/tutorial.yaml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_yaml() -> &'static str {
        include_str!("../../assets/content/tutorial.yaml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_yaml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/content/tutorial.yaml")
                .expect("Failed to read assets/content/tutorial.yaml")
                .into_boxed_str(),
        )
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
        let src = Self::read_yaml();
        serde_yaml::from_str(src).expect("Failed to parse assets/content/objectives.yaml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_yaml() -> &'static str {
        include_str!("../../assets/content/objectives.yaml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_yaml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/content/objectives.yaml")
                .expect("Failed to read assets/content/objectives.yaml")
                .into_boxed_str(),
        )
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
        let src = Self::read_yaml();
        serde_yaml::from_str(src).expect("Failed to parse assets/content/requests.yaml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_yaml() -> &'static str {
        include_str!("../../assets/content/requests.yaml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_yaml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/content/requests.yaml")
                .expect("Failed to read assets/content/requests.yaml")
                .into_boxed_str(),
        )
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
        let src = Self::read_yaml();
        serde_yaml::from_str(src).expect("Failed to parse assets/content/logs.yaml")
    }

    #[cfg(any(target_arch = "wasm32", target_os = "android"))]
    fn read_yaml() -> &'static str {
        include_str!("../../assets/content/logs.yaml")
    }

    #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
    fn read_yaml() -> &'static str {
        Box::leak(
            std::fs::read_to_string("assets/content/logs.yaml")
                .expect("Failed to read assets/content/logs.yaml")
                .into_boxed_str(),
        )
    }
}
