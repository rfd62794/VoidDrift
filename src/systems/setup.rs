use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use bevy_egui::EguiContextSettings;
use rand::{Rng, SeedableRng};
use crate::constants::*;
use crate::components::*;

/// Clean up all entities before setting up a new game
pub fn cleanup_world_entities(
    mut commands: Commands,
    entities: Query<Entity, With<Transform>>, // Remove all entities with Transform (game objects)
) {
    info!("Cleaning up {} entities before new game", entities.iter().count());
    
    // Use commands.despawn() instead of despawn_recursive() to avoid B0003 errors
    // Collect entities first to avoid modifying query while iterating
    let entities_to_despawn: Vec<Entity> = entities.iter().collect();
    
    for entity in entities_to_despawn {
        // Try to despawn, but ignore if entity doesn't exist
        if commands.get_entity(entity).is_some() {
            commands.entity(entity).despawn();
        }
    }
}

/// Spawns the world objects, ship, and HUD.
pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_delta: ResMut<CameraDelta>,
    mut map_pan_state: ResMut<MapPanState>,
    mut opening_sequence: ResMut<OpeningSequence>,
    mut signal_log: ResMut<SignalLog>,
    mut drawer_state: ResMut<DrawerState>,
    mut world_view_rect: ResMut<WorldViewRect>,
    mut queue: ResMut<ShipQueue>,
) {
    info!("[Voidrift Phase 4] Final Production Build. PresentMode: Fifo.");

    // Reset resources to clean state
    *camera_delta = CameraDelta::default();
    *map_pan_state = MapPanState::default();
    *opening_sequence = OpeningSequence { phase: OpeningPhase::Adrift, timer: 0.0, beat_timer: 0.0 };
    *drawer_state = DrawerState::Collapsed;
    *world_view_rect = WorldViewRect::default();
    
    // Reset SignalLog completely
    *signal_log = SignalLog::default();

    init_quest_log(&mut commands);
    spawn_starfield(&mut commands, &mut meshes, &mut materials);
    spawn_camera(&mut commands);
    let ship1 = spawn_player_ship(&mut commands, &mut meshes, &mut materials, &asset_server);
    queue.available_ships.push(ship1);
    spawn_station(&mut commands, &mut meshes, &mut materials);
    spawn_berths(&mut commands);
    spawn_sectors(&mut commands, &mut meshes, &mut materials, &asset_server);
    spawn_map_connectors(&mut commands, &mut meshes, &mut materials);
    spawn_destination_highlight(&mut commands, &mut meshes, &mut materials);
}

fn init_quest_log(commands: &mut Commands) {
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
                description: "Discover Sector 3".to_string(),
                progress_current: None,
                progress_target: None,
                state: ObjectiveState::Active, // Start active for expansion
            },
            QuestObjective {
                id: 6,
                description: "Mine Carbon 3".to_string(),
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
}

fn spawn_starfield(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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
    for _ in 0..200 {
        let x: f32 = rng.gen_range(-900.0..900.0);
        let y: f32 = rng.gen_range(-1000.0..1000.0);
        commands.spawn((
            StarLayer(0.05),
            Mesh2d(star_sm.clone()),
            MeshMaterial2d(far_mat.clone()),
            Transform::from_xyz(x, y, Z_STARS_FAR),
        ));
    }
    for _ in 0..80 {
        let x: f32 = rng.gen_range(-900.0..900.0);
        let y: f32 = rng.gen_range(-1000.0..1000.0);
        commands.spawn((
            StarLayer(0.15),
            Mesh2d(star_lg.clone()),
            MeshMaterial2d(near_mat.clone()),
            Transform::from_xyz(x, y, Z_STARS_NEAR),
        ));
    }
}

fn spawn_camera(commands: &mut Commands) {
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
}

fn spawn_player_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
) -> Entity {
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
        },
        Mesh2d(meshes.add(triangle_mesh(20.0, 28.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
        Transform::from_xyz(-1000.0, -800.0, Z_SHIP),
    )).id();
    
    commands.entity(parent_ent).with_children(|parent| {
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
        // [STEP 10] WORLD-SPACE CARGO LABELS (Phase 10)
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
    
    parent_ent
}

fn spawn_station(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
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
            hull_plate_reserves: 0.0,
            thruster_reserves: 0.0,
            ship_hulls: 0.0,
            ai_cores: 0.0,
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

fn spawn_berths(commands: &mut Commands) {
    commands.spawn((Berth { arm_index: BERTH_1_ARM_INDEX, occupied_by: None, berth_type: BerthType::Player }, Name::new("Berth1")));
    commands.spawn((Berth { arm_index: BERTH_2_ARM_INDEX, occupied_by: None, berth_type: BerthType::Drone }, Name::new("Berth2")));
    commands.spawn((Berth { arm_index: BERTH_3_ARM_INDEX, occupied_by: None, berth_type: BerthType::Open }, Name::new("Berth3")));
}

fn spawn_sectors(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
) {
    spawn_asteroid_field(commands, meshes, materials, asset_server, SECTOR_1_POS, OreDeposit::Iron, 1234, "S1");
    spawn_asteroid_field(commands, meshes, materials, asset_server, SECTOR_2_POS, OreDeposit::Tungsten, 2345, "S2");
    spawn_asteroid_field(commands, meshes, materials, asset_server, SECTOR_3_POS, OreDeposit::Nickel, 3456, "S3");
}

fn spawn_map_connectors(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    spawn_map_connector(commands, meshes, materials, STATION_POS, SECTOR_1_POS);
    spawn_map_connector(commands, meshes, materials, STATION_POS, SECTOR_2_POS);
    spawn_map_connector(commands, meshes, materials, STATION_POS, SECTOR_3_POS);
}

fn spawn_destination_highlight(
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

fn spawn_asteroid_field(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    asset_server: &AssetServer,
    position: Vec2,
    ore: OreDeposit,
    seed: u64,
    sector_id: &str,
) {
    use bevy::sprite::AlphaMode2d;

    let base_color = match ore {
        OreDeposit::Iron => COLOR_IRON,
        OreDeposit::Tungsten => COLOR_TUNGSTEN,
        OreDeposit::Nickel => COLOR_NICKEL,
    };

    let is_gated = ore_laser_required(&ore) != LaserTier::Basic;
    let final_color = if is_gated {
        // Desaturate for gated fields
        let [r, g, b, _] = base_color.to_srgba().to_f32_array();
        let gray = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        Color::srgb(
            r * 0.7 + gray * 0.3,
            g * 0.7 + gray * 0.3,
            b * 0.7 + gray * 0.3,
        )
    } else {
        base_color
    };

    let radius = match ore {
        OreDeposit::Iron => ASTEROID_RADIUS_IRON,
        OreDeposit::Tungsten => ASTEROID_RADIUS_TUNGSTEN,
        OreDeposit::Nickel => ASTEROID_RADIUS_NICKEL,
    };

    let asteroid_entity = commands.spawn((
        MapMarker,
        AsteroidField { 
            ore_deposit: ore,
            depleted: false 
        },
        Mesh2d(meshes.add(generate_ore_mesh(&ore, seed))),
        MeshMaterial2d(materials.add(final_color)),
        Transform::from_xyz(position.x, position.y, Z_ENVIRONMENT),
    )).id();

    commands.entity(asteroid_entity).with_children(|parent| {
        // 1. MAP ICON
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: final_color,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));

        // 2. MAP LABEL (S1, S2, etc)
        parent.spawn((
            MapElement,
            Text2d::new(sector_id),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -36.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));

        // 3. ORE NAME LABEL (World space)
        let name = ore_name(&ore);
        parent.spawn((
            Text2d::new(name),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgba(0.8, 0.8, 0.8, 0.6)),
            Transform::from_xyz(0.0, -(radius + 12.0), Z_HUD - Z_ENVIRONMENT),
        ));

        // 4. LASER REQUIREMENT LABEL (If gated)
        if is_gated {
            let req_text = match ore_laser_required(&ore) {
                LaserTier::Tungsten => "[TUNGSTEN LASER REQ]",
                LaserTier::Composite => "[COMPOSITE LASER REQ]",
                _ => "",
            };
            parent.spawn((
                Text2d::new(req_text),
                TextFont {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 8.0,
                    ..default()
                },
                TextColor(Color::srgba(1.0, 0.3, 0.3, 0.8)),
                Transform::from_xyz(0.0, -(radius + 24.0), Z_HUD - Z_ENVIRONMENT),
            ));
        }
    });
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

pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64) -> Mesh {
    match ore {
        OreDeposit::Iron     => generate_iron_mesh(seed),
        OreDeposit::Tungsten => generate_tungsten_mesh(seed),
        OreDeposit::Nickel   => generate_nickel_mesh(seed),
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
