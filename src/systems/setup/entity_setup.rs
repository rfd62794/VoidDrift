use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::{Rng, SeedableRng};
use crate::components::*;
use crate::components::resources::MaxDispatch;
use crate::constants::*;

pub fn spawn_opening_drone(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
) {
    let parent_ent = commands.spawn((
        InOpeningSequence,
        LastHeading(0.0),
        Ship {
            state: ShipState::Idle,
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron,
            cargo_capacity: CARGO_CAPACITY,
            laser_tier: LaserTier::Basic,
            current_mining_target: None,
        },
        AutonomousShipTag,
        Mesh2d(meshes.add(triangle_mesh(20.0, 28.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.6, 1.0))),
        Transform::from_xyz(-1000.0, -800.0, Z_SHIP),
    )).id();
    
    commands.entity(parent_ent).with_children(|parent| {
        parent.spawn((
            ThrusterGlow,
            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.0))),
            Transform::from_xyz(0.0, -18.0, 0.1),
            Visibility::Hidden,
        ));
        parent.spawn((
            MiningBeam,
            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.6))),
            Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP),
            Visibility::Hidden,
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP),
        ));
        parent.spawn((
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05),
        ));
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(triangle_mesh(12.0, 16.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: Color::srgb(0.0, 0.6, 1.0),
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)),
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
            TextColor(Color::srgba(0.0, 1.0, 1.0, 0.8)),
            Transform::from_xyz(0.0, 36.0, Z_HUD - Z_SHIP),
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
            Transform::from_xyz(0.0, 12.0, Z_HUD - Z_SHIP),
        ));
    });
}

pub fn spawn_station(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    mut max_dispatch: ResMut<MaxDispatch>,
) {
    let station_max_dispatch = 5;
    max_dispatch.0 = station_max_dispatch;

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
            log: std::collections::VecDeque::new(),
            rotation: 0.0,
            rotation_speed: STATION_ROTATION_SPEED,
            dock_state: StationDockState::Rotating,
            resume_timer: 0.0,
            cargo_capacity_multiplier: 1.0,
            ship_speed_multiplier: 1.0,
            power_multiplier: 1.0,
            max_active_asteroids: 3,
        },
        StationQueues::default(),
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_ENVIRONMENT),
        Visibility::Visible,
    ))
    .with_children(|parent| {
        parent.spawn((
            StationVisualsContainer,
            Transform::IDENTITY,
            Visibility::Visible,
        ))
        .with_children(|vis| {
            vis.spawn((
                StationHub,
                Mesh2d(meshes.add(Circle::new(STATION_HUB_RADIUS))),
                MeshMaterial2d(materials.add(Color::srgb(0.33, 0.27, 0.0))),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));
            for i in 0..6 {
                let angle = (i as f32) * (std::f32::consts::TAU / 6.0);
                let is_active = i < STATION_BERTH_INITIAL;
                let length = if is_active { STATION_ARM_LENGTH } else { STATION_STUB_LENGTH };
                let color = if is_active { Color::srgb(0.6, 0.6, 0.6) } else { Color::srgb(0.12, 0.12, 0.12) };
                vis.spawn((
                    Mesh2d(meshes.add(Rectangle::new(STATION_ARM_THICKNESS, length))),
                    MeshMaterial2d(materials.add(ColorMaterial { color, alpha_mode: AlphaMode2d::Opaque, ..default() })),
                    Transform::from_rotation(Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2))
                        .with_translation(Vec3::new(angle.cos() * (length / 2.0), angle.sin() * (length / 2.0), -0.1)),
                )).with_children(|arm| {
                    if is_active {
                        arm.spawn((
                            BerthVisual(i as u8),
                            Mesh2d(meshes.add(Circle::new(STATION_BERTH_RADIUS))),
                            MeshMaterial2d(materials.add(ColorMaterial { color: Color::srgb(0.4, 0.4, 0.4), alpha_mode: AlphaMode2d::Opaque, ..default() })),
                            Transform::from_xyz(0.0, length / 2.0, 0.1),
                        ));
                    }
                });
            }
        });
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(12.0))),
            MeshMaterial2d(materials.add(ColorMaterial { color: COLOR_MAP_STATION, alpha_mode: AlphaMode2d::Opaque, ..default() })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        )).with_children(|map_icon| {
            for i in 0..3 {
                let angle = (i as f32) * (std::f32::consts::TAU / 3.0);
                map_icon.spawn((
                    MapElement,
                    Mesh2d(meshes.add(Rectangle::new(4.0, 20.0))),
                    MeshMaterial2d(materials.add(ColorMaterial { color: COLOR_MAP_STATION, ..default() })),
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
            Transform::from_xyz(0.0, -40.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    });
}

pub fn spawn_berths(commands: &mut Commands) {
    commands.spawn((Berth { arm_index: BERTH_1_ARM_INDEX, occupied_by: None, berth_type: BerthType::Player }, Name::new("Berth1")));
    commands.spawn((Berth { arm_index: BERTH_2_ARM_INDEX, occupied_by: None, berth_type: BerthType::Drone }, Name::new("Berth2")));
    commands.spawn((Berth { arm_index: BERTH_3_ARM_INDEX, occupied_by: None, berth_type: BerthType::Open }, Name::new("Berth3")));
}



pub fn spawn_destination_highlight(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
        Transform::from_xyz(0.0, 0.0, Z_HUD - 0.1), // Slightly behind markers
        Visibility::Hidden,
    ));
}

pub fn spawn_tutorial_highlight(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
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
        Transform::from_xyz(0.0, 0.0, Z_HUD - 0.05),
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

pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64) -> Mesh {
    match ore {
        OreDeposit::Iron     => generate_iron_mesh(seed),
        OreDeposit::Tungsten => generate_tungsten_mesh(seed),
        OreDeposit::Nickel   => generate_nickel_mesh(seed),
        OreDeposit::Aluminum => generate_iron_mesh(seed),
    }
}


pub fn generate_iron_mesh(seed: u64) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let base_radius = ASTEROID_RADIUS_IRON;
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


pub fn generate_tungsten_mesh(seed: u64) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let base_radius = ASTEROID_RADIUS_TUNGSTEN;
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

pub fn generate_nickel_mesh(seed: u64) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let base_radius = ASTEROID_RADIUS_NICKEL;
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
