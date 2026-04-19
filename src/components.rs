use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    SpaceView,
    MapView,
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
}

#[derive(Component)]
pub struct AutopilotTarget {
    pub destination: Vec2,
    pub target_entity: Option<Entity>,
}

#[derive(Component)]
pub struct AsteroidField {
    pub ore_type: OreType,
    pub depleted: bool,
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
    pub log: std::collections::VecDeque<String>,
}

#[derive(Component)]
pub struct MapMarker;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CargoBarFill;

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
