use bevy::prelude::*;
use std::collections::VecDeque;

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
    pub cargo_type: crate::components::OreDeposit,
    pub cargo_capacity: u32,
    pub laser_tier: crate::components::LaserTier,
    pub current_mining_target: Option<Entity>,
}

#[derive(Component)]
pub struct ActiveAsteroid {
    pub ore_type: crate::components::OreDeposit,
    pub ore_remaining: f32,
    pub lifespan_timer: f32,
}

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum OreDeposit {
    Iron,
    Tungsten,
    Nickel,
    Aluminum,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LaserTier {
    Basic,
    Tungsten,
    Composite,
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
    pub aluminum_reserves: f32,
    pub aluminum_ingots: f32,
    pub aluminum_canisters: f32,
    pub hull_plate_reserves: f32,
    pub thruster_reserves: f32,
    pub ai_cores: f32,
    pub drone_build_progress: f32,  // Fractional accumulator [0.0, 1.0)
    pub log: VecDeque<String>,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub dock_state: StationDockState,
    pub resume_timer: f32,
    pub cargo_capacity_multiplier: f32,
    pub ship_speed_multiplier: f32,
    pub power_multiplier: f32,
    pub max_drones: u32,
    pub max_active_asteroids: u32,
}

#[derive(PartialEq, Debug, Default, Copy, Clone)]
pub enum StationDockState {
    #[default]
    Rotating,      // Normal rotation at STATION_ROTATION_SPEED
    Slowing,       // Incoming ship detected — decelerating
    Paused,        // Ship arrived — fully stopped
    Resuming,      // Ship docked — accelerating back to full speed
}

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
pub struct AutonomousAssignment {
    pub target_pos: Vec2,
    pub ore_type: OreDeposit,
    pub sector_name: String,
}

#[derive(Component, Default)]
pub struct StationQueues {
    pub iron_refinery: Option<crate::components::ProcessingJob>,
    pub tungsten_refinery: Option<crate::components::ProcessingJob>,
    pub nickel_refinery: Option<crate::components::ProcessingJob>,
    pub aluminum_refinery: Option<crate::components::ProcessingJob>,
    pub hull_forge:         Option<crate::components::ProcessingJob>,
    pub core_fabricator:    Option<crate::components::ProcessingJob>,
    pub canister_forge:     Option<crate::components::ProcessingJob>,
}
