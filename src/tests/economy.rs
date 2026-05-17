use bevy::prelude::*;
use crate::components::*;
use crate::config::VisualConfig;
use crate::systems::game_loop::economy::ship_docked_economy_system;
use crate::systems::persistence::save::AutosaveEvent;
use super::test_station;

fn economy_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.insert_resource(VisualConfig::load());
    app.init_resource::<RequestsTabState>();
    app.init_resource::<FleetCount>();
    app.add_event::<ShipDockedWithCargo>();
    app.add_event::<ShipDockedWithBottle>();
    app.add_event::<FulfillRequestEvent>();
    app.add_event::<RepairStationEvent>();
    app.add_event::<AutosaveEvent>();
    app.add_systems(Update, ship_docked_economy_system);
    app
}

#[test]
fn ship_docked_with_cargo_increments_iron_reserves() {
    let mut app = economy_app();
    let station = app.world_mut().spawn((test_station(), Transform::default())).id();
    // Send cargo event before update
    app.world_mut().resource_mut::<Events<ShipDockedWithCargo>>().send(ShipDockedWithCargo {
        ship_entity: station, // entity field — not actually used for cargo path
        ore_type: OreDeposit::Iron,
        amount: 10.0,
        despawn: false,
    });
    app.update();
    let mut q = app.world_mut().query::<&Station>();
    let s = q.single(app.world());
    assert_eq!(s.iron_reserves, 10.0, "iron_reserves should increase by event amount");
}

#[test]
fn ship_docked_with_cargo_does_not_despawn_drone() {
    let mut app = economy_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    let drone = app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
    )).id();
    app.world_mut().resource_mut::<Events<ShipDockedWithCargo>>().send(ShipDockedWithCargo {
        ship_entity: drone,
        ore_type: OreDeposit::Iron,
        amount: 50.0,
        despawn: false,
    });
    app.update();
    assert!(
        app.world().get_entity(drone).is_ok(),
        "Drone must not be despawned when despawn: false"
    );
}

#[test]
fn ship_docked_with_cargo_despawn_false_does_not_touch_fleet_count() {
    let mut app = economy_app();
    app.world_mut().spawn((test_station(), Transform::default()));
    let dummy = app.world_mut().spawn(Transform::default()).id();
    app.world_mut().resource_mut::<Events<ShipDockedWithCargo>>().send(ShipDockedWithCargo {
        ship_entity: dummy,
        ore_type: OreDeposit::Iron,
        amount: 10.0,
        despawn: false,
    });
    app.update();
    let fleet = app.world().resource::<FleetCount>();
    assert_eq!(fleet.total, 0, "ship_docked_economy_system must not modify FleetCount");
    assert_eq!(fleet.available, 0);
    assert_eq!(fleet.deployed, 0);
}
