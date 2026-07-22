export type DroneState = 'Holding' | 'Outbound' | 'Mining' | 'Returning' | 'Unloading';

export interface Drone {
  id: number;
  state: DroneState;
  x: number;
  y: number;
  cargo: number;
  targetAsteroidId: number | null;
  targetX: number;
  targetY: number;
  headingAngle: number; // radians, for rendering
}

export interface Asteroid {
  id: number;
  x: number;
  y: number;
  oreRemaining: number;
  maxOre: number;
  lifespan: number;
  // Pre-generated polygon vertices (relative to asteroid center)
  polyVerts: Array<{ x: number; y: number }>;
}

export interface GameState {
  // Entities
  drones: Drone[];
  asteroids: Asteroid[];
  _nextDroneId: number;
  _nextAsteroidId: number;

  // Station resources
  ironOre: number;
  ironIngots: number;
  hullPlates: number;

  // Production accumulators (fractional, continuous)
  refineryProgress: number; // 0..1 accumulator
  forgeProgress: number;    // 0..1 accumulator
  droneBuildProgress: number; // 0..1 accumulator

  // Station rotation
  stationRotation: number; // radians

  // Timers
  asteroidRespawnTimer: number;

  // Stats
  totalDronesBuilt: number;
  totalOreHarvested: number;
  tick: number;
  elapsedSeconds: number;
}

// What we persist to localStorage (all numbers/arrays)
export interface SaveData {
  version: number;
  ironOre: number;
  ironIngots: number;
  hullPlates: number;
  refineryProgress: number;
  forgeProgress: number;
  droneBuildProgress: number;
  totalDronesBuilt: number;
  totalOreHarvested: number;
  elapsedSeconds: number;
  droneCount: number; // just the count — drones respawn at station
}
