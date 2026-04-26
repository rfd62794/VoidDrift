# Phase 1c: Asteroid Lifecycle System Directive
**Agent:** Antigravity (Windsurf + Claude/Gemini)  
**Objective:** Implement asteroid lifespan, three-stage breakup, auto-retargeting, and respawn timer  
**Status:** Ready for implementation  
**Estimated time:** 3-4 hours  
**Target:** Working on device by end of session

---

## Overview

VoidDrift's core loop is working (mine → refine → build drones). Phase 1c adds **dynamic asteroid lifecycle** to create urgency and discovery.

**Mechanic:**
- Asteroids spawn at distance, spiral inward toward event horizon
- Three degradation stages: Large → Medium → Small
- When piece breaks, ship auto-targets nearest new piece
- Small pieces disintegrate and disappear
- New asteroids respawn on timer (constant supply)

**Result:** Mining feels alive. Time pressure. Natural respawning.

---

## Part 1: Component Updates

### ActiveAsteroid Component

**File:** `src/components.rs`

**Current (approximate):**
```rust
#[derive(Component)]
pub struct ActiveAsteroid {
    pub ore_type: OreDeposit,
    // other fields
}
```

**Update to:**
```rust
#[derive(Component)]
pub struct ActiveAsteroid {
    pub ore_type: OreDeposit,
    pub ore_remaining: f32,           // Ore count (floats fine)
    pub lifespan_timer: f32,          // Time until disintegration (seconds)
    pub max_lifespan: f32,            // Total lifespan (from spawn)
    pub stage: AsteroidStage,         // Large, Medium, Small
    pub is_spiraling: bool,           // Orbiting inward
}

#[derive(Clone, Copy, PartialEq)]
pub enum AsteroidStage {
    Large,
    Medium,
    Small,
}
```

**Rationale:** Track lifecycle state, degradation progress, stage for visual/mechanical feedback.

---

## Part 2: Constants

### Asteroid Lifecycle Constants

**File:** `src/constants.rs`

**Add:**
```rust
// Asteroid Lifecycle
pub const ASTEROID_MAX_LIFESPAN_SECS: f32 = 120.0;      // 2 minutes before disintegration
pub const ASTEROID_BREAKUP_LARGE_THRESHOLD: f32 = 0.66; // Break at 66% ore remaining
pub const ASTEROID_BREAKUP_MEDIUM_THRESHOLD: f32 = 0.33; // Break at 33% ore remaining
pub const ASTEROID_RESPAWN_TIMER_SECS: f32 = 5.0;       // New asteroid every 5 seconds
pub const ASTEROID_SPIRAL_SPEED: f32 = 10.0;            // Pixels per second moving inward
pub const ASTEROID_BREAKUP_SPREAD: f32 = 50.0;          // How far apart new pieces spawn
pub const ASTEROID_LARGE_ORE: f32 = 100.0;              // Large piece ore amount
pub const ASTEROID_MEDIUM_ORE: f32 = 50.0;              // Medium piece ore amount
pub const ASTEROID_SMALL_ORE: f32 = 25.0;               // Small piece ore amount
```

**Tuning notes:**
- Lifespan: 120s means player has clear window but not infinite
- Breakup thresholds: Create 3 distinct mining phases
- Respawn: 5s keeps steady stream
- Spiral speed: Visual effect, adjust for feel

---

## Part 3: Asteroid Spawning (Update setup.rs)

### Asteroid Spawn System

**File:** `src/systems/setup.rs` (or new `src/systems/asteroid_spawn.rs`)

**Changes:**

```rust
pub fn spawn_initial_asteroids(
    mut commands: Commands,
    mut respawn_timer: ResMut<AsteroidRespawnTimer>,
) {
    // Spawn initial asteroid at spawn distance
    spawn_asteroid(&mut commands, AsteroidStage::Large, true);
    respawn_timer.timer = Timer::from_seconds(ASTEROID_RESPAWN_TIMER_SECS, TimerMode::Once);
}

fn spawn_asteroid(
    commands: &mut Commands,
    stage: AsteroidStage,
    is_initial: bool,
) {
    let (ore_amount, position) = match stage {
        AsteroidStage::Large => (ASTEROID_LARGE_ORE, Vec3::new(-500.0, 0.0, 0.0)), // Spawn far left
        AsteroidStage::Medium => (ASTEROID_MEDIUM_ORE, Vec3::new(-250.0, rand(-50..50), 0.0)),
        AsteroidStage::Small => (ASTEROID_SMALL_ORE, Vec3::new(-100.0, rand(-30..30), 0.0)),
    };
    
    commands.spawn((
        ActiveAsteroid {
            ore_type: OreDeposit::Iron, // or random/varies
            ore_remaining: ore_amount,
            lifespan_timer: ASTEROID_MAX_LIFESPAN_SECS,
            max_lifespan: ASTEROID_MAX_LIFESPAN_SECS,
            stage,
            is_spiraling: true,
        },
        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        // ... other components
    ));
}
```

**Key points:**
- Large spawns at distance (left side)
- Medium/Small spawn closer (for clarity, or as breakup)
- Random Y position (variety)
- All spawn with full ore amount for their stage

---

## Part 4: Asteroid Lifecycle System

### New System: Asteroid Degradation

**File:** `src/systems/asteroid_lifecycle.rs` (new file)

```rust
use bevy::prelude::*;
use crate::components::*;
use crate::constants::*;

pub fn asteroid_lifecycle_system(
    mut asteroids: Query<(&mut ActiveAsteroid, &mut Transform), Without<Station>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut asteroid, mut transform) in asteroids.iter_mut() {
        // Decrease lifespan
        asteroid.lifespan_timer -= time.delta_secs();
        
        // Spiral inward (move toward center)
        transform.translation.x += ASTEROID_SPIRAL_SPEED * time.delta_secs();
        
        // Check for breakup thresholds
        let ore_ratio = asteroid.ore_remaining / match asteroid.stage {
            AsteroidStage::Large => ASTEROID_LARGE_ORE,
            AsteroidStage::Medium => ASTEROID_MEDIUM_ORE,
            AsteroidStage::Small => ASTEROID_SMALL_ORE,
        };
        
        // Breakup logic
        if asteroid.stage == AsteroidStage::Large && ore_ratio < ASTEROID_BREAKUP_LARGE_THRESHOLD {
            asteroid_break_into_mediums(&mut commands, &transform, asteroid.ore_type);
            // Mark for despawn (don't despawn yet, let it finish)
            asteroid.stage = AsteroidStage::Large; // Keep tracking until mined out
        } else if asteroid.stage == AsteroidStage::Medium && ore_ratio < ASTEROID_BREAKUP_MEDIUM_THRESHOLD {
            asteroid_break_into_smalls(&mut commands, &transform, asteroid.ore_type);
        }
        
        // Disintegrate when lifespan expires
        if asteroid.lifespan_timer <= 0.0 {
            commands.entity(asteroid.entity()).despawn(); // Remove asteroid
        }
    }
}

fn asteroid_break_into_mediums(
    commands: &mut Commands,
    parent_transform: &Transform,
    ore_type: OreDeposit,
) {
    for i in 0..2 {
        let offset = ASTEROID_BREAKUP_SPREAD * (i as f32 - 0.5);
        let new_pos = parent_transform.translation + Vec3::new(0.0, offset, 0.0);
        
        commands.spawn((
            ActiveAsteroid {
                ore_type,
                ore_remaining: ASTEROID_MEDIUM_ORE,
                lifespan_timer: ASTEROID_MAX_LIFESPAN_SECS,
                max_lifespan: ASTEROID_MAX_LIFESPAN_SECS,
                stage: AsteroidStage::Medium,
                is_spiraling: true,
            },
            Transform::from_translation(new_pos),
            // ... other components
        ));
    }
}

fn asteroid_break_into_smalls(
    commands: &mut Commands,
    parent_transform: &Transform,
    ore_type: OreDeposit,
) {
    for i in 0..2 {
        let offset = ASTEROID_BREAKUP_SPREAD * 0.5 * (i as f32 - 0.5);
        let new_pos = parent_transform.translation + Vec3::new(0.0, offset, 0.0);
        
        commands.spawn((
            ActiveAsteroid {
                ore_type,
                ore_remaining: ASTEROID_SMALL_ORE,
                lifespan_timer: ASTEROID_MAX_LIFESPAN_SECS,
                max_lifespan: ASTEROID_MAX_LIFESPAN_SECS,
                stage: AsteroidStage::Small,
                is_spiraling: true,
            },
            Transform::from_translation(new_pos),
            // ... other components
        ));
    }
}
```

---

## Part 5: Asteroid Respawn System

### Respawn Timer Resource

**File:** `src/components.rs`

**Add:**
```rust
#[derive(Resource)]
pub struct AsteroidRespawnTimer {
    pub timer: Timer,
}
```

### Respawn System

**File:** `src/systems/asteroid_spawn.rs`

```rust
pub fn asteroid_respawn_system(
    mut respawn_timer: ResMut<AsteroidRespawnTimer>,
    mut commands: Commands,
    time: Res<Time>,
    asteroids: Query<&ActiveAsteroid>,
) {
    respawn_timer.timer.tick(time.delta());
    
    if respawn_timer.timer.finished() {
        // Spawn new asteroid
        spawn_asteroid(&mut commands, AsteroidStage::Large, false);
        respawn_timer.timer.reset();
    }
}
```

---

## Part 6: Mining System Updates

### Mining Auto-Retarget

**File:** `src/systems/mining.rs`

**Current logic (approximate):**
```rust
if ship.state == ShipState::Mining {
    // Extract ore
    let ore_extracted = MINING_RATE * time.delta_secs();
    ship.cargo = (ship.cargo + ore_extracted).min(ship.cargo_capacity as f32);
    
    // When cargo full, return
    if ship.cargo >= ship.cargo_capacity as f32 {
        ship.state = ShipState::ReturningToStation;
    }
}
```

**Update to:**
```rust
if ship.state == ShipState::Mining {
    // Get current mining target
    if let Some(target_entity) = get_mining_target(&ship) {
        if let Ok(mut asteroid) = asteroids.get_mut(target_entity) {
            // Extract ore
            let ore_extracted = MINING_RATE * time.delta_secs();
            ship.cargo = (ship.cargo + ore_extracted).min(ship.cargo_capacity as f32);
            asteroid.ore_remaining -= ore_extracted;
            
            // If target depleted or disintegrated, retarget
            if asteroid.ore_remaining <= 0.0 {
                // Find nearest new asteroid
                if let Some(new_target) = find_nearest_asteroid(&ship_transform, &asteroids) {
                    ship.current_mining_target = new_target;
                    // Continue mining (no state change)
                } else {
                    // No asteroids, return to station
                    ship.state = ShipState::ReturningToStation;
                }
            }
            
            // Cargo full, return
            if ship.cargo >= ship.cargo_capacity as f32 {
                ship.state = ShipState::ReturningToStation;
            }
        }
    }
}
```

**Key change:**
- When asteroid depletes, find nearest one
- Auto-retarget without state change
- Mining continues smoothly

---

## Part 7: System Registration

### lib.rs Updates

**File:** `src/lib.rs`

**Add systems to schedule:**
```rust
.add_systems(Update, (
    // Asteroid systems
    asteroid_spawn::asteroid_respawn_system,
    asteroid_lifecycle::asteroid_lifecycle_system,
    
    // Existing systems
    mining::mining_system,
    autopilot::autopilot_system,
    // ... rest
).chain())
```

**Order matters:**
1. Asteroid lifecycle (despawns, breakups)
2. Mining (gets new targets)
3. Autopilot (targets are stable)

---

## Part 8: Visual Feedback (Optional for Phase 1c)

### Asteroid Visual Updates

**Current:** Static asteroid mesh

**Can add (if time):**
- Color shift (stable → orange → red as lifespan expires)
- Scale shrink (visual disintegration)
- Spiral animation (offset movement inward)

**Not required for Phase 1c.** Core mechanic first.

---

## Part 9: Testing Checklist

### Build & Device Test

- [ ] Code compiles cleanly
- [ ] APK builds successfully
- [ ] Game boots without crashes
- [ ] Asteroids spawn at distance
- [ ] Asteroids spiral inward visually
- [ ] Mining laser targets asteroids
- [ ] Asteroid breaks into smaller pieces at thresholds
- [ ] Ship auto-retargets when piece breaks
- [ ] Mining continues smoothly through breakups
- [ ] Small pieces disintegrate naturally
- [ ] New asteroids respawn on timer
- [ ] No infinite loops or hangs
- [ ] Play for 30+ minutes
  - [ ] Feel natural (not too fast, not too slow)
  - [ ] No crashes
  - [ ] Progression makes sense

### Balance Tuning (After working)

- [ ] Lifespan (120s too long/short?)
- [ ] Respawn timer (5s too frequent?)
- [ ] Breakup thresholds (66%/33% feel right?)
- [ ] Spiral speed (visual pleasing?)
- [ ] Ore amounts (progression balanced?)

---

## Part 10: Acceptance Criteria

**Phase 1c Complete when:**
- [ ] Asteroids have full lifecycle (spawn → spiral → break → disintegrate)
- [ ] Three stages working (Large → Medium → Small)
- [ ] Ship auto-retargets when pieces break
- [ ] Respawn creates constant supply
- [ ] No crashes or infinite loops
- [ ] Plays naturally for 30+ minutes
- [ ] Device testing passes
- [ ] Commit ready with clean message

---

## Implementation Strategy

### Session Approach

1. **Start:** Update components.rs (add ActiveAsteroid fields)
2. **Test:** `cargo check` after each major change
3. **Implement:** Create asteroid_lifecycle.rs system
4. **Implement:** Create asteroid_spawn.rs respawn system
5. **Update:** Mining.rs auto-retarget logic
6. **Register:** Add systems to lib.rs
7. **Build:** `./build_android.ps1`
8. **Test:** Play on device 30 minutes
9. **Tune:** Adjust constants for feel
10. **Commit:** Clean commit message

---

## Commit Message (Final)

```
feat: Phase 1c - asteroid lifecycle system

- Add lifespan timer to asteroids (120s before disintegration)
- Implement three-stage breakup: Large → Medium → Small
- New stage field tracks degradation progress
- Asteroids spiral inward toward event horizon
- Ship auto-retargets when piece breaks or depletes
- New respawn system (5s timer) creates constant supply
- Mining continues smoothly through breakups
- Tested on device: 30+ min gameplay loop

Constants tuned for feel:
- Lifespan: 120 seconds
- Breakup thresholds: 66% → 33%
- Respawn: 5 seconds
- Spiral speed: 10 px/sec
- Ore amounts: Large 100, Medium 50, Small 25

Mechanics:
- Large asteroid has 100 ore, breaks at 66% remaining
- Medium has 50 ore, breaks at 33%
- Small has 25 ore, disintegrates when gone or lifespan expires
- Ship always targets nearest unmined piece
```

---

## Success Indicators

**When you're done:**
- Asteroids feel alive (they degrade, break, disappear)
- Time pressure exists (asteroids vanish, creates urgency)
- Mining is engaging (watch pieces break, ship retargets)
- Respawn is natural (never empty, never overwhelming)
- Progression to Phase 2 is clear (now need upgrades to increase capacity)

---

## Notes for Antigravity

- Use Claude for architecture questions, Gemini for debugging
- Ask if uncertain about system ordering or query patterns
- Device testing is critical (feel matters more than perfection)
- Constants are tuning knobs, not locked (adjust after testing)
- If something feels off, iterate rather than overthinking

---

**This is Phase 1c. Build it. Test it. Ship it.**

**Then refactor, then Phase 2.**

**Go.**
