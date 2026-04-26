use bevy::prelude::*;
use crate::components::*;

pub fn asteroid_lifecycle_system(
    mut asteroids: Query<(Entity, &mut ActiveAsteroid), Without<Station>>,
    ships: Query<(&Ship, Option<&AutopilotTarget>)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut asteroid) in asteroids.iter_mut() {
        // Disintegrate if fully depleted
        if asteroid.ore_remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        // Check if any ship is currently targeting this asteroid (navigating or mining)
        let is_targeted = ships.iter().any(|(ship, autopilot)| {
            ship.current_mining_target == Some(entity) || 
            autopilot.map_or(false, |ap| ap.target_entity == Some(entity))
        });

        if !is_targeted {
            // Only decrease lifespan if no one is targeting it
            asteroid.lifespan_timer -= time.delta_secs();
        }

        // Disintegrate if lifespan expires AND no ship is targeting it
        if !is_targeted && asteroid.lifespan_timer <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
