# Voidrift — Architecture
**Date:** April 2026
**Source:** Read from `src/` — Layer 1 document. Do not write design vision here.

---

## Technology Stack

| Component | Version | Notes |
| :--- | :--- | :--- |
| Engine | Bevy 0.15.3 | Pinned. Do not upgrade without new directive. |
| UI | bevy_egui 0.33 / egui 0.31 | Current HUD framework. Migration to Bevy UI planned. |
| Logging | bevy_log 0.15.3 | Separate crate, not bundled with default features. |
| RNG | rand 0.8 | Seeded procedural mesh generation only. |
| Platform | Android API 35 | Primary target: Moto G 2025, Mali-G57 GPU. |
| NDK | r29 | cargo-ndk toolchain. |
| Build mode | `android-game-activity` | GameActivity, not NativeActivity. |

Bevy is configured with `default-features = false`. Active features: `bevy_winit`, `bevy_window`, `bevy_render`, `bevy_sprite`, `bevy_text`, `bevy_state`, `bevy_asset`, `bevy_core_pipeline`, `android_shared_stdcxx`, `android-game-activity`.

**Notable absences:** `bevy_ui`, `bevy_picking` — not yet enabled. Required for the Bevy UI migration.

---

## Module Structure

*Reflects post-Phase 5 refactor. Each file has a single, documented responsibility.*

| File | Lines | Responsibility |
| :--- | :--- | :--- |
| `src/lib.rs` | ~140 | App setup, plugin registration, resource init, system scheduling only. No logic. |
| `src/constants.rs` | ~130 | All game constants — single source of truth for tuning. |
| `src/components/` | — | Split across focused files (see below). No logic in any file. |
| `src/components/game_state.rs` | — | `Station`, `Ship`, `AutonomousShip`, `Berth`, `StationQueues` structs. |
| `src/components/markers.rs` | — | Marker components: `StarLayer`, `MapMarker`, `MapElement`, `AutopilotTarget`, etc. |
| `src/components/resources.rs` | — | ECS Resources: `SignalLog`, `ShipQueue`, `CameraDelta`, `RequestsTabState`, etc. |
| `src/components/ui_state.rs` | — | UI-only state structs: `ActiveStationTab`, `DrawerState`, `ProductionTabState`, `CollectedRequest`. |
| `src/components/utilities.rs` | — | Helper functions: `ore_name`, `ore_laser_required`, `berth_world_pos`. |
| `src/systems/setup/entity_setup.rs` | — | Station, berth, opening drone, destination highlight spawn functions. |
| `src/systems/setup/world_spawn.rs` | — | `setup_world` entry point, starfield, camera spawn. |
| `src/systems/setup/quest_init.rs` | — | Quest log initialization. |
| `src/systems/asteroid/spawn.rs` | — | `spawn_initial_asteroids`, `asteroid_respawn_system`, `spawn_asteroid`. |
| `src/systems/asteroid/lifecycle.rs` | — | `asteroid_lifecycle_system` — lifespan timer, despawn, drone-target pause. |
| `src/systems/game_loop/mining.rs` | — | Ore extraction, laser tier gate, beam scaling, depletion coloring. `power_multiplier` applied here. |
| `src/systems/game_loop/auto_process.rs` | — | Auto-refine, auto-forge, auto-build-drones. |
| `src/systems/game_loop/autonomous.rs` | — | Autonomous drone FSM (Outbound / Mining / Returning / Unloading / Holding). |
| `src/systems/ship_control/autopilot.rs` | — | Player ship navigation, arrival detection, docking, bottle collection (⚠️ Phase 3 target for extraction). |
| `src/systems/ship_control/asteroid_input.rs` | — | Touch input for manual asteroid targeting. |
| `src/systems/ui/hud/mod.rs` | — | egui HUD entry: signal strip, drawer, tab routing. |
| `src/systems/ui/hud/content.rs` | — | PRODUCTION and REQUESTS tab content, fulfillment logic, upgrade writes. |
| `src/systems/ui/station_tabs.rs` | — | Station drawer tab definitions. |
| `src/systems/ui/tutorial.rs` | — | Tutorial popup system. |
| `src/systems/narrative/opening_sequence.rs` | — | Opening cinematic FSM and drone movement. |
| `src/systems/narrative/signal.rs` | — | Signal log fire conditions and quest advancement. |
| `src/systems/narrative/bottle.rs` | — | Bottle spawn timer, tap input, collection output. |
| `src/systems/narrative/quest.rs` | — | Quest log update system. |
| `src/systems/persistence/save.rs` | — | `SaveData`, autosave system, save request handler. |
| `src/systems/visuals/visuals.rs` | — | Starfield parallax (absolute positioning), station rotation FSM, ship heading, thruster glow, berth colors. |
| `src/systems/visuals/map.rs` | — | Camera follow, map visibility, pinch zoom, map pan. |
| `src/systems/visuals/viewport.rs` | — | `ui_layout_system`, `drawer_viewport_system`. |

---

## System Execution Order

Systems are registered in `lib.rs` in three blocks.

### Startup
```
Startup: setup_world
```

### State Transitions
```
OnEnter(MapView):  enter_map_view, show_map_elements
OnExit(MapView):   exit_map_view, hide_map_elements
```

### Update — Group 1 (chained, visual/movement priority)
```
autopilot_system
camera_follow_system
starfield_scroll_system
station_rotation_system
docked_ship_system
docked_autonomous_ship_system   ← after rotation to prevent jitter
berth_occupancy_system
station_visual_system
ship_rotation_system
thruster_glow_system
```
All chained with `.chain()` — execute in sequence each frame.

### Update — Group 2 (gameplay, economy, UI)
```
mining_system
autopilot_system               ← registered twice (known issue)
autonomous_ship_system
autonomous_beam_system         ← .after(autonomous_ship_system)
ship_cargo_display_system
autonomous_ship_cargo_display_system
cargo_label_system
hud_ui_system
station_visual_system          ← registered twice (known issue)
station_status_system
station_maintenance_system
ship_self_preservation_system
processing_queue_system
auto_dock_system
map_highlight_system
map_input_system
pinch_zoom_system
map_pan_system
opening_sequence_system
signal_system
tutorial_system
```

**Note:** `quest_update_system` (defined in `quest.rs`) is **not registered** in either group. This is a known bug — Objective 3 progress bar does not update at runtime.

**INV-005 (Tuple Partition):** Bevy's Update schedule has a 20-system tuple limit. The two groups exist to stay within this limit. Never add systems without checking current group sizes.

---

## ECS Constraints

### INV-004: Universal Disjointness (Total Lockdown)

Every system that queries `&mut Transform` MUST include `Without<T>` filters for all major entity types that other concurrent queries might access. Violating this causes `B0001` runtime panics on Android (Mali-G57) that `cargo check` does not catch.

Canonical filter sets by entity type:

| Query Target | Required Without Filters |
| :--- | :--- |
| Ship (`&mut Transform`) | `Without<Station>`, `Without<AsteroidField>`, `Without<MiningBeam>`, `Without<MainCamera>`, `Without<StarLayer>`, `Without<StationVisualsContainer>`, `Without<DestinationHighlight>`, `Without<ShipCargoBarFill>`, `Without<Berth>` |
| Station (`&mut Transform`) | `Without<Ship>`, `Without<AutonomousShip>`, `Without<MiningBeam>`, `Without<MainCamera>`, `Without<StarLayer>`, `Without<StationVisualsContainer>`, `Without<AsteroidField>`, `Without<Berth>` |
| Beam (`&mut Transform`) | `Without<Ship>`, `Without<Station>`, `Without<AsteroidField>`, `Without<AutonomousShip>`, `Without<MainCamera>`, `Without<StarLayer>`, `Without<StationVisualsContainer>`, `Without<DestinationHighlight>`, `Without<ShipCargoBarFill>`, `Without<Berth>` |
| Camera (`&mut Transform`) | `Without<Ship>`, `Without<StarLayer>` |
| Stars (`&mut Transform`) | `Without<MainCamera>` |

### INV-006: DockedAt Pattern

Ships docked at a berth MUST have `DockedAt(Entity)` pointing to their berth. Both `docked_ship_system` and `docked_autonomous_ship_system` use this to re-lock world position to the rotating arm each tick. Never remove `DockedAt` without transitioning ship state to `Navigating` or `Idle` first.

### INV-007: One-Time Trigger Pattern

`SignalLog.fired` (HashSet) and `TutorialState.shown` (HashSet) are **never cleared** during a session. IDs 19, 20, 21, 28–33 are exceptions — implemented as refirable via `fire_refirable()` which removes from `fired` when reset condition is met.

### INV-008: AlphaMode2d::Opaque for Background Elements

All background mesh entities (stars, station arms, connectors, asteroid boundary rings, map icons) MUST use `AlphaMode2d::Opaque` in their `ColorMaterial`. Using `Blend` on the Mali-G57 causes Z-sorting flicker that Z-layer adjustment alone cannot fix. Achieve dimming through color values, not alpha.

---

## Entity Hierarchies

### Station Entity Tree
```
Station (MapMarker, Transform: Z_ENVIRONMENT = 0.5)
├── StationVisualsContainer (rotates each frame)
│   ├── StationHub (Circle mesh, Z local 0.0)
│   └── Arm × 6 (Rectangle mesh, Z local -0.1)
│       └── BerthVisual(arm_index) (Circle mesh, Z local +0.1) [arms 0–2 only]
├── MapElement (Circle, Z: Z_MAP_MARKERS = 0.6, hidden in SpaceView)
│   └── MapElement × 3 (arm spokes, inherited visibility)
└── MapElement (Text2d "BASE", hidden in SpaceView)

Berth(Player) — separate entity, arm_index 0
Berth(Drone)  — separate entity, arm_index 1
Berth(Open)   — separate entity, arm_index 2
```

### Player Ship Entity Tree
```
Ship (PlayerShip, LastHeading, Transform: Z_SHIP = 1.0)
├── ThrusterGlow (Rectangle, Z local +0.1, hidden when idle)
├── MiningBeam (Rectangle, Z: Z_BEAM - Z_SHIP = -0.2, hidden when not mining)
├── CargoBar background (Rectangle, Z local +0.1)
├── ShipCargoBarFill (Rectangle, Z local +0.15, scales on X)
├── MapElement (triangle mesh, hidden in SpaceView)
├── CargoOreLabel (Text2d, Z: Z_HUD - Z_SHIP)
└── CargoCountLabel (Text2d, Z: Z_HUD - Z_SHIP)
```

### Autonomous Ship Entity Tree
```
AutonomousShip (AutonomousShipTag, LastHeading, AutonomousAssignment, Transform: Z_SHIP)
├── ThrusterGlow
├── MiningBeam
├── CargoBar background
├── ShipCargoBarFill
└── MapElement (triangle, hidden in SpaceView)
```

### Asteroid Entity Tree
```
AsteroidField (MapMarker, Transform: Z_ENVIRONMENT = 0.5)
├── MapElement (Circle icon, hidden in SpaceView)
├── MapElement (Text2d sector ID label, hidden in SpaceView)
├── OreNameLabel (Text2d, world-space, always visible)
└── LaserReqLabel (Text2d, world-space, gated ores only)
```

---

## Sector Layout

| Sector | Position | Ore | Laser Required |
| :--- | :--- | :--- | :--- |
| S1 | (320, 140) | Magnetite | Basic |
| S2 | (-220, 340) | Iron | Basic |
| S3 | (380, -280) | Carbon | Basic |
| S4 | (-520, -380) | Tungsten | Tungsten |
| S5 | (680, 320) | Titanite | Tungsten |
| S6 | (-650, 520) | CrystalCore | Composite |

All sectors connected to STATION_POS (0,0) via `MapConnector` line meshes.

---

## Key Data Flows

### Signal Pipeline
```
Game condition met → signal_system calls fire_signal(id, text)
→ SignalLog.entries.push_back / fired.insert
→ hud_ui_system reads SignalLog.entries → renders to egui strip
→ signal_system reads SignalLog.fired → advances QuestLog states
```

### Tutorial Pipeline
```
Game condition met → tutorial_system checks TutorialState.shown
→ TutorialState.active = Some(popup)
→ hud_ui_system renders centered egui Window
→ Dismiss button → TutorialState.shown.insert(id), active = None
```

### Quest Progression
```
Signal fired → signal_system match on quest ID
→ QuestObjective.state set (Active / Complete)
→ quest_update_system updates progress_current (Objective 3 only)
→ hud_ui_system renders QuestLog panel
```

### Processing Queue
```
UI button click → economy::queue_job(station, queue, op, batches)
  → resources deducted upfront
  → ProcessingJob inserted into StationQueues slot
→ processing_queue_system ticks timer each frame
→ batch_time reached → output deposited, job.batches -= 1
→ queue emptied → StationQueues slot = None
```

### Docking Flow
```
map_input_system / opening_sequence: insert AutopilotTarget
→ autopilot_system moves ship toward berth_world_pos (recalculated each tick)
→ distance < ARRIVAL_THRESHOLD → ShipState::Docked
  → DockedAt(berth_entity) inserted
  → AutopilotTarget removed
  → auto_dock_system fires (RemovedComponents<AutopilotTarget>)
    → auto-unload cargo if enabled
    → auto-queue smelt if enabled
→ docked_ship_system locks ship position to berth each tick
```

### Station Rotation with Docking
```
station_rotation_system detects ship approaching (< 200 units)
→ dock_state: Rotating → Slowing (decelerate)
→ dock_state: Slowing → Paused (rotation_speed reaches 0)
→ on dock event: dock_state → Resuming (after STATION_RESUME_DELAY = 1.5s)
→ dock_state: Resuming → Rotating (accelerate back to STATION_ROTATION_SPEED)
```

---

## Rendering Architecture

### Z-Layer Table

| Constant | Value | Content |
| :--- | :--- | :--- |
| `Z_HUD` | 2.0 | World-space text labels, destination ring |
| `Z_CARGO_BAR` | 1.1 | Cargo bar meshes |
| `Z_SHIP` | 1.0 | Player and autonomous ship meshes |
| `Z_BEAM` | 0.8 | Mining laser beam mesh |
| `Z_MAP_MARKERS` | 0.6 | Map mode icons and labels |
| `Z_ENVIRONMENT` | 0.5 | Station hub, asteroid fields |
| `Z_CONNECTORS` | -5.0 | Map route line meshes |
| `Z_STARS_NEAR` | -50.0 | Parallax star layer (factor 0.15) |
| `Z_STARS_FAR` | -100.0 | Parallax star layer (factor 0.05) |

Camera is placed at Z = 1000.0 with `far = 1200.0` to see `Z_STARS_FAR`.

### AlphaMode2d Rationale
All background mesh entities use `AlphaMode2d::Opaque`. On Mali-G57 (Moto G 2025), `Blend` mode triggers depth-sort flicker that cannot be resolved by Z-layer adjustment. Color value is used to achieve visual dimming instead of alpha channel.

### Starfield
- 800 far stars (2×2 px, factor 0.05) at `Z_STARS_FAR`
- 300 near stars (3×3 px, factor 0.15) at `Z_STARS_NEAR`
- Seeded with `0xDEAD_BEEF` for deterministic layout
- Generated within circular radius of 2400.0 world units centered on Station
- Positioned using absolute offset formula: `pos = orig_pos + cam_pos * (1.0 - layer)` — no wrapping, no delta accumulation
- `StarLayer` component stores `{ layer: f32, orig_pos: Vec2 }` — `orig_pos` is the generation-time world position

---

## Input Architecture

### Touch Event Routing
```
TouchInput event (raw)
│
├── pinch_zoom_system  — 2 fingers: adjust OrthographicProjection.scale
│   └── suppresses single-touch inputs while active
│
├── map_pan_system     — 1 finger: pan camera offset
│   ├── SpaceView: breaks ship follow-focus, sets pan offset to ship pos
│   └── MapView: pans around STATION_POS
│
└── map_input_system   — 1 finger tap on MapMarker (80px radius hit test)
    └── dispatches AutopilotTarget + transitions to SpaceView if in MapView
```

### Camera Behavior
- **SpaceView (focused):** camera follows ship transform.
- **SpaceView (panned):** camera follows `cumulative_offset` (broken from ship).
- **MapView:** camera follows `STATION_POS + cumulative_offset`.
- **FOCUS button:** resets `is_focused = true`, `cumulative_offset = Vec2::ZERO`, `projection.scale = 1.0`.

### Opening Sequence Input Lock
All touch input systems check `opening.phase != OpeningPhase::Complete` and return early during the cinematic intro.

---

## Known Architectural Strain (Phase 3 Target)

The following patterns were identified during Phase 2 and are queued for Phase 3 refactor:

**1. God Systems**
`autopilot.rs` handles navigation geometry, state machine transitions, docking sequences, AND narrative bottle collection. A bracket in the wrong place broke the narrative pipeline silently — the `CarryingBottle` branch was unreachable for the entire Phase 2 sprint. Systems with multiple unrelated responsibilities produce bugs that compile correctly and fail at runtime.

**2. UI as Business Logic**
`content.rs` directly mutates `station.power_multiplier += 0.25` on fulfillment. No central economy system enforces what that mutation means downstream. The multiplier was written by the UI and read by nothing for multiple sprints before it was manually wired to `mining.rs`.

**3. Hardcoded Fallbacks**
Mining and autopilot systems implicitly rescue drones when things go wrong (e.g., defaulting to `ShipState::Mining` when an entity disappears). This creates silent error-recovery loops that look correct until they accumulate.

**4. Initialization Scatter**
Legacy `spawn_sectors` and new `spawn_initial_asteroids` ran simultaneously in the same schedule phase without coordination, causing 5–6 asteroids to spawn instead of 3. Fixed in Phase 2 by removing the legacy spawner.

**Phase 3 Target Architecture:**
```
Event bus pattern:
  autopilot.rs fires ArrivedAtTarget(Entity)
  → narrative system checks if entity is Bottle → fires collection event
  → upgrade system listens for fulfillment event → applies multipliers
  UI writes intent only → economy system enforces meaning
  Initialization: spawn_initial_asteroids is sole asteroid spawner
```
