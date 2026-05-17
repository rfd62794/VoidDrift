use bevy::prelude::*;
use crate::components::*;
use crate::systems::game_loop::autonomous::fleet_count_system;
use super::minimal_app;

fn fleet_app() -> App {
    let mut app = minimal_app();
    app.add_systems(Update, fleet_count_system);
    app
}

#[test]
fn fleet_count_reflects_single_idle_drone() {
    let mut app = fleet_app();
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
    ));
    app.update();
    let fleet = app.world().resource::<FleetCount>();
    assert_eq!(fleet.total, 1);
    assert_eq!(fleet.available, 1);
    assert_eq!(fleet.deployed, 0);
}

#[test]
fn fleet_count_reflects_dispatched_drone() {
    let mut app = fleet_app();
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Outbound, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        AutonomousAssignment {
            target_pos: Vec2::new(200.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let fleet = app.world().resource::<FleetCount>();
    assert_eq!(fleet.total, 1);
    assert_eq!(fleet.available, 0);
    assert_eq!(fleet.deployed, 1);
}

#[test]
fn fleet_count_reflects_mixed_fleet() {
    let mut app = fleet_app();
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
    ));
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Outbound, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        AutonomousAssignment {
            target_pos: Vec2::new(200.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let fleet = app.world().resource::<FleetCount>();
    assert_eq!(fleet.total, 2);
    assert_eq!(fleet.available, 1);
    assert_eq!(fleet.deployed, 1);
}

#[test]
fn fleet_count_zero_with_no_drones() {
    let mut app = fleet_app();
    app.update();
    let fleet = app.world().resource::<FleetCount>();
    assert_eq!(fleet.total, 0);
    assert_eq!(fleet.available, 0);
    assert_eq!(fleet.deployed, 0);
}
