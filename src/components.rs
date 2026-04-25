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
    pub cargo_type: OreDeposit,
    pub cargo_capacity: u32,
    pub laser_tier: LaserTier,
}

#[derive(Component)]
pub struct AutopilotTarget {
    pub destination: Vec2,
    pub target_entity: Option<Entity>,
}

#[derive(Component)]
pub struct AsteroidField {
    pub ore_deposit: OreDeposit,
    pub depleted: bool,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum OreDeposit {
    Iron,
    Tungsten,
    Nickel,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LaserTier {
    Basic,
    Tungsten,
    Composite,
}

pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Iron     => "IRON",
        OreDeposit::Tungsten => "TUNGSTEN",
        OreDeposit::Nickel   => "NICKEL",
    }
}

pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Iron     => LaserTier::Basic,
        OreDeposit::Tungsten => LaserTier::Basic,
        OreDeposit::Nickel   => LaserTier::Basic,
    }
}

#[derive(Component)]
pub struct Station {
    pub repair_progress: f32,
    pub online: bool,
    pub iron_reserves: f32,
    pub iron_ingots: f32,
    pub tungsten_reserves: f32,
    pub tungsten_ingots: f32,
    pub nickel_reserves: f32,
    pub nickel_ingots: f32,
    pub hull_plate_reserves: f32,
    pub thruster_reserves: f32,
    pub ship_hulls: f32,
    pub ai_cores: f32,
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

#[derive(Component)]
pub struct MenuCamera;

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
    pub cargo_type: OreDeposit,
}

#[derive(Component)]
pub struct DockedAt(pub Entity);

#[derive(Component)]
pub struct AutonomousAssignment {
    pub target_pos: Vec2,
    pub ore_type: OreDeposit,
    pub sector_name: String,
}

#[derive(Component)]
pub struct ShipCargoBarFill;

#[derive(Component)]
pub struct StarLayer(pub f32);

#[derive(Component)]
pub struct LastHeading(pub f32);

#[derive(Component)]
pub struct InOpeningSequence;

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

#[derive(Resource, PartialEq, Debug, Clone, Copy, Default)]
pub enum DrawerState {
    #[default]
    Collapsed,
    Expanded,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct UiLayout {
    pub screen_width: f32,
    pub screen_height: f32,
    pub handle_height: f32,
    pub signal_height: f32,
    pub primary_tab_height: f32,
    pub secondary_tab_height: f32,
    pub content_height: f32,
}

impl Default for UiLayout {
    fn default() -> Self {
        Self {
            screen_width: 720.0,
            screen_height: 1604.0,
            handle_height: 32.0,
            signal_height: 64.0,
            primary_tab_height: 48.0,
            secondary_tab_height: 48.0,
            content_height: 358.0,
        }
    }
}

#[derive(Resource, Default, PartialEq, Debug, Clone, Copy)]
pub enum ActiveStationTab {
    Station,
    Fleet,
    #[default]
    Cargo,
    Iron,
    Tungsten,
    Nickel,
    Upgrades,
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
    HullForge,
    CoreFabricator,
}

#[derive(Component, Default)]
pub struct StationQueues {
    pub iron_refinery: Option<ProcessingJob>,
    pub tungsten_refinery: Option<ProcessingJob>,
    pub nickel_refinery: Option<ProcessingJob>,
    pub hull_forge:         Option<ProcessingJob>,
    pub core_fabricator:    Option<ProcessingJob>,
}

#[derive(Resource, Clone)]
pub struct ProductionToggles {
    pub refine_iron: bool,
    pub refine_tungsten: bool,
    pub refine_nickel: bool,
    pub forge_hull: bool,
    pub forge_thruster: bool,
    pub forge_core: bool,
}

impl Default for ProductionToggles {
    fn default() -> Self {
        Self {
            refine_iron: true,
            refine_tungsten: true,
            refine_nickel: true,
            forge_hull: true,
            forge_thruster: true,
            forge_core: true,
        }
    }
}

#[derive(Resource, Default)]
pub struct ShipQueue {
    pub available_ships: Vec<Entity>,
    pub active_ships: Vec<Entity>,
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
