# Voidrift — Refactor Directive: Module Structure + Mining Beam Completion
**Status:** Approved — Ready for Execution  
**Type:** Structural Refactor + Visual Completion  
**Date:** April 2026  
**Depends On:** Visual Polish Steps 1-4 VERIFIED ✅

---

## 1. Objective

Two things in one directive:

1. **Complete the Mining Beam** (Visual Polish Step 5) — the beam visibility logic for autonomous ships is currently broken. Fix it.
2. **Split `lib.rs` into a proper module structure** — the file has outgrown single-file architecture. The agent is hitting structural friction. This refactor is mechanical — move code, fix imports, zero logic changes, zero gameplay impact.

Map polish (Step 6) is deferred until this directive passes. A clean module structure makes map polish significantly easier.

---

## 2. Scope Boundaries

> ⚠️ HARD LIMIT: No logic changes. No gameplay changes. No economy changes. Move code and fix the beam visibility bug only.

**In scope:**
- Mining beam visibility fix for autonomous ships
- Complete module split of `src/lib.rs` into the structure defined in §4
- Fix all import paths after split
- Verify compilation with zero errors

**Explicitly out of scope:**
- Map polish (Step 6) — deferred to next directive
- Any economy, power, or resource changes
- Any new systems or components
- Any gameplay logic changes
- Sound

---

## 3. Mining Beam Fix

### 3.1 The Bug

The autonomous ship mining beam is not hiding correctly when the ship exits Mining state. The visibility toggle is only firing on state exit, not being enforced on every non-Mining tick.

### 3.2 The Fix

In `autonomous_ship_system`, the beam visibility must be set on **every tick** based on current state — not just on transition:

```rust
// Pseudocode — runs every tick for each autonomous ship
match ship.state {
    AutonomousShipState::Mining => {
        // show beam, update geometry
        *beam_visibility = Visibility::Visible;
        // ... stretch beam from ship to field position
    }
    _ => {
        // hide beam on every non-Mining tick
        *beam_visibility = Visibility::Hidden;
    }
}
```

Same pattern must be verified for the player ship beam in `mining_system`.

### 3.3 Beam Colors
- Player ship beam: Cyan, 60% opacity
- Autonomous ship beam: Orange, 60% opacity

### 3.4 Verification
- Player ship beam appears when mining, disappears when navigating or docked
- Autonomous ship beam appears when that ship is mining, disappears otherwise
- Two ships mining simultaneously shows two beams independently

---

## 4. Module Structure

### 4.1 Target Structure

```
src/
  lib.rs              ← App setup and plugin registration ONLY
  constants.rs        ← Every const in one place
  components.rs       ← All Component and Resource structs
  systems/
    mod.rs            ← pub use declarations for all systems
    autopilot.rs      ← autopilot_system, ship_rotation_system
    mining.rs         ← mining_system, mining_beam_system
    economy.rs        ← refinery, forge, assembly, power systems
    autonomous.rs     ← autonomous_ship_system, drone routing
    visuals.rs        ← starfield_scroll_system, thruster_glow_system
    ui.rs             ← hud_ui_system, docking panel, egui
    map.rs            ← map view, sector markers, handle_input
    setup.rs          ← setup_world, all entity spawning
```

### 4.2 lib.rs After Refactor

`lib.rs` should contain ONLY:

```rust
mod constants;
mod components;
mod systems;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(components::CameraDelta::default())
        .insert_resource(components::GameState::SpaceView)
        // ... other resources
        .add_systems(Startup, systems::setup::setup_world)
        .add_systems(Update, (
            systems::autopilot::autopilot_system,
            systems::mining::mining_system,
            systems::mining::mining_beam_system,
            systems::economy::refinery_system,
            systems::economy::power_system,
            systems::autonomous::autonomous_ship_system,
            systems::visuals::starfield_scroll_system,
            systems::visuals::thruster_glow_system,
            systems::visuals::ship_rotation_system,
            systems::ui::hud_ui_system,
            systems::map::handle_input,
            systems::map::camera_follow_system,
        ))
        .run();
}
```

Nothing else in lib.rs. If it's not app setup, it doesn't belong here.

### 4.3 constants.rs

Every `const` currently in `lib.rs` moves here. All must remain `pub` so other modules can access them.

```rust
// constants.rs
pub const SHIP_SPEED: f32 = 180.0;
pub const CARGO_CAPACITY: u32 = 100;
pub const MINING_RATE: f32 = 20.0;
pub const REFINERY_RATIO: u32 = 10;
pub const REPAIR_COST: u32 = 25;
pub const AI_CORE_COST_CELLS: u32 = 50;
pub const HULL_PLATE_COST_CARBON: u32 = 5;
pub const SHIP_HULL_COST_PLATES: u32 = 3;
pub const POWER_COST_CYCLE_TOTAL: u32 = 4;
pub const POWER_COST_REFINERY: u32 = 1;
pub const POWER_COST_HULL_FORGE: u32 = 2;
pub const POWER_COST_SHIP_FORGE: u32 = 3;
pub const POWER_COST_AI_FABRICATE: u32 = 5;
pub const SHIP_POWER_MAX: f32 = 10.0;
pub const SHIP_POWER_FLOOR: f32 = 3.0;
pub const SHIP_POWER_COST_TRANSIT: f32 = 1.0;
pub const SHIP_POWER_COST_MINING: f32 = 2.0;
pub const POWER_CELL_RESTORE_VALUE: f32 = 3.0;
pub const EGUI_SCALE: f32 = 3.0;
pub const ARRIVAL_THRESHOLD: f32 = 8.0;
pub const HULL_REFINERY_RATIO: u32 = 5;
pub const AI_CORE_COST: u32 = 50;
pub const POWER_WARNING_INTERVAL: f32 = 30.0;
pub const EMERGENCY_REFINE_COST: u32 = 10;
pub const STATION_POWER_MAX: f32 = 50.0;
pub const STATION_POWER_FLOOR: f32 = 10.0;
pub const STATION_POWER_RESTORE_VALUE: f32 = 5.0;
// Add any missing constants found during audit
```

### 4.4 components.rs

Every `#[derive(Component)]` and `#[derive(Resource)]` struct moves here. All `pub`.

Key structs to move:
- `Ship`, `ShipState`
- `Station`
- `AutonomousShip`, `AutonomousShipState`
- `AsteroidField`
- `AutopilotTarget`
- `Drone` (if still exists — remove if retired)
- `StarLayer`, `ThrusterGlow`, `MiningBeam`
- `LastHeading`
- `MapMarker`, `SectorTarget`
- `AiCore` (confirm fully removed from Phase 9)
- `CameraDelta` (Resource)
- `OreType` enum
- `GameState` enum
- Any other structs currently in lib.rs

### 4.5 systems/mod.rs

```rust
pub mod autopilot;
pub mod mining;
pub mod economy;
pub mod autonomous;
pub mod visuals;
pub mod ui;
pub mod map;
pub mod setup;
```

### 4.6 System File Assignments

| System/Function | Target File |
|----------------|-------------|
| `autopilot_system` | `systems/autopilot.rs` |
| `ship_rotation_system` | `systems/autopilot.rs` |
| `camera_follow_system` | `systems/map.rs` |
| `mining_system` | `systems/mining.rs` |
| `mining_beam_system` | `systems/mining.rs` |
| `starfield_scroll_system` | `systems/visuals.rs` |
| `thruster_glow_system` | `systems/visuals.rs` |
| `autonomous_ship_system` | `systems/autonomous.rs` |
| `hud_ui_system` | `systems/ui.rs` |
| `handle_input` | `systems/map.rs` |
| `refinery_system` (if split) | `systems/economy.rs` |
| `power_system` (if split) | `systems/economy.rs` |
| `station_maintenance` (if split) | `systems/economy.rs` |
| `setup_world` | `systems/setup.rs` |
| All spawn helpers | `systems/setup.rs` |

---

## 5. Implementation Sequence

**Step 1 — Mining Beam Fix First**
Fix the autonomous ship beam visibility bug before touching file structure. Verify on device. This is the simpler change and should be isolated.

**Step 2 — Audit Before Moving**
Before moving any code, run a full audit of `lib.rs`:
- List every `const` → goes to `constants.rs`
- List every `Component`/`Resource` struct → goes to `components.rs`
- List every `fn` → assign to target module per §4.6
- Flag anything that doesn't fit cleanly — report before moving

**Step 3 — Create Files**
Create all new files empty with correct module declarations. Verify project compiles with empty modules before adding content.

**Step 4 — Move Constants**
Move all constants to `constants.rs`. Add `use crate::constants::*` to every file that needs them. Verify compilation.

**Step 5 — Move Components**
Move all structs to `components.rs`. Fix imports. Verify compilation.

**Step 6 — Move Systems One File at a Time**
Move one system file at a time. Verify compilation after each move. Do not move all files simultaneously.

Suggested order:
1. `setup.rs` — setup_world and spawning
2. `visuals.rs` — starfield, glow, rotation (no dependencies on economy)
3. `autopilot.rs` — movement systems
4. `mining.rs` — mining and beam
5. `autonomous.rs` — drone loop
6. `economy.rs` — refinery, forge, power
7. `map.rs` — input and camera
8. `ui.rs` — egui HUD (most complex, last)

**Step 7 — Clean lib.rs**
Strip lib.rs to app setup only. Verify compilation. Verify no dead code warnings.

---

## 6. Import Pattern

Every system file will need:

```rust
use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;
```

UI file additionally needs:
```rust
use bevy_egui::{egui, EguiContexts};
```

Setup file additionally needs:
```rust
use rand::{Rng, SeedableRng, rngs::StdRng};
```

---

## 7. File Scope

| File | Action |
|------|--------|
| `src/lib.rs` | MODIFY — strip to app setup only |
| `src/constants.rs` | CREATE |
| `src/components.rs` | CREATE |
| `src/systems/mod.rs` | CREATE |
| `src/systems/autopilot.rs` | CREATE |
| `src/systems/mining.rs` | CREATE |
| `src/systems/economy.rs` | CREATE |
| `src/systems/autonomous.rs` | CREATE |
| `src/systems/visuals.rs` | CREATE |
| `src/systems/ui.rs` | CREATE |
| `src/systems/map.rs` | CREATE |
| `src/systems/setup.rs` | CREATE |
| `Cargo.toml` | READ-ONLY |

---

## 8. Completion Criteria

- [ ] Mining beam fixed — player and autonomous beams both correct
- [ ] Zero compiler errors after full refactor
- [ ] Zero compiler warnings from dead code or unused imports
- [ ] `lib.rs` contains only app setup — nothing else
- [ ] All constants in `constants.rs`
- [ ] All components in `components.rs`
- [ ] Each system in its assigned file
- [ ] App builds and deploys to Moto G 2025
- [ ] All existing visual polish (Steps 1-4) still working after refactor
- [ ] Mining beam working on device (Step 5)

**Gate:** Deploy to device. All Steps 1-5 visually confirmed working. Screenshot showing ship triangle, starfield, asteroid polygon, thruster glow, and mining beam simultaneously.

---

## 9. Note to Agent

This is a mechanical refactor. The goal is zero logic changes. If during the move you notice a bug, a smell, or an opportunity to improve something — note it in a comment but do not fix it. Fix it in a separate directive. The refactor and the fixes are separate concerns and must not be mixed.

If any system has unclear module ownership — report it before moving it. Do not guess.

---

*Voidrift Module Refactor Directive | April 2026 | RFD IT Services Ltd.*  
*Move code. Fix imports. Fix the beam. Nothing else.*
