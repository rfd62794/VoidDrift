# Voidrift — Current State
**Date:** April 2026
**Build:** v0.4.x (post Phase 10 + Phase B)
**Source:** Read from `src/` — Layer 1 document. Do not write design vision here.

## Test Floor
No automated tests. All verification performed on physical hardware: Moto G 2025 (Android API 35, Mali-G57 GPU).

---

## System Inventory

### Group 1 — Gameplay & Logistics (chained tuple in `lib.rs`)

| Function | File | Purpose |
| :--- | :--- | :--- |
| `autopilot_system` | `autopilot.rs` | Moves the player ship toward `AutopilotTarget.destination` each tick; resolves docking on arrival. |
| `docked_ship_system` | `autopilot.rs` | Every tick, re-locks a docked player ship's `Transform` to its rotating berth world position. |
| `camera_follow_system` | `map.rs` | Drives camera position: tracks ship in SpaceView, tracks `STATION_POS + pan_offset` in MapView. |
| `starfield_scroll_system` | `visuals.rs` | Parallax-scrolls 200 star entities (two layers: factor 0.05 and 0.15); wraps at ±700/±500. |
| `station_rotation_system` | `visuals.rs` | State-machine rotation: Rotating → Slowing → Paused → Resuming on approach/departure. |
| `docked_autonomous_ship_system` | `autonomous.rs` | Locks docked AI ship transform to Berth 2 world position following station rotation. |
| `berth_occupancy_system` | `visuals.rs` | Updates berth circle colors: cyan (player), orange (drone), grey (empty). |
| `station_visual_system` | `ui.rs` | Switches station hub mesh color between gold (online) and dim brown (offline). |
| `ship_rotation_system` | `visuals.rs` | Rotates ship meshes to face travel direction; persists last heading via `LastHeading`. |
| `thruster_glow_system` | `visuals.rs` | Shows/hides `ThrusterGlow` child mesh based on ship movement state. |

### Group 2 — Station, Narrative & UI (second Update tuple in `lib.rs`)

| Function | File | Purpose |
| :--- | :--- | :--- |
| `mining_system` | `mining.rs` | Extracts ore at `MINING_RATE`; enforces laser tier gate; marks asteroids depleted on fill. |
| `autopilot_system` | `autopilot.rs` | *Registered twice* — also runs in Group 2 for input responsiveness. |
| `autonomous_ship_system` | `autonomous.rs` | 5-state FSM: Holding → Outbound → Mining → Returning → Unloading; deposits ore on arrival. |
| `autonomous_beam_system` | `autonomous.rs` | Scales and positions AI mining beam child mesh during Mining state. |
| `ship_cargo_display_system` | `ui.rs` | Scales `ShipCargoBarFill` mesh; pulses cyan at ≥95% capacity. |
| `autonomous_ship_cargo_display_system` | `ui.rs` | Same bar scaling for AI ship child mesh. |
| `cargo_label_system` | `ui.rs` | Updates `Text2d` ore name and count labels on player ship children. |
| `hud_ui_system` | `ui.rs` | Primary egui pass: signal strip, left panel, quest panel, tab detail panels, tutorial popup. |
| `station_visual_system` | `ui.rs` | *Also in Group 2* for consistent hub color after economic ticks. |
| `station_status_system` | `economy.rs` | Logs power warnings; logs autonomous unit holding state. |
| `station_maintenance_system` | `economy.rs` | Every 10s: consumes 1 power cell if station power < floor; logs suspension. |
| `ship_self_preservation_system` | `economy.rs` | Emergency power: consume onboard cell → emergency refine → force return to station. |
| `processing_queue_system` | `economy.rs` | Ticks four parallel `ProcessingJob` queues; deposits output to station reserves on batch complete. |
| `auto_dock_system` | `economy.rs` | On `AutopilotTarget` removal: auto-unload and optionally auto-queue smelt based on settings. |
| `map_highlight_system` | `map.rs` | Shows/hides `DestinationHighlight` ring at autopilot target position. |
| `map_input_system` | `map.rs` | Touch tap on `MapMarker`: dispatches autopilot target; suppressed during multi-touch. |
| `pinch_zoom_system` | `map.rs` | Two-finger pinch adjusts `OrthographicProjection.scale`; clamps to ZOOM_MIN/ZOOM_MAX. |
| `map_pan_system` | `map.rs` | Single-touch drag pans camera offset; breaks focus in SpaceView; suppressed during pinch. |
| `opening_sequence_system` | `narrative.rs` | 6-phase cinematic intro: Adrift → SignalIdentified → AutoPiloting → InRange → Docked → Complete. |
| `signal_system` | `narrative.rs` | Fires narrative signals (IDs 1–33) based on game state; updates quest log as side-effect. |
| `tutorial_system` | `narrative.rs` | Fires 6 contextual tutorial popups (T-001–T-006) based on game conditions; one-time only. |
| `quest_update_system` | `quest.rs` | Updates Objective 3 progress bar with current `station.power_cells` count. |

**Startup:**

| Function | File | Purpose |
| :--- | :--- | :--- |
| `setup_world` | `setup.rs` | Spawns all entities: starfield, camera, player ship, station, 6 asteroid sectors, map connectors, berths. |

---

## Component Inventory

### Data Components (on entities)

| Component | Fields | Purpose |
| :--- | :--- | :--- |
| `Ship` | state, speed, cargo, cargo_type, cargo_capacity, power, power_cells, laser_tier | All player ship runtime state. |
| `Station` | repair_progress, online, magnetite_reserves, carbon_reserves, hull_plate_reserves, ship_hulls, ai_cores, power_cells, power, maintenance_timer, dock_state, rotation, rotation_speed, resume_timer, log | All station runtime state including economy reserves. |
| `StationQueues` | magnetite_refinery, carbon_refinery, hull_forge, core_fabricator | Four parallel `Option<ProcessingJob>` production slots. |
| `AsteroidField` | ore_type (OreType), ore_deposit (OreDeposit), depleted | Ore source state. |
| `AutopilotTarget` | destination (Vec2), target_entity (Option<Entity>) | Navigation command; removed on arrival. |
| `AutonomousShip` | state, cargo, cargo_type, power | AI drone runtime state. |
| `AutonomousAssignment` | target_pos, ore_type, sector_name | AI drone current mission target. |
| `DockedAt` | (Entity) | Links docked ship to its `Berth` entity. |
| `Berth` | arm_index, occupied_by, berth_type | Logical docking slot (Player / Drone / Open). |
| `BerthVisual` | (u8 arm_index) | Mesh marker for berth circle color updates. |

### Marker Components (identity / visibility filtering)

| Component | Purpose |
| :--- | :--- |
| `PlayerShip` | Identifies the player ship for disjointness filters. |
| `AutonomousShipTag` | Identifies AI ships for disjointness filters. |
| `MainCamera` | Identifies the primary orthographic camera. |
| `MapMarker` | Entities that appear as map tap targets. |
| `MapElement` | Entities hidden in SpaceView, visible in MapView. |
| `MapIcon` | Child map icon mesh. |
| `MapLabel` | Child map text label. |
| `MapConnector` | Line mesh connecting map nodes. |
| `StationVisualsContainer` | Parent of all rotating station meshes. |
| `StationHub` | Station centre circle mesh. |
| `ThrusterGlow` | Engine glow child mesh. |
| `MiningBeam` | Mining laser child mesh. |
| `ShipCargoBarFill` | Cargo bar fill mesh (both player and AI ships). |
| `CargoBarFill` | *Unused duplicate* — exists in components.rs but never queried. |
| `CargoOreLabel` | World-space Text2d ore type label on player ship. |
| `CargoCountLabel` | World-space Text2d cargo count label on player ship. |
| `DestinationHighlight` | White ring shown at autopilot destination. |
| `StarLayer` | (f32 parallax_factor) — identifies star entities. |
| `LastHeading` | (f32) — persists ship rotation between frames. |

### Resources

| Resource | Purpose |
| :--- | :--- |
| `CameraDelta` | Per-frame world-space camera movement vector; used by parallax system. |
| `SignalLog` | Narrative message queue (VecDeque), fired-ID HashSet, refirable timing map. |
| `SignalStripExpanded` | (bool) — controls strip height between 60px and 180px. |
| `OpeningSequence` | Current cinematic phase and elapsed timer. |
| `ActiveStationTab` | Currently selected station tab (Reserves/Power/Smelter/Forge/ShipPort). |
| `ForgeSettings` | Batch quantity preference (One/Ten/All) — declared, not yet used in UI. |
| `AutoDockSettings` | auto_unload, auto_smelt_magnetite, auto_smelt_carbon toggles. |
| `QuestLog` | `Vec<QuestObjective>` with state (Locked/Active/Complete) and optional progress bars. |
| `TutorialState` | shown HashSet (fired IDs) + active Option<TutorialPopup>. |
| `MapPanState` | last_position, cumulative_offset, is_focused for pan/focus logic. |

---

## Current Economy (as implemented)

### Processing Operations

| Operation | Tab | Input | Output | Time | Power Cost |
| :--- | :--- | :--- | :--- | :--- | :--- |
| `MagnetiteRefinery` | REFINERY | 10 Magnetite | 1 Power Cell | 20s | 1 |
| `CarbonRefinery` | REFINERY | 5 Carbon | 1 Hull Plate | 30s | 2 |
| `HullForge` | FORGE | 3 Hull Plates | 1 Ship Hull | 45s | 3 |
| `CoreFabricator` | FORGE | 55 Power Cells | 1 AI Core | 60s | 5 |

### Station Repair
- Repair cost: **25 Power Cells** consumed from station reserves.
- Button in RESERVES tab: only shown when `station.online == false`.
- Repairs set `repair_progress = 1.0` and `online = true`.

### Ship Port
- **Assemble Autonomous Ship**: costs 1 Ship Hull + 1 AI Core; spawns drone entity.
- **Top Up Ship**: costs 3 Power Cells; adds 3 power cells to ship (max 5 onboard).

### Power Economy
- Station power: max 50.0, floor 10.0; consumed by processing; restored by maintenance.
- Ship power: max 10.0, floor 3.0; consumed by transit (1.0) and mining (2.0).
- Emergency refine: 10 Magnetite → power restored (no cell consumed).

---

## Current Quest Chain (as implemented in `setup.rs` + `narrative.rs`)

| ID | Description | Activated By | Completed By |
| :--- | :--- | :--- | :--- |
| 1 | Locate the signal source | Start (Active) | Signal 4 (STRUCTURE DETECTED) |
| 2 | Dock at the derelict station | Signal 4 | Signal 5 (DOCKING COMPLETE) |
| 3 | Repair the station | Signal 5 | Signal 11 (STATION ONLINE); tracks power_cells progress toward 25 |
| 4 | Build an AI Core | Signal 11 | Signal 13 (AI CORE NOMINAL) |
| 5 | Discover Sector 3 | Signal 13 | Signal 14 (CARBON SIGNATURE) |
| 6 | Mine Carbon 3 | Signal 14 | Signal 16 (SHIP HULL COMPLETE) |
| 7 | Assemble autonomous ship | Signal 16 | Signal 17 (AUTONOMOUS UNIT LAUNCHED) |

Note: Objective 5 is initialized as `Active` (not `Locked`) in setup — this appears to be an intentional shortcut for expansion testing.

---

## Current Signal Triggers (as implemented in `narrative.rs`)

### Opening Sequence Signals (one-time, phase-gated)

| ID | Text | Trigger |
| :--- | :--- | :--- |
| 1 | `SIGNAL RECEIVED.` | Game start (always fires) |
| 2 | `SOURCE IDENTIFIED. BEARING 047.` | SignalIdentified phase, timer ≥ 2s |
| 3 | `MOVING TO INVESTIGATE.` | AutoPiloting phase |
| 4 | `STRUCTURE DETECTED. DERELICT CLASS.` | InRange phase |
| 5 | `DOCKING COMPLETE.` | Docked or Complete phase |
| 6 | `POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.` | Docked phase, timer ≥ 1s |

### Post-Opening One-Time Signals

| ID | Text | Trigger Condition |
| :--- | :--- | :--- |
| 7 | `REPAIRS POSSIBLE. MATERIALS REQUIRED.` | Opening complete |
| 8 | `MAGNETITE ACQUIRED. REFINERY READY.` | magnetite_reserves > 0 |
| 9 | `POWER CELLS PRODUCED. REPAIR THRESHOLD: 25.` | power_cells > 0 |
| 10 | `REPAIR THRESHOLD MET. INITIATE WHEN READY.` | power_cells ≥ 25 |
| 11 | `POWER RESTORED. STATION ONLINE.` | station.online == true |
| 12 | `AI CORE FABRICATION NOW AVAILABLE.` | 2s after signal 11 |
| 13 | `AI CORE NOMINAL. SECTOR 7 SCAN INITIATED.` | ai_cores > 0 |
| 14 | `CARBON SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 7.` | 3s after signal 13 |
| 15 | `HULL PLATE FABRICATED. FORGE AVAILABLE.` | hull_plate_reserves > 0 |
| 16 | `SHIP HULL COMPLETE. ASSEMBLY POSSIBLE.` | ship_hulls > 0 |
| 17 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.` | auto_ship count ≥ 1 |
| 18 | `AUTONOMOUS UNIT LAUNCHED. SECTOR 7 ASSIGNED.` | auto_ship count ≥ 2 |
| 25 | `SMELTER OPERATIONAL. MANUAL MODE.` | station offline (first dock) |
| 26 | `FORGE OPERATIONAL. MANUAL MODE.` | 1s after signal 25 |
| 27 | `AUTOMATED SYSTEMS ONLINE.` | station.online == true (fires alongside 11) |

### Refirable Signals (reset on condition exit)

| ID | Text | Fires When | Resets When |
| :--- | :--- | :--- | :--- |
| 19 | `POWER RESERVES CRITICAL. MINING RUN REQUIRED.` | power_cells < 5 | power_cells ≥ 8 |
| 20 | `AUTONOMOUS UNIT HOLDING. POWER INSUFFICIENT.` | any drone Holding | no drones Holding |
| 21 | `AUTONOMOUS UNIT DISPATCHED.` | any drone active AND signal 20 fired | drone not active |
| 28 | `INCOMING VESSEL DETECTED. DOCKING SEQUENCE INITIATED.` | dock_state == Slowing | dock_state == Rotating |
| 29 | `ROTATION SUSPENDED. BERTH ALIGNED.` | dock_state == Paused | dock_state == Slowing |
| 30 | `DOCKING COMPLETE. ROTATION RESUMING.` | dock_state == Resuming | dock_state == Rotating |
| 31 | `VESSEL DEPARTED. BERTH CLEAR.` | Rotating + no ship docked + signal 30 fired | dock_state == Resuming |
| 32 | `INDUSTRIAL PROCESSING ACTIVE. PARALLEL QUEUES COMMENCED.` | any queue active | no queues active |
| 33 | `PROCESSING QUEUES EMPTY. PRODUCTION HALTED.` | no queues + signal 32 fired | any queue active |

---

## Current Department Structure (as implemented)

### RESERVES Tab
Displays: Magnetite, Carbon, Hull Plates, Power Cells, AI Cores, Ship Hulls.
Settings: Auto-Unload toggle, Auto-Smelt Magnetite toggle, Auto-Smelt Carbon toggle.
Action: REPAIR STATION button (25 Power Cells, shown only when offline).

### POWER Tab
Displays: Station power bar and Ship power bar side by side. Read-only.

### REFINERY Tab (labelled "SMELTER" in `ActiveStationTab` enum, "REFINERY" in UI)
Two queue cards:
- Magnetite × 10 → Power Cell × 1 (20s, 1 power)
- Carbon × 5 → Hull Plate × 1 (30s, 2 power)

### FORGE Tab
Two queue cards:
- Hull Plate × 3 → Ship Hull × 1 (45s, 3 power)
- Power Cell × 55 → AI Core × 1 (60s, 5 power)

### SHIP PORT Tab
- Assemble & Deploy Autonomous Ship (costs 1 Ship Hull + 1 AI Core)
- Top Up Ship (costs 3 Power Cells, max 5 onboard)

---

## Known Technical Issues

| Issue | Location | Severity |
| :--- | :--- | :--- |
| **`CargoBarFill` / `ShipCargoBarFill` duplicate** | `components.rs:127,157` | Low — `CargoBarFill` is never queried; dead code. Should be removed. |
| **`autopilot_system` registered twice** | `lib.rs:48,70` | Medium — runs in both chained Group 1 and Group 2; harmless but wasteful. |
| **`setup_world` monolith** | `setup.rs` (676 lines) | Medium — spawning, mesh generation, map connectors, and berths all in one function. |
| **`ui.rs` monolith** | `ui.rs` (454 lines) | Medium — signal strip, left panel, quest panel, all tabs, tutorial, and helpers in one file. |
| **`narrative.rs` monolith** | `narrative.rs` (450 lines) | Medium — opening sequence, signal system, and tutorial system should be separate modules. |
| **OreType cargo mapping** | `setup.rs:388–395` | Low — Iron, Tungsten, Titanite, CrystalCore all map to `OreType::Magnetite` in cargo logic; only Magnetite and Carbon are distinct cargo types. |
| **`ForgeSettings` declared but unused** | `components.rs:295–305` | Low — `ForgeQuantity` enum and `ForgeSettings` resource exist but quantity selection is not wired into `queue_job`. |
| **`quest_update_system` not registered** | `lib.rs` (not in either Update group) | **High** — `quest_update_system` in `quest.rs` is defined but never added to the app via `.add_systems`. Objective 3 progress bar does not update at runtime. |

---

## Open Directives (approved, not yet executed)

| Directive | Status | Description |
| :--- | :--- | :--- |
| **Bevy UI Migration + Code Refactor** | Approved — next | Replace egui HUD with native Bevy UI nodes; split oversized modules. |
| **Economy Redesign** | Designed — blocked on above | Three resource tracks, FORGE/CRAFTER rename, Repair Kits, Engine tiers, Stargate. See `docs/design/ECONOMY.md`. |
