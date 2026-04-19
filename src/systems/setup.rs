use bevy::prelude::*;
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

    // ── STARFIELD ────────────────────────────────────────────────────────────
    {
        use bevy::sprite::AlphaMode2d;
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
            far: 1100.0, // Ensure Z_STARS_FAR (-100) is visible from Z=900
            ..default()
        },
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 900.0), 
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
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_SHIP),
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
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 0.0, Z_HUD - Z_SHIP).with_scale(Vec3::splat(2.0)), 
            Visibility::Hidden,
        ));
    });

    // STATION / SECTORS setup
    commands.spawn((
        MapMarker,
        Station { 
            repair_progress: 0.0, 
            online: false,
            magnetite_reserves: 0.0,
            carbon_reserves: 0.0,
            hull_plate_reserves: 0,
            ship_hulls: 0,
            ai_cores: 0,
            power_cells: 0,
            power: STATION_POWER_MAX,
            maintenance_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            last_power_warning_time: 0.0,
            log: std::collections::VecDeque::from([
                "SYSTEMS INITIALIZED.".to_string(),
            ]),
        },
        Mesh2d(meshes.add(Rectangle::new(40.0, 40.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.0))),
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, Z_ENVIRONMENT),
    ))
    .with_children(|parent| {
        // [STEP 6] MAP ICON
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(16.0))),
            MeshMaterial2d(materials.add(COLOR_MAP_STATION)),
            Transform::from_xyz(0.0, 0.0, Z_MAP_MARKERS - Z_ENVIRONMENT).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));
        // [STEP 6] MAP LABEL
        parent.spawn((
            MapElement,
            Text2d::new("BASE"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -40.0, Z_MAP_MARKERS - Z_ENVIRONMENT + 0.1),
            Visibility::Hidden,
        ));
    });

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
            MeshMaterial2d(materials.add(COLOR_MAP_S1)),
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
            MeshMaterial2d(materials.add(COLOR_MAP_S7)),
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
            MeshMaterial2d(materials.add(COLOR_MAP_S3)),
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
        Mesh2d(meshes.add(Circle::new(40.0))), // Large ring
        MeshMaterial2d(materials.add(Color::srgba(1.0, 1.0, 1.0, 0.1))), // Static dim white
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
