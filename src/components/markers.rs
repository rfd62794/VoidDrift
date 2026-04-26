use bevy::prelude::*;

#[derive(Component)]
pub struct MapMarker;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct MenuCamera;

#[derive(Component)]
pub struct DockedAt(pub Entity);

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

#[derive(Component)]
pub struct AutopilotTarget {
    pub destination: Vec2,
    pub target_entity: Option<Entity>,
}

#[derive(Component)] pub struct CargoOreLabel;
#[derive(Component)] pub struct CargoCountLabel;
