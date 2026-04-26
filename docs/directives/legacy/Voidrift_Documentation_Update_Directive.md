# Voidrift — Documentation Update Directive: Post-Refactor & Visual Polish
**Status:** Approved — Ready for Execution  
**Type:** Documentation Only — No code changes  
**Date:** April 2026  
**Depends On:** Module Refactor COMPLETE ✅ | Visual Polish Steps 1-5 COMPLETE ✅

---

## 1. Objective

Update all repository documentation to reflect the current state of the codebase after two major sessions:

1. **Visual Polish** — starfield, asteroid polygons, ship triangles, thruster glows, mining beams
2. **Module Refactor** — `lib.rs` split into `constants.rs`, `components.rs`, and `src/systems/` with 8 dedicated files

No code changes. Documentation files only.

---

## 2. Scope

**In scope:**
- `docs/state/current.md` — overwrite with current state
- `ARCHITECTURE.md` — overwrite with new module structure
- `CHANGELOG.md` — add new entries for visual polish and refactor
- `AGENT_CONTRACT.md` — update file registry and structure section
- `docs/phases/` — add phase summaries for visual polish and refactor
- `docs/adr/` — add ADR-006 for module structure decision
- `README.md` — update project structure section

**Explicitly out of scope:**
- Any changes to `src/`
- Any changes to `Cargo.toml`
- Any changes to build scripts
- Any gameplay or economy documentation changes

---

## 3. File Specifications

### 3.1 docs/state/current.md

Overwrite completely. Use this schema:

```markdown
# Project State
updated: 2026-04-19
agent: human

## Status
phase: Post-Phase 9 — Visual Polish Complete, Map Polish Next
test_floor: N/A — Gate evidence on physical device.
last_directive: Voidrift_Module_Refactor_Directive.md

## What Is Built
Full production chain economy running on Moto G 2025 (API 35). Two-track 
resource system (Magnetite/Carbon), autonomous fleet with smart routing, 
power economy with self-preservation, station AI telemetry. Visual polish 
complete through Step 5: parallax starfield, asteroid polygons, ship 
triangles with rotation, thruster glows, mining beams. Codebase modularized 
into 8 system files. Ship does not yet stop short of asteroid — known issue, 
deferred.

## What Is Next
Step 6: Map Polish — circle markers, connection lines, sector labels.
After map polish: module-aware ADR documentation, then post-slice economy 
expansion (five-ore mineral map, laser tiers, sector progression).

## Known Issues
- Ship does not stop short of asteroid on arrival — overshoots slightly
- Unused import warnings from .add_systems references — cosmetic only

## Open Questions
- SECTOR_3_POS confirmed as (-200.0, 300.0)? Verify against autonomous logic
- Refinery and power inline chunks in economy.rs — extraction named correctly?

## Economy Constants (Locked)
SHIP_SPEED: 180.0
CARGO_CAPACITY: 100
MINING_RATE: 20.0
REFINERY_RATIO: 10
HULL_REFINERY_RATIO: 5
REPAIR_COST: 25
AI_CORE_COST_CELLS: 50
HULL_PLATE_COST_CARBON: 5
SHIP_HULL_COST_PLATES: 3
POWER_COST_CYCLE_TOTAL: 4
POWER_COST_REFINERY: 1
POWER_COST_HULL_FORGE: 2
POWER_COST_SHIP_FORGE: 3
POWER_COST_AI_FABRICATE: 5
SHIP_POWER_MAX: 10.0
SHIP_POWER_FLOOR: 3.0
EGUI_SCALE: 3.0

## Post-Slice Roadmap (Captured — Not Yet Scoped)
Five-ore mineral map: Magnetite, Iron, Carbon, Tungsten, Titanite
Laser tiers: Basic (Steel) → Tungsten → Composite (Crystal Matrix)
Sector progression: Sectors 1-5, geographic gates
Asteroid cores: Laser-gated deeper material in existing fields
Crystal layer: TBD design session
Power Core: Power Cells + Crystal Matrix (concept, not specced)
Trader mechanic: First Tungsten Laser via trade (Autonomous Ship + Titanium Hull)
Blueprint system: Discovered/purchased recipes
Pause/Resume autonomous ships
Ship ceiling review (3 ships?)
Async background refinery
Hex map / procedural system generation (long term)

## Recent Decisions
- ADR-001: PresentMode::Fifo mandatory on Mali GPU
- ADR-002: Mesh2d for world-space primitives
- ADR-003: bevy_egui for all HUD and UI, EGUI_SCALE=3.0
- ADR-004: Bevy 0.15 pinned for Android stability
- ADR-005: Autonomous agents use dedicated systems
- ADR-006: Module structure — lib.rs split into systems/ (pending formal ADR)
- MINING_RATE tuned to 20.0, SHIP_SPEED to 180.0 for mobile RTT
- Five-ore economy design locked: Magnetite/Iron/Carbon/Tungsten/Titanite
```

### 3.2 ARCHITECTURE.md

Overwrite completely. Must reflect new module structure:

```markdown
# Voidrift Architecture

## Engine & Platform
- Bevy 0.15 (Rust)
- Android target: aarch64-linux-android (API 35)
- Primary test device: Moto G 2025
- UI: bevy_egui 0.33 (EGUI_SCALE=3.0)
- Rendering: Mesh2d only (ADR-002)

## Module Structure

src/
  lib.rs              — App setup and plugin registration only
  constants.rs        — All game constants in one place
  components.rs       — All Component and Resource structs
  systems/
    mod.rs            — pub use declarations
    setup.rs          — setup_world, entity spawning, mesh generators
    autopilot.rs      — autopilot_system, ship_rotation_system
    mining.rs         — mining_system, mining_beam_system
    economy.rs        — station_status_system, power_maintenance, self_preservation
    autonomous.rs     — autonomous_ship_system, routing, state machine
    visuals.rs        — starfield_scroll_system, thruster_glow_system
    ui.rs             — hud_ui_system, docking panel, egui HUD
    map.rs            — handle_input, camera_follow_system, map view

## Key Systems

[List every system with one-line description — read from current codebase]

## Component Inventory

[List every Component struct with fields and purpose — read from components.rs]

## ShipState Machine

Idle → Navigating → Mining → Docked → Idle
Idle → Navigating → Idle (station arrival)

## AutonomousShipState Machine

Holding → Outbound → Mining → Returning → Unloading → Holding

## Hardware Constraints (ADRs)

ADR-001: PresentMode::Fifo — mandatory, Mali GPU buffer starvation
ADR-002: Mesh2d only — Sprite triggers gralloc format errors
ADR-003: bevy_egui — Text2d and camera-parented Mesh2d both fail on device
ADR-004: Bevy 0.15 pinned — most stable Android documentation
ADR-005: Dedicated systems for autonomous agents
ADR-006: Module structure — lib.rs app setup only
```

### 3.3 CHANGELOG.md

Add two new entries above the existing `[0.1.0]` entry:

```markdown
## [0.3.0] — April 2026 — Module Refactor

### Changed
- lib.rs split into constants.rs, components.rs, and src/systems/ with 8 files
- Each system in its own file: autopilot, mining, economy, autonomous, visuals, ui, map, setup
- lib.rs now contains app setup only

### Technical
- Progressive one-file-at-a-time migration with compile verification at each step
- Zero logic changes — pure structural reorganisation
- Import pattern established: use crate::constants::*, use crate::components::*

---

## [0.2.0] — April 2026 — Visual Polish

### Added
- Parallax starfield — 150 far stars + 50 near stars, two-speed scroll
- Asteroid irregular polygon — 8-vertex seeded random shape, depleted state
- Ship triangle mesh — directional, rotates to face travel direction
- Thruster glow — state-based visibility, player cyan / autonomous orange
- Mining beam — dynamic line from ship to asteroid while mining

### Fixed
- Subpixel shimmer on stars — integer sizes eliminate GPU anti-aliasing flicker
- Camera stutter — chained autopilot → camera → starfield execution order
- Autonomous ship beam visibility — enforced on every tick not just state exit

### Technical
- StdRng::seed_from_u64 used for asteroid shape generation (SmallRng feature not enabled)
- CameraDelta resource tracks world-space camera movement per tick
- LastHeading component preserves ship facing when idle or docked
```

### 3.4 AGENT_CONTRACT.md

Update the STRUCTURE and FILE_REGISTRY sections to reflect new module layout:

```markdown
## STRUCTURE
src/lib.rs          : App setup and plugin registration only
src/constants.rs    : All game constants — single source of truth
src/components.rs   : All Component and Resource structs
src/systems/        : One file per system concern
  autopilot.rs      : Ship movement and rotation
  mining.rs         : Ore extraction and mining beam
  economy.rs        : Refinery, forge, power, station maintenance
  autonomous.rs     : Autonomous ship state machine and routing
  visuals.rs        : Starfield, thruster glow, visual effects
  ui.rs             : egui HUD, docking panel, station log
  map.rs            : Input handling, camera, map view
  setup.rs          : World setup, entity spawning, mesh generators
android/            : Gradle wrapper project for Android APK packaging
assets/             : Game assets — fonts/FiraSans-Bold.ttf
docs/adr/           : Architectural Decision Records — locked after acceptance
docs/phases/        : Phase summaries — archival, never edited after creation
docs/state/         : current.md only — always current
.cargo/             : NDK linker configuration — build critical

## INVARIANTS
hardware    : Physical device evidence required at every gate
scope       : Every directive lists explicit file scope
adrs        : No architectural decision without an ADR
phases      : No phase begins without prior gate passing on device
build       : PresentMode::Fifo mandatory — do not change
ui          : bevy_egui only for HUD — no Text2d, no camera-parented Mesh2d
modules     : lib.rs is app setup only — no logic, no components, no constants
constants   : All constants in constants.rs — never hardcode inline
```

### 3.5 docs/adr/ADR-006-module-structure.md

Create new ADR:

```markdown
# ADR-006: Module Structure — lib.rs App Setup Only
**Date:** April 2026  
**Status:** Accepted

## Context
lib.rs grew to 1000+ lines across Phases 0-9. The agent began hitting 
structural friction — difficulty locating systems, import conflicts, and 
unclear ownership of functions. Single-file architecture was no longer 
sustainable.

## Decision
Split lib.rs into a module hierarchy:
- constants.rs: all constants
- components.rs: all Component and Resource structs  
- systems/: one file per system concern (8 files)
- lib.rs: app setup only

## Rationale
- Each system file has single responsibility
- Constants and components are importable from any system file
- lib.rs as pure app setup is readable at a glance
- Progressive one-file migration with compile verification prevents regression
- Module boundaries make future agent sessions faster to orient

## Consequences
- All new systems go in systems/ — never in lib.rs
- All new constants go in constants.rs — never hardcoded inline
- All new components go in components.rs
- Import pattern: use crate::constants::*; use crate::components::*;
- lib.rs changes are rare — only for new plugin registration or system scheduling
```

### 3.6 docs/phases/ — New Phase Summaries

Create two new archival files:

**`docs/phases/phase_visual_polish.md`**
- What was added: starfield, polygons, triangles, glows, beams
- Hardware issues encountered: subpixel shimmer, camera stutter, beam visibility bug
- How each was fixed
- Evidence: gate screenshot showing all 5 elements simultaneously
- Status: Steps 1-5 complete, Step 6 (map polish) pending

**`docs/phases/phase_module_refactor.md`**
- Context: lib.rs exceeded sustainable size after Phase 9
- What was created: 8 system files, constants.rs, components.rs
- Migration sequence: one file at a time, compile after each
- Zero logic changes — pure structural reorganisation
- Known issues: unused import warnings (cosmetic)
- Status: Complete

### 3.7 README.md

Update the Project Structure section only:

Replace the old flat structure with:

```
VoidDrift/
├── src/
│   ├── lib.rs              # App setup and plugin registration only
│   ├── constants.rs        # All game constants
│   ├── components.rs       # All ECS components and resources
│   └── systems/
│       ├── mod.rs          # Module declarations
│       ├── setup.rs        # World setup and entity spawning
│       ├── autopilot.rs    # Ship movement and rotation
│       ├── mining.rs       # Ore extraction and mining beam
│       ├── economy.rs      # Refinery, forge, power economy
│       ├── autonomous.rs   # Autonomous ship state machine
│       ├── visuals.rs      # Starfield, thruster glow, effects
│       ├── ui.rs           # egui HUD and docking panel
│       └── map.rs          # Input, camera, map view
├── android/                # Gradle wrapper for Android APK
├── assets/fonts/           # FiraSans-Bold.ttf
├── docs/
│   ├── adr/                # 6 Architectural Decision Records
│   ├── phases/             # Phase summaries (archival)
│   └── state/current.md   # Always-current project state
├── build_android.ps1       # Full build + deploy pipeline
├── capture_gate_evidence.ps1
└── Cargo.toml
```

---

## 4. File Scope

| File | Action |
|------|--------|
| `docs/state/current.md` | OVERWRITE |
| `ARCHITECTURE.md` | OVERWRITE |
| `CHANGELOG.md` | MODIFY — add two entries |
| `AGENT_CONTRACT.md` | MODIFY — structure and registry sections |
| `docs/adr/ADR-006-module-structure.md` | CREATE |
| `docs/phases/phase_visual_polish.md` | CREATE |
| `docs/phases/phase_module_refactor.md` | CREATE |
| `README.md` | MODIFY — project structure section only |
| `src/` | READ-ONLY — no code changes |
| `Cargo.toml` | READ-ONLY |

---

## 5. Completion Criteria

- [ ] All 8 files created or updated
- [ ] `current.md` reflects actual current state — no aspirational language
- [ ] `ARCHITECTURE.md` system and component inventories read from actual codebase — not invented
- [ ] `CHANGELOG.md` entries accurate — hardware fixes named specifically
- [ ] `AGENT_CONTRACT.md` invariants include module rules
- [ ] `ADR-006` formally documents the module decision
- [ ] Phase summaries accurate — read walkthrough docs before writing
- [ ] `README.md` structure matches actual directory layout

---

## 6. Instruction to Agent

Read the actual codebase before writing `ARCHITECTURE.md`. The system inventory and component inventory must reflect what actually exists in `systems/` and `components.rs` — not what was planned in earlier directives. If a system was renamed or combined during the refactor, document what actually exists.

For phase summaries: read the walkthrough artifacts from the visual polish and refactor sessions before writing. Accuracy over polish.

---

*Voidrift Documentation Update Directive | April 2026 | RFD IT Services Ltd.*  
*Documentation only. No code changes under any circumstances.*
