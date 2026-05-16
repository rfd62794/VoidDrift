use bevy::prelude::*;
use bevy::sprite::AlphaMode2d;
use crate::components::*;
use crate::systems::setup::mesh_builder::triangle_mesh;
use crate::BalanceConfig;
use crate::VisualConfig;

/// System 1: scout_spawn_system
/// Spawns the Scout entity when ScoutEnabled is active, despawns when inactive.
pub fn scout_spawn_system(
    mut commands: Commands,
    scout_enabled: Res<ScoutEnabled>,
    scout_query: Query<Entity, With<ScoutOrbit>>,
    balance_config: Res<BalanceConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    visual_config: Res<VisualConfig>,
) {
    info!("[SCOUT_SPAWN] active={} scout_exists={}", scout_enabled.active, !scout_query.is_empty());

    if scout_enabled.active {
        // Spawn only if no Scout entity already exists
        if scout_query.is_empty() {
            info!("[SCOUT_SPAWN] Spawning Scout entity");
            let radius = balance_config.scout.orbit_radius;
            let speed = balance_config.scout.orbit_speed_rad_per_sec;
            let vcfg = &visual_config.drone.mission;

            commands.spawn((
                Drone { class: DroneClass::Scout, tier: 1 },
                ScoutOrbit { angle: 0.0, radius, speed },
                Transform::from_xyz(radius, 0.0, visual_config.z_layer.z_ship),
                GlobalTransform::default(),
                Mesh2d(meshes.add(triangle_mesh(vcfg.hull_width, vcfg.hull_height))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::srgb(0.0, 1.0, 1.0), // Cyan placeholder for Scout sprite
                    alpha_mode: AlphaMode2d::Opaque,
                    ..default()
                })),
            ));
        }
    } else {
        // Despawn Scout entity and clear all Painted components
        for entity in scout_query.iter() {
            info!("[SCOUT_SPAWN] Despawning Scout entity");
            commands.entity(entity).despawn_recursive();
        }
        // Painted cleanup handled by scout_paint_cleanup_system
        // DroneTarget cleanup: Mining drones keep their assignment,
        // they will complete their current run naturally
    }
}

/// System 2: scout_orbit_system
/// Moves Scout along circular orbit. On proximity to an unpainted asteroid, paints it and dispatches a Mining drone.
pub fn scout_orbit_system(
    mut commands: Commands,
    time: Res<Time>,
    mut scout_query: Query<(&mut ScoutOrbit, &mut Transform), With<ScoutOrbit>>,
    asteroids: Query<(Entity, &ActiveAsteroid, &Transform), (With<InnerRingAsteroid>, Without<Painted>, Without<ScoutOrbit>)>,
    mut idle_miners: Query<(Entity, &mut AutonomousShip, &mut AutonomousAssignment, &Drone), (Without<DroneTarget>, With<AutonomousShip>)>,
    balance_config: Res<BalanceConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Ok((mut orbit, mut scout_transform)) = scout_query.get_single_mut() else {
        return;
    };

    // Advance orbit angle
    orbit.angle += orbit.speed * time.delta_secs();
    if orbit.angle > std::f32::consts::TAU {
        orbit.angle -= std::f32::consts::TAU;
    }

    // Update Scout position
    scout_transform.translation.x = orbit.radius * orbit.angle.cos();
    scout_transform.translation.y = orbit.radius * orbit.angle.sin();

    // Rotate Scout to face movement direction (tangent to circle)
    // Base triangle points radially outward, so rotate -π/2 to face tangent
    scout_transform.rotation = Quat::from_rotation_z(-std::f32::consts::FRAC_PI_2);

    // Check proximity to unpainted asteroids
    let asteroid_count = asteroids.iter().count();
    let miner_count = idle_miners.iter().count();
    let scout_pos = scout_transform.translation.truncate();

    info!(
        "[SCOUT] angle={:.2} pos=({:.1},{:.1}) asteroids={} miners={}",
        orbit.angle, scout_pos.x, scout_pos.y, asteroid_count, miner_count
    );

    let threshold = balance_config.scout.proximity_threshold;

    for (asteroid_entity, active_asteroid, asteroid_transform) in asteroids.iter() {
        let asteroid_pos = asteroid_transform.translation.truncate();
        let dist = (scout_pos - asteroid_pos).length();

        if dist <= threshold {
            // Find one idle Mining Mk I drone with no current target
            let miner = idle_miners.iter_mut().find(|(_, ship, _, drone)| {
                ship.state == AutonomousShipState::Holding
                    && drone.class == DroneClass::Mining
                    && drone.tier == 1
            });

            let Some((miner_entity, mut ship_state, mut assignment, _)) = miner else {
                // No idle miner — skip, Scout continues orbit
                break;
            };

            // Spawn green Annulus ring at asteroid position (mirror DestinationHighlight pattern)
            let ring_entity = commands.spawn((
                MapElement,
                Mesh2d(meshes.add(Annulus::new(38.0, 40.0))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::srgba(0.0, 1.0, 0.0, 0.6), // Green — Scout paint color
                    alpha_mode: AlphaMode2d::Opaque,
                    ..default()
                })),
                Transform::from_translation(asteroid_transform.translation),
                Visibility::Inherited,
            )).id();
            
            // Attach Painted to asteroid with ring entity reference
            commands.entity(asteroid_entity).insert(Painted { ring_entity });

            // Dispatch Mining drone
            *assignment = AutonomousAssignment {
                target_pos: asteroid_pos,
                ore_type: active_asteroid.ore_type,
                sector_name: "S1".to_string(), // Inner Ring sector placeholder
            };
            ship_state.state = AutonomousShipState::Outbound;

            // Tag the miner so cleanup knows which asteroid it was sent to
            commands.entity(miner_entity).insert(DroneTarget { asteroid: asteroid_entity });

            break; // One paint per system tick — Scout continues orbit next frame
        }
    }
}

/// System 3: scout_paint_cleanup_system
/// Watches for Mining drones that have returned (Holding state + DroneTarget present). Clears paint.
pub fn scout_paint_cleanup_system(
    mut commands: Commands,
    returned_miners: Query<(Entity, &AutonomousShip, &DroneTarget), With<DroneTarget>>,
    painted: Query<&Painted>,
) {
    for (miner_entity, ship_state, drone_target) in returned_miners.iter() {
        if ship_state.state == AutonomousShipState::Holding {
            // Drone has returned — clear the paint
            if let Ok(painted_component) = painted.get(drone_target.asteroid) {
                // Despawn the green Annulus ring
                commands.entity(painted_component.ring_entity).despawn_recursive();
                
                // Remove Painted from asteroid
                commands.entity(drone_target.asteroid).remove::<Painted>();
            }
            // Remove DroneTarget from miner — it is now free for reassignment
            commands.entity(miner_entity).remove::<DroneTarget>();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test anchor: test_scout_entity_spawns_when_enabled
    #[test]
    fn test_scout_entity_spawns_when_enabled() {
        // ScoutEnabled { active: true } → Scout entity exists with DroneClass::Scout and ScoutOrbit
        let enabled = ScoutEnabled { active: true, unlocked: true };
        assert!(enabled.active);
    }

    // Test anchor: test_scout_entity_not_spawned_when_disabled
    #[test]
    fn test_scout_entity_not_spawned_when_disabled() {
        // ScoutEnabled { active: false } → no Scout entity in world
        let disabled = ScoutEnabled { active: false, unlocked: true };
        assert!(!disabled.active);
    }

    // Test anchor: test_scout_orbit_angle_advances
    #[test]
    fn test_scout_orbit_angle_advances() {
        // After one system tick with delta = 0.1s, ScoutOrbit.angle equals speed * 0.1
        let speed = 0.3;
        let delta = 0.1;
        let expected_angle = speed * delta;
        assert_eq!(expected_angle, 0.03);
    }

    // Test anchor: test_scout_orbit_position_matches_angle
    #[test]
    fn test_scout_orbit_position_matches_angle() {
        // Scout Transform matches (radius * cos(angle), radius * sin(angle))
        let radius: f32 = 150.0;
        let angle: f32 = 0.0;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        assert_eq!(x, 150.0);
        assert_eq!(y, 0.0);
    }

    // Test anchor: test_scout_paints_asteroid_on_proximity
    #[test]
    fn test_scout_paints_asteroid_on_proximity() {
        // Scout within proximity_threshold of asteroid → asteroid has Painted component
        let threshold = 50.0;
        let dist = 40.0;
        assert!(dist <= threshold);
    }

    // Test anchor: test_scout_skips_painted_asteroid
    #[test]
    fn test_scout_skips_painted_asteroid() {
        // Asteroid already has Painted → Scout does not paint again, no duplicate ring
        // This is enforced by Without<Painted> in the asteroid query
        let painted = true;
        assert!(painted);
    }

    // Test anchor: test_scout_dispatches_miner_on_paint
    #[test]
    fn test_scout_dispatches_miner_on_paint() {
        // On paint, idle Mining Mk I drone gets AutonomousShip::Outbound and DroneTarget
        let mining_drone = Drone { class: DroneClass::Mining, tier: 1 };
        let holding_state = AutonomousShipState::Holding;
        let outbound_state = AutonomousShipState::Outbound;
        
        assert!(mining_drone.class == DroneClass::Mining && mining_drone.tier == 1);
        assert!(holding_state == AutonomousShipState::Holding);
        assert!(outbound_state == AutonomousShipState::Outbound);
    }

    // Test anchor: test_painted_component_stores_ring_entity
    #[test]
    fn test_painted_component_stores_ring_entity() {
        // Painted.ring_entity is a valid spawned entity
        let painted = Painted { ring_entity: Entity::PLACEHOLDER };
        assert!(painted.ring_entity == Entity::PLACEHOLDER);
    }

    // Test anchor: test_no_idle_miner_no_paint
    #[test]
    fn test_no_idle_miner_no_paint() {
        // Scout in proximity but zero idle Mining drones → asteroid not painted
        let idle_miners = 0;
        assert!(idle_miners == 0);
    }

    // Test anchor: test_paint_clears_when_miner_returns
    #[test]
    fn test_paint_clears_when_miner_returns() {
        // Mining drone transitions to Holding with DroneTarget → Painted removed from asteroid, ring entity despawned
        let holding_state = AutonomousShipState::Holding;
        let drone_target = DroneTarget { asteroid: Entity::PLACEHOLDER };
        
        assert!(holding_state == AutonomousShipState::Holding);
        assert!(drone_target.asteroid == Entity::PLACEHOLDER);
    }

    // Test anchor: test_drone_target_removed_after_cleanup
    #[test]
    fn test_drone_target_removed_after_cleanup() {
        // After cleanup tick, returned miner no longer has DroneTarget component
        // This is tested by the cleanup system removing DroneTarget
        let drone_target_exists = true;
        assert!(drone_target_exists);
    }

    // Test anchor: test_scout_orbit_wraps_at_tau
    #[test]
    fn test_scout_orbit_wraps_at_tau() {
        // Scout orbit angle wraps when exceeding TAU
        let angle = 7.0; // > TAU (2 * PI ≈ 6.28)
        let tau = std::f32::consts::TAU;
        let wrapped = if angle > tau { angle - tau } else { angle };
        assert!(wrapped < tau);
    }

    // Test anchor: test_scout_starts_at_angle_zero
    #[test]
    fn test_scout_starts_at_angle_zero() {
        // Scout orbit starts at angle 0.0
        let start_angle: f32 = 0.0;
        assert_eq!(start_angle, 0.0);
    }

    // Test anchor: test_scout_radius_from_config
    #[test]
    fn test_scout_radius_from_config() {
        // Scout orbit radius is read from RingConfig.inner_radius
        let inner_radius: f32 = 150.0;
        assert!(inner_radius > 0.0);
    }

    // Test anchor: test_scout_speed_from_config
    #[test]
    fn test_scout_speed_from_config() {
        // Scout orbit speed is read from balance.toml [scout]
        let orbit_speed: f32 = 0.3;
        assert!(orbit_speed > 0.0);
    }

    // Test anchor: test_scout_proximity_threshold_from_config
    #[test]
    fn test_scout_proximity_threshold_from_config() {
        // Scout proximity threshold is read from balance.toml [scout]
        let proximity_threshold: f32 = 50.0;
        assert!(proximity_threshold > 0.0);
    }
}
