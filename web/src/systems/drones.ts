import type { GameState, Drone, Asteroid } from '../types.ts';
import { C } from '../constants.ts';

function dist(ax: number, ay: number, bx: number, by: number): number {
  const dx = bx - ax;
  const dy = by - ay;
  return Math.sqrt(dx * dx + dy * dy);
}

function moveToward(
  drone: Drone,
  tx: number,
  ty: number,
  speed: number,
  dt: number,
): boolean {
  const dx = tx - drone.x;
  const dy = ty - drone.y;
  const d = Math.sqrt(dx * dx + dy * dy);
  if (d < 1) return true; // already there

  drone.headingAngle = Math.atan2(dy, dx);

  const step = speed * dt;
  if (step >= d) {
    drone.x = tx;
    drone.y = ty;
    return true;
  }
  drone.x += (dx / d) * step;
  drone.y += (dy / d) * step;
  return false;
}

function pickNearestAsteroid(drone: Drone, asteroids: Asteroid[]): Asteroid | null {
  let best: Asteroid | null = null;
  let bestD = Infinity;
  for (const ast of asteroids) {
    if (ast.oreRemaining <= 0) continue;
    const d = dist(drone.x, drone.y, ast.x, ast.y);
    if (d < bestD) {
      bestD = d;
      best = ast;
    }
  }
  return best;
}

export function updateDrones(state: GameState, dt: number): void {
  for (const drone of state.drones) {
    switch (drone.state) {
      case 'Holding': {
        // Snap to station
        drone.x = 0;
        drone.y = 0;
        // Pick an asteroid and depart
        const target = pickNearestAsteroid(drone, state.asteroids);
        if (target) {
          drone.targetAsteroidId = target.id;
          drone.targetX = target.x;
          drone.targetY = target.y;
          drone.state = 'Outbound';
        }
        break;
      }

      case 'Outbound': {
        // Refresh target position (asteroid doesn't move but might have despawned)
        const targetAst = state.asteroids.find(a => a.id === drone.targetAsteroidId);
        if (!targetAst || targetAst.oreRemaining <= 0) {
          // Retarget or return home
          const newTarget = pickNearestAsteroid(drone, state.asteroids);
          if (newTarget) {
            drone.targetAsteroidId = newTarget.id;
            drone.targetX = newTarget.x;
            drone.targetY = newTarget.y;
          } else {
            drone.state = 'Returning';
            break;
          }
        } else {
          drone.targetX = targetAst.x;
          drone.targetY = targetAst.y;
        }

        const arrived = dist(drone.x, drone.y, drone.targetX, drone.targetY) < C.ARRIVAL_THRESHOLD_MINING;
        if (arrived) {
          drone.state = 'Mining';
        } else {
          moveToward(drone, drone.targetX, drone.targetY, C.SHIP_SPEED, dt);
        }
        break;
      }

      case 'Mining': {
        const ast = state.asteroids.find(a => a.id === drone.targetAsteroidId);
        if (!ast || ast.oreRemaining <= 0) {
          // Asteroid depleted — find another or return
          const newTarget = pickNearestAsteroid(drone, state.asteroids);
          if (newTarget && drone.cargo < C.CARGO_CAPACITY) {
            drone.targetAsteroidId = newTarget.id;
            drone.targetX = newTarget.x;
            drone.targetY = newTarget.y;
            drone.state = 'Outbound';
          } else {
            drone.state = 'Returning';
          }
          break;
        }

        // Snap drone onto asteroid surface
        const d = dist(drone.x, drone.y, ast.x, ast.y);
        if (d > C.ARRIVAL_THRESHOLD_MINING) {
          // Drifted — move back
          moveToward(drone, ast.x, ast.y, C.SHIP_SPEED, dt);
          break;
        }

        const mined = Math.min(C.MINING_RATE * dt, ast.oreRemaining, C.CARGO_CAPACITY - drone.cargo);
        ast.oreRemaining -= mined;
        drone.cargo += mined;
        state.totalOreHarvested += mined;

        if (drone.cargo >= C.CARGO_CAPACITY) {
          drone.state = 'Returning';
        }
        break;
      }

      case 'Returning': {
        const atStation = dist(drone.x, drone.y, 0, 0) < C.ARRIVAL_THRESHOLD;
        if (atStation) {
          drone.state = 'Unloading';
        } else {
          moveToward(drone, 0, 0, C.SHIP_SPEED, dt);
        }
        break;
      }

      case 'Unloading': {
        state.ironOre += drone.cargo;
        drone.cargo = 0;
        drone.targetAsteroidId = null;
        drone.state = 'Holding';
        break;
      }
    }
  }
}
