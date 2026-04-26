use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use rand::Rng;
use crate::components::*;
use crate::constants::*;

pub fn spawn_initial_asteroids(
    mut commands: Commands,
    mut respawn_timer: ResMut<AsteroidRespawnTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    existing_asteroids: Query<&Transform, With<ActiveAsteroid>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn initial asteroids (one of each type around their sectors)
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Iron);
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Tungsten);
    spawn_asteroid(&mut commands, &mut meshes, &mut materials, &existing_asteroids, &asset_server, OreDeposit::Nickel);
    respawn_timer.timer = Timer::from_seconds(ASTEROID_RESPAWN_TIMER_SECS, TimerMode::Once);
}

pub fn spawn_asteroid(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    existing_asteroids: &Query<&Transform, With<ActiveAsteroid>>,
    asset_server: &Res<AssetServer>,
    ore_type: OreDeposit,
) -> bool {
    let mut rng = rand::thread_rng();
    
    // Base position based on ore type
    let base_pos = match ore_type {
        OreDeposit::Iron => SECTOR_1_POS,
        OreDeposit::Tungsten => SECTOR_2_POS,
        OreDeposit::Nickel => SECTOR_3_POS,
    };

    let mut position = Vec2::ZERO;
    let mut found_spot = false;

    // Try to find a valid spot
    for _ in 0..10 {
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let distance = rng.gen_range(50.0..150.0);
        let offset = Vec2::new(angle.cos() * distance, angle.sin() * distance);
        let candidate_pos = base_pos + offset;

        let mut too_close = false;
        for existing_transform in existing_asteroids.iter() {
            let existing_pos = existing_transform.translation.truncate();
            if candidate_pos.distance(existing_pos) < ASTEROID_MIN_SPAWN_DISTANCE {
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

    let color = match ore_type {
        OreDeposit::Iron => COLOR_IRON,
        OreDeposit::Tungsten => COLOR_TUNGSTEN,
        OreDeposit::Nickel => COLOR_NICKEL,
    };

    let radius = match ore_type {
        OreDeposit::Iron => ASTEROID_RADIUS_IRON,
        OreDeposit::Tungsten => ASTEROID_RADIUS_TUNGSTEN,
        OreDeposit::Nickel => ASTEROID_RADIUS_NICKEL,
    };

    let is_gated = match ore_type {
        OreDeposit::Iron => false,
        OreDeposit::Tungsten | OreDeposit::Nickel => true,
    };

    // Vary the ore amount slightly
    let ore_amount = ASTEROID_BASE_ORE * rng.gen_range(0.8..1.2);

    let asteroid_entity = commands.spawn((
        MapMarker, // Add MapMarker so it can be targeted in MapView
        ActiveAsteroid {
            ore_type,
            ore_remaining: ore_amount,
            lifespan_timer: ASTEROID_MAX_LIFESPAN_SECS,
        },
        Transform::from_xyz(position.x, position.y, Z_ENVIRONMENT),
        GlobalTransform::default(),
        Visibility::default(),
        Mesh2d(meshes.add(crate::systems::setup::generate_ore_mesh(&ore_type, rng.gen()))),
        MeshMaterial2d(materials.add(color)),
    )).id();

    let sector_id = match ore_type {
        OreDeposit::Iron => "S1",
        OreDeposit::Tungsten => "S2",
        OreDeposit::Nickel => "S3",
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
) {
    respawn_timer.timer.tick(time.delta());
    
    if respawn_timer.timer.finished() {
        let mut rng = rand::thread_rng();
        // Pick a random ore type to spawn
        let target_ore_type = match rng.gen_range(0..3) {
            0 => OreDeposit::Iron,
            1 => OreDeposit::Tungsten,
            _ => OreDeposit::Nickel,
        };

        // Check cap for this field
        let mut field_asteroids: Vec<(Entity, f32)> = Vec::new();
        for (entity, asteroid, _transform) in existing_asteroids.iter() {
            if asteroid.ore_type == target_ore_type {
                field_asteroids.push((entity, asteroid.lifespan_timer));
            }
        }

        if field_asteroids.len() >= ASTEROID_MAX_PER_FIELD {
            // Remove the oldest (lowest lifespan)
            field_asteroids.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            if let Some((oldest_entity, _)) = field_asteroids.first() {
                commands.entity(*oldest_entity).despawn_recursive();
            }
        }

        spawn_asteroid(&mut commands, &mut meshes, &mut materials, &transform_query, &asset_server, target_ore_type);
        respawn_timer.timer.reset();
    }
}
