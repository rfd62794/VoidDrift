import type { GameState } from '../types.ts';
import { C } from '../constants.ts';
import { spawnAsteroid } from '../state.ts';

export function updateAsteroids(state: GameState, dt: number): void {
  // Tick lifespans and remove expired/depleted
  state.asteroids = state.asteroids.filter(ast => {
    ast.lifespan -= dt;
    return ast.lifespan > 0 && ast.oreRemaining > 0;
  });

  // Respawn timer
  if (state.asteroids.length < C.MAX_ASTEROIDS) {
    state.asteroidRespawnTimer -= dt;
    if (state.asteroidRespawnTimer <= 0) {
      state.asteroids.push(spawnAsteroid());
      state.asteroidRespawnTimer = C.ASTEROID_RESPAWN_TIMER;
    }
  } else {
    state.asteroidRespawnTimer = C.ASTEROID_RESPAWN_TIMER;
  }
}
