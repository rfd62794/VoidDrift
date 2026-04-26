# Phase 5: Codebase Refactor (Option B - Moderate)
**Agent:** Antigravity (Windsurf SWE-1.5 or Gemini 3.1 Pro)  
**Objective:** Reorganize codebase by domain. Split bloated files. Clean foundation for Phase 2+.  
**Status:** Ready for implementation  
**Estimated time:** 4-6 hours  
**Target:** Compiles cleanly, no gameplay changes, improved maintainability

---

## Overview

VoidDrift's Phase 1c core loop is complete and working. Before adding Phase 2 (station modules/upgrades), the codebase needs organizational improvements.

**Current state:**
- `components.rs`: 438 lines (bloated, mixed concerns)
- `src/systems/`: 23 flat files (hard to navigate)
- `setup.rs`: 656 lines (oversized, multiple responsibilities)
- Tight coupling in some systems (autonomous, HUD)

**Target state:**
- `src/components/`: Split by domain (5-6 files)
- `src/systems/`: Organized into logical modules (6-7 module groups)
- `setup.rs`: Split by responsibility (3-4 files)
- Clean queries, clear dependencies

**Result:** 50% improvement in code organization. Scales well for Phase 2+.

---

## Part 1: Component Organization

### Current components.rs (438 lines)

Mixed concerns:
- Game state (Ship, Station, etc.)
- UI state (ActiveStationTab, DrawerState)
- Resources (SaveData, OpeningSequence, SignalLog)
- Markers (MainCamera, InOpeningSequence)
- Utilities (helper functions)

### Target: `src/components/` (directory)

```
src/components/
├── mod.rs                  # Re-exports all components
├── game_state.rs           # Ship, Station, Drone, Asteroid
├── ui_state.rs             # ActiveStationTab, UIComponent, DrawerState
├── resources.rs            # SaveData, OpeningSequence, SignalLog, etc.
├── markers.rs              # MainCamera, InOpeningSequence, etc.
└── utilities.rs            # Helper functions (berth_world_pos, ore_name, etc.)
```

### Detailed File Breakdown

#### **mod.rs** (Entry point)
```rust
pub mod game_state;
pub mod ui_state;
pub mod resources;
pub mod markers;
pub mod utilities;

pub use game_state::*;
pub use ui_state::*;
pub use resources::*;
pub use markers::*;
pub use utilities::*;
```

#### **game_state.rs** (Game logic components)
- `Ship` (state, cargo, position)
- `Station` (ore, ingots, drones)
- `ActiveAsteroid` (ore_remaining, lifespan_timer)
- `AsteroidRespawnTimer`
- `ShipQueue`
- Related enums (ShipState, OreDeposit, LaserTier)

#### **ui_state.rs** (UI-only components)
- `ActiveStationTab`
- `UIComponent`
- `DrawerState`
- `UiLayout`
- Any UI-specific markers

#### **resources.rs** (Global resources)
- `SaveData`
- `OpeningSequence`
- `SignalLog`
- `QuestLog`
- `GameState` (if separate from components)
- Any other Res<T> types

#### **markers.rs** (Component tags/markers)
- `MainCamera`
- `InOpeningSequence`
- `MapMarker`
- `DestinationHighlight`
- Any other zero-data tag components

#### **utilities.rs** (Helper functions)
- `berth_world_pos()`
- `ore_name()`
- `ore_laser_required()`
- Any utility functions currently in components.rs

### Implementation Steps

1. Create `src/components/` directory
2. Create `mod.rs` with re-exports
3. Move/split components into domain files
4. Update all `use` statements across codebase
5. Run `cargo check` after each major move
6. Verify no compile errors

---

## Part 2: System Module Organization

### Current: 23 flat files in `src/systems/`

Hard to navigate. Related systems scattered.

### Target: Logical module structure

```
src/systems/
├── mod.rs                  # Register all system modules
├── game_loop/
│   ├── mod.rs
│   ├── mining.rs
│   ├── auto_process.rs
│   └── autonomous.rs
├── ship_control/
│   ├── mod.rs
│   ├── autopilot.rs
│   └── asteroid_input.rs
├── asteroid/
│   ├── mod.rs
│   ├── spawn.rs
│   └── lifecycle.rs
├── ui/
│   ├── mod.rs
│   ├── hud/ (existing)
│   ├── station_tabs.rs
│   └── tutorial.rs
├── persistence/
│   ├── mod.rs
│   └── save.rs
├── narrative/
│   ├── mod.rs
│   ├── opening_sequence.rs
│   ├── signal.rs
│   └── quest.rs
├── visuals/
│   ├── mod.rs
│   ├── map.rs
│   ├── viewport.rs
│   └── visuals.rs
└── setup/
    ├── mod.rs
    ├── world_spawn.rs
    ├── entity_setup.rs
    └── quest_init.rs
```

### Module Responsibilities

**game_loop/** — Core production and mining
- mining.rs: Extract ore from asteroids
- auto_process.rs: Auto-refine and forge
- autonomous.rs: Autonomous ship behavior

**ship_control/** — Ship movement and targeting
- autopilot.rs: Navigation logic
- asteroid_input.rs: Player input for mining targets

**asteroid/** — Asteroid spawning and lifecycle
- spawn.rs: Respawn logic
- lifecycle.rs: Lifespan and despawn

**ui/** — All UI systems
- hud/: HUD rendering and state
- station_tabs.rs: Tab switching logic
- tutorial.rs: Tutorial prompts

**persistence/** — Save/load
- save.rs: Save and restore game state

**narrative/** — Story and communication
- opening_sequence.rs: Opening cinematic
- signal.rs: ECHO AI communication
- quest.rs: Quest tracking

**visuals/** — Rendering and visual feedback
- map.rs: Map rendering
- viewport.rs: Camera/viewport management
- visuals.rs: General visual effects

**setup/** — World initialization
- world_spawn.rs: Starfield, sectors, initial entities
- entity_setup.rs: Berths, map elements, UI entities
- quest_init.rs: Quest log initialization

### Implementation Steps

1. Create module directories
2. Move related systems into each module
3. Create `mod.rs` files with re-exports
4. Update lib.rs system registration
5. Fix import statements
6. Run `cargo check` after major reorganizations
7. Verify system execution order unchanged

---

## Part 3: Setup.rs Split

### Current: setup.rs (656 lines)

Too large. Mixed responsibilities:
- World spawning (starfield, sectors, camera)
- Entity setup (berths, UI, map elements)
- Quest initialization

### Target: `src/systems/setup/` (3 files)

#### **world_spawn.rs** (~200 lines)
```rust
pub fn setup_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Spawn starfield layers
    // Spawn camera
    // Spawn sectors/zones
    // Initialize initial asteroids
}
```

#### **entity_setup.rs** (~200 lines)
```rust
pub fn spawn_berths(/* ... */) { }
pub fn spawn_map_elements(/* ... */) { }
pub fn spawn_ui_entities(/* ... */) { }
```

#### **quest_init.rs** (~100 lines)
```rust
pub fn initialize_quests(/* ... */) { }
```

#### **mod.rs** (Orchestrator)
```rust
pub fn setup_systems() {
    world_spawn();
    entity_setup();
    quest_init();
}
```

### Implementation Steps

1. Create `src/systems/setup/` directory
2. Move world spawning into world_spawn.rs
3. Move entity spawning into entity_setup.rs
4. Move quest logic into quest_init.rs
5. Create mod.rs that exports all
6. Update lib.rs startup to call setup::setup_systems()
7. Run `cargo check` after splits

---

## Part 4: Query Optimization

### Current Issues

Many systems have verbose Universal Disjointness filters:
```rust
Query<(Entity, &mut Ship), (Without<Station>, Without<AsteroidField>, ...)>
```

### Target: Type aliases for common patterns

```rust
// At top of systems/mod.rs or systems/game_loop/mod.rs
type ShipQuery<'w, 's> = Query<'w, 's, (Entity, &'static mut Ship), Without<Station>>;
type AsteroidQuery<'w, 's> = Query<'w, 's, &'static mut ActiveAsteroid>;
type StationQuery<'w, 's> = Query<'w, 's, &'static mut Station>;
```

Then use:
```rust
pub fn mining_system(mut ships: ShipQuery) {
    // Cleaner, reusable
}
```

### Implementation Steps

1. Identify recurring query patterns
2. Create type aliases
3. Replace verbose queries with aliases
4. Document query patterns in module comments
5. Run `cargo check` to verify

---

## Part 5: System Registration Order

### Current lib.rs registration

Systems registered in order. Need to verify execution order is correct after reorganization.

### Order Matters

1. **Setup systems** first (world initialization)
2. **Game loop systems** (mining, auto-process)
3. **Ship control systems** (autopilot, targeting)
4. **Asteroid systems** (spawn, lifecycle)
5. **Persistence systems** (save)
6. **UI systems** (HUD, tabs)
7. **Narrative systems** (opening, signals)
8. **Visual systems** (map, viewport)

### Verification

After reorganization, verify in lib.rs:
```rust
.add_systems(Startup, (
    setup::setup_systems,
))
.add_systems(Update, (
    game_loop::mining_system,
    game_loop::auto_process_system,
    ship_control::autopilot_system,
    asteroid::spawn_system,
    asteroid::lifecycle_system,
    persistence::save_system,
    ui::hud_system,
    narrative::opening_sequence_system,
    visuals::map_render_system,
    // ... rest
).chain())
```

---

## Part 6: Implementation Checklist

### Phase 1: Component Organization (1-1.5 hours)

- [ ] Create `src/components/` directory
- [ ] Create mod.rs with re-exports
- [ ] Split components into 5 domain files
- [ ] Move utilities to utilities.rs
- [ ] Update all `use` statements
- [ ] `cargo check` — no errors
- [ ] Verify no unused imports

### Phase 2: System Module Organization (2-2.5 hours)

- [ ] Create module directories (game_loop/, ship_control/, etc.)
- [ ] Move systems into appropriate modules
- [ ] Create mod.rs files for each module
- [ ] Update lib.rs system registration
- [ ] Fix import statements
- [ ] `cargo check` — no errors
- [ ] Verify system execution order

### Phase 3: Setup.rs Split (1 hour)

- [ ] Create `src/systems/setup/` directory
- [ ] Split setup.rs into 3 files (world_spawn, entity_setup, quest_init)
- [ ] Create setup/mod.rs
- [ ] Update lib.rs startup call
- [ ] `cargo check` — no errors

### Phase 4: Query Optimization (0.5 hours)

- [ ] Identify recurring query patterns
- [ ] Create type aliases
- [ ] Replace verbose queries
- [ ] `cargo check` — no errors
- [ ] Document patterns in comments

### Phase 5: Verification & Cleanup (0.5-1 hour)

- [ ] Full `cargo check`
- [ ] No warnings
- [ ] System registration order verified
- [ ] No unused imports/code
- [ ] Clean git state

---

## Part 7: Testing & Verification

### Build & Device Test

After completing refactor:

```bash
cargo check              # Verify compilation
./build_android.ps1      # Build APK
# Test on device for 15 minutes
```

**Verify:**
- [ ] App boots normally
- [ ] No crashes
- [ ] Core gameplay loop works (mine → refine → build)
- [ ] Asteroids spawn/despawn
- [ ] Ships mine correctly
- [ ] Save/load works
- [ ] No performance regressions

---

## Part 8: Commit Strategy

### Commit in logical chunks

```bash
# 1. Component refactor
git add src/components/
git commit -m "refactor: reorganize components by domain

Split components.rs (438 lines) into domain-specific modules:
- game_state.rs: Ship, Station, Asteroid, etc.
- ui_state.rs: UI-specific components
- resources.rs: Global resources
- markers.rs: Tag components
- utilities.rs: Helper functions

No gameplay changes. All systems reorganized for clarity."

# 2. System module organization
git add src/systems/
git commit -m "refactor: organize systems into logical modules

Group 23 flat system files into 7 modules:
- game_loop/: mining, auto_process, autonomous
- ship_control/: autopilot, asteroid_input
- asteroid/: spawn, lifecycle
- ui/: hud, station_tabs, tutorial
- persistence/: save
- narrative/: opening_sequence, signal, quest
- visuals/: map, viewport, visuals

No gameplay changes. System registration order preserved."

# 3. Setup split
git commit -m "refactor: split oversized setup.rs by responsibility

Split setup.rs (656 lines) into domain modules:
- world_spawn.rs: Starfield, camera, sectors
- entity_setup.rs: Berths, UI, map elements
- quest_init.rs: Quest initialization

No gameplay changes. Improves maintainability."

# 4. Query optimization
git commit -m "refactor: add type aliases for common queries

Create type aliases for recurring query patterns:
- ShipQuery
- AsteroidQuery
- StationQuery

Improves readability. No gameplay changes."

# 5. Final cleanup
git commit -m "refactor: Phase 5 codebase reorganization (complete)

Reorganized codebase by domain for Phase 2+ development.
- Components: 1 file → 5 domain modules
- Systems: 23 flat files → 7 logical modules
- Setup: 656 lines → 3 responsibility files
- Queries: Type aliases for readability

No gameplay changes. 50% improvement in code organization.
All systems verified. Ready for Phase 2."

git tag v1.2-refactored
```

---

## Part 9: Success Criteria

**Refactor is complete when:**

- [ ] Components split into 5 domain modules
- [ ] Systems organized into 7 logical modules
- [ ] Setup.rs split into 3 responsibility files
- [ ] Query type aliases implemented
- [ ] Zero compile errors
- [ ] Zero warnings
- [ ] System execution order unchanged
- [ ] Core gameplay loop works identically
- [ ] Device testing passes
- [ ] All commits clean and logical

---

## Part 10: Rollback Plan

If major issues arise:

```bash
# Revert to last working state
git reset --hard v1.1-phase1c-final

# Or revert specific commits
git revert <commit-hash>
```

---

## Notes for Antigravity

- Take it step-by-step. Compile after each major change.
- Don't parallelize. Do components first, then systems, then setup.
- When in doubt, ask Claude for architecture questions.
- Device testing is essential. Run on Moto G after completion.
- If something feels off, revert and ask before continuing.

---

## Timeline

- **Phase 1: Components** — 1-1.5 hours
- **Phase 2: Systems** — 2-2.5 hours
- **Phase 3: Setup** — 1 hour
- **Phase 4: Queries** — 0.5 hours
- **Phase 5: Verification** — 0.5-1 hour

**Total: 4-6 hours**

---

**This refactor is non-invasive. No gameplay changes. Just organization.**

Clean foundation for Phase 2 (station modules/upgrades).

**Go.**
