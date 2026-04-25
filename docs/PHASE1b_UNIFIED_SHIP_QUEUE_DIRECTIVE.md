# PHASE 1b: Unified Ship Queue System
**Objective:** Remove PlayerShip component, unify all ships into one queue system, ships return to queue after mining one asteroid  
**Status:** Ready for implementation  
**Estimated time:** 2-3 hours  
**Risk level:** Medium (ship logic refactor)

---

## Overview

After Phase 1b:
- ✅ No PlayerShip component (all ships identical)
- ✅ Opening sequence tags Ship #1 with `InOpeningSequence`
- ✅ When opening ends, tag removed, Ship #1 joins queue
- ✅ All ships use identical mining/docking/unloading logic
- ✅ Click asteroid = assign next available ship
- ✅ Ship mines, returns to station, unloads, goes back to queue
- ✅ One-shot cycle: Mine → Dock → Queue (ready for next assignment)

---

## Architecture Changes

### Current State
- `PlayerShip` component marks the player's ship
- Separate opening sequence logic for player ship
- Drones in a queue, player ship separate
- Ships don't reliably return to queue

### New State
- No `PlayerShip` component
- `InOpeningSequence` tag marks the ship doing intro
- All ships in one unified queue
- When mining completes, ship automatically returns to queue
- Opening sequence uses Ship #1, removes tag when done

---

## Changes Required

### 1. Update components.rs — Remove PlayerShip, Add InOpeningSequence Tag

**File:** `src/components.rs`

**Find and DELETE:**
```rust
#[derive(Component)]
pub struct PlayerShip;
```

**Add new tag:**
```rust
#[derive(Component)]
pub struct InOpeningSequence;
```

**Verify Ship struct has all needed fields:**
```rust
#[derive(Component)]
pub struct Ship {
    pub state: ShipState,
    pub speed: f32,
    pub cargo: f32,
    pub cargo_type: OreDeposit,
    pub cargo_capacity: u32,
    pub laser_tier: LaserTier,
}
```

(No changes needed to Ship itself, just remove PlayerShip)

---

### 2. Update setup.rs — Remove PlayerShip Spawning

**File:** `src/systems/setup.rs`

**Find spawn_player_ship function:**

**Current:**
```rust
pub fn spawn_player_ship(...) {
    commands.spawn((
        PlayerShip,  // ← REMOVE THIS LINE
        Ship { ... },
        // ... other components
    ));
}
```

**Update to:**
```rust
pub fn spawn_player_ship(...) {
    commands.spawn((
        InOpeningSequence,  // ← ADD THIS
        Ship { ... },
        // ... other components
    ));
}
```

(That's it — one tag swap)

---

### 3. Update opening_sequence.rs — Use InOpeningSequence Tag

**File:** `src/systems/opening_sequence.rs`

**Find all references to PlayerShip:**

**Old:**
```rust
let mut player_query = Query<(&mut Transform, &mut Ship), With<PlayerShip>>;
```

**New:**
```rust
let mut player_query = Query<(&mut Transform, &mut Ship), With<InOpeningSequence>>;
```

**When opening sequence ends, REMOVE the tag:**

**At the end of opening_sequence_system (or wherever you mark it complete):**

```rust
// Mark sequence complete
opening_phase.phase = OpeningPhase::Complete;

// Remove the InOpeningSequence tag so ship joins queue
commands.entity(player_ship_entity).remove::<InOpeningSequence>();
```

(This automatically makes Ship #1 available for queue assignment)

---

### 4. Update autopilot.rs — Unify Docking Logic

**File:** `src/systems/autopilot.rs`

**Goal:** Make all ships dock the same way (no special player ship logic)

**Find the docking sequence:**

**Current (approximate):**
```rust
if let Ok((_station_ent, mut station, _)) = station_query.get_single_mut() {
    // Unload cargo
    match ship.cargo_type {
        OreDeposit::Iron => station.iron_ore += ship.cargo,
        OreDeposit::Tungsten => station.tungsten_ore += ship.cargo,
        OreDeposit::Nickel => station.nickel_ore += ship.cargo,
    }
    ship.cargo = 0.0;
    
    ship.state = ShipState::Docked;
    // ... rest of docking logic
}
```

**Make sure this logic is identical for all ships. If there's PlayerShip-specific code, remove it:**

```rust
// BAD (player ship special):
if has_component::<PlayerShip>(entity) {
    // Do something special
} else {
    // Drones do something else
}
```

**Should become:**
```rust
// GOOD (all ships identical):
// Same logic for everyone
```

**After docking/unloading, ship should return to queue automatically.**

---

### 5. Create/Update drone_queue.rs — Make Queue Accept All Ships

**File:** `src/systems/drone_queue.rs`

**The queue system should:**
- Track all ships (not just drones)
- When ship docks, mark it as available
- When player clicks asteroid, assign next available ship

**Key logic:**

```rust
pub struct ShipQueue {
    pub available_ships: Vec<Entity>,  // All ships waiting for assignment
    pub assigned_ship: Option<Entity>, // Currently mining
}

pub fn ship_queue_system(
    mut queue: ResMut<ShipQueue>,
    mut ships: Query<(&mut Ship, &Transform)>,
    asteroids: Query<(Entity, &ActiveAsteroid)>,
) {
    // If currently assigned ship is done mining, return it to queue
    if let Some(ship_entity) = queue.assigned_ship {
        if let Ok((ship, _)) = ships.get(ship_entity) {
            if ship.state == ShipState::Docked {
                // Ship docked, return to queue
                queue.available_ships.push(ship_entity);
                queue.assigned_ship = None;
            }
        }
    }
    
    // If no ship assigned and queue has ships, they're ready for next click
}
```

---

### 6. Update asteroid_input.rs — Assign Next Ship from Queue

**File:** `src/systems/asteroid_input.rs`

**When player clicks asteroid:**

```rust
pub fn asteroid_input_system(
    mut commands: Commands,
    mut queue: ResMut<ShipQueue>,
    asteroids: Query<(Entity, &ActiveAsteroid)>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(asteroid_entity) = find_clicked_asteroid(...) {
            // Get next available ship from queue
            if let Some(ship_entity) = queue.available_ships.pop() {
                // Set ship's mining target
                commands.entity(ship_entity).insert((
                    AutopilotTarget(asteroid_entity),
                    MiningTarget(asteroid_entity),
                ));
                
                // Mark ship as assigned (no longer in queue)
                queue.assigned_ship = Some(ship_entity);
            }
        }
    }
}
```

---

### 7. Update mining.rs — Ensure Ships Return After Mining

**File:** `src/systems/mining.rs`

**Verify mining_system sets ship state correctly:**

```rust
pub fn mining_system(
    mut ships: Query<&mut Ship>,
    mut asteroids: Query<&mut ActiveAsteroid>,
    mut station: Query<&mut Station>,
) {
    for mut ship in ships.iter_mut() {
        if ship.state == ShipState::Mining {
            // Extract ore
            let ore_extracted = MINING_RATE * time.delta_secs();
            ship.cargo = (ship.cargo + ore_extracted).min(ship.cargo_capacity as f32);
            
            // When cargo full, set state to return
            if ship.cargo >= ship.cargo_capacity as f32 {
                ship.state = ShipState::ReturningToStation;
            }
        }
    }
}
```

(Ensure ship transitions to ReturningToStation → Docked automatically)

---

### 8. Update lib.rs — Initialize New Queue Resource

**File:** `src/lib.rs`

**Add resource initialization:**

```rust
.insert_resource(ShipQueue {
    available_ships: vec![],  // Will be populated when ships are spawned
    assigned_ship: None,
})
```

**Register/order systems properly:**

```rust
.add_systems(Update, (
    systems::asteroid_input::asteroid_input_system,
    systems::mining::mining_system,
    systems::autopilot::autopilot_system,
    systems::drone_queue::ship_queue_system,
    // ... rest
).chain())
```

(Order: input assigns ship → mining extracts ore → autopilot returns to station → queue marks available)

---

### 9. Update setup.rs — Add Ships to Queue on Spawn

**File:** `src/systems/setup.rs`

**When ships are spawned (initial + drones), add them to queue:**

```rust
pub fn setup_world(
    mut commands: Commands,
    mut queue: ResMut<ShipQueue>,
) {
    // Spawn initial ship
    let ship1 = commands.spawn((
        InOpeningSequence,
        Ship { ... },
        // ...
    )).id();
    
    // Add to queue (will be removed when opening sequence starts)
    queue.available_ships.push(ship1);
    
    // Spawn initial drones
    for i in 0..3 {
        let drone = commands.spawn((
            Ship { ... },
            // ...
        )).id();
        queue.available_ships.push(drone);
    }
}
```

(Or populate queue from a separate initialization system)

---

## Compilation & Testing

### Step-by-Step

1. **Update components.rs** (remove PlayerShip, add InOpeningSequence)
   - `cargo check`

2. **Update setup.rs** (swap tag, add ships to queue)
   - `cargo check`

3. **Update opening_sequence.rs** (use InOpeningSequence, remove tag on complete)
   - `cargo check`

4. **Update autopilot.rs** (ensure unified docking)
   - `cargo check`

5. **Update/create drone_queue.rs** (unified queue system)
   - `cargo check`

6. **Update asteroid_input.rs** (assign next ship from queue)
   - `cargo check`

7. **Update mining.rs** (verify return logic)
   - `cargo check`

8. **Update lib.rs** (initialize queue, register systems)
   - `cargo check`

9. **Build APK and test on device**

### Device Testing

After build:
- [ ] Opening sequence plays (Ship #1 has `InOpeningSequence` tag)
- [ ] Opening completes, tag removed
- [ ] Click asteroid → Ship #1 launches
- [ ] Ship mines for ~5 seconds (cargo fills)
- [ ] Ship returns to station automatically
- [ ] Ship docks, cargo unloads
- [ ] Click another asteroid → Ship #2 launches
- [ ] Queue shows correct count (available ships)
- [ ] After Ship #2 docks, it's available for next click
- [ ] Save/load works (queue state persists)

---

## Commit

When working:

```bash
git add -A
git commit -m "Phase 1b: Unified ship queue system

- Remove PlayerShip component (all ships identical)
- Add InOpeningSequence tag for opening sequence ship
- Opening sequence uses Ship #1, removes tag when complete
- All ships use identical mining/docking/unloading logic
- Unified queue: click asteroid = assign next available ship
- Ship completes mining cycle, returns to queue automatically
- One-shot per assignment: Mine → Dock → Queue (ready for next)"

git tag v0.5.19-unified-queue
git push origin dev --tags
```

---

## If Something Breaks

Common issues:
- **Ship doesn't return to queue:** Check autopilot sets state to Docked
- **Opening sequence can't find ship:** Verify InOpeningSequence tag is added in spawn
- **Compile error about removed PlayerShip:** Search codebase for remaining PlayerShip references
- **Queue doesn't have ships:** Check setup.rs populates queue on world init

Show the error and we'll fix it.

---

## Next Phase

Once this works, **Phase 1c: Asteroid Inventory + Lifecycle**
- Asteroids have ore count (deplete as mined)
- When depleted, asteroid despawns
- New asteroid spawns after timer
- Creates respawn cycle

---

**This phase unifies everything into one system. Clean, simple, maintainable.**

**Go.**
