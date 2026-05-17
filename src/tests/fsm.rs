use bevy::prelude::*;
use crate::components::*;
use crate::config::{BalanceConfig, VisualConfig};
use crate::drone::fsm::autonomous_ship_system;
use super::test_station;

fn fsm_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(BalanceConfig::load());
    app.insert_resource(VisualConfig::load());
    app.add_event::<ShipDockedWithCargo>();
    app.add_systems(Update, autonomous_ship_system);
    app
}

#[test]
fn drone_in_holding_without_assignment_stays_holding() {
    let mut app = fsm_app();
    // No AutonomousAssignment → drone is not in ship_query → system skips it
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
    ));
    app.update();
    let mut q = app.world_mut().query::<&AutonomousShip>();
    let ship = q.single(app.world());
    assert_eq!(ship.state, AutonomousShipState::Holding);
}

#[test]
fn drone_in_holding_with_assignment_transitions_to_outbound() {
    let mut app = fsm_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
        AutonomousAssignment {
            target_pos: Vec2::new(200.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let mut q = app.world_mut().query::<&AutonomousShip>();
    let ship = q.single(app.world());
    assert_eq!(ship.state, AutonomousShipState::Outbound);
}

#[test]
fn drone_in_outbound_without_reaching_target_stays_outbound() {
    let mut app = fsm_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    // Drone at origin, target 500 units away — arrival_threshold_mining = 40.0
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Outbound, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::from_translation(Vec3::ZERO),
        AutonomousAssignment {
            target_pos: Vec2::new(500.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let mut q = app.world_mut().query::<&AutonomousShip>();
    let ship = q.single(app.world());
    assert_eq!(ship.state, AutonomousShipState::Outbound);
}

#[test]
fn drone_cargo_full_transitions_returning() {
    let mut app = fsm_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    let cfg = BalanceConfig::load();
    let capacity = cfg.mining.cargo_capacity as f32;
    // Cargo already at capacity — triggers Returning regardless of time delta
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Mining, cargo: capacity, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
        AutonomousAssignment {
            target_pos: Vec2::new(300.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let mut q = app.world_mut().query::<&AutonomousShip>();
    let ship = q.single(app.world());
    assert_eq!(ship.state, AutonomousShipState::Returning);
}

#[test]
fn drone_unloading_transitions_to_holding_and_zeroes_cargo() {
    let mut app = fsm_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Unloading, cargo: 50.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
        AutonomousAssignment {
            target_pos: Vec2::new(300.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let mut q = app.world_mut().query::<&AutonomousShip>();
    let ship = q.single(app.world());
    assert_eq!(ship.state, AutonomousShipState::Holding);
    assert_eq!(ship.cargo, 0.0);
}

#[test]
fn drone_unloading_fires_ship_docked_with_cargo_event() {
    let mut app = fsm_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Unloading, cargo: 50.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
        AutonomousAssignment {
            target_pos: Vec2::new(300.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    let events = app.world().resource::<Events<ShipDockedWithCargo>>();
    assert_eq!(events.len(), 1, "Expected exactly one ShipDockedWithCargo event");
    let mut cursor = events.get_cursor();
    let fired: Vec<_> = cursor.read(events).collect();
    assert!(!fired[0].despawn, "Autonomous drone unload must have despawn: false");
}
