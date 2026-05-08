use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::Rng;
use crate::components::*;
use crate::constants::*;
use crate::config::{BalanceConfig, VisualConfig};
use crate::config::visual::rgb;

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
    for _ in 0..10 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        // Using existing radial boundary distances (roughly ~200.0 to 500.0 from origin)
        let distance = rng.gen_range(200.0..500.0);
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

    let asteroid_entity = commands.spawn((
        MapMarker, // Add MapMarker so it can be targeted in MapView
        ActiveAsteroid {
            ore_type,
            ore_remaining: ore_amount,
            lifespan_timer: cfg.asteroid.max_lifespan_secs,
        },
        Transform::from_xyz(position.x, position.y, Z_ENVIRONMENT),
        GlobalTransform::default(),
        Visibility::default(),
        Mesh2d(meshes.add(crate::systems::setup::generate_ore_mesh(&ore_type, rng.gen(), cfg))),
        MeshMaterial2d(materials.add(color)),
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
        let name = crate::components::ore_name(&ore_type);
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
                Transform::from_xyz(0.0, -(radius + 24.0), Z_HUD - Z_ENVIRONMENT),
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
