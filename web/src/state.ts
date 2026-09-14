import type { GameState, Drone, Asteroid } from './types.ts';
import { C } from './constants.ts';

let _droneId = 0;
let _asteroidId = 0;

export function makeId() {
  return ++_droneId;
}

export function makeAsteroidId() {
  return ++_asteroidId;
}

function randomPolyVerts(radius: number, count: number): Array<{ x: number; y: number }> {
  const verts: Array<{ x: number; y: number }> = [];
  for (let i = 0; i < count; i++) {
    const angle = (i / count) * Math.PI * 2 + (Math.random() - 0.5) * 0.6;
    const r = radius * (0.7 + Math.random() * 0.55);
    verts.push({ x: Math.cos(angle) * r, y: Math.sin(angle) * r });
  }
  return verts;
}

export function spawnDrone(x = 0, y = 0): Drone {
  return {
    id: makeId(),
    state: 'Holding',
    x,
    y,
    cargo: 0,
    targetAsteroidId: null,
    targetX: 0,
    targetY: 0,
    headingAngle: -Math.PI / 2,
  };
}

export function spawnAsteroid(): Asteroid {
  const angle = Math.random() * Math.PI * 2;
  const dist = C.SPAWN_DIST_MIN + Math.random() * (C.SPAWN_DIST_MAX - C.SPAWN_DIST_MIN);
  return {
    id: makeAsteroidId(),
    x: Math.cos(angle) * dist,
    y: Math.sin(angle) * dist,
    oreRemaining: C.ASTEROID_BASE_ORE,
    maxOre: C.ASTEROID_BASE_ORE,
    lifespan: C.ASTEROID_MAX_LIFESPAN,
    polyVerts: randomPolyVerts(C.ASTEROID_RADIUS_IRON, 10),
  };
}

export function createFreshState(): GameState {
  const drone = spawnDrone();
  return {
    drones: [drone],
    asteroids: [],
    _nextDroneId: 1,
    _nextAsteroidId: 0,

    ironOre: 0,
    ironIngots: 0,
    hullPlates: 0,

    refineryProgress: 0,
    forgeProgress: 0,
    droneBuildProgress: 0,

    stationRotation: 0,

    asteroidRespawnTimer: 0,

    totalDronesBuilt: 1,
    totalOreHarvested: 0,
    tick: 0,
    elapsedSeconds: 0,
  };
}
