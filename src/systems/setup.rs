use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use bevy_egui::EguiContextSettings;
use rand::{Rng, SeedableRng};
use crate::constants::*;
use crate::components::*;

/// Spawns the world objects, ship, and HUD.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    info!("[Voidrift Phase 4] Final Production Build. PresentMode: Fifo.");

    // [QUEST TRACKER] Initialization
    commands.insert_resource(QuestLog {
        panel_open: false,
        objectives: vec![
            QuestObjective {
                id: 1,
                description: "Locate the signal source".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Active,
            },
            QuestObjective {
                id: 2,
                description: "Dock at the derelict station".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 3,
                description: "Repair the station".to_string(),
                progress_current: Some(0),
                progress_target: Some(25),
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 4,
                description: "Build an AI Core".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 5,
                description: "Discover Sector 7".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 6,
                description: "Mine Carbon".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
            QuestObjective {
                id: 7,
                description: "Assemble autonomous ship".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Locked,
            },
        ],
    });

    // ── STARFIELD ────────────────────────────────────────────────────────────
    {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD_BEEF_u64);
        let far_mat  = materials.add(ColorMaterial {
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            alpha_mode: AlphaMode2d::Opaque,
            ..default()
        });
        let near_mat = materials.add(ColorMaterial {
            color: Color::srgba(1.0, 1.0, 1.0, 1.0),
            alpha_mode: AlphaMode2d::Opaque,
            ..default()
        });
        
        // Stars are fully opaque and pushed far back to ensure Opaque2d phase
        // and avoid Z-fighting/shimmering on mobile hardware.
        let star_sm  = meshes.add(Rectangle::new(2.0, 2.0));
        let star_lg  = meshes.add(Rectangle::new(3.0, 3.0));

        for _ in 0..150 {
            let x: f32 = rng.gen_range(-700.0..700.0);
            let y: f32 = rng.gen_range(-500.0..500.0);
            commands.spawn((
                StarLayer(0.05),
                Mesh2d(star_sm.clone()),
                MeshMaterial2d(far_mat.clone()),
                Transform::from_xyz(x, y, Z_STARS_FAR), 
            ));
        }
        for _ in 0..50 {
            let x: f32 = rng.gen_range(-700.0..700.0);
            let y: f32 = rng.gen_range(-500.0..500.0);
            commands.spawn((
                StarLayer(0.15),
                Mesh2d(star_lg.clone()),
                MeshMaterial2d(near_mat.clone()),
                Transform::from_xyz(x, y, Z_STARS_NEAR), 
            ));
        }
    }
    // ─────────────────────────────────────────────────────────────────────────

    // 1. CAMERA
    commands.spawn((
        Camera2d::default(),
        OrthographicProjection {
            far: 1200.0, // Headroom for Z_STARS_FAR (-100) from Z=1000
            ..OrthographicProjection::default_2d()
        },
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 1000.0), 
        EguiContextSettings {
            scale_factor: EGUI_SCALE,
            ..default()
        },
    ));

    // 2. SHIP
    commands.spawn((
        PlayerShip,
        LastHeading(0.0),
        Ship { 
            state: ShipState::Idle, 
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: OreType::Empty,
            cargo_capacity: CARGO_CAPACITY,
            power: SHIP_POWER_MAX,
            power_cells: 0,
        },
        Mesh2d(meshes.add(triangle_mesh(20.0, 28.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
        Transform::from_xyz(-1000.0, -800.0, Z_SHIP),
    ))
    .with_children(|parent| {
        // [Z SYSTEM] Parent Z_SHIP (1.0) + local offsets
        parent.spawn((
            ThrusterGlow,
            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))), 
            Transform::from_xyz(0.0, -18.0, 0.1), // Global 1.1
            Visibility::Hidden,
        ));
        parent.spawn((
            MiningBeam,
            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))),
            MeshMaterial2d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.6))), 
            Transform::from_xyz(0.0, 0.0, Z_BEAM - Z_SHIP), // Global Z_BEAM (0.8)
            Visibility::Hidden,
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(0.0, 24.0, Z_CARGO_BAR - Z_SHIP), // Global Z_CARGO_BAR (1.1)
        ));
        parent.spawn((
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, (Z_CARGO_BAR - Z_SHIP) + 0.05), // Slightly above bar back
        ));

        // [STEP 6] SHIP MAP MARKER
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(triangle_mesh(12.0, 16.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: Color::srgb(0.0, 1.0, 1.0),
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), 
            Visibility::Hidden,
        ));
    });

    // STATION / SECTORS setup
    let _station_ent = commands.spawn((
        MapMarker,
        Station {
            repair_progress: 1.0,
            online: true,
            magnetite_reserves: 50.0,
            carbon_reserves: 25.0,
            hull_plate_reserves: 0,
            ship_hulls: 0,
            ai_cores: 0,
            power_cells: 5,
            power: STATION_POWER_MAX,
            maintenance_timer: Timer::from_seconds(10.0, TimerMode::Repeating),
            last_power_warning_time: -100.0,
            log: std::collections::VecDeque::new(),
            rotation: 0.0,
            rotation_speed: STATION_ROTATION_SPEED,
            dock_state: StationDockState::Rotating,
            resume_timer: 0.0,
        },
        StationQueues::default(),
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_ENVIRONMENT),
        Visibility::Visible,
    ))
    .with_children(|parent| {
        // [PHASE A] ROTATING VISUAL CONTAINER
        parent.spawn((
            StationVisualsContainer,
            Transform::IDENTITY,
            Visibility::Visible,
        ))
        .with_children(|vis| {
            // CENTRAL HUB
            vis.spawn((
                StationHub,
                Mesh2d(meshes.add(Circle::new(STATION_HUB_RADIUS))),
                MeshMaterial2d(materials.add(Color::srgb(0.33, 0.27, 0.0))), // Initial dark amber
                Transform::from_xyz(0.0, 0.0, 0.0),
            ));

            // ARMS & BERTHS
            for i in 0..6 {
                let angle = (i as f32) * (std::f32::consts::TAU / 6.0);
                let is_active = i < STATION_BERTHS_INITIAL;
                let length = if is_active { STATION_ARM_LENGTH } else { STATION_STUB_LENGTH };
                let color = if is_active { Color::srgb(0.6, 0.6, 0.6) } else { Color::srgb(0.12, 0.12, 0.12) };

                // Arm spoke
                // We use height as the length, so at 0 rotation it points North.
                // We want 'angle' to correspond to the direction of the arm.
                // We use width=thick, height=length.
                vis.spawn((
                    Mesh2d(meshes.add(Rectangle::new(STATION_ARM_THICKNESS, length))),
                    MeshMaterial2d(materials.add(ColorMaterial {
                        color,
                        alpha_mode: AlphaMode2d::Opaque,
                        ..default()
                    })),
                    // Pivot rectangle: translate by half length in the direction of the angle
                    // Rotation must be adjusted because Bevy Rectangle 0deg = North (+Y).
                    // We want 0 rad = East (+X). So we rotate by (angle - PI/2).
                    Transform::from_rotation(Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2))
                        .with_translation(Vec3::new(
                            angle.cos() * (length / 2.0),
                            angle.sin() * (length / 2.0),
                            -0.1, // Slightly behind hub
                        )),
                )).with_children(|arm| {
                    if is_active {
                        // Berth circle at end of active arm
                        // Local offset: 0 along width, length/2 along the rotated height axis
                        arm.spawn((
                            BerthVisual(i as u8),
                            Mesh2d(meshes.add(Circle::new(STATION_BERTH_RADIUS))),
                            MeshMaterial2d(materials.add(ColorMaterial {
                                color: Color::srgb(0.4, 0.4, 0.4),
                                alpha_mode: AlphaMode2d::Opaque,
                                ..default()
                            })),
                            Transform::from_xyz(0.0, length / 2.0, 0.1),
                        ));
                    }
                });
            }
        });

        // [STEP 6] MAP ICON (North-aligned, sibling to VisualsContainer)
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(12.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: COLOR_MAP_STATION,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        )).with_children(|map_icon| {
            // 3 Small static spokes for map identity
            for i in 0..3 {
                let angle = (i as f32) * (std::f32::consts::TAU / 3.0);
                map_icon.spawn((
                    MapElement,
                    Mesh2d(meshes.add(Rectangle::new(4.0, 20.0))),
                    MeshMaterial2d(materials.add(ColorMaterial {
                        color: COLOR_MAP_STATION,
                        ..default()
                    })),
                    Transform::from_rotation(Quat::from_rotation_z(angle))
                        .with_translation(Vec3::new(angle.cos() * 10.0, angle.sin() * 10.0, -0.1)),
                    Visibility::Inherited,
                ));
            }
        });
        // [STEP 6] MAP LABEL
        parent.spawn((
            MapElement,
            Text2d::new("BASE"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -40.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    }).id();

    // LOGICAL BERTH ENTITIES (Phase B)
    commands.spawn((
        Berth {
            arm_index: BERTH_1_ARM_INDEX,
            occupied_by: None,
            berth_type: BerthType::Player,
        },
        Name::new("Berth1"),
    ));
    commands.spawn((
        Berth {
            arm_index: BERTH_2_ARM_INDEX,
            occupied_by: None,
            berth_type: BerthType::Drone,
        },
        Name::new("Berth2"),
    ));
    commands.spawn((
        Berth {
            arm_index: BERTH_3_ARM_INDEX,
            occupied_by: None,
            berth_type: BerthType::Open,
        },
        Name::new("Berth3"),
    ));

    // Sector 1: Magnetite (Initial)
    commands.spawn((
        MapMarker,
        AsteroidField { ore_type: OreType::Magnetite, depleted: false },
        Mesh2d(meshes.add(generate_asteroid_mesh(1234))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.3, 0.3))), // Reddish
        Transform::from_xyz(SECTOR_1_POS.x, SECTOR_1_POS.y, Z_ENVIRONMENT),
    ))
    .with_children(|parent| {
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: COLOR_MAP_S1,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));
        parent.spawn((
            MapElement,
            Text2d::new("S1"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -36.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    });

    // Sector 7: Carbon (Hidden)
    commands.spawn((
        AsteroidField { ore_type: OreType::Carbon, depleted: false },
        Mesh2d(meshes.add(generate_asteroid_mesh(5678))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.8, 0.3))), // Greenish
        Transform::from_xyz(SECTOR_7_POS.x, SECTOR_7_POS.y, Z_ENVIRONMENT),
        Visibility::Hidden,
    ))
    .with_children(|parent| {
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: COLOR_MAP_S7,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));
        parent.spawn((
            MapElement,
            Text2d::new("S7"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -36.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    });

    // [STEP 6] SECTOR 3: Unexplored
    commands.spawn((
        Transform::from_xyz(SECTOR_3_POS.x, SECTOR_3_POS.y, Z_ENVIRONMENT),
        Visibility::Hidden, // World hidden
    ))
    .with_children(|parent| {
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: COLOR_MAP_S3,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));
        parent.spawn((
            MapElement,
            Text2d::new("???"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(COLOR_MAP_S3), // Dimmed text
            Transform::from_xyz(0.0, -36.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    });

    // [STEP 6] MAP CONNECTORS (Hub Topology)
    spawn_map_connector(&mut commands, &mut meshes, &mut materials, STATION_POS, SECTOR_1_POS);
    spawn_map_connector(&mut commands, &mut meshes, &mut materials, STATION_POS, SECTOR_7_POS);
    spawn_map_connector(&mut commands, &mut meshes, &mut materials, STATION_POS, SECTOR_3_POS);

    // [STEP 6] DESTINATION HIGHLIGHT
    commands.spawn((
        MapElement,
        DestinationHighlight,
        Mesh2d(meshes.add(Annulus::new(38.0, 40.0))), // White ring border
        MeshMaterial2d(materials.add(ColorMaterial {
            color: Color::srgba(1.0, 1.0, 1.0, 0.4), // Semi-transparent white
            alpha_mode: AlphaMode2d::Opaque, // Use Opaque to maintain flicker-free depth sorting
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, Z_HUD - 0.1), // Slightly behind markers
        Visibility::Hidden,
    ));
}

fn spawn_map_connector(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start: Vec2,
    end: Vec2,
) {
    use bevy::sprite::AlphaMode2d;
    let mid = (start + end) / 2.0;
    let diff = end - start;
    let length = diff.length();
    let angle = diff.y.atan2(diff.x);

    commands.spawn((
        MapElement,
        MapConnector,
        Mesh2d(meshes.add(Rectangle::new(length, 2.0))),
        MeshMaterial2d(materials.add(ColorMaterial {
            color: COLOR_MAP_LINE,
            alpha_mode: AlphaMode2d::Opaque,
            ..default()
        })),
        Transform::from_xyz(mid.x, mid.y, Z_CONNECTORS) 
            .with_rotation(Quat::from_rotation_z(angle)),
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

pub fn generate_asteroid_mesh(seed: u64) -> Mesh {
    use bevy::render::mesh::{Indices, PrimitiveTopology};
    use std::f32::consts::TAU;

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let base_radius = 24.0;
    
    // Generate 8 vertices around a circle
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    
    // Center vertex for triangle fan conversion
    vertices.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 0.0, 1.0]);
    uvs.push([0.5, 0.5]);

    for i in 0..8 {
        let angle = (i as f32 / 8.0) * TAU;
        let radius = base_radius + rng.gen_range(-6.0..6.0);
        
        let x = angle.cos() * radius;
        let y = angle.sin() * radius;
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);
        uvs.push([(x / 48.0) + 0.5, (y / 48.0) + 0.5]); // crude mapping
    }

    // Triangle list indices (fan behavior)
    let indices = vec![
        0, 1, 2,
        0, 2, 3,
        0, 3, 4,
        0, 4, 5,
        0, 5, 6,
        0, 6, 7,
        0, 7, 8,
        0, 8, 1,
    ];

    Mesh::new(PrimitiveTopology::TriangleList, Default::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_indices(Indices::U32(indices))
}
