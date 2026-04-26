use bevy::prelude::*;
use crate::components::*;

pub fn asteroid_lifecycle_system(
    mut asteroids: Query<(Entity, &mut ActiveAsteroid), Without<Station>>,
    ships: Query<&Ship>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut asteroid) in asteroids.iter_mut() {
        // Decrease lifespan
        asteroid.lifespan_timer -= time.delta_secs();
        
        // Disintegrate if fully depleted
        if asteroid.ore_remaining <= 0.0 {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        // Check if any ship is currently targeting this asteroid
        let is_being_mined = ships.iter().any(|ship| {
            ship.current_mining_target == Some(entity)
        });

        // Disintegrate if lifespan expires AND no ship is mining it
        if !is_being_mined && asteroid.lifespan_timer <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}
