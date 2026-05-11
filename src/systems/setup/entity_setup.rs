use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::{Rng, SeedableRng};
use crate::components::*;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::rgb;
use crate::systems::visuals::{build_mesh_from_polygon, generate_rocket_points};
use crate::systems::visuals::component_nodes::RocketConfig;
use bevy_egui::egui::Color32;
use crate::spawn_drone_core_children;

/// Convert ShipOpeningConfig to RocketConfig for mesh generation.
fn opening_config_to_rocket_config(ship_cfg: &crate::config::visual::ShipOpeningConfig) -> RocketConfig {
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

pub fn spawn_opening_drone(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
    cfg: &BalanceConfig,
    vcfg: &VisualConfig,
) {
    let rocket_config = opening_config_to_rocket_config(&vcfg.ship.opening);
    let rocket_parts = generate_rocket_points(&rocket_config);

    let parent_ent = commands.spawn((
        InOpeningSequence,
        LastHeading(0.0),
        Ship {
            state: ShipState::Idle,
            speed: cfg.mining.ship_speed,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron,
            cargo_capacity: cfg.mining.cargo_capacity,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        Transform::from_xyz(-1000.0, -800.0, vcfg.z_layer.z_ship),
    )).id();

    // Spawn rocket parts as children
    spawn_rocket_part(commands, meshes, materials, parent_ent, &rocket_parts.body, rocket_config.color_body, 0.0);
    spawn_rocket_part(commands, meshes, materials, parent_ent, &rocket_parts.nose, rocket_config.color_nose, 1.5);
    spawn_rocket_part(commands, meshes, materials, parent_ent, &rocket_parts.fin_left, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, parent_ent, &rocket_parts.fin_right, rocket_config.color_fins, 0.5);
    spawn_rocket_part(commands, meshes, materials, parent_ent, &rocket_parts.fin_center, rocket_config.color_fins, 0.5);

    let od = &vcfg.drone.opening;
    commands.entity(parent_ent).with_children(|parent| {
        spawn_drone_core_children!(parent, meshes, materials, od, vcfg);
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(triangle_mesh(od.map_icon_w, od.map_icon_h))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: rgb(od.color_hull),
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, vcfg.z_layer.z_hud - vcfg.z_layer.z_ship).with_scale(Vec3::splat(2.0)),
            Visibility::Hidden,
        ));
        parent.spawn((
            CargoOreLabel,
            Text2d::new("EMPTY"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 10.0,
                ..default()
            },
            TextColor(crate::config::visual::rgba(od.color_beam, 0.8)),
            Transform::from_xyz(0.0, 36.0, vcfg.z_layer.z_hud - vcfg.z_layer.z_ship),
        ));
        parent.spawn((
            CargoCountLabel,
            Text2d::new("0 / 100"),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgba(1.0, 1.0, 1.0, 0.8)),
            Transform::from_xyz(0.0, 12.0, vcfg.z_layer.z_hud - vcfg.z_layer.z_ship),
        ));
    });
}

/// Macro to spawn the 4 shared drone child entities (ThrusterGlow, MiningBeam, cargo bars).
/// Used by both spawn_opening_drone and restore.rs spawn_saved_drones.
#[macro_export]
macro_rules! spawn_drone_core_children {
    ($parent:expr, $meshes:expr, $materials:expr, $drone_config:expr, $vcfg:expr) => {
        $parent.spawn((
            ThrusterGlow,
            Mesh2d($meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d($materials.add(rgb($drone_config.color_thruster))),
            Transform::from_xyz(0.0, -18.0, 0.1),
            Visibility::Hidden,
        ));
        $parent.spawn((
            MiningBeam,
            Mesh2d($meshes.add(Rectangle::new(2.0, 1.0))),
            MeshMaterial2d($materials.add(crate::config::visual::rgba($drone_config.color_beam, $drone_config.beam_alpha))),
            Transform::from_xyz(0.0, 0.0, $vcfg.z_layer.z_beam - $vcfg.z_layer.z_ship),
            Visibility::Hidden,
        ));
        $parent.spawn((
            Mesh2d($meshes.add(Rectangle::new($drone_config.cargo_bar_w, $drone_config.cargo_bar_h))),
            MeshMaterial2d($materials.add(rgb($drone_config.color_cargo_bg))),
            Transform::from_xyz(0.0, 24.0, $vcfg.z_layer.z_cargo_bar - $vcfg.z_layer.z_ship),
        ));
        $parent.spawn((
            ShipCargoBarFill,
            Mesh2d($meshes.add(Rectangle::new($drone_config.cargo_bar_w, $drone_config.cargo_bar_h))),
            MeshMaterial2d($materials.add(rgb($drone_config.color_cargo_fill))),
            Transform::from_xyz(0.0, 24.0, ($vcfg.z_layer.z_cargo_bar - $vcfg.z_layer.z_ship) + 0.05),
        ));
    };
}

pub fn spawn_station(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut max_dispatch: ResMut<MaxDispatch>,
    bcfg: &BalanceConfig,
    vcfg: &VisualConfig,
) {
    let station_max_dispatch = 5;
    max_dispatch.0 = station_max_dispatch;

    // Calculate rotation speed from config
    let rotation_speed = std::f32::consts::TAU / bcfg.station.rotation_speed_divisor;

    commands.spawn((
        MapMarker,
        Station {
            repair_progress: 1.0,
            online: true,
            iron_reserves: 0.0,
            iron_ingots: 0.0,
            tungsten_reserves: 0.0,
            tungsten_ingots: 0.0,
            nickel_reserves: 0.0,
            nickel_ingots: 0.0,
            aluminum_reserves: 0.0,
            aluminum_ingots: 0.0,
            aluminum_canisters: 0.0,
            hull_plate_reserves: 0.0,
            thruster_reserves: 0.0,
            ai_cores: 0.0,
            max_dispatch: station_max_dispatch,
            drone_build_progress: 0.0,
            drone_count: 0,
            log: std::collections::VecDeque::new(),
            rotation: 0.0,
            rotation_speed: rotation_speed,
            dock_state: StationDockState::Rotating,
            resume_timer: 0.0,
            cargo_capacity_multiplier: 1.0,
            ship_speed_multiplier: 1.0,
            power_multiplier: 1.0,
            max_active_asteroids: 3,
        },
        StationQueues::default(),
        Transform::from_xyz(crate::constants::STATION_POS.x, crate::constants::STATION_POS.y, vcfg.z_layer.z_environment),
        Visibility::Visible,
    ))
    .with_children(|parent| {
        parent.spawn((
            StationVisualsContainer,
            Transform::IDENTITY,
            Visibility::Visible,
        ))
        .with_children(|vis| {
            let sv = &vcfg.station;
            vis.spawn((
                StationHub,
                Mesh2d(meshes.add(Circle::new(sv.hub_radius))),
                MeshMaterial2d(materials.add(rgb(sv.color_hub_offline))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            for i in 0..6 {
                let angle = (i as f32) * (std::f32::consts::TAU / 6.0);
                let is_active = i < sv.berth_initial;
                let length = if is_active { sv.arm_length } else { sv.stub_length };
                let color = if is_active { rgb(sv.color_arm_active) } else { rgb(sv.color_arm_stub) };
                vis.spawn((
                    Mesh2d(meshes.add(Rectangle::new(sv.arm_thickness, length))),
                    MeshMaterial2d(materials.add(ColorMaterial { color, alpha_mode: AlphaMode2d::Opaque, ..default() })),
                    Transform::from_rotation(Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2))
                        .with_translation(Vec3::new(angle.cos() * (length / 2.0), angle.sin() * (length / 2.0), -0.1)),
                )).with_children(|arm| {
                    if is_active {
                        arm.spawn((
                            BerthVisual(i as u8),
                            Mesh2d(meshes.add(Circle::new(sv.berth_radius))),
                            MeshMaterial2d(materials.add(ColorMaterial { color: rgb(sv.color_berth_empty), alpha_mode: AlphaMode2d::Opaque, ..default() })),
                            Transform::from_xyz(0.0, length / 2.0, 0.1),
                        ));
                    }
                });
            }
        });
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(12.0))),
            MeshMaterial2d(materials.add(ColorMaterial { color: Color::srgba(vcfg.map_colors.color_map_station[0], vcfg.map_colors.color_map_station[1], vcfg.map_colors.color_map_station[2], 1.0), alpha_mode: AlphaMode2d::Opaque, ..default() })),
            Transform::from_xyz(0.0, 0.0, vcfg.z_layer.z_map_markers - vcfg.z_layer.z_environment).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        )).with_children(|map_icon| {
            for i in 0..3 {
                let angle = (i as f32) * (std::f32::consts::TAU / 3.0);
                map_icon.spawn((
                    MapElement,
                    Mesh2d(meshes.add(Rectangle::new(4.0, 20.0))),
                    MeshMaterial2d(materials.add(ColorMaterial { color: Color::srgba(vcfg.map_colors.color_map_station[0], vcfg.map_colors.color_map_station[1], vcfg.map_colors.color_map_station[2], 1.0), ..default() })),
                    Transform::from_rotation(Quat::from_rotation_z(angle)).with_translation(Vec3::new(angle.cos() * 10.0, angle.sin() * 10.0, -0.1)),
                    Visibility::Inherited,
                ));
            }
        });
        parent.spawn((
            MapElement,
            Text2d::new("BASE"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -40.0, vcfg.z_layer.z_map_markers - vcfg.z_layer.z_environment + 0.1),
            Visibility::Hidden,
        ));
    });
}

pub fn spawn_berths(commands: &mut Commands, vcfg: &VisualConfig) {
    commands.spawn((Berth { arm_index: vcfg.station.berth_1_arm_index, occupied_by: None, berth_type: BerthType::Player }, Name::new("Berth1")));
    commands.spawn((Berth { arm_index: vcfg.station.berth_2_arm_index, occupied_by: None, berth_type: BerthType::Drone }, Name::new("Berth2")));
    commands.spawn((Berth { arm_index: vcfg.station.berth_3_arm_index, occupied_by: None, berth_type: BerthType::Open }, Name::new("Berth3")));
}



pub fn spawn_destination_highlight(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    vcfg: &VisualConfig,
) {
    commands.spawn((
        MapElement,
        DestinationHighlight,
        Mesh2d(meshes.add(Annulus::new(38.0, 40.0))), // White ring border
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::srgba(1.0, 1.0, 1.0, 0.4), // Semi-transparent white
            alpha_mode: AlphaMode2d::Opaque,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, vcfg.z_layer.z_hud - 0.1), // Slightly behind markers
        Visibility::Hidden,
    ));
}

pub fn spawn_tutorial_highlight(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    vcfg: &VisualConfig,
) {
    commands.spawn((
        MapElement,
        TutorialHighlight,
        Mesh2d(meshes.add(Annulus::new(38.0, 40.0))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::srgba(0.0, 1.0, 1.0, 0.6), // Cyan — distinct from white DestinationHighlight
            alpha_mode: AlphaMode2d::Opaque,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, vcfg.z_layer.z_hud - 0.05),
        Visibility::Hidden,
    ));
}



// ── VISUAL HELPERS ───────────────────────────────────────────────────────────
pub fn triangle_mesh(w: f32, h: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    
    // Pointing up (+Y is forward in 2D)
    let vertices = vec![
        [0.0, h / 2.0, 0.0],
        [-w / 2.0, -h / 2.0, 0.0],
        [w / 2.0, -h / 2.0, 0.0],
    ];
    let normals = vec![[0.0, 0.0, 1.0]; 3];
    let uvs = vec![[0.5, 1.0], [0.0, 0.0], [1.0, 0.0]];
    let indices = vec![0, 1, 2];

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}

pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64, cfg: &BalanceConfig) -> Mesh {
    match ore {
        OreDeposit::Iron     => generate_iron_mesh_with_radius(seed, cfg.asteroid.radius_iron),
        OreDeposit::Tungsten => generate_tungsten_mesh_with_radius(seed, cfg.asteroid.radius_tungsten),
        OreDeposit::Nickel   => generate_nickel_mesh_with_radius(seed, cfg.asteroid.radius_nickel),
        OreDeposit::Aluminum => generate_iron_mesh_with_radius(seed, cfg.asteroid.radius_aluminum),
    }
}


pub fn generate_iron_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(8..=10);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.25..base_radius * 0.25);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}


pub fn generate_tungsten_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(6..=8);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.08..base_radius * 0.08);
        // Blocky: modify angle to align more to grid
        let snappy_angle = (angle * (vertex_count as f32 / TAU)).round() * (TAU / vertex_count as f32);
        let x = snappy_angle.cos() * radius;
        let y = snappy_angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}

pub fn generate_nickel_mesh_with_radius(seed: u64, base_radius: f32) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let vertex_count = rng.gen_range(10..=12);
    let mut vertices = vec![[0.0, 0.0, 0.0]];
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];
    for i in 0..vertex_count {
        let angle = (i as f32 / vertex_count as f32) * TAU;
        let radius = base_radius + rng.gen_range(-base_radius * 0.15..base_radius * 0.15);
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / (base_radius * 2.0)) + 0.5, (y / (base_radius * 2.0)) + 0.5]);
    }
    let mut indices = Vec::new();
    for i in 1..vertex_count { indices.extend_from_slice(&[0, i, i + 1]); }
    indices.extend_from_slice(&[0, vertex_count, 1]);
    Mesh::new(PrimitiveTopology::TriangleList, Default::default()).with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices).with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals).with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs).with_inserted_indices(Indices::U32(indices))
}
