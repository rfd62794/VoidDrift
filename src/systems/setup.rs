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
    // 150 far stars (Z=0.1, opacity 40%, 1.5×1.5) + 50 near stars (Z=0.2, 70%, 2.5×2.5).
    // Stars are semi-attached to camera: they track camera movement at (1-parallax)
    // speed, so they appear to drift backward at (parallax) speed — classic parallax.
    // Wrap at ±700×±500 from camera to ensure seamless coverage.
    {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0xDEAD_BEEF_u64);
        let far_mat  = materials.add(Color::srgba(1.0, 1.0, 1.0, 0.4));
        let near_mat = materials.add(Color::srgba(1.0, 1.0, 1.0, 0.7));
        
        // Use whole integer sizes (2.0 and 3.0) instead of fractional (1.5 and 2.5).
        // Fractional sizes on high-DPI mobile screens cause subpixel aliasing (shimmer/flickering).
        let star_sm  = meshes.add(Rectangle::new(2.0, 2.0));
        let star_lg  = meshes.add(Rectangle::new(3.0, 3.0));

        for _ in 0..150 {
            let x: f32 = rng.gen_range(-700.0..700.0);
            let y: f32 = rng.gen_range(-500.0..500.0);
            commands.spawn((
                StarLayer(0.05),
                Mesh2d(star_sm.clone()),
                MeshMaterial2d(far_mat.clone()),
                Transform::from_xyz(x, y, 0.1),
            ));
        }
        for _ in 0..50 {
            let x: f32 = rng.gen_range(-700.0..700.0);
            let y: f32 = rng.gen_range(-500.0..500.0);
            commands.spawn((
                StarLayer(0.15),
                Mesh2d(star_lg.clone()),
                MeshMaterial2d(near_mat.clone()),
                Transform::from_xyz(x, y, 0.2),
            ));
        }
    }
    // ─────────────────────────────────────────────────────────────────────────

    // 1. CAMERA
    commands.spawn((
        Camera2d::default(),
        MainCamera,
        Transform::from_xyz(0.0, 0.0, 999.0),
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
        Transform::from_xyz(0.0, 0.0, 1.0),
    ))
    .with_children(|parent| {
        // [POLISH] Thruster Glow
        parent.spawn((
            ThrusterGlow,
            Mesh2d(meshes.add(Rectangle::new(6.0, 8.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))), // Orange for player
            Transform::from_xyz(0.0, -18.0, -0.1),
            Visibility::Hidden,
        ));
        // [POLISH] Mining Beam
        parent.spawn((
            MiningBeam,
            Mesh2d(meshes.add(Rectangle::new(2.0, 1.0))), // 1.0 initial height
            MeshMaterial2d(materials.add(Color::srgba(0.0, 1.0, 1.0, 0.6))), // Cyan for player
            Transform::from_xyz(0.0, 0.0, -0.2),
            Visibility::Hidden,
        ));
        parent.spawn((
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.2, 0.2, 0.2))),
            Transform::from_xyz(0.0, 24.0, 1.1),
        ));
        parent.spawn((
            ShipCargoBarFill,
            Mesh2d(meshes.add(Rectangle::new(40.0, 6.0))),
            MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 1.0))),
            Transform::from_xyz(0.0, 24.0, 1.2),
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
        Transform::from_xyz(STATION_POS.x, STATION_POS.y, 0.5),
    ));

    // Sector 1: Magnetite (Initial)
    commands.spawn((
        MapMarker,
        AsteroidField { ore_type: OreType::Magnetite, depleted: false },
        Mesh2d(meshes.add(generate_asteroid_mesh(1234))),
        MeshMaterial2d(materials.add(Color::srgb(0.8, 0.3, 0.3))), // Reddish
        Transform::from_xyz(SECTOR_1_POS.x, SECTOR_1_POS.y, 0.5),
    ));

    // Sector 7: Carbon (Hidden)
    // We spawn it without MapMarker initially
    commands.spawn((
        AsteroidField { ore_type: OreType::Carbon, depleted: false },
        Mesh2d(meshes.add(generate_asteroid_mesh(5678))),
        MeshMaterial2d(materials.add(Color::srgb(0.3, 0.8, 0.3))), // Greenish
        Transform::from_xyz(SECTOR_7_POS.x, SECTOR_7_POS.y, 0.5),
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
