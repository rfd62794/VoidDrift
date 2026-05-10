use bevy::prelude::*;
use crate::components::*;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::{rgb, rgba};
use crate::systems::visuals::{build_mesh_from_polygon, generate_rocket_points};
use crate::systems::visuals::component_nodes::RocketConfig;
use bevy_egui::egui::Color32;

/// Convert ShipDroneConfig to RocketConfig for mesh generation.
fn ship_config_to_rocket_config(ship_cfg: &crate::config::visual::ShipDroneConfig) -> RocketConfig {
    RocketConfig {
        width: ship_cfg.width,
        height: ship_cfg.height,
        color_body: Color32::from_rgb(ship_cfg.color_body[0], ship_cfg.color_body[1], ship_cfg.color_body[2]),
        color_nose: Color32::from_rgb(ship_cfg.color_nose[0], ship_cfg.color_nose[1], ship_cfg.color_nose[2]),
        color_fins: Color32::from_rgb(ship_cfg.color_fins[0], ship_cfg.color_fins[1], ship_cfg.color_fins[2]),
        color_exhaust: Color32::from_rgb(ship_cfg.color_exhaust[0], ship_cfg.color_exhaust[1], ship_cfg.color_exhaust[2]),
        nose_height_ratio: ship_cfg.nose_height_ratio,
        fin_width_ratio: ship_cfg.fin_width_ratio,
        fin_height_ratio: ship_cfg.fin_height_ratio,
        exhaust_radius: ship_cfg.exhaust_radius,
        porthole_radius: ship_cfg.porthole_radius,
        porthole_offset_y: ship_cfg.porthole_offset_y,
    }
}

/// Spawn a single rocket part as a child entity.
fn spawn_rocket_part(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    parent: Entity,
    points: &[Vec2],
    color: Color32,
    z_offset: f32,
) {
    let mesh = build_mesh_from_polygon(points);
    let material = ColorMaterial::from(Color::srgba_u8(color.r(), color.g(), color.b(), color.a()));

    commands.spawn((
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(material)),
        Transform::from_translation(Vec3::Z * z_offset),
    )).set_parent(parent);
}

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
    bcfg: &BalanceConfig,
    vcfg: &VisualConfig,
) -> Entity {
    let rocket_config = ship_config_to_rocket_config(&vcfg.ship.drone);
    let rocket_parts = generate_rocket_points(&rocket_config);

    let ship_ent = commands.spawn((
        Ship {
            state: ShipState::Navigating,
            speed: bcfg.mining.ship_speed,
            cargo: 0.0,
            cargo_type: ore_type,
            cargo_capacity: bcfg.mining.cargo_capacity,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        LastHeading(0.0),
        target,
        Transform::from_xyz(start_pos.x, start_pos.y, vcfg.z_layer.z_ship),
    )).id();

    // Spawn rocket parts as children
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.body, rocket_config.color_body, 0.0);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.nose, rocket_config.color_nose, 1.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_left, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_right, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_center, rocket_config.color_fins, 0.5);

    let md = &vcfg.drone.mission;
    let z_ship = vcfg.z_layer.z_ship;
    let z_beam = vcfg.z_layer.z_beam;
    let z_cargo_bar = vcfg.z_layer.z_cargo_bar;
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
            Transform::from_xyz(0.0, 0.0, z_beam - z_ship),
            Visibility::Hidden,
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(md.cargo_bar_w, md.cargo_bar_h))),
            MeshMaterial2d(materials.add(rgb(md.color_cargo_bg))),
            Transform::from_xyz(0.0, 24.0, z_cargo_bar - z_ship),
        ));
        parent.spawn((
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(md.cargo_bar_w, md.cargo_bar_h))),
            MeshMaterial2d(materials.add(rgb(md.color_cargo_fill))),
            Transform::from_xyz(0.0, 24.0, (z_cargo_bar - z_ship) + 0.05),
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
    bcfg: &BalanceConfig,
    vcfg: &VisualConfig,
) -> Entity {
    let rocket_config = ship_config_to_rocket_config(&vcfg.ship.drone);
    let rocket_parts = generate_rocket_points(&rocket_config);

    let md = &vcfg.drone.mission;
    let ship_ent = commands.spawn((
        Ship {
            state: ShipState::Navigating,
            speed: bcfg.mining.ship_speed,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron, // dummy — bottle carrier has no ore
            cargo_capacity: bcfg.mining.cargo_capacity,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        LastHeading(0.0),
        target,
        Transform::from_xyz(start_pos.x, start_pos.y, vcfg.z_layer.z_ship),
    )).id();

    // Spawn rocket parts as children
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.body, rocket_config.color_body, 0.0);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.nose, rocket_config.color_nose, 1.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_left, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_right, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, ship_ent, &rocket_parts.fin_center, rocket_config.color_fins, 0.5);

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
