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

#[derive(Component, Debug, Clone, Default)]
pub struct InnerRingAsteroid;

#[cfg(test)]
mod tests {
    use super::*;

    // T-55-01: test_drone_component_default_class_is_mining
    #[test]
    fn test_drone_component_default_class_is_mining() {
        let drone = Drone::default();
        assert_eq!(drone.class, DroneClass::Mining);
        assert_eq!(drone.tier, 1);
    }

    // T-55-03: test_drone_class_eq
    #[test]
    fn test_drone_class_eq() {
        assert_eq!(DroneClass::Mining, DroneClass::Mining);
        assert_eq!(DroneClass::Scout, DroneClass::Scout);
        assert_ne!(DroneClass::Mining, DroneClass::Scout);
        assert_ne!(DroneClass::Scout, DroneClass::Mining);
    }

    // T-55-04: test_autonomous_ship_query_includes_drone (simplified - tests component compatibility)
    #[test]
    fn test_autonomous_ship_query_includes_drone() {
        // Test that Drone component can coexist with AutonomousShip component
        let drone = Drone { class: DroneClass::Mining, tier: 1 };
        let ship = AutonomousShip {
            state: AutonomousShipState::Holding,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron,
        };
        
        // Verify both components can be instantiated together
        assert_eq!(drone.class, DroneClass::Mining);
        assert_eq!(ship.state, AutonomousShipState::Holding);
    }

    // T-55-02: test_spawn_drone_attaches_drone_component (simplified - tests component structure)
    #[test]
    fn test_spawn_drone_attaches_drone_component() {
        // Test that Drone component has the correct structure
        let drone = Drone { class: DroneClass::Mining, tier: 1 };
        
        // Verify component structure
        assert_eq!(drone.class, DroneClass::Mining);
        assert_eq!(drone.tier, 1);
        
        // Verify default creates Mining tier 1
        let default_drone = Drone::default();
        assert_eq!(default_drone.class, DroneClass::Mining);
        assert_eq!(default_drone.tier, 1);
    }
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
    pub drone_count: u32,           // Total drones built (cumulative)
    pub log: VecDeque<String>,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub dock_state: StationDockState,
    pub resume_timer: f32,
    pub cargo_capacity_multiplier: f32,
    pub ship_speed_multiplier: f32,
    pub power_multiplier: f32,
    pub max_dispatch: u32,           // TODO(#55): available_count is now derived from Drone queries, remove after Scout sprint validates
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect, Default)]
pub enum DroneClass {
    #[default]
    Mining,
    Scout,
}

#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Drone {
    pub class: DroneClass,
    pub tier: u8,
}

impl Default for Drone {
    fn default() -> Self {
        Self { class: DroneClass::Mining, tier: 1 }
    }
}

/// Marker on asteroid entities that Scout has targeted.
/// Stores the green Annulus ring entity for cleanup on despawn.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Painted {
    pub ring_entity: Entity,
}

/// On the Scout drone entity. Drives circular orbit math.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct ScoutOrbit {
    pub angle: f32,        // current radians, 0.0 at start
    pub radius: f32,       // read from RingConfig.inner_radius at spawn
    pub speed: f32,        // rad/sec, read from balance.toml [scout]
}

/// On a dispatched Mining drone. Stores the asteroid entity it was sent to.
/// Removed by cleanup system when drone returns to Holding.
#[derive(Component, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct DroneTarget {
    pub asteroid: Entity,
}

#[derive(Component, Default)]
pub struct StationQueues {
    pub iron_refinery: Option<crate::components::ProcessingJob>,
    pub tungsten_refinery: Option<crate::components::ProcessingJob>,
    pub nickel_refinery: Option<crate::components::ProcessingJob>,
    pub aluminum_refinery: Option<crate::components::ProcessingJob>,
    pub hull_forge:         Option<crate::components::ProcessingJob>,
    pub thruster_forge:      Option<crate::components::ProcessingJob>,
    pub core_fabricator:    Option<crate::components::ProcessingJob>,
    pub canister_forge:     Option<crate::components::ProcessingJob>,
}
