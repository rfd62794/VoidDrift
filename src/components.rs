use bevy::prelude::*;
use std::collections::{VecDeque, HashSet};

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    SpaceView,
    MapView,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
}


#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum OreType {
    #[default]
    Empty,
    Magnetite,
    Carbon,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ShipState {
    Idle,
    Navigating,
    Mining,
    Docked,
}

#[derive(Component)]
pub struct Ship {
    pub state: ShipState,
    pub speed: f32,
    pub cargo: f32,
    pub cargo_type: OreType,
    pub cargo_capacity: u32,
    pub power: f32,
    pub power_cells: u32,
    pub laser_tier: LaserTier,
}

#[derive(Component)]
pub struct AutopilotTarget {
    pub destination: Vec2,
    pub target_entity: Option<Entity>,
}

#[derive(Component)]
pub struct AsteroidField {
    pub ore_type: OreType,
    pub ore_deposit: OreDeposit,
    pub depleted: bool,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum OreDeposit {
    Magnetite,
    Iron,
    Carbon,
    Tungsten,
    Titanite,
    CrystalCore,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LaserTier {
    Basic,
    Tungsten,
    Composite,
}

pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Magnetite  => "MAGNETITE",
        OreDeposit::Iron       => "IRON",
        OreDeposit::Carbon     => "CARBON",
        OreDeposit::Tungsten   => "TUNGSTEN",
        OreDeposit::Titanite   => "TITANITE",
        OreDeposit::CrystalCore => "CRYSTAL",
    }
}

pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Magnetite  => LaserTier::Basic,
        OreDeposit::Iron       => LaserTier::Basic,
        OreDeposit::Carbon     => LaserTier::Basic,
        OreDeposit::Tungsten   => LaserTier::Tungsten,
        OreDeposit::Titanite   => LaserTier::Tungsten,
        OreDeposit::CrystalCore => LaserTier::Composite,
    }
}

#[derive(Component)]
pub struct Station {
    pub repair_progress: f32,
    pub online: bool,
    pub magnetite_reserves: f32,
    pub carbon_reserves: f32,
    pub hull_plate_reserves: u32,
    pub ship_hulls: u32,
    pub ai_cores: u32,
    pub power_cells: u32,
    pub power: f32,
    pub maintenance_timer: Timer,
    pub last_power_warning_time: f32,
    pub log: VecDeque<String>,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub dock_state: StationDockState,
    pub resume_timer: f32,
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum StationDockState {
    #[default]
    Rotating,      // Normal rotation at STATION_ROTATION_SPEED
    Slowing,       // Incoming ship detected — decelerating
    Paused,        // Ship arrived — fully stopped
    Resuming,      // Ship docked — accelerating back to full speed
}

#[derive(Component)]
pub struct MapMarker;

#[derive(Component)]
pub struct MainCamera;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AutonomousShipState {
    Holding,
    Outbound,
    Mining,
    Returning,
    Unloading,
}

#[derive(Component)]
pub struct AutonomousShip {
    pub state: AutonomousShipState,
    pub cargo: f32,
    pub cargo_type: OreType,
    pub power: f32,
}

#[derive(Component)]
pub struct DockedAt(pub Entity);

#[derive(Component)]
pub struct AutonomousAssignment {
    pub target_pos: Vec2,
    pub ore_type: OreType,
    pub sector_name: String,
}

#[derive(Component)]
pub struct ShipCargoBarFill;

#[derive(Component)]
pub struct StarLayer(pub f32);

#[derive(Component)]
pub struct LastHeading(pub f32);

#[derive(Component)]
pub struct PlayerShip;

#[derive(Component)]
pub struct ThrusterGlow;

#[derive(Component)]
pub struct MiningBeam;

#[derive(Component)]
pub struct AutonomousShipTag;

#[derive(Component)]
pub struct StationVisualsContainer;

#[derive(Component)]
pub struct StationHub;

#[derive(Component)]
pub struct Berth {
    pub arm_index: u8,
    pub occupied_by: Option<Entity>,
    pub berth_type: BerthType,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum BerthType {
    Player,   // Berth 1 — always player
    Drone,    // Berth 2 — autonomous ship
    Open,     // Berth 3 — NPC/visitor
}

#[derive(Component)]
pub struct BerthVisual(pub u8); // arm_index

#[derive(Component)]
pub struct MapElement; // Marker for visibility toggling

#[derive(Component)]
pub struct MapIcon;

#[derive(Component)]
pub struct MapLabel;

#[derive(Component)]
pub struct MapConnector;

#[derive(Component)]
pub struct DestinationHighlight;

#[derive(Resource, Default)]
pub struct CameraDelta(pub Vec2);

// ── NARRATIVE & UI OVERHAUL SYSTEMS ──────────────────────────────────────────

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

// ── UTILITIES ────────────────────────────────────────────────────────────────

pub fn berth_world_pos(
    station_pos: Vec2,
    station_rotation: f32,
    arm_index: u8,
) -> Vec2 {
    let arm_angle = station_rotation + (arm_index as f32 * std::f32::consts::TAU / 6.0);
    station_pos + Vec2::new(
        arm_angle.cos() * crate::constants::STATION_ARM_LENGTH,
        arm_angle.sin() * crate::constants::STATION_ARM_LENGTH,
    )
}

#[derive(Resource)]
pub struct OpeningSequence {
    pub phase: OpeningPhase,
    pub timer: f32,
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

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
pub enum ActiveStationTab {
    #[default]
    Reserves,
    Power,
    Smelter,
    Forge,
    ShipPort,
    Market, // Added back for future parity
    Fleet,  // Added back for future parity
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
    MagnetiteRefinery,   // Magnetite → Power Cells
    CarbonRefinery,      // Carbon → Hull Plates
    HullForge,           // Hull Plates → Ship Hull
    CoreFabricator,      // Power Cells → AI Core
}

#[derive(Component, Default)]
pub struct StationQueues {
    pub magnetite_refinery: Option<ProcessingJob>,
    pub carbon_refinery:    Option<ProcessingJob>,
    pub hull_forge:         Option<ProcessingJob>,
    pub core_fabricator:    Option<ProcessingJob>,
}

#[derive(Resource)]
pub struct AutoDockSettings {
    pub auto_unload: bool,           // default: true
    pub auto_smelt_magnetite: bool,  // default: false
    pub auto_smelt_carbon: bool,     // default: false
}

impl Default for AutoDockSettings {
    fn default() -> Self {
        Self {
            auto_unload: true,
            auto_smelt_magnetite: false,
            auto_smelt_carbon: false,
        }
    }
}

// ── TUTORIAL & UX ────────────────────────────────────────────────────────────

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

#[derive(Component)] pub struct CargoOreLabel;
#[derive(Component)] pub struct CargoCountLabel;

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
            is_focused: true,
        }
    }
}
