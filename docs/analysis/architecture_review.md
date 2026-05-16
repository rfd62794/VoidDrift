# Voidrift Architecture Review

**Date:** January 2025
**Build:** v3.1.0-sprint5-visual-overhaul
**Reviewer:** Architecture Audit
**Status:** Fixable - Targeted Refactoring Recommended

---

## Executive Summary

Voidrift's architecture is fundamentally sound with clear Layer 1/2/3 separation (ADR-016). The codebase follows established patterns (Universal Disjointness, Module Structure, System Partitioning) and the Scout Mk I integration demonstrates solid architectural decision-making (ADR-019).

**Verdict:** **Fixable** - No redesign required. Targeted refactoring of identified tech debt will restore architectural health.

**Key Findings:**
- **Architecture:** Layer 1/2/3 boundaries are respected and well-documented
- **Scout Integration:** Well-designed 3-system pattern following ADR-019
- **Documentation:** ARCHITECTURE.md contains outdated information (system registration notes)
- **Tech Debt:** 5 identified items requiring targeted fixes (auto_forge_system, god classes, duplication, hardcoding)
- **Complexity:** Manageable for single-developer project; no critical coupling issues

---

## Part 1: Module Mapping

### Layer 1: Engine (Infrastructure)

| File | Lines | Responsibility | Ownership |
| :--- | :--- | :--- | :--- |
| `src/lib.rs` | 275 | App setup, plugin registration, system scheduling, resource init | Engine |
| `src/constants.rs` | - | Game constants (sector positions) | Engine |
| `src/config/mod.rs` | 9 | Config module exports | Engine |
| `src/config/balance.rs` | - | BalanceConfig struct and loader | Engine |
| `src/config/visual.rs` | - | VisualConfig struct and loader | Engine |
| `src/config/content.rs` | - | ContentConfig, TutorialConfig, QuestConfig, RequestConfig, LogsConfig | Engine |
| `src/components/mod.rs` | 16 | Component module exports | Engine |
| `src/components/game_state.rs` | 220 | Ship, Station, AutonomousShip, AsteroidField, Painted, ScoutOrbit, DroneTarget, StationQueues | Engine |
| `src/components/markers.rs` | - | Marker components for entity filtering | Engine |
| `src/components/resources.rs` | 278 | ECS Resources: GameState, DeviceType, ViewState, AsteroidRespawnTimer, ShipQueue, SignalLog, QuestLog, OpeningSequence, ScoutEnabled, etc. | Engine |
| `src/components/ui_state.rs` | - | UI state: ActiveStationTab, DrawerState, ProductionTabState, RequestsTabState, etc. | Engine |
| `src/components/queries.rs` | - | Query type aliases for common filters | Engine |
| `src/components/utilities.rs` | - | Helper functions: ore_name, ore_laser_required, berth_world_pos | Engine |
| `src/components/events.rs` | - | Event definitions | Engine |
| `src/systems/persistence/mod.rs` | - | Persistence module exports | Engine |
| `src/systems/persistence/io.rs` | - | File I/O operations | Engine |
| `src/systems/persistence/save.rs` | - | SaveData struct, save/load logic, file paths | Engine |
| `src/systems/persistence/schema.rs` | - | Save schema definitions | Engine |
| `src/systems/persistence/systems.rs` | 164 | Autosave, save request systems | Engine |
| `src/systems/setup/mod.rs` | - | Setup module exports | Engine |
| `src/systems/setup/mesh_builder.rs` | - | Mesh building utilities | Engine |
| `src/systems/setup/world_spawn.rs` | 136 | setup_world entry, starfield/camera spawn, cleanup/reset | Engine |
| `src/systems/setup/entity_setup.rs` | 411 | Station, berth, opening drone, highlight spawn functions | Engine |
| `src/systems/setup/quest_init.rs` | 28 | Quest log initialization from config | Engine |
| `src/ui_kit/mod.rs` | - | UI kit module exports | Presentation (reusable) |
| `src/ui_kit/styles.rs` | 37 | ButtonStyle, HighlightKind enums | Presentation (reusable) |
| `src/ui_kit/primitives.rs` | 59 | vd_button function with amber pulse highlight | Presentation (reusable) |

### Layer 2: Game (Mechanics)

| File | Lines | Responsibility | Ownership |
| :--- | :--- | :--- | :--- |
| `src/systems/game_loop/mod.rs` | 6 | Game loop module exports | Game |
| `src/systems/game_loop/mining.rs` | 188 | Ore extraction, laser tier gate, beam scaling, depletion | Game |
| `src/systems/game_loop/auto_process.rs` | 145 | Auto-refine, auto-forge, auto-build-drones | Game |
| `src/systems/game_loop/autonomous.rs` | 128 | Autonomous drone FSM (Outbound/Mining/Returning/Unloading/Holding) | Game |
| `src/systems/game_loop/scout_dispatch.rs` | 338 | Scout spawn, orbit-paint-dispatch, paint cleanup (3 systems) | Game |
| `src/systems/game_loop/economy.rs` | 72 | Ship docked economy: cargo unload, request fulfillment, repair | Game |
| `src/systems/ship_control/mod.rs` | - | Ship control module exports | Game |
| `src/systems/ship_control/autopilot.rs` | 149 | Ship navigation, arrival detection, docking, bottle collection | Game |
| `src/systems/ship_control/asteroid_input.rs` | 126 | Touch/click input for manual asteroid targeting | Game |
| `src/systems/ship_control/ship_spawn.rs` | 170 | Drone ship spawning, bottle drone spawning | Game |
| `src/systems/asteroid/mod.rs` | 3 | Asteroid module exports | Game |
| `src/systems/asteroid/spawn.rs` | 254 | Asteroid spawning, procedural mesh generation, respawn | Game |
| `src/systems/asteroid/lifecycle.rs` | 34 | Asteroid lifecycle: lifespan timer, despawn | Game |
| `src/systems/narrative/mod.rs` | - | Narrative module exports | Game |
| `src/systems/narrative/signal.rs` | 214 | Signal firing based on game state (30+ triggers) | Game |
| `src/systems/narrative/opening_sequence.rs` | 139 | Opening cinematic FSM (6 phases) | Game |
| `src/systems/narrative/bottle.rs` | 155 | Bottle spawn timer, input, collection | Game |
| `src/systems/narrative/quest.rs` | 86 | Quest objective state and progress updates | Game |
| `src/systems/narrative/narrative_events.rs` | 49 | Narrative event handling (bottle, opening, laser) | Game |
| `src/systems/narrative/content_router.rs` | 152 | Echo content routing (event and ambient) | Game |
| `src/systems/narrative/logs.rs` | 51 | Log entry unlock checking | Game |

### Layer 3: Presentation (UI + Visuals)

| File | Lines | Responsibility | Ownership |
| :--- | :--- | :--- | :--- |
| `src/systems/ui/mod.rs` | - | UI module exports | Presentation |
| `src/systems/ui/hud/mod.rs` | 437 | egui HUD module exports, main hud_ui_system | Presentation |
| `src/systems/ui/hud/state_machine.rs` | 20 | Drawer state machine from game state transitions | Presentation |
| `src/systems/ui/hud/content.rs` | 551 | Production/Requests tab content, procedural symbol rendering, Scout toggle | Presentation |
| `src/systems/ui/hud/prod_tree.rs` | 450 | Production tree UI with zoom/pan, node rendering, arrow drawing | Presentation |
| `src/systems/ui/hud/overlays.rs` | 214 | Tutorial and telemetry opt-in painted overlays | Presentation |
| `src/systems/ui/hud/buttons.rs` | 64 | HUD buttons for fleet count, pipeline toggle, save, focus | Presentation |
| `src/systems/ui/station_tabs.rs` | 99 | Station tab rendering and queue cards | Presentation |
| `src/systems/ui/tutorial.rs` | 204 | Tutorial popup system and highlight positioning | Presentation |
| `src/systems/visuals/mod.rs` | - | Visuals module exports | Presentation |
| `src/systems/visuals/visuals.rs` | 207 | Thruster glow, ship rotation, starfield scroll, station rotation, berth occupancy | Presentation |
| `src/systems/visuals/map.rs` | 214 | Camera follow, map visibility, pinch zoom, map pan, map highlight | Presentation |
| `src/systems/visuals/viewport.rs` | 54 | Viewport system for egui CentralPanel mirroring | Presentation |
| `src/systems/visuals/ore_polygon.rs` | 64 | Procedural ore polygon rendering (egui painter) | Presentation |
| `src/systems/visuals/ingot_node.rs` | 68 | Procedural ingot node rendering (egui painter) | Presentation |
| `src/systems/visuals/component_nodes.rs` | 465 | Procedural component rendering (6 drawing functions) | Presentation |
| `src/systems/visuals/mesh_builder.rs` | 207 | Bevy mesh builders: polygons, ore bands, rocket parts | Presentation |
| `src/systems/visuals/debug_log.rs` | 43 | Debug logging system (global static mutex) | Presentation |
| `src/scenes/mod.rs` | 2 | Scenes module exports | Presentation |
| `src/scenes/main_menu.rs` | 273 | Main menu UI, save/load, starfield/camera spawn | Presentation |
| `src/scenes/restore.rs` | 201 | Game state restoration from save data | Presentation |
| `src/scenes/save_overlay.rs` | 100 | Save/load overlay UI system | Presentation |
| `src/scenes/menu_starfield.rs` | 73 | Menu starfield spawn and drift system | Presentation |

---

## Part 2: Scout Mk I Integration Trace

### Overview
The Scout Mk I automation feature follows ADR-019 (Orbit-Paint-Dispatch Pattern) and demonstrates solid architectural design with clear separation of concerns.

### Data Flow

```
UI Toggle (Hangar tab)
    ↓
ScoutEnabled.active toggled (resource in components/resources.rs)
    ↓
scout_spawn_system (game_loop/scout_dispatch.rs)
    ├─ if active && no Scout entity → spawn Scout entity
    └─ if !active → despawn Scout entity
    ↓
scout_orbit_system (game_loop/scout_dispatch.rs)
    ├─ Move Scout along circular orbit (ScoutOrbit component)
    ├─ Check proximity to Inner Ring asteroids
    ├─ If unpainted asteroid within threshold:
    │   ├─ Attach Painted component + spawn green Annulus ring
    │   ├─ Dispatch idle Mining drone (set Outbound state)
    │   ├─ Insert DroneTarget component on miner
    │   └─ Break (one paint per tick)
    └─ Skip already-painted asteroids (Without<Painted> query)
    ↓
Mining drone completes mission → returns to Holding state
    ↓
scout_paint_cleanup_system (game_loop/scout_dispatch.rs)
    ├─ Detect miner in Holding state with DroneTarget
    ├─ Despawn green Annulus ring
    ├─ Remove Painted from asteroid
    └─ Remove DroneTarget from miner (free for reassignment)
```

### Components

| Component | Location | Purpose |
| :--- | :--- | :--- |
| `ScoutEnabled` | components/resources.rs | Resource controlling Scout activation (active, unlocked) |
| `ScoutOrbit` | components/game_state.rs | On Scout entity: angle, radius, speed for orbit math |
| `Painted` | components/game_state.rs | On asteroid: marker with ring_entity for cleanup |
| `DroneTarget` | components/game_state.rs | On dispatched miner: stores asteroid entity reference |

### Systems

| System | File | Responsibility |
| :--- | :--- | :--- |
| `scout_spawn_system` | game_loop/scout_dispatch.rs | Spawn/despawn Scout entity based on ScoutEnabled |
| `scout_orbit_system` | game_loop/scout_dispatch.rs | Orbit movement, paint asteroids, dispatch miners |
| `scout_paint_cleanup_system` | game_loop/scout_dispatch.rs | Clear paint when miners return |

### Persistence

- **Save:** `systems/persistence/systems.rs` collects `scout_enabled.active` into SaveData
- **Load:** `scenes/restore.rs` restores ScoutEnabled from save data
- **UI:** `systems/ui/hud/content.rs` (lines 397-412) provides ON/OFF toggle button

### Assessment

**Strengths:**
- Clear 3-system separation (spawn, orbit, cleanup)
- Proper component usage (ScoutOrbit, Painted, DroneTarget)
- Resource-driven state (ScoutEnabled)
- Clean paint lifecycle (attach → dispatch → cleanup)
- Follows ADR-019 pattern exactly
- Well-documented with inline comments
- Comprehensive unit tests (15 test anchors in scout_dispatch.rs)

**Weaknesses:**
- None identified

**Conclusion:** Scout integration is architecturally sound and serves as a model for future autonomous agent features.

---

## Part 3: Tech Debt Inventory

### TD-001: auto_forge_system Inline Processing

**Location:** `src/systems/game_loop/auto_process.rs` (lines 45-85)

**Issue:** auto_forge_system processes inline by directly modifying station reserves (iron_ingots, tungsten_ingots, nickel_ingots) instead of using the ProcessingJob pattern like other production systems.

**Current Pattern:**
```rust
// Direct inline modification
station.iron_ingots -= actual_hull_batches * hull_cost;
station.hull_plate_reserves += actual_hull_batches;
```

**Expected Pattern:** (used by other production)
```rust
// Should use StationQueues with ProcessingJob
station.iron_ingots -= cost;
station_queues.hull_forge = Some(ProcessingJob { ... });
// processing_queue_system ticks timer each frame
```

**Impact:** 
- Inconsistent with other production systems
- No visual queue representation for forging
- Cannot be paused/cleared by player
- Violates established production pattern

**Fix:** Refactor to use StationQueues + ProcessingJob pattern (4-6 hours)

---

### TD-002: ARCHITECTURE.md Outdated Information

**Location:** `docs/ARCHITECTURE.md` (lines 126, 133)

**Issue:** System registration notes are outdated

**Documented (Incorrect):**
- "quest_update_system is NOT registered in either group"
- "autopilot_system registered twice"
- "station_visual_system registered twice"

**Actual (Verified via lib.rs):**
- quest_update_system IS registered at line 212
- autopilot_system registered ONCE at line 165
- station_visual_system registered ONCE at line 173

**Impact:** Misleading documentation for new developers

**Fix:** Update ARCHITECTURE.md with current system registration (30 minutes)

---

### TD-003: God Classes

**Location:** Multiple files

**Issue:** Several files exceed single-responsibility guidelines

| File | Lines | Mixed Concerns |
| :--- | :--- | :--- |
| `hud/mod.rs` | 437 | HUD exports, main hud_ui_system (1040 total with submodules) |
| `hud/content.rs` | 551 | Production, Requests, Logs tab rendering, Scout toggle |
| `resources.rs` | 278 | States, resources, station component, narrative resources |
| `save.rs` | 468 | Save data struct, save/load logic, file paths |
| `main_menu.rs` | 273 | Menu UI, save/load logic, starfield/camera spawning |
| `component_nodes.rs` | 465 | 6 different drawing functions (thruster, hull, canister, AI core, rocket, drone bay) |

**Impact:** Reduced maintainability, harder to locate code, cognitive load

**Fix:** Split into focused modules (8-12 hours total)

---

### TD-004: Code Duplication

**Location:** `ship_control/ship_spawn.rs` and `setup/entity_setup.rs`

**Issue:** Rocket spawning logic duplicated

**Duplicate Code:**
- `spawn_drone_ship` (ship_spawn.rs lines 50-121)
- `spawn_bottle_drone` (ship_spawn.rs lines 124-178)
- `spawn_opening_drone` (entity_setup.rs lines 50-121)
- `spawn_rocket_part` helper used in both files
- `spawn_drone_core_children` macro in entity_setup.rs

**Impact:** 
- Changes to rocket appearance require updates in multiple files
- Inconsistent child entity structure
- Maintenance burden

**Fix:** Extract to shared `rocket_spawner.rs` module (2-3 hours)

---

### TD-005: Hardcoded Values

**Location:** Multiple files

**Issue:** Game balance and logic hardcoded instead of config-driven

| File | Hardcoded Values |
| :--- | :--- |
| `signal.rs` | 30+ signal triggers with inline conditions |
| `mining.rs` | Laser tier validation (match on LaserTier enum) |
| `constants.rs` | Sector positions (S1, S2, S3 coordinates) |

**Impact:** 
- Balance changes require code changes
- No runtime tuning
- Violates config-driven design principle

**Fix:** Move to config files (balance.toml, content YAML) (4-6 hours)

---

## Part 4: Architectural Verdict

### Verdict: FIXABLE

**Rationale:**

1. **Layer 1/2/3 Boundaries Respected**
   - Clear dependency direction (Layer N depends only on Layer < N)
   - No circular dependencies detected
   - Well-documented in ADR-016

2. **Established Patterns Working**
   - Universal Disjointness (INV-004) preventing runtime panics
   - Module Structure (ADR-006) providing clear organization
   - System Partitioning (ADR-007) avoiding tuple limits
   - Scout integration (ADR-019) demonstrating good design

3. **Tech Debt Is Localized**
   - No systemic architectural flaws
   - All identified debt is fixable with targeted refactoring
   - No critical coupling issues
   - No performance bottlenecks from architecture

4. **Manageable Complexity**
   - 73 Rust files across 3 layers
   - Single-developer project scale
   - Clear ownership per module
   - Good documentation (ARCHITECTURE.md, ADRs)

5. **Scout Integration Validates Architecture**
   - New feature successfully integrated following established patterns
   - 3-system pattern works cleanly
   - Component/resource usage is consistent
   - Persistence integration is straightforward

### Recommended Action Plan

**Phase 1: Quick Wins (1-2 days)**
- Update ARCHITECTURE.md outdated notes (TD-002)
- Fix auto_forge_system to use ProcessingJob pattern (TD-001)

**Phase 2: Deduplication (1 day)**
- Extract rocket spawning to shared module (TD-004)

**Phase 3: Config Migration (1-2 days)**
- Move signal triggers to content YAML (TD-005)
- Move sector positions to balance.toml (TD-005)

**Phase 4: Module Splitting (2-3 days)**
- Split component_nodes.rs into separate drawing modules (TD-003)
- Split hud/content.rs by tab responsibility (TD-003)
- Split resources.rs by concern (TD-003)

**Total Estimated Effort:** 5-8 days

### Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
| :--- | :--- | :--- | :--- |
| Regression from refactoring | Low | Medium | Commit per phase, device testing |
| Breaking save compatibility | Low | High | Preserve SaveData schema, version bump |
| Introducing new bugs | Low | Medium | Unit tests for refactored code |
| Scope creep | Medium | Low | Strict phase boundaries |

---

## Part 5: Recommendations

### Immediate Actions

1. **Update ARCHITECTURE.md** - Remove outdated system registration notes
2. **Fix auto_forge_system** - Align with ProcessingJob pattern
3. **Document Scout pattern** - Add to ARCHITECTURE.md as reference for future autonomous agents

### Medium-Term Improvements

1. **Extract rocket_spawner.rs** - Consolidate duplicate rocket spawning logic
2. **Config-driven signals** - Move signal triggers to content YAML
3. **Split component_nodes.rs** - Separate drawing functions into focused modules

### Long-Term Considerations

1. **Bevy UI Migration** - Plan migration from bevy_egui to Bevy UI (mentioned in ARCHITECTURE.md)
2. **Query Optimization** - Create type aliases for common filter sets (VOIDRIFT_REFACTOR_ANALYSIS.md Option B)
3. **Module Organization** - Continue following Layer 1/2/3 boundaries for new features

---

## Part 6: Conclusion

Voidrift's architecture is **fundamentally sound** with clear separation of concerns, established patterns, and good documentation. The identified tech debt is **localized and fixable** with targeted refactoring. No redesign is required.

The Scout Mk I integration demonstrates that the architecture can successfully accommodate new features while maintaining clean boundaries and following established patterns.

**Next Steps:**
1. Execute Phase 1 quick wins (ARCHITECTURE.md update, auto_forge_system fix)
2. Proceed with remaining phases as development schedule permits
3. Continue following Layer 1/2/3 boundaries for all new features
4. Update documentation as patterns evolve

**Architecture Health Score:** 8/10 (Fixable with targeted refactoring)

---

*Review Complete: January 2025*
