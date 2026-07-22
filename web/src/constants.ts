// Ported from assets/balance.toml and assets/visual.toml

export const C = {
  // Mining
  SHIP_SPEED: 210,
  CARGO_CAPACITY: 100,
  MINING_RATE: 22,
  ARRIVAL_THRESHOLD: 8,
  ARRIVAL_THRESHOLD_MINING: 40,

  // Refinery (10 ore → 1 ingot, 4s per batch tick)
  REFINERY_RATIO: 10,
  IRON_TIME: 4.0,

  // Forge
  HULL_TIME: 5.0,
  HULL_COST_IRON_INGOTS: 2,

  // Drone build — MVP: only Hull Plates required (no Tungsten/Nickel in MVP)
  DRONE_BUILD_TIME: 18.0,
  DRONE_COST_HULLS: 3.0,
  DRONE_MAX_ACTIVE: 5,

  // Asteroids
  ASTEROID_BASE_ORE: 100,
  ASTEROID_MAX_LIFESPAN: 40,
  ASTEROID_RESPAWN_TIMER: 5,
  ASTEROID_RADIUS_IRON: 20,

  // Spawning
  MAX_ASTEROIDS: 3,
  SPAWN_DIST_MIN: 200,
  SPAWN_DIST_MAX: 400,

  // Station visual
  STATION_HUB_RADIUS: 40,
  STATION_ARM_LENGTH: 120,
  STATION_ARM_THICKNESS: 6,
  STATION_BERTH_RADIUS: 22,
  STATION_ROTATION_SPEED: 0.012, // rad/s

  // Colors (as CSS strings)
  COL_BG: '#050A10',
  COL_HUB: '#FFD700',
  COL_ARM: '#3D3D3D',
  COL_BERTH: '#666666',
  COL_DRONE: '#00CC33',
  COL_DRONE_THRUSTER: '#FF4400',
  COL_BEAM: 'rgba(0,255,160,0.55)',
  COL_IRON_AST: '#C06025',
  COL_IRON_AST_DEPLETED: '#2E2E2E',
  COL_CARGO_BAR_BG: '#222222',
  COL_CARGO_BAR_FG: '#00CC33',
  COL_TEXT: '#00CC66',
  COL_DIM: '#4A5A4A',
  COL_HUD_BG: 'rgba(5,8,14,0.97)',
  COL_ACCENT: '#00DDAA',

  // Save key
  SAVE_KEY: 'voiddrift_save_v1',

  // Autosave interval frames (~5s at 60fps)
  AUTOSAVE_INTERVAL: 300,
} as const;
