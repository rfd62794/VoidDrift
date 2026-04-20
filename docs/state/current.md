# Voidrift — Current State
**Date:** April 2026
**Build:** v0.4.x (post Phase B + Tutorial UX)

## Test Floor
[no automated tests; verified on Moto G 2025 (API 35)]

## Completed Systems

### Gameplay & Logistics
| Function | File | Description |
| :--- | :--- | :--- |
| `autopilot_system` | `autopilot.rs` | Interpolates ship position toward `AutopilotTarget`. |
| `docked_ship_system` | `autopilot.rs` | Synchronizes docked ship transform with rotating berth position. |
| `mining_system` | `mining.rs` | Extracts ore from asteroids based on laser tier gates. |
| `autonomous_ship_system` | `autonomous.rs` | Finite state machine for AI mining drones. |
| `autonomous_beam_system` | `autonomous.rs` | Controls visibility of mining beams for autonomous units. |
| `docked_autonomous_ship_system` | `autonomous.rs` | specialized berth sync for AI units. |
| `station_status_system` | `economy.rs` | Monitors station health and power reserves. |
| `ship_self_preservation_system` | `economy.rs` | Handles emergency power recovery and return-to-base for player. |
| `station_maintenance_system` | `economy.rs` | Ticks periodic power consumption for active station systems. |
| `processing_queue_system` | `economy.rs` | Ticks four parallel production queues for refinement and assembly. |
| `auto_dock_system` | `economy.rs` | Triggered auto-unload and auto-smelt logic upon arrival. |
| `quest_update_system` | `quest.rs` | Updates objective progress based on game events. |

### Station, Narrative & UI
| Function | File | Description |
| :--- | :--- | :--- |
| `hud_ui_system` | `ui.rs` | Primary egui implementation for all tabs, signals, and popups. |
| `station_visual_system` | `ui.rs` | Updates station hub color based on power status. |
| `ship_cargo_display_system` | `ui.rs` | Manages world-space cargo bar and pulsing fullness feedback. |
| `cargo_label_system` | `ui.rs` | Updates world-space text labels for ship cargo. |
| `autonomous_ship_cargo_display_system` | `ui.rs` | Specialized cargo bar for AI units. |
| `camera_follow_system` | `map.rs` | Manages camera constraints in Space vs Map view. |
| `map_input_system` | `map.rs` | Handles touch navigation and tap-to-intercept logic. |
| `pinch_zoom_system` | `map.rs` | Multi-touch zooming for strategic overview. |
| `map_pan_system` | `map.rs` | Single-touch panning restricted to Map View. |
| `opening_sequence_system` | `narrative.rs` | Cinematic intro orchestrator. |
| `signal_system` | `narrative.rs` | Narrative telemetry logic across 30+ trigger IDs. |
| `tutorial_system` | `narrative.rs` | Context-aware one-time instructional popups. |
| `starfield_scroll_system` | `visuals.rs` | Parallax star rendering. |
| `station_rotation_system` | `visuals.rs` | Physical rotation of the station hub and arm container. |
| `ship_rotation_system` | `visuals.rs` | Rotates ship meshes toward travel velocity. |
| `thruster_glow_system` | `visuals.rs` | State-based visibility for engine visual effects. |
| `berth_occupancy_system` | `visuals.rs` | Toggles visual indicators for station berths. |
| `setup_world` | `setup.rs` | Massive startup system for spawning all world entities. |

## Active Resources
*   **ClearColor**: Background clear color.
*   **CameraDelta**: Per-tick world-space camera movement (used for parallax).
*   **SignalLog**: Narrative history and fired ID tracking.
*   **SignalStripExpanded**: UI toggle for the bottom message log.
*   **OpeningSequence**: Cinematic phase and timer.
*   **ActiveStationTab**: Currently selected docking screen.
*   **ForgeSettings**: Settings for batch production.
*   **AutoDockSettings**: User preferences for auto-unload/smelt.
*   **QuestLog**: Objective list and progression state.
*   **TutorialState**: Contextual guidance tracking (shown IDs + active popup).
*   **MapPanState**: Persistent touch position for map dragging.

## Active Components  
*   **Ship**: Primary player state (cargo, power, laser tier).
*   **Station**: Primary base state (reserves, rotation, dock state).
*   **AsteroidField**: Ore type, deposit amount, and depletion status.
*   **AutopilotTarget**: Navigation destination and target entity.
*   **AutonomousShip**: State and assignment for AI units.
*   **DockedAt**: Logical link between a ship and its berth entity.
*   **Berth**: Station slot data (arm index, occupancy).
*   **MapElement**: Marker for visibility toggling in map mode.
*   **MainCamera**: Primary orthographic camera.
*   **StationQueues**: Collection of four parallel industrial jobs.
*   **PlayerShip**: Identity marker for disjointness filtering.
*   **AutonomousShipTag**: Identity marker for AI units.
*   **CargoBarFill / ShipCargoBarFill**: HUD scale indicators.

## Known Issues
*   **Massive setup_world**: `setup.rs` lines 9-496 (487 lines) handles too many responsibilities.
*   **Component Redundancy**: `CargoBarFill` and `ShipCargoBarFill` serve near-identical purposes and should be unified.
*   **Oversized Files**: `ui.rs` (405), `narrative.rs` (407).
*   **Universal Disjointness Verbosity**: System queries are becoming extremely long due to mandatory `Without<>` filters.

## Open Directives
*   **Documentation Refactor (Part 1)**: [ACTIVE] Aligning docs with code.

## Next Queued Work
*   Code Refactor (Part 2): Component decomposition and file splitting.
*   Directive B: Ship hull material upgrades and multi-device scaling pass.
