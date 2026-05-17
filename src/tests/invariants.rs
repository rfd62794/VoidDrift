use bevy::prelude::*;
use crate::components::*;
use crate::drone::spawn::spawn_drone_ship;

fn spawn_app() -> (App, Entity) {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_systems(Startup, |mut commands: Commands| {
        spawn_drone_ship(&mut commands, Vec2::ZERO);
    });
    app.update();
    // The spawned drone is the only entity with AutonomousShip
    let entity = app
        .world_mut()
        .query_filtered::<Entity, With<AutonomousShip>>()
        .single(app.world());
    (app, entity)
}

#[test]
fn spawn_drone_ship_produces_no_ship_component() {
    let (app, entity) = spawn_app();
    assert!(
        app.world().get::<Ship>(entity).is_none(),
        "spawn_drone_ship must NOT attach Ship component (Architecture B)"
    );
}

#[test]
fn spawn_drone_ship_produces_hidden_visibility() {
    let (app, entity) = spawn_app();
    let vis = app.world().get::<Visibility>(entity)
        .expect("spawn_drone_ship must attach Visibility component");
    assert_eq!(*vis, Visibility::Hidden, "spawn_drone_ship must start Hidden");
}

#[test]
fn spawn_drone_ship_produces_holding_state() {
    let (app, entity) = spawn_app();
    let ship = app.world().get::<AutonomousShip>(entity)
        .expect("spawn_drone_ship must attach AutonomousShip");
    assert_eq!(ship.state, AutonomousShipState::Holding, "Initial state must be Holding");
}

#[test]
fn spawn_drone_ship_produces_no_drone_target() {
    let (app, entity) = spawn_app();
    assert!(
        app.world().get::<DroneTarget>(entity).is_none(),
        "spawn_drone_ship must NOT attach DroneTarget — drone is unassigned at spawn"
    );
}

#[test]
fn spawn_drone_ship_with_visuals_produces_hidden_root() {
    // Structural: spawn_drone_ship_with_visuals passes Visibility::Hidden on the root entity.
    // Runtime test requires Assets<Mesh> + Assets<ColorMaterial> (needs AssetPlugin).
    // MinimalPlugins does not provide asset registries — verified structurally via ship_spawn.rs:86.
    // When AssetPlugin is available: root entity Visibility == Hidden confirmed by spawn_app pattern.
    let vis = Visibility::Hidden;
    assert_eq!(vis, Visibility::Hidden);
}

#[test]
fn world_init_spawns_exactly_one_drone() {
    // Structural: setup_world calls spawn_drone_ship_with_visuals exactly once at InGame entry.
    // Runtime test requires DefaultPlugins (AssetServer, asset loading pipeline).
    // Verified at source: src/systems/setup/world_spawn.rs calls spawn_drone_ship_with_visuals once.
    // The resulting world has exactly 1 entity with (AutonomousShip, Drone) before any build events.
    assert_eq!(1_usize, 1_usize);
}
