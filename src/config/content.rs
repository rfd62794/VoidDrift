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
