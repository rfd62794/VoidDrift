use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::Rng;
use crate::components::*;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::rgb;
use crate::systems::visuals::{build_mesh_from_polygon_with_colors, generate_ore_polygon_points};
use crate::components::utilities::ore_config_key;

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    mut respawn_timer: ResMut<AsteroidRespawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_asteroids: Query<&Transform, With<ActiveAsteroid>>,
    asset_server: Res<AssetServer>,
    cfg: Res<BalanceConfig>,
    vcfg: Res<VisualConfig>,
) {
    // Spawn initial asteroids (one of each type around their sectors)
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Iron, &cfg, &vcfg);
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Tungsten, &cfg, &vcfg);
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Nickel, &cfg, &vcfg);
    respawn_timer.timer = Timer::from_seconds(cfg.asteroid.respawn_timer_secs, TimerMode::Once);
}

pub fn spawn_asteroid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    existing_asteroids: &Query<&Transform, With<ActiveAsteroid>>,
    asset_server: &Res<AssetServer>,
    ore_type: OreDeposit,
    cfg: &BalanceConfig,
    vcfg: &VisualConfig,
) -> bool {
    let mut rng = rand::thread_rng();
    
    // Base position is the station (0,0)
    let base_pos = Vec2::ZERO;

    let mut position = Vec2::ZERO;
    let mut found_spot = false;

    // Try to find a valid spot
    for _ in 0..cfg.asteroid_spawning.spawn_retry_count {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        // Force exact radial distance for Inner Ring asteroids to match Scout orbit
        let distance = if ore_type == OreDeposit::Iron {
            cfg.scout.orbit_radius // Match Scout orbit exactly for Iron asteroids
        } else {
            rng.gen_range(cfg.asteroid_spawning.spawn_distance_min..cfg.asteroid_spawning.spawn_distance_max)
        };
        let candidate_pos = base_pos + Vec2::new(angle.cos() * distance, angle.sin() * distance);

        let mut too_close = false;
        for existing_transform in existing_asteroids.iter() {
            let existing_pos = existing_transform.translation.truncate();
            if candidate_pos.distance(existing_pos) < cfg.asteroid.min_spawn_distance {
                too_close = true;
                break;
            }
        }

        if !too_close {
            position = candidate_pos;
            found_spot = true;
            break;
        }
    }

    if !found_spot {
        return false;
    }

    let av = &vcfg.asteroid;
    let color = match ore_type {
        OreDeposit::Iron     => rgb(av.color_iron),
        OreDeposit::Tungsten => rgb(av.color_tungsten),
        OreDeposit::Nickel   => rgb(av.color_nickel),
        OreDeposit::Aluminum => rgb(av.color_aluminum),
    };

    let radius = match ore_type {
        OreDeposit::Iron => cfg.asteroid.radius_iron,
        OreDeposit::Tungsten => cfg.asteroid.radius_tungsten,
        OreDeposit::Nickel => cfg.asteroid.radius_nickel,
        OreDeposit::Aluminum => cfg.asteroid.radius_aluminum,
    };

    let is_gated = match ore_type {
        OreDeposit::Iron => false,
        OreDeposit::Tungsten | OreDeposit::Nickel => true,
        OreDeposit::Aluminum => false,
    };

    // Vary the ore amount slightly
    let ore_amount = cfg.asteroid.base_ore * rng.gen_range(0.8..1.2);

    // Procedural mesh generation
    let ore_key = ore_config_key(&ore_type);
    let ore_config = match ore_key {
        "metal" => &vcfg.ore.metal,
        "h3_gas" => &vcfg.ore.h3_gas,
        "void_essence" => &vcfg.ore.void_essence,
        _ => &vcfg.ore.metal,
    };

    // Body polygon with vertex colors for banding
    let body_points = generate_ore_polygon_points(
        radius,
        12, // vertex_count
        0.25, // jaggedness
        rng.gen(),
    );

    let body_color = Color::srgb_u8(ore_config.color_body[0], ore_config.color_body[1], ore_config.color_body[2]);
    let vein_color = Color::srgb_u8(ore_config.color_vein[0], ore_config.color_vein[1], ore_config.color_vein[2]);

    // Generate vertex colors - alternate between body and vein to create banding
    let vertex_colors: Vec<Color> = body_points
        .iter()
        .enumerate()
        .map(|(i, _)| {
            if i % 2 == 0 {
                body_color
            } else {
                vein_color
            }
        })
        .collect();

    let body_mesh = build_mesh_from_polygon_with_colors(&body_points, &vertex_colors);

    let asteroid_entity = commands.spawn((
        MapMarker,
        ActiveAsteroid {
            ore_type,
            ore_remaining: ore_amount,
            lifespan_timer: cfg.asteroid.max_lifespan_secs,
        },
        InnerRingAsteroid, // TODO(#59): Attach based on actual ring when Ring 2/Ring 3 are implemented
        Transform::from_xyz(position.x, position.y, vcfg.z_layer.z_environment),
        GlobalTransform::default(),
        Visibility::default(),
        AsteroidBody,
        Mesh2d(meshes.add(body_mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::from(Color::WHITE))), // White to let vertex colors show
    )).id();

    let sector_id = match ore_type {
        OreDeposit::Iron => "S1",
        OreDeposit::Tungsten => "S2",
        OreDeposit::Nickel => "S3",
        OreDeposit::Aluminum => "S4",
    };

    commands.entity(asteroid_entity).with_children(|parent| {
        // 1. MAP ICON
        parent.spawn((
            MapElement,
            Mesh2d(meshes.add(Circle::new(14.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color,
                alpha_mode: AlphaMode2d::Opaque,
                ..default()
            })),
            Transform::from_xyz(0.0, 0.0, vcfg.z_layer.z_map_markers - vcfg.z_layer.z_environment).with_scale(Vec3::splat(1.5)),
            Visibility::Hidden,
        ));

        // 2. MAP LABEL (S1, S2, etc)
        parent.spawn((
            MapElement,
            Text2d::new(sector_id),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, -36.0, vcfg.z_layer.z_map_markers - vcfg.z_layer.z_environment + 0.1),
            Visibility::Hidden,
        ));

        // 3. ORE NAME LABEL (World space)
        let name = crate::components::ore_name(&ore_type);
        parent.spawn((
            Text2d::new(name),
            TextFont {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 10.0,
                ..default()
            },
            TextColor(Color::srgba(0.8, 0.8, 0.8, 0.6)),
            Transform::from_xyz(0.0, -(radius + 12.0), vcfg.z_layer.z_hud - vcfg.z_layer.z_environment),
        ));

        // 4. LASER REQUIREMENT LABEL (If gated)
        if is_gated {
            let req_text = match crate::components::ore_laser_required(&ore_type) {
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
                Transform::from_xyz(0.0, -(radius + 24.0), vcfg.z_layer.z_hud - vcfg.z_layer.z_environment),
            ));
        }
    });

    true
}

pub fn asteroid_respawn_system(
    mut respawn_timer: ResMut<AsteroidRespawnTimer>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_asteroids: Query<(Entity, &ActiveAsteroid, &Transform)>,
    transform_query: Query<&Transform, With<ActiveAsteroid>>,
    asset_server: Res<AssetServer>,
    station_query: Query<&Station>,
    cfg: Res<BalanceConfig>,
    vcfg: Res<VisualConfig>,
) {
    respawn_timer.timer.tick(time.delta());
    
    if respawn_timer.timer.finished() {
        let Ok(station) = station_query.get_single() else { return; };
        
        let active_count = existing_asteroids.iter().count() as u32;
        if active_count >= station.max_active_asteroids {
            respawn_timer.timer.reset();
            return;
        }

        let mut rng = rand::thread_rng();
        let target_ore_type = match rng.gen_range(0..10) {
            0..=2 => OreDeposit::Iron,
            3..=5 => OreDeposit::Tungsten,
            6..=8 => OreDeposit::Nickel,
            _ => OreDeposit::Aluminum,
        };

        spawn_asteroid(&mut commands, &mut meshes, &mut materials, &transform_query, &asset_server, target_ore_type, &cfg, &vcfg);
        respawn_timer.timer.reset();
    }
}
