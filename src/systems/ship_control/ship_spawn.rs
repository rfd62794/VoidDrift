use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;
use crate::config::VisualConfig;
use crate::config::visual::{rgb, rgba};

/// Spawns a drone ship entity at `start_pos` heading toward `target`.
/// Single source of truth for drone ship appearance and component bundle.
/// Used by both `asteroid_input_system` and `bottle_input_system`.
pub fn spawn_drone_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start_pos: Vec2,
    target: AutopilotTarget,
    ore_type: OreDeposit,
    vcfg: &VisualConfig,
) -> Entity {
    let ship_ent = commands.spawn((
        Ship {
            state: ShipState::Navigating,
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: ore_type,
            cargo_capacity: CARGO_CAPACITY,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        LastHeading(0.0),
        target,
        Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(vcfg.drone.mission.hull_w, vcfg.drone.mission.hull_h))),
        MeshMaterial2d(materials.add(rgb(vcfg.drone.mission.color_hull))),
        Transform::from_xyz(start_pos.x, start_pos.y, Z_SHIP),
    )).id();

    let md = &vcfg.drone.mission;
    commands.entity(ship_ent).with_children(|parent| {
        parent.spawn((
            ThrusterGlow,
            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d(materials.add(rgb(md.color_thruster))),
            Transform::from_xyz(0.0, -18.0, 0.1),
            Visibility::Hidden,
        ));
        parent.spawn((
            MiningBeam,
            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
            MeshMaterial2d(materials.add(rgba(md.color_beam, md.beam_alpha))),
            Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP),
            Visibility::Hidden,
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(md.cargo_bar_w, md.cargo_bar_h))),
            MeshMaterial2d(materials.add(rgb(md.color_cargo_bg))),
            Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP),
        ));
        parent.spawn((
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(md.cargo_bar_w, md.cargo_bar_h))),
            MeshMaterial2d(materials.add(rgb(md.color_cargo_fill))),
            Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05),
        ));
    });

    ship_ent
}

/// Spawns a bottle-collection drone — no mining beam or cargo bar children needed.
pub fn spawn_bottle_drone(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start_pos: Vec2,
    target: AutopilotTarget,
    vcfg: &VisualConfig,
) -> Entity {
    let md = &vcfg.drone.mission;
    let ship_ent = commands.spawn((
        Ship {
            state: ShipState::Navigating,
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron, // dummy — bottle carrier has no ore
            cargo_capacity: CARGO_CAPACITY,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        LastHeading(0.0),
        target,
        Mesh2d(meshes.add(crate::systems::setup::triangle_mesh(md.hull_w, md.hull_h))),
        MeshMaterial2d(materials.add(rgb(md.color_hull))),
        Transform::from_xyz(start_pos.x, start_pos.y, Z_SHIP),
    )).id();

    commands.entity(ship_ent).with_children(|parent| {
        parent.spawn((
            ThrusterGlow,
            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d(materials.add(rgb(md.color_thruster))),
            Transform::from_xyz(0.0, -18.0, 0.1),
            Visibility::Hidden,
        ));
    });

    ship_ent
}
