use bevy::prelude::*;
use std::collections::{VecDeque, HashSet};

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    SpaceView,
    MapView,
}

#[derive(Resource, Clone, Copy, PartialEq, Eq, Default)]
pub enum DeviceType {
    #[default]
    Desktop,
    Mobile,
}

#[derive(Resource, Default)]
pub struct ViewState {
    pub show_production_tree: bool,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}

#[derive(Resource)]
pub struct AsteroidRespawnTimer {
    pub timer: Timer,
}

impl Default for AsteroidRespawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.0, TimerMode::Once),
        }
    }
}

/// Drone pool. Ships are spawned on dispatch and despawned on dock.
/// available_count is the number of ships ready to be assigned.
#[derive(Resource, Default)]
pub struct ShipQueue {
    pub available_count: u32,
}

#[derive(Resource, Default)]
pub struct MaxDispatch(pub u32);

#[derive(Resource, Default)]
pub struct SignalLog {
    pub entries: VecDeque<String>,
    pub fired: HashSet<u32>,
    pub last_fired_at: std::collections::HashMap<u32, f32>, // For refirable IDs
}

#[derive(Resource, Default)]
pub struct SignalStripExpanded(pub bool);

#[derive(PartialEq, Clone, Debug)]
pub enum ObjectiveState {
    Locked,    // not yet revealed — shown as dim placeholder
    Active,    // current objective — highlighted
    Complete,  // done — greyed with checkmark
}

#[derive(Clone, Debug)]
pub struct QuestObjective {
    pub id: u32,
    pub description: String,
    pub progress_current: Option<u32>,  // None if no progress bar
    pub progress_target: Option<u32>,
    pub state: ObjectiveState,
}

#[derive(Resource, Default, Clone, Debug)]
pub struct QuestLog {
    pub objectives: Vec<QuestObjective>,
    pub panel_open: bool,
}

#[derive(Resource)]
pub struct OpeningSequence {
    pub phase: OpeningPhase,
    pub timer: f32,
    pub beat_timer: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum OpeningPhase {
    Adrift,           // Waiting — show S-001
    SignalIdentified, // 2s timer — show S-002
    AutoPiloting,     // Ship moving to station — show S-003
    InRange,          // Station visible — show S-004
    Docked,           // Auto-docked — show S-005, S-006
    Complete,         // Player has control — show S-007, UI unlocks
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct WorldViewRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub canvas_w: f32,
    pub canvas_h: f32,
}

impl Default for WorldViewRect {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, w: 240.0, h: 534.0, canvas_w: 240.0, canvas_h: 534.0 }
    }
}

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
pub enum ForgeQuantity {
    #[default]
    One,
    Ten,
    All,
}

#[derive(Resource, Default)]
pub struct ForgeSettings {
    pub quantity: ForgeQuantity,
}

#[derive(Clone, Debug)]
pub struct ProcessingJob {
    pub operation: ProcessingOperation,
    pub batches: u32,           // number of batches queued (including current)
    pub timer: f32,             // seconds remaining on current batch
    pub completed: u32,         // batches completed so far in this session
    pub clearing: bool,         // if true, finish current batch then None
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ProcessingOperation {
    IronRefinery,
    TungstenRefinery,
    NickelRefinery,
    AluminumRefinery,
    HullForge,
    ThrusterForge,
    CoreFabricator,
    AluminumCanisterForge,
}

#[derive(Resource, Clone)]
pub struct ProductionToggles {
    pub refine_iron: bool,
    pub refine_tungsten: bool,
    pub refine_nickel: bool,
    pub refine_aluminum: bool,
    pub forge_hull: bool,
    pub forge_thruster: bool,
    pub forge_core: bool,
    pub forge_aluminum_canister: bool,
    pub build_drones: bool,
}

impl Default for ProductionToggles {
    fn default() -> Self {
        Self {
            refine_iron: true,
            refine_tungsten: true,
            refine_nickel: true,
            refine_aluminum: true,
            forge_hull: true,
            forge_thruster: true,
            forge_core: true,
            forge_aluminum_canister: true,
            build_drones: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct TutorialState {
    pub shown: HashSet<u32>,            // IDs of pop-ups already shown
    pub active: Option<TutorialPopup>,  // currently visible pop-up
}

#[derive(Clone)]
pub struct TutorialPopup {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub button_label: String,
}

#[derive(Resource)]
pub struct MapPanState {
    pub last_position: Option<Vec2>,
    pub cumulative_offset: Vec2,
    pub is_focused: bool,
}

impl Default for MapPanState {
    fn default() -> Self {
        Self {
            last_position: None,
            cumulative_offset: Vec2::ZERO,
            is_focused: false,
        }
    }
}

/// Tracks content pipeline state — fired one-shot IDs and ambient timer.
#[derive(Resource, Default)]
pub struct ContentState {
    /// IDs of one-shot lines already fired (prevents re-firing).
    pub fired_one_shots: HashSet<String>,
    /// Tracks which trigger conditions have been observed (e.g. "first_bottle_collected").
    pub observed_triggers: HashSet<String>,
    /// Seconds until next ambient line fires. Randomised on each fire.
    pub ambient_timer: f32,
}

#[derive(Resource, Default)]
pub struct CameraDelta(pub Vec2);
