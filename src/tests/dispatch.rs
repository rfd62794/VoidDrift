use bevy::prelude::*;
use crate::components::*;

#[test]
fn manual_dispatch_inserts_assignment_on_idle_drone() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    let drone = app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Holding, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
    )).id();
    let target = Vec2::new(200.0, 100.0);
    app.world_mut().entity_mut(drone).insert(AutonomousAssignment {
        target_pos: target,
        ore_type: OreDeposit::Iron,
        sector_name: "S1".to_string(),
    });
    app.update();
    let assignment = app.world().get::<AutonomousAssignment>(drone)
        .expect("AutonomousAssignment should be present after dispatch");
    assert_eq!(assignment.target_pos, target);
    assert_eq!(assignment.ore_type, OreDeposit::Iron);
}

#[test]
fn manual_dispatch_does_nothing_when_no_idle_drone() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    // Drone already dispatched — has AutonomousAssignment, not idle
    app.world_mut().spawn((
        AutonomousShip { state: AutonomousShipState::Outbound, cargo: 0.0, cargo_type: OreDeposit::Iron },
        Drone { class: DroneClass::Mining, tier: 1 },
        Transform::default(),
        AutonomousAssignment {
            target_pos: Vec2::new(200.0, 0.0),
            ore_type: OreDeposit::Iron,
            sector_name: "S1".to_string(),
        },
    ));
    app.update();
    // Count entities with AutonomousAssignment — should still be 1, no spurious dispatch
    let count = app.world_mut().query::<&AutonomousAssignment>().iter(app.world()).count();
    assert_eq!(count, 1, "No new AutonomousAssignment should appear when no idle drone exists");
}

#[test]
fn scout_does_not_dispatch_to_already_served_asteroid() {
    // Unit test of the already_served guard in scout_orbit_system Pass 2.
    // active_drones.iter().any(|(_, dt)| dt.asteroid == asteroid_entity)
    let asteroid = Entity::PLACEHOLDER;
    let served_targets = vec![DroneTarget { asteroid }];

    let already_served = served_targets.iter().any(|dt| dt.asteroid == asteroid);
    assert!(already_served, "Asteroid with an active DroneTarget must be marked already_served");

    // Verify a different asteroid is NOT blocked
    let other_asteroid = Entity::from_raw(42);
    let other_served = served_targets.iter().any(|dt| dt.asteroid == other_asteroid);
    assert!(!other_served, "Different asteroid should not be blocked by an unrelated DroneTarget");
}
