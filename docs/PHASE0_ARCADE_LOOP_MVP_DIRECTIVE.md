# PHASE 0: Arcade Loop MVP
**Objective:** Convert Voidrift from "station management" to "arcade asteroid tapping with emergent drone scaling"  
**Status:** Ready for Antigravity implementation  
**Estimated time:** 4-6 hours of focused coding + playtesting  
**Risk level:** Medium (new systems, but isolated from existing code)

---

## Overview

After Phase 0, Voidrift will be:
- ✅ Proc-gen asteroids (new one every 5 sec, 30 sec lifespan, max 3 active)
- ✅ Simple tap-to-send UI (tap asteroid → next drone in queue launches)
- ✅ Auto-processing (ore deposits → auto-refines → auto-forges → drones build)
- ✅ Drone queue (visible count, drones auto-return to queue when asteroid done)
- ✅ Multi-cycle mining (if asteroid lasts long enough, drone mines it multiple times before returning)
- ✅ Map shows only: asteroids, station, drone queue count
- ✅ Tab drawer shows: resource totals (iron/tungsten/nickel), drone count

**What gets removed:**
- ❌ All tabs except Resources tab
- ❌ All "assign drone" UI
- ❌ All "manage production queue" UI
- ❌ Station visual complexity (simplified)

**What stays:**
- ✅ Save/load
- ✅ Drones visually mining
- ✅ Station receiving ore
- ✅ Refinery/forge queues (invisible, just auto-process)
- ✅ Drone building (automatic)

---

## New Systems to Add

### 1. Asteroid Spawner System

**Location:** `src/systems/asteroid_spawner.rs` (NEW)

**Responsibility:**
- Spawn new asteroids on timer (every 5 seconds)
- Each asteroid: random ore type, random position, 30 sec lifespan
- Max 3 active asteroids simultaneously
- Despawn when timer expires or when fully mined

**Key Components:**
- `AsteroidSpawnerConfig` (resource: holds timers, spawn interval, max count)
- `ActiveAsteroid` (component: tracks ore type, lifespan, depletion progress)
- `AsteroidSpawnerState` (resource: tracks spawn timer, active count)

**Pseudo-code:**
```rust
pub fn asteroid_spawner_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawner_state: ResMut<AsteroidSpawnerState>,
    config: Res<AsteroidSpawnerConfig>,
    active_asteroids: Query<&ActiveAsteroid>,
) {
    // Tick spawn timer
    spawner_state.spawn_timer.tick(time.delta());
    
    // Check if we can spawn a new asteroid
    if spawner_state.spawn_timer.finished() && active_asteroids.iter().count() < config.max_active {
        // Pick random ore type
        let ore = random_ore_type();
        
        // Pick random position (not too close to station)
        let position = random_position_away_from_station();
        
        // Spawn asteroid entity
        commands.spawn((
            ActiveAsteroid {
                ore_type: ore,
                depletion_timer: Timer::from_seconds(30.0, TimerMode::Once),
                current_depletion: 0.0,
            },
            Transform::from_xyz(position.x, position.y, Z_ASTEROID),
            // ... mesh, material, etc.
        ));
        
        spawner_state.spawn_timer.reset();
    }
}
```

---

### 2. Drone Queue System

**Location:** `src/systems/drone_queue.rs` (NEW)

**Responsibility:**
- Maintain a queue of available drones
- When asteroid is tapped, dequeue next drone and assign it
- Track drone state (available, mining, returning)
- Auto-return drone to queue when mission complete

**Key Components:**
- `DroneQueue` (resource: Vec of available drones, count)
- `DroneAssignment` (component: tracks assigned asteroid, time mining, cycles on same asteroid)

**Pseudo-code:**
```rust
pub fn drone_queue_system(
    mut commands: Commands,
    mut drone_queue: ResMut<DroneQueue>,
    mut assignments: Query<&mut DroneAssignment>,
    asteroids: Query<(Entity, &ActiveAsteroid, &Transform)>,
    mut ships: Query<&mut Ship>,
) {
    // For each drone on assignment:
    for mut assignment in assignments.iter_mut() {
        let asteroid_entity = assignment.target_asteroid;
        
        if let Ok((ent, asteroid, ast_transform)) = asteroids.get(asteroid_entity) {
            // If asteroid still exists, continue mining
            if asteroid.depletion_timer.remaining() > 0.0 {
                // Mining logic (ore accumulates)
                assignment.time_mining += delta_time;
                
                // When mining cycle complete, check:
                // - If asteroid still has ore: start another cycle on same asteroid
                // - If asteroid depleted or timer expired: return to queue
            }
        } else {
            // Asteroid is gone, return drone to queue
            drone_queue.available.push(assignment.drone_id);
            // Remove assignment component
        }
    }
}
```

---

### 3. Asteroid Input System

**Location:** `src/systems/asteroid_input.rs` (NEW)

**Responsibility:**
- Detect when player taps an asteroid
- Dequeue next drone and assign it
- Update drone count in UI

**Pseudo-code:**
```rust
pub fn asteroid_input_system(
    mut commands: Commands,
    mut drone_queue: ResMut<DroneQueue>,
    asteroids: Query<(Entity, &ActiveAsteroid), With<Clickable>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        // Get mouse position, cast ray, find asteroid
        if let Some(asteroid_entity) = find_clicked_asteroid(...) {
            // Dequeue next drone
            if let Some(drone_id) = drone_queue.available.pop() {
                // Spawn drone assignment
                commands.spawn((
                    DroneAssignment {
                        drone_id,
                        target_asteroid: asteroid_entity,
                        time_mining: 0.0,
                    },
                ));
            }
        }
    }
}
```

---

### 4. Auto-Processing System

**Location:** Modify `src/systems/economy.rs` (or create `src/systems/auto_process.rs`)

**Responsibility:**
- When drone returns ore to station, auto-refine it
- When ingots available, auto-forge them
- When products available, auto-build drones
- All invisible, just numbers updating

**Pseudo-code:**
```rust
pub fn auto_refine_system(
    mut station: Query<&mut Station>,
) {
    if let Ok(mut station) = station.get_single_mut() {
        // Convert ore to ingots at fixed rate (per second)
        let refine_rate = 5.0; // ingots per second
        
        let iron_ingots = (station.iron_reserves / REFINERY_RATIO).min(refine_rate * delta_time);
        station.iron_reserves -= iron_ingots * REFINERY_RATIO;
        station.iron_ingots += iron_ingots;
        
        // Same for tungsten, nickel
    }
}

pub fn auto_forge_system(
    mut station: Query<&mut Station>,
) {
    // Similar logic: ingots → products
    // Iron Ingot → Hull
    // Tungsten Ingot → Thruster
    // Nickel Ingot → AI Core
}

pub fn auto_build_drones_system(
    mut commands: Commands,
    mut station: Query<&mut Station>,
    mut drone_queue: ResMut<DroneQueue>,
) {
    if let Ok(mut station) = station.get_single_mut() {
        // If we have all 3 products, build a drone
        while station.hull >= 1 && station.thruster >= 1 && station.ai_core >= 1 {
            station.hull -= 1;
            station.thruster -= 1;
            station.ai_core -= 1;
            
            // Add drone to queue
            drone_queue.available.push(DroneId::new());
        }
    }
}
```

---

## Components to Add

**File:** `src/components.rs`

Add these new component types:

```rust
// Asteroid spawning
#[derive(Resource)]
pub struct AsteroidSpawnerConfig {
    pub spawn_interval: f32,  // 5 seconds
    pub asteroid_lifespan: f32, // 30 seconds
    pub max_active: usize,    // 3 asteroids
}

#[derive(Resource)]
pub struct AsteroidSpawnerState {
    pub spawn_timer: Timer,
}

#[derive(Component)]
pub struct ActiveAsteroid {
    pub ore_type: OreDeposit,
    pub depletion_timer: Timer,
    pub current_depletion: f32,
}

// Drone queue
#[derive(Resource, Clone)]
pub struct DroneQueue {
    pub available: Vec<DroneId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DroneId(u32);

#[derive(Component)]
pub struct DroneAssignment {
    pub drone_id: DroneId,
    pub target_asteroid: Entity,
    pub time_mining: f32,
    pub cycles_on_asteroid: u32,
}

// Resources in station (new fields)
#[derive(Component)]
pub struct Station {
    // ... existing fields ...
    
    // Ore (raw)
    pub iron_ore: f32,
    pub tungsten_ore: f32,
    pub nickel_ore: f32,
    
    // Ingots (refined)
    pub iron_ingots: f32,
    pub tungsten_ingots: f32,
    pub nickel_ingots: f32,
    
    // Products (forged)
    pub hull: u32,
    pub thruster: u32,
    pub ai_core: u32,
}
```

---

## UI Changes

### Main View

**Remove:**
- All tabs except Resources tab
- Drawer open/close complexity
- All production queue rendering

**Keep:**
- Map showing asteroids
- Station at center
- Drone count visible (e.g., "Drones available: 5")

**Add:**
- Clickable asteroids (tap to send drone)
- Asteroid ore type indicator (color/icon showing Iron/Tungsten/Nickel)
- Timer indicator on each asteroid (shows remaining time)

### Resources Tab

**Show:**
```
RESOURCES
Iron Ore:     50.0
Tungsten Ore: 25.0
Nickel Ore:   10.0

Iron Ingots:      5.0
Tungsten Ingots:  2.0
Nickel Ingots:    1.0

Hulls:       1
Thrusters:   0
AI Cores:    0

Drones Available: 5
Drones Mining:    2
```

**Implementation:** Single panel, no complex tabs, just grid layout.

---

## File Structure

**New files to create:**
- `src/systems/asteroid_spawner.rs`
- `src/systems/drone_queue.rs`
- `src/systems/asteroid_input.rs`
- `src/systems/auto_process.rs` (or extend economy.rs)

**Files to modify:**
- `src/components.rs` (add new component types)
- `src/constants.rs` (add spawner config constants)
- `src/systems/hud.rs` (simplify UI, remove tabs)
- `src/systems/setup.rs` (initialize new systems)
- `src/lib.rs` (register new systems, initialize resources)

---

## Constants

**Add to `src/constants.rs`:**

```rust
// Asteroid Spawner Config
pub const ASTEROID_SPAWN_INTERVAL: f32 = 5.0;      // seconds between spawns
pub const ASTEROID_LIFESPAN: f32 = 30.0;           // seconds per asteroid
pub const ASTEROID_MAX_ACTIVE: usize = 3;          // max simultaneous asteroids

// Mining rates (ore per second)
pub const MINING_RATE: f32 = 10.0;                 // ore collected per second
pub const REFINERY_RATE: f32 = 5.0;                // ingots produced per second
pub const FORGE_RATE: f32 = 2.0;                   // products produced per second

// Drone building
pub const DRONE_BUILD_COST_HULL: u32 = 1;
pub const DRONE_BUILD_COST_THRUSTER: u32 = 1;
pub const DRONE_BUILD_COST_AI_CORE: u32 = 1;

// Asteroid spawning positions
pub const ASTEROID_SPAWN_RADIUS_MIN: f32 = 200.0;
pub const ASTEROID_SPAWN_RADIUS_MAX: f32 = 600.0;
pub const ASTEROID_SPAWN_ANGLE_VARIATION: f32 = 360.0;
```

---

## System Registration Order

**In `src/lib.rs`, Update schedule:**

```rust
.add_systems(Update, (
    // Asteroid spawning & management
    systems::asteroid_spawner::asteroid_spawner_system,
    systems::asteroid_input::asteroid_input_system,
    systems::drone_queue::drone_queue_system,
    
    // Mining (drones extract ore from asteroids)
    systems::mining::mining_system, // modify to work with ActiveAsteroid
    
    // Auto-processing (invisible, just updates numbers)
    systems::auto_process::auto_refine_system,
    systems::auto_process::auto_forge_system,
    systems::auto_process::auto_build_drones_system,
    
    // UI (shows current state)
    systems::ui::hud_ui_system,
    
    // Visuals
    systems::visuals::starfield_scroll_system,
    systems::visuals::station_rotation_system,
).chain())
```

---

## Implementation Steps for Antigravity

### STEP 1: Add Components

**File:** `src/components.rs`

Add all new component types listed above (AsteroidSpawnerConfig, ActiveAsteroid, DroneQueue, DroneAssignment).

**Verify:** `cargo check` — should have no errors for new types

---

### STEP 2: Create Asteroid Spawner System

**File:** `src/systems/asteroid_spawner.rs` (NEW)

Implement `asteroid_spawner_system()` that:
- Tracks spawn timer
- Spawns new asteroids every 5 seconds
- Each with random ore type
- Each with 30 sec lifespan
- Max 3 active
- Despawns when timer expires

**Key function:**
```rust
fn random_ore_type() -> OreDeposit {
    let rand_val = rand::random::<f32>();
    match rand_val {
        0.0..0.33 => OreDeposit::Iron,
        0.33..0.66 => OreDeposit::Tungsten,
        _ => OreDeposit::Nickel,
    }
}

fn random_spawn_position() -> Vec2 {
    let angle = rand::random::<f32>() * 360.0_f32.to_radians();
    let distance = rand::random::<f32>() * (ASTEROID_SPAWN_RADIUS_MAX - ASTEROID_SPAWN_RADIUS_MIN) + ASTEROID_SPAWN_RADIUS_MIN;
    Vec2::new(
        angle.cos() * distance,
        angle.sin() * distance,
    )
}
```

**Verify:** `cargo check`

---

### STEP 3: Create Drone Queue System

**File:** `src/systems/drone_queue.rs` (NEW)

Implement systems:
- `drone_queue_input_system()` — tap asteroid, dequeue drone, assign it
- `drone_assignment_system()` — track drone progress, auto-cycle if asteroid alive, return to queue if not

**Key logic:**
```rust
pub fn drone_assignment_system(
    mut commands: Commands,
    mut assignments: Query<(Entity, &mut DroneAssignment)>,
    asteroids: Query<(Entity, &ActiveAsteroid)>,
    mut drone_queue: ResMut<DroneQueue>,
) {
    for (assignment_entity, mut assignment) in assignments.iter_mut() {
        if let Ok((ast_entity, asteroid)) = asteroids.get(assignment.target_asteroid) {
            // Asteroid still exists, keep mining
            if asteroid.depletion_timer.finished() {
                // Asteroid depleted, return drone to queue
                drone_queue.available.push(assignment.drone_id);
                commands.entity(assignment_entity).despawn();
            }
        } else {
            // Asteroid gone, return drone to queue
            drone_queue.available.push(assignment.drone_id);
            commands.entity(assignment_entity).despawn();
        }
    }
}
```

**Verify:** `cargo check`

---

### STEP 4: Create Asteroid Input System

**File:** `src/systems/asteroid_input.rs` (NEW)

Implement `asteroid_input_system()` that:
- Detects mouse click on asteroid
- Dequeues next drone
- Spawns DroneAssignment component
- Updates UI drone count

**Verify:** `cargo check`

---

### STEP 5: Create Auto-Process System

**File:** `src/systems/auto_process.rs` (NEW)

Implement three systems:
1. `auto_refine_system()` — Ore → Ingots (per second)
2. `auto_forge_system()` — Ingots → Products (per second)
3. `auto_build_drones_system()` — Products → Drones (automatic when available)

**Key rates from constants:**
- Refinery: 5 ingots/sec
- Forge: 2 products/sec
- Drones: instant when 3 products available

**Verify:** `cargo check`

---

### STEP 6: Modify mining_system to Work with ActiveAsteroid

**File:** `src/systems/mining.rs`

Change from mining static asteroids to:
- Query `(DroneAssignment, Ship)` pairs
- For each assignment, find its ActiveAsteroid
- Extract ore at MINING_RATE per second
- When drone returns to station, deposit ore to Station

**Key change:**
```rust
pub fn mining_system(
    mut assignments: Query<(&mut DroneAssignment, &mut Ship)>,
    mut asteroids: Query<&mut ActiveAsteroid>,
    mut station: Query<&mut Station>,
    time: Res<Time>,
) {
    for (mut assignment, mut ship) in assignments.iter_mut() {
        if let Ok(mut asteroid) = asteroids.get_mut(assignment.target_asteroid) {
            // Mine ore from asteroid
            let ore_extracted = MINING_RATE * time.delta_secs();
            ship.cargo = (ship.cargo + ore_extracted).min(ship.cargo_capacity as f32);
            
            // Deplete asteroid
            asteroid.current_depletion += ore_extracted;
            
            // When ship full or asteroid depleted, return to station
            if ship.cargo >= ship.cargo_capacity as f32 || asteroid.current_depletion >= 1000.0 {
                // Deposit ore to station based on ship.cargo_type
                if let Ok(mut station) = station.get_single_mut() {
                    match ship.cargo_type {
                        OreDeposit::Iron => station.iron_ore += ship.cargo,
                        OreDeposit::Tungsten => station.tungsten_ore += ship.cargo,
                        OreDeposit::Nickel => station.nickel_ore += ship.cargo,
                    }
                }
                ship.cargo = 0.0;
            }
        }
    }
}
```

**Verify:** `cargo check`

---

### STEP 7: Simplify UI

**File:** `src/systems/hud.rs`

**Delete:**
- All tabs except Resources
- All tab switching logic
- Power rendering
- Production queue UI
- Drawer state machine complexity

**Keep:**
- Resources grid display
- Station visual

**Add:**
- Drone queue count display (e.g., "Drones: 5")
- Asteroid count display (e.g., "Active: 2/3")

**Verify:** `cargo check`

---

### STEP 8: Update Constants

**File:** `src/constants.rs`

Add asteroid spawner, mining rate, refinery rate, forge rate constants (see Constants section above).

**Verify:** `cargo check`

---

### STEP 9: Initialize New Systems and Resources

**File:** `src/lib.rs`

In `main()` function:

```rust
.insert_resource(AsteroidSpawnerConfig {
    spawn_interval: ASTEROID_SPAWN_INTERVAL,
    asteroid_lifespan: ASTEROID_LIFESPAN,
    max_active: ASTEROID_MAX_ACTIVE,
})
.insert_resource(AsteroidSpawnerState {
    spawn_timer: Timer::from_seconds(ASTEROID_SPAWN_INTERVAL, TimerMode::Repeating),
})
.insert_resource(DroneQueue {
    available: vec![DroneId(1), DroneId(2), DroneId(3)], // Start with 3 drones
})
```

Register new systems in Update schedule (see System Registration Order above).

**Verify:** `cargo check`

---

### STEP 10: Update Station Initialization

**File:** `src/systems/setup.rs`

In `spawn_station()`, add new fields:

```rust
Station {
    // ... existing fields ...
    iron_ore: 0.0,
    tungsten_ore: 0.0,
    nickel_ore: 0.0,
    iron_ingots: 0.0,
    tungsten_ingots: 0.0,
    nickel_ingots: 0.0,
    hull: 0,
    thruster: 0,
    ai_core: 0,
}
```

**Verify:** `cargo check`

---

## Playtesting Checklist

After all steps compile, test on device:

- [ ] Game starts, map visible
- [ ] Asteroids spawn every 5 seconds (watch them appear)
- [ ] Max 3 asteroids active at once
- [ ] Can tap asteroid (drone launches, flies to it)
- [ ] Drone mines ore visibly
- [ ] Ore accumulates in Station (watch numbers in Resources tab)
- [ ] Ore auto-refines to ingots
- [ ] Ingots auto-forge to products
- [ ] When 3 products available, drone auto-builds
- [ ] New drone appears in queue
- [ ] Drone queue count updates correctly
- [ ] Multiple drones can be assigned simultaneously
- [ ] If asteroid lasts long enough, drone cycles on it multiple times
- [ ] When asteroid depletes, drone returns to queue
- [ ] Can tap same asteroid multiple times if drone hasn't finished
- [ ] Save/load still works (CONTINUE button)

---

## Tuning Knobs (After Playtesting)

Once arcade loop works, adjust these constants for feel:

**Mining speed:**
- `MINING_RATE` — How much ore per second (currently 10.0)
- Test: Too fast = trivial, Too slow = tedious

**Ship speed:**
- `SHIP_SPEED` in existing constants
- Test: How long to reach asteroid? Should feel snappy

**Refinery/Forge speed:**
- `REFINERY_RATE` (ingots/sec, currently 5.0)
- `FORGE_RATE` (products/sec, currently 2.0)
- Test: Is ore piling up? Are products scarce?

**Asteroid spawning:**
- `ASTEROID_SPAWN_INTERVAL` (5 seconds) — How often new asteroids appear
- `ASTEROID_LIFESPAN` (30 seconds) — How long they last
- Test: Is there always something to tap? Or is it chaotic?

---

## Rollback Plan

If something breaks:

```bash
git status
git diff src/ | head -100  # See what changed
git checkout -- src/  # Revert all
```

Then re-read the failing step and debug.

---

## You're Ready

This is the core of the arcade loop. Once working:
- Game has emergent gameplay (proc-gen asteroids create unique challenges each session)
- Simple UI (just tap to send)
- Satisfying feedback loop (resources accumulate, drones build, scaling happens)

**Next steps (after playtesting):**
- Phase 1: Refactor code organization (if still interested)
- Phase 2: Add complexity (upgrades, disaster events, deeper progression)

But first: **Build it, test it, feel it.**

Good luck.
