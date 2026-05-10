# Voidrift — Current State
**Date:** May 2026
**Build:** v3.1.0-sprint5-visual-overhaul
**Architecture:** Layer 1/2/3 (Engine/Game/Presentation)
**Source:** Read from `src/` — Layer 1 document. Do not write design vision here.

## Test Floor
No automated tests. All verification performed on physical hardware: Moto G 2025 (Android API 35, Mali-G57 GPU).

---

## Architecture Overview

The codebase is organized into three layers with strict dependency rules:

### Layer 1: Engine (Infrastructure)
**Directories:** `src/lib.rs`, `src/config/`, `src/components/`, `src/systems/persistence/`, `src/systems/setup/`
**Responsibility:** Core infrastructure — app setup, config loading, ECS components, save/load, entity spawning
**Dependencies:** None (base layer)
**Dependents:** Layer 2 and Layer 3

### Layer 2: Game (Mechanics)
**Directories:** `src/systems/game_loop/`, `src/systems/ship_control/`, `src/systems/asteroid/`, `src/systems/narrative/`
**Responsibility:** Gameplay logic — mining, refining, autonomous ships, narrative, quest progression
**Dependencies:** Layer 1 only
**Dependents:** Layer 3

### Layer 3: Presentation (UI + Visuals)
**Directories:** `src/systems/ui/`, `src/systems/visuals/`, `src/scenes/`
**Responsibility:** Rendering and interface — HUD, menus, visual effects, camera
**Dependencies:** Layer 1 and Layer 2
**Dependents:** None (top layer)

---

## System Inventory

### Layer 1: Engine (Infrastructure)

| Function | File | Purpose |
| :--- | :--- | :--- |
| `cleanup_world_entities` | `world_spawn.rs` | Removes all entities with Transform before new game |
| `reset_game_resources` | `world_spawn.rs` | Resets all runtime resources to clean state |
| `setup_world` | `world_spawn.rs` | Spawns starfield, camera, opening drone, station, berths, highlights |
| `spawn_starfield` | `world_spawn.rs` | Spawns procedural starfield with parallax layers |
| `spawn_camera` | `world_spawn.rs` | Spawns main camera with egui context settings |
| `spawn_opening_drone` | `entity_setup.rs` | Spawns opening sequence drone with rocket mesh |
| `spawn_station` | `entity_setup.rs` | Spawns station hub, arms, and visual containers |
| `spawn_berths` | `entity_setup.rs` | Spawns 6 berth entities around station |
| `spawn_destination_highlight` | `entity_setup.rs` | Spawns destination highlight ring |
| `spawn_tutorial_highlight` | `entity_setup.rs` | Spawns tutorial highlight ring |
| `init_quest_log` | `quest_init.rs` | Initializes quest log from config |

### Layer 2: Game (Mechanics)

#### Game Loop
| Function | File | Purpose |
| :--- | :--- | :--- |
| `mining_system` | `mining.rs` | Ore extraction, laser tier gate, beam scaling, depletion coloring |
| `auto_refine_system` | `auto_process.rs` | Auto-process ore into ingots (iron, tungsten, nickel, aluminum) |
| `auto_forge_system` | `auto_process.rs` | Auto-forge components (hull plates, thrusters, AI cores, canisters) |
| `auto_build_drones_system` | `auto_process.rs` | Continuously assembles drones from components |
| `autonomous_ship_system` | `autonomous.rs` | 5-state FSM: Holding → Outbound → Mining → Returning → Unloading |
| `autonomous_beam_system` | `autonomous.rs` | Scales and positions AI mining beam during Mining state |
| `ship_docked_economy_system` | `economy.rs` | Handles cargo unload, request fulfillment, repair, dispatch events |

#### Ship Control
| Function | File | Purpose |
| :--- | :--- | :--- |
| `autopilot_system` | `autopilot.rs` | Moves ships toward AutopilotTarget; resolves docking on arrival |
| `asteroid_input_system` | `asteroid_input.rs` | Touch/click input for manual asteroid targeting |
| `spawn_drone_ship` | `ship_spawn.rs` | Spawns drone ship entity with rocket mesh |
| `spawn_bottle_drone` | `ship_spawn.rs` | Spawns drone ship for bottle collection |

#### Asteroid
| Function | File | Purpose |
| :--- | :--- | :--- |
| `spawn_initial_asteroids` | `spawn.rs` | Spawns initial asteroids (one of each type) |
| `spawn_asteroid` | `spawn.rs` | Spawns single asteroid with procedural mesh and map elements |
| `asteroid_respawn_system` | `spawn.rs` | Respawns asteroids on timer, respects max_active_asteroids |
| `asteroid_lifecycle_system` | `lifecycle.rs` | Despawns depleted asteroids, manages lifespan |

#### Narrative
| Function | File | Purpose |
| :--- | :--- | :--- |
| `signal_system` | `signal.rs` | Fires narrative signals based on game state (30+ triggers) |
| `opening_sequence_system` | `opening_sequence.rs` | 6-phase cinematic intro |
| `quest_signal_system` | `quest.rs` | Updates quest objective states based on SignalFired events |
| `quest_update_system` | `quest.rs` | Updates real-time quest progress bars |
| `narrative_event_system` | `narrative_events.rs` | Handles bottle, opening, laser, narrative events |
| `content_event_system` | `content_router.rs` | Fires Echo content based on game events |
| `content_ambient_system` | `content_router.rs` | Fires ambient Echo lines on timer |
| `bottle_spawn_system` | `bottle.rs` | Spawns narrative bottle entity on timer |
| `bottle_input_system` | `bottle.rs` | Touch/click input for bottle collection |
| `check_log_unlocks` | `logs.rs` | Checks and unlocks log entries based on triggers |

### Layer 3: Presentation (UI + Visuals)

#### UI
| Function | File | Purpose |
| :--- | :--- | :--- |
| `hud_ui_system` | `hud/mod.rs` | Primary egui pass: signal strip, drawer, tabs, production tree, tutorial |
| `render_tab_content` | `hud/content.rs` | Renders Production and Requests tab content with procedural symbols |
| `ship_cargo_display_system` | `hud/mod.rs` | Scales ship cargo bar mesh; pulses cyan at ≥95% capacity |
| `cargo_label_system` | `hud/mod.rs` | Updates ore name and count labels on ship |
| `station_visual_system` | `hud/mod.rs` | Switches station hub color (online/offline) |
| `sync_max_drones_system` | `hud/mod.rs` | Syncs station.max_dispatch to MaxDispatch resource |
| `update_drawer_state` | `hud/state_machine.rs` | Drives drawer state from game state transitions |
| `add_log_entry` | `station_tabs.rs` | Adds log entry to station log with max line limit |
| `render_queue_card` | `station_tabs.rs` | Renders processing queue card with progress |
| `tutorial_system` | `tutorial.rs` | Fires tutorial popups based on game conditions |

#### Visuals
| Function | File | Purpose |
| :--- | :--- | :--- |
| `thruster_glow_system` | `visuals.rs` | Shows/hides thruster glow based on ship movement |
| `ship_rotation_system` | `visuals.rs` | Rotates ships to face travel direction |
| `starfield_scroll_system` | `visuals.rs` | Parallax-scrolls star entities anchored to station |
| `station_rotation_system` | `visuals.rs` | State-machine rotation: Rotating → Slowing → Paused → Resuming |
| `camera_follow_system` | `map.rs` | Tracks ship in SpaceView, tracks station in MapView |
| `show_map_elements` | `map.rs` | Shows map elements in MapView |
| `hide_map_elements` | `map.rs` | Hides map elements in SpaceView |
| `map_highlight_system` | `map.rs` | Shows destination highlight ring at autopilot target |
| `pinch_zoom_system` | `map.rs` | Two-finger pinch adjusts camera zoom |
| `drawer_viewport_system` | `viewport.rs` | Sets camera viewport to match egui CentralPanel rect |

#### Scenes
| Function | File | Purpose |
| :--- | :--- | :--- |
| `setup_main_menu` | `main_menu.rs` | Spawns menu camera and starfield, loads save lists |
| `main_menu_system` | `main_menu.rs` | Renders main menu UI with play/stage/developer modes |

---

## Component Inventory

### Data Components (on entities)

| Component | Fields | Purpose |
| :--- | :--- | :--- |
| `Ship` | state, speed, cargo, cargo_type, cargo_capacity, laser_tier, current_mining_target | Player ship runtime state |
| `Station` | repair_progress, online, iron/tungsten/nickel/aluminum reserves & ingots, hull_plate/thruster/ai_core/aluminum_canister reserves, drone_build_progress, drone_count, log, rotation, rotation_speed, dock_state, resume_timer, multipliers, max_dispatch, max_active_asteroids | All station runtime state including economy |
| `StationQueues` | iron/tungsten/nickel/aluminum refinery, hull/thruster/core/canister forge | 8 parallel `Option<ProcessingJob>` production slots |
| `ActiveAsteroid` | ore_type, ore_remaining, lifespan_timer | Asteroid mining state |
| `AutonomousShip` | state, cargo, cargo_type | AI drone runtime state |
| `AutonomousAssignment` | target_pos, ore_type, sector_name | AI drone current mission target |
| `DockedAt` | (Entity) | Links docked ship to its Berth entity |
| `Berth` | arm_index, berth_type | Logical docking slot (Player / Drone) |
| `ProcessingJob` | timer, batches, completed, clearing | Production queue job state |

### Marker Components (identity / visibility filtering)

| Component | Purpose |
| :--- | :--- |
| `MapMarker` | Entities that appear as map tap targets |
| `MapElement` | Entities hidden in SpaceView, visible in MapView |
| `MapIcon` | Child map icon mesh |
| `MapLabel` | Child map text label |
| `MapConnector` | Line mesh connecting map nodes |
| `DestinationHighlight` | White ring shown at autopilot destination |
| `MainCamera` | Primary orthographic camera |
| `MenuCamera` | Main menu camera |
| `StarLayer` | Parallax factor for star entities |
| `StationVisualsContainer` | Parent of rotating station meshes |
| `StationHub` | Station center circle mesh |
| `BerthVisual` | Berth circle mesh |
| `Berth` | Logical berth marker |
| `ShipCargoBarFill` | Cargo bar fill mesh |
| `ThrusterGlow` | Engine glow child mesh |
| `MiningBeam` | Mining laser child mesh |
| `AsteroidBody` | Asteroid mesh marker |
| `AsteroidBand` | Asteroid band mesh marker |
| `InOpeningSequence` | Opening sequence ship marker |
| `AutonomousShipTag` | AI ship marker |
| `TutorialHighlight` | Tutorial highlight ring |
| `AutopilotTarget` | Navigation command component |
| `LastHeading` | Persists ship rotation between frames |
| `ActiveBottle` | Narrative bottle marker |
| `CarryingBottle` | Ship carrying bottle marker |
| `MenuStar` | Menu starfield marker |

### Resources

| Resource | Purpose |
| :--- | :--- |
| `GameState` | SpaceView / MapView / Menu state |
| `AppState` | MainMenu / InGame state |
| `SignalLog` | Narrative message queue, fired IDs, timing |
| `QuestLog` | Quest objectives with state and progress |
| `OpeningSequence` | Cinematic phase and timer |
| `ShipQueue` | Available drone count for dispatch |
| `MaxDispatch` | Max dispatch limit for HUD display |
| `CameraDelta` | Per-frame camera movement for parallax |
| `WorldViewRect` | HUD drawer rect for viewport sync |
| `MapPanState` | Pan offset and focus state |
| `ForgeSettings` | Forge production toggles |
| `ProductionToggles` | Production system toggles |
| `TutorialState` | Tutorial popup state and shown IDs |
| `ActiveStationTab` | Current station tab (Cargo/Production/Requests/Logs) |
| `DrawerState` | Collapsed/Expanded state |
| `UiLayout` | UI layout dimensions |
| `ProductionTabState` | Production tab selected ore |
| `RequestsTabState` | Requests tab state and collected requests |
| `ContentState` | Echo content state and observed triggers |
| `DeviceType` | Desktop / Mobile device type |
| `BottleSpawnTimer` | Bottle spawn timer |
| `MainMenuState` | Menu state and save lists |
| `TelemetryConsent` | Opt-in telemetry consent |
| `TelemetrySessionCounter` | Session counter for re-prompt |

---

## Current Economy (as implemented)

### Ore Types (4)
- **Iron** (S1 sector) → Iron Ingot → Hull Plate
- **Tungsten** (S2 sector) → Tungsten Ingot → Thruster
- **Nickel** (S3 sector) → Nickel Ingot → AI Core
- **Aluminum** (outer ring) → Aluminum Ingot → Aluminum Canister

### Processing Operations (8)

| Operation | Input | Output | Time |
| :--- | :--- | :--- | :--- |
| Iron Refinery | 25 Iron | 1 Iron Ingot | 20s |
| Tungsten Refinery | 25 Tungsten | 1 Tungsten Ingot | 20s |
| Nickel Refinery | 25 Nickel | 1 Nickel Ingot | 20s |
| Aluminum Refinery | 25 Aluminum | 1 Aluminum Ingot | 20s |
| Hull Forge | 3 Iron Ingots | 1 Hull Plate | 30s |
| Thruster Forge | 3 Tungsten Ingots | 1 Thruster | 30s |
| Core Fabricator | 3 Nickel Ingots | 1 AI Core | 30s |
| Canister Forge | 3 Aluminum Ingots | 1 Aluminum Canister | 30s |

### Drone Assembly
- Cost: 1 Hull Plate + 1 Thruster + 1 AI Core + 1 Aluminum Canister
- Result: Spawns autonomous drone ship
- Location: Berth 2 (dedicated drone berth)

### Station Repair
- Repair cost: **25 Iron Ingots** consumed from station reserves
- Button in Cargo tab: only shown when `station.online == false`
- Repairs set `repair_progress = 1.0` and `online = true`

### Laser Tiers (3)
- **Tier 1**: Mines Iron ore only
- **Tier 2**: Mines Iron + Tungsten
- **Tier 3**: Mines Iron + Tungsten + Nickel + Aluminum

---

## Current Quest Chain (as implemented in `quest_init.rs` + `narrative.rs`)

| ID | Description | Activated By | Completed By |
| :--- | :--- | :--- | :--- |
| 1 | Locate the signal source | Start (Active) | Signal 4 (STRUCTURE DETECTED) |
| 2 | Dock at the derelict station | Signal 4 | Signal 5 (DOCKING COMPLETE) |
| 3 | Repair the station | Signal 5 | Signal 11 (STATION ONLINE); tracks iron_ingots progress toward 25 |
| 4 | Build an AI Core | Signal 11 | Signal 13 (AI CORE NOMINAL) |
| 5 | Discover Sector 3 | Signal 13 | Signal 14 (NICKEL SIGNATURE) |
| 6 | Mine Nickel | Signal 14 | Signal 16 (AI CORE COMPLETE) |
| 7 | Assemble autonomous ship | Signal 16 | Signal 17 (AUTONOMOUS UNIT LAUNCHED) |

Note: Objective 5 is initialized as `Active` (not `Locked`) in setup — this appears to be an intentional shortcut for expansion testing.

---

## Current Signal Triggers (as implemented in `signal.rs`)

### Opening Sequence Signals (one-time, phase-gated)

| ID | Text | Trigger |
| :--- | :--- | :--- |
| 1000 | `SIGNAL RECEIVED.` | Game start (always fires) |
| 1001 | `SOURCE IDENTIFIED. BEARING 047.` | SignalIdentified phase, timer ≥ 2s |
| 1002 | `MOVING TO INVESTIGATE.` | AutoPiloting phase |
| 1003 | `STRUCTURE DETECTED. DERELICT CLASS.` | InRange phase |
| 1004 | `DOCKING COMPLETE.` | Docked or Complete phase |
| 1005 | `POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.` | Docked phase, timer ≥ 1s |
| 1006 | `SYSTEM RESTORE REQUIRED. 25 IRON INGOTS NEEDED.` | Docked phase, timer ≥ 2s |
| 1007 | `RESTORE INITIATED. POWER GRID ONLINE.` | Repair complete |
| 1008 | `STATION ONLINE. PRODUCTION SYSTEMS ACTIVE.` | Station online |
| 1009 | `IRON REFINERY OPERATIONAL.` | Iron ingots > 0 |
| 1010 | `TUNGSTEN REFINERY OPERATIONAL.` | Tungsten ingots > 0 |
| 1011 | `NICKEL REFINERY OPERATIONAL.` | Nickel ingots > 0 |
| 1012 | `ALUMINUM REFINERY OPERATIONAL.` | Aluminum ingots > 0 |
| 1013 | `AI CORE NOMINAL. SECTOR 3 SCAN INITIATED.` | AI cores > 0 |
| 1014 | `NICKEL SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 3.` | 3s after signal 13 |
| 1015 | `HULL PLATE FABRICATED. FORGE AVAILABLE.` | Hull plates > 0 |
| 1016 | `THRUSTER FABRICATED. FORGE AVAILABLE.` | Thrusters > 0 |
| 1017 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.` | Drone count ≥ 1 |

### Post-Opening One-Time Signals

| ID | Text | Trigger Condition |
| :--- | :--- | :--- |
| 1 | `IRON ACQUIRED. REFINERY READY.` | iron_reserves > 0 |
| 2 | `TUNGSTEN ACQUIRED. REFINERY READY.` | tungsten_reserves > 0 |
| 3 | `NICKEL ACQUIRED. REFINERY READY.` | nickel_reserves > 0 |
| 4 | `ALUMINUM ACQUIRED. REFINERY READY.` | aluminum_reserves > 0 |
| 5 | `IRON INGOTS PRODUCED. REPAIR THRESHOLD: 25.` | iron_ingots > 0 |
| 6 | `REPAIR THRESHOLD MET. INITIATE WHEN READY.` | iron_ingots ≥ 25 |
| 7 | `POWER RESTORED. STATION ONLINE.` | station.online == true |
| 8 | `AI CORE FABRICATION NOW AVAILABLE.` | 2s after signal 7 |
| 9 | `AI CORE NOMINAL. SECTOR 3 SCAN INITIATED.` | ai_cores > 0 |
| 10 | `NICKEL SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 3.` | 3s after signal 9 |
| 11 | `HULL PLATE FABRICATED. FORGE AVAILABLE.` | hull_plate_reserves > 0 |
| 12 | `THRUSTER FABRICATED. FORGE AVAILABLE.` | thruster_reserves > 0 |
| 13 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.` | drone count ≥ 1 |
| 14 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 2 ASSIGNED.` | drone count ≥ 2 |
| 15 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 3 ASSIGNED.` | drone count ≥ 3 |

### Refirable Signals (reset on condition exit)

| ID | Text | Fires When | Resets When |
| :--- | :--- | :--- | :--- |
| 16 | `IRON RESERVES CRITICAL. MINING RUN REQUIRED.` | iron_ingots < 5 | iron_ingots ≥ 8 |
| 17 | `AUTONOMOUS UNIT HOLDING. COMPONENTS INSUFFICIENT.` | any drone Holding | no drones Holding |
| 18 | `AUTONOMOUS UNIT DISPATCHED.` | any drone active AND signal 17 fired | drone not active |
| 19 | `INCOMING VESSEL DETECTED. DOCKING SEQUENCE INITIATED.` | dock_state == Slowing | dock_state == Rotating |
| 20 | `ROTATION SUSPENDED. BERTH ALIGNED.` | dock_state == Paused | dock_state == Slowing |
| 21 | `DOCKING COMPLETE. ROTATION RESUMING.` | dock_state == Resuming | dock_state == Rotating |
| 22 | `VESSEL DEPARTED. BERTH CLEAR.` | Rotating + no ship docked + signal 21 fired | dock_state == Resuming |
| 23 | `INDUSTRIAL PROCESSING ACTIVE. PARALLEL QUEUES COMMENCED.` | any queue active | no queues active |
| 24 | `PROCESSING QUEUES EMPTY. PRODUCTION HALTED.` | no queues + signal 23 fired | any queue active |

**Note:** Signal triggers are currently hardcoded in `signal.rs`. A config-driven trigger system is planned (see issue #28).

---

## Current Department Structure (as implemented)

### Cargo Tab
Displays: Iron, Tungsten, Nickel, Aluminum reserves and ingots; Hull Plate, Thruster, AI Core, Aluminum Canister reserves.
Settings: None.
Action: REPAIR STATION button (25 Iron Ingots, shown only when offline).
Production Tree: 4×4 grid showing ore → ingot → component → drone bay pipeline.

### Production Tab
Displays: 8 queue cards (4 refineries + 4 forges):
- Iron Refinery: 25 Iron → 1 Iron Ingot (20s)
- Tungsten Refinery: 25 Tungsten → 1 Tungsten Ingot (20s)
- Nickel Refinery: 25 Nickel → 1 Nickel Ingot (20s)
- Aluminum Refinery: 25 Aluminum → 1 Aluminum Ingot (20s)
- Hull Forge: 3 Iron Ingots → 1 Hull Plate (30s)
- Thruster Forge: 3 Tungsten Ingots → 1 Thruster (30s)
- Core Fabricator: 3 Nickel Ingots → 1 AI Core (30s)
- Canister Forge: 3 Aluminum Ingots → 1 Aluminum Canister (30s)
Settings: Production toggles for each operation.
Action: Queue batches (non-refundable), clear queue.

### Requests Tab
Displays: Faction requests (not yet implemented in full).
Action: Fulfill requests (not yet implemented).

### Logs Tab
Displays: Narrative log entries unlocked via triggers.
Action: Read log entries (read-only).

### Signal Strip
Displays: ECHO AI communication at bottom of screen.
Action: Expand/collapse (64px collapsed, 180px expanded).

---

## Known Technical Issues

| Issue | Location | Severity |
| :--- | :--- | :--- |
| **`hud/mod.rs` god class** | `systems/ui/hud/mod.rs` (1040 lines) | **High** — TD-001: mixes multiple UI systems (cargo display, station visuals, production tree, tabs). Needs splitting into focused modules. |
| **`resources.rs` god class** | `components/resources.rs` (226 lines) | **High** — mixes states, resources, station component, narrative resources. Needs splitting into states/resources/station/narrative modules. |
| **`save.rs` god class** | `systems/persistence/save.rs` (468 lines) | **High** — mixes save data struct, save/load logic, file paths. Needs splitting into save_data/save_system/save_paths. |
| **`main_menu.rs` god class** | `scenes/main_menu.rs` (639 lines) | **High** — mixes menu UI, save/load logic, starfield/camera spawning. Needs splitting into menu_ui/save_load/menu_setup. |
| **`component_nodes.rs` large file** | `systems/visuals/component_nodes.rs` (466 lines) | Medium — contains 6 different drawing functions. Needs splitting into per-component files. |
| **Rocket spawning duplication** | `entity_setup.rs` and `ship_spawn.rs` | Medium — duplicate rocket spawning logic between opening drone and mission drone. Needs shared rocket_spawner.rs. |
| **Ore mesh generation duplication** | `entity_setup.rs` | Medium — duplicate mesh generation code for each ore type. Needs shared ore_mesh_builder.rs. |
| **Signal triggers hardcoded** | `systems/narrative/signal.rs` | Medium — 30+ hardcoded signal triggers with inline conditions. Needs config-driven trigger system. |
| **auto_forge processes inline** | `systems/game_loop/auto_process.rs` | Medium — auto_forge_system processes inline not via StationQueues. Tech debt. |
| **Laser tier validation hardcoded** | `systems/game_loop/mining.rs` | Medium — laser tier validation hardcoded (lines 33-38). Should be config-driven. |
| **ASTEROID_MAX_PER_FIELD too sparse** | `config/balance.toml` | Low — set to 1, too sparse for gameplay. Should be increased. |
| **Dead code** | `systems/visuals/viewport.rs` | Low — ui_layout_system is no-op function. Should be removed. |
| **Legacy tutorial beats** | `systems/ui/tutorial.rs` | Low — T-001 to T-006 never fire (opening ship despawned at Complete). Should be removed. |
| **Sector positions hardcoded** | `constants.rs` | Low — sector positions still in constants.rs. Should be moved to config. |
| **Color utilities in config** | `config/visual.rs` | Low — color conversion functions mixed with config. Should be in visual_utils.rs. |

---

## Open Directives (approved, not yet executed)

### Launch Blockers (ship before $4.99)
| Issue | Status | Description |
| :--- | :--- | :--- |
| #5 Android Store Assets | Pending | Android store assets for Google Play submission |
| #9 Audio Pass | Pending | Audio implementation (sound effects, ambient) |
| #7 Production Tree — Zoom/Scroll | Pending | Zoom/scroll functionality for production tree |
| #13 Tutorial Refinement & Symbol Bar Logic | Pending | Tutorial UX refinement and symbol bar logic |
| #18 ASTEROID_MAX_PER_FIELD too sparse | Pending | Increase asteroid density for better gameplay |
| #11 Bevy Community Post | Pending | Write Bevy community post for visibility |

### Structural Rework (dev branch, between sprints)
| Issue | Status | Description |
| :--- | :--- | :--- |
| #23 Layer 1: Split resources.rs | Pending | Split resources.rs into states/resources/station/narrative modules |
| #24 Layer 1: Split save.rs | Pending | Split save.rs into save_data/save_system/save_paths |
| #25 Layer 1: Move color utilities | Pending | Move color utilities out of visual.rs into visual_utils.rs |
| #26 Layer 2: Create shared rocket_spawner.rs | Pending | Eliminate entity_setup/ship_spawn duplication |
| #27 Layer 2: Create shared ore_mesh_builder.rs | Pending | Eliminate per-type mesh duplication |
| #28 Layer 2: Config-driven signal triggers | Pending | Replace hardcoded if-else chains in signal.rs |
| #29 Layer 2: Split auto_process.rs | Pending | Split into refinery/forge/drone_assembly modules |
| #30 Layer 3: Split main_menu.rs | Pending | Split into menu_ui/save_load/menu_setup |
| #31 Layer 3: Split component_nodes.rs | Pending | Split into per-component files |
| #32 Layer 3: Remove dead code | Pending | Remove ui_layout_system no-op, legacy T-001 to T-006 beats |
| #16 TD-001: hud/mod.rs God Class Refactor | Pending | Split hud/mod.rs into focused modules |
| #19 Laser tier validation hardcoded | Pending | Move laser tier validation to config |

### Deferred
| Issue | Status | Description |
| :--- | :--- | :--- |
| #12 AntColony GDD Stub | Deferred | AntColony game design document |
| #14 Remote Telemetry DB Tools | Deferred | Remote telemetry database tools |
| #8 Ore Banding Refinement | Deferred | Ore banding visual refinement |

---

## Architecture Decision Records (ADRs)

### Recent ADRs (May 2026)
| ADR | Topic | Status |
| :--- | :--- | :--- |
| ADR-012 | No-sprites procedural math visual language | Accepted |
| ADR-013 | Bevy mesh for world entities, egui painter for UI layer | Accepted |
| ADR-014 | Opt-in anonymous telemetry via FastAPI/SQLite | Accepted |
| ADR-015 | Canvas CSS approach for WASM iframe fullscreen | Accepted |
| ADR-016 | Layer 1/2/3 engine/game/presentation architecture | Accepted |
| ADR-017 | $4.99 premium pricing with high-intent filtering | Accepted |
| ADR-018 | Cargo bay chain layout as miniature pipeline view | Accepted |

### Historical ADRs
| ADR | Topic | Status |
| :--- | :--- | :--- |
| ADR-001 | Mandatory PresentMode::Fifo for Mali GPUs | Accepted |
| ADR-002 | Mesh2D world primitives | Accepted |
| ADR-003 | Bevy_egui HUD framework | Accepted |
| ADR-004 | Bevy 0.15 pinned version | Accepted |
| ADR-005 | Autonomous ship dedicated systems | Accepted |
| ADR-006 | Module structure | Accepted |
| ADR-007 | System partitioning | Accepted |
| ADR-008 | Universal disjointness (Mali-G57 crash fix) | Accepted |
| ADR-009 | Tutorial trigger pattern | Accepted |
| ADR-010 | Narrative scope | Accepted |
| ADR-011 | Event bus single responsibility principle | Accepted |

---

## Pricing Strategy

- **Price point:** $4.99 premium (one-time purchase, no microtransactions, no ads)
- **Platform:** itch.io only (niche audience, quality-focused)
- **Value proposition:** Complete game, no DLC, no microtransactions, no ads, one purchase
- **Filtering mechanism:** Higher price naturally filters for high-intent players
- **Rationale:** Quality-focused audience, meaningful revenue for solo development, no monetization complexity

See ADR-017 for full rationale.
