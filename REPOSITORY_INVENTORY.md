# VoidDrift Repository Inventory

**Generated:** 2025-01-18  
**Repository:** c:\Github\VoidDrift  
**Branch:** dev  
**Latest Tag:** v3.0.10-tutorial-complete  
**Total Rust Files:** 73  
**Total Config Files:** 7  
**Total Document Files:** 121+  
**Total Build Scripts:** 13  

---

## Directory Tree

```
VoidDrift/
├── .cargo/
├── .claude/
├── .git/
├── .github/
│   └── (3 items)
├── android/
│   └── (6 items)
├── assets/
│   ├── content/
│   │   ├── echo.yaml
│   │   ├── logs.yaml
│   │   ├── objectives.yaml
│   │   ├── requests.yaml
│   │   └── tutorial.yaml
│   ├── fonts/
│   │   └── (1 item)
│   ├── balance.toml
│   ├── visual.toml
│   └── .gitkeep
├── context/
│   └── (1 item)
├── docs/
│   ├── adr/
│   │   └── (22 items)
│   ├── analysis/
│   │   └── (1 item)
│   ├── archive/
│   │   └── (34 items)
│   ├── design/
│   │   └── (4 items)
│   ├── directives/
│   │   └── (28 items)
│   ├── phases/
│   │   └── (14 items)
│   ├── state/
│   │   └── (1 item)
│   ├── AGENT_CONTRACT.md
│   ├── ARCHITECTURE.md
│   ├── CHANGELOG.md
│   ├── DEVELOPER.md
│   ├── DEVELOPMENT_PIPELINE.md
│   ├── FUTURE_DESIGN_NOTES.md
│   ├── GDD_v1_ARCHIVED.md
│   ├── GDD_v2.md
│   ├── NARRATIVE_JUSTIFICATION.md
│   ├── README.md
│   ├── VOIDRIFT_REFACTOR_ANALYSIS.md
│   ├── WASM_BUILD.md
│   ├── WASM_ITCH_SIZING.md
│   ├── WINDSURF.md
│   ├── codebase-audit-2026-05-10.md
│   ├── narrative_canon.md
│   └── roadmap.md
├── examples/
│   └── (0 items)
├── pkg/
│   └── (0 items)
├── rfd-telemetry/
│   └── (5 items)
├── saves/
│   └── (0 items)
├── screenshots/
│   └── (0 items)
├── scripts/
│   ├── OBS_SETUP.md
│   ├── gh_tools.ps1
│   ├── local_itch_preview.html
│   ├── obs_scene.json
│   ├── record_demo.ps1
│   ├── serve_wasm.py
│   ├── shot_guide.html
│   └── trim_demo.ps1
├── src/
│   ├── components/
│   │   ├── events.rs
│   │   ├── game_state.rs
│   │   ├── markers.rs
│   │   ├── mod.rs
│   │   ├── queries.rs
│   │   ├── resources.rs
│   │   ├── ui_state.rs
│   │   └── utilities.rs
│   ├── config/
│   │   ├── balance.rs
│   │   ├── content.rs
│   │   ├── mod.rs
│   │   └── visual.rs
│   ├── constants.rs
│   ├── lib.rs
│   ├── scenes/
│   │   ├── main_menu.rs
│   │   ├── menu_starfield.rs
│   │   ├── mod.rs
│   │   ├── restore.rs
│   │   └── save_overlay.rs
│   ├── systems/
│   │   ├── asteroid/
│   │   │   ├── lifecycle.rs
│   │   │   ├── mod.rs
│   │   │   └── spawn.rs
│   │   ├── game_loop/
│   │   │   ├── auto_process.rs
│   │   │   ├── autonomous.rs
│   │   │   ├── economy.rs
│   │   │   ├── mining.rs
│   │   │   ├── mod.rs
│   │   │   └── scout_dispatch.rs
│   │   ├── narrative/
│   │   │   ├── bottle.rs
│   │   │   ├── content_router.rs
│   │   │   ├── logs.rs
│   │   │   ├── mod.rs
│   │   │   ├── narrative_events.rs
│   │   │   ├── opening_sequence.rs
│   │   │   ├── quest.rs
│   │   │   └── signal.rs
│   │   ├── persistence/
│   │   │   ├── io.rs
│   │   │   ├── mod.rs
│   │   │   ├── save.rs
│   │   │   ├── schema.rs
│   │   │   └── systems.rs
│   │   ├── setup/
│   │   │   ├── entity_setup.rs
│   │   │   ├── mesh_builder.rs
│   │   │   ├── mod.rs
│   │   │   ├── quest_init.rs
│   │   │   └── world_spawn.rs
│   │   ├── ship_control/
│   │   │   ├── asteroid_input.rs
│   │   │   ├── autopilot.rs
│   │   │   ├── mod.rs
│   │   │   └── ship_spawn.rs
│   │   ├── telemetry/
│   │   │   └── mod.rs
│   │   ├── ui/
│   │   │   ├── hud/
│   │   │   │   ├── buttons.rs
│   │   │   │   ├── content.rs
│   │   │   │   ├── mod.rs
│   │   │   │   ├── overlays.rs
│   │   │   │   ├── prod_tree.rs
│   │   │   │   └── state_machine.rs
│   │   │   ├── mod.rs
│   │   │   ├── station_tabs.rs
│   │   │   └── tutorial.rs
│   │   ├── visuals/
│   │   │   ├── component_nodes.rs
│   │   │   ├── debug_log.rs
│   │   │   ├── ingot_node.rs
│   │   │   ├── map.rs
│   │   │   ├── mesh_builder.rs
│   │   │   ├── mod.rs
│   │   │   ├── ore_polygon.rs
│   │   │   ├── viewport.rs
│   │   │   └── visuals.rs
│   │   └── mod.rs
│   └── ui_kit/
│       ├── mod.rs
│       ├── primitives.rs
│       └── styles.rs
├── target/
│   └── (0 items)
├── videos/
│   └── (0 items)
├── .gitignore
├── .publish.env
├── .publish.env.example
├── AGENT_CONTRACT.md
├── BUILDING.md
├── CHANGELOG.md
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── TUTORIAL_POPUP_VERIFICATION.md
├── android/
├── bake_android.ps1
├── bake_wasm.ps1
├── build_android.ps1
├── build_desktop.ps1
├── build_output.txt
├── build_wasm.ps1
├── capture_gate_evidence.ps1
├── index.html
├── publish.ps1
├── run.ps1
├── setup_env.ps1
├── test_output.log
├── verify.ps1
└── voidrift.keystore
```

---

## Rust Source Files (src/**/*.rs)

### Root Level

**c:\Github\VoidDrift\src\lib.rs** (280 lines)
- Purpose: Main application entry point, Bevy app setup, plugin registration, system scheduling
- Functions: `main` (native), `run_app` (shared), platform-specific entry points
- Systems Registered: `setup_world`, `hud_ui_system`, `mining_system`, `autonomous_ship_system`, `autopilot_system`, etc.
- Status: Active, production

**c:\Github\VoidDrift\src\constants.rs** (15 lines)
- Purpose: Global game constants (station position, sector positions)
- Constants: `STATION_POS`, various `SECTOR_POS` coordinates
- Status: Active, most constants moved to TOML config

### Components (src/components/)

**c:\Github\VoidDrift\src\components\mod.rs** (16 lines)
- Purpose: Module declarations for all component sub-modules
- Exports: `game_state`, `ui_state`, `resources`, `markers`, `utilities`, `queries`, `events`
- Status: Active

**c:\Github\VoidDrift\src\components\game_state.rs** (220 lines)
- Purpose: Core game state components and enums
- Components/Enums: `Ship`, `ShipState`, `ActiveAsteroid`, `OreDeposit`, `LaserTier`, `Station`, `StationDockState`, `AutonomousShipState`, `AutonomousShip`, `AutonomousAssignment`, `DroneClass`, `Drone`, `Painted`, `ScoutOrbit`, `DroneTarget`, `StationQueues`
- Tests: `Drone` and `AutonomousShip` component unit tests
- Status: Active, production

**c:\Github\VoidDrift\src\components\ui_state.rs** (89 lines)
- Purpose: UI-related state resources and enums
- Resources/Enums: `ActiveStationTab`, `DrawerState`, `UiLayout`, `ProductionTabState`, `OreType`, `RequestsTabState`, `CollectedRequest`, `FactionId`, `RequestId`
- Status: Active

**c:\Github\VoidDrift\src\components\resources.rs** (278 lines)
- Purpose: Bevy resources for global game state
- Resources: `GameState`, `DeviceType`, `ViewState`, `AppState`, `AsteroidRespawnTimer`, `ShipQueue`, `MaxDispatch`, `SignalLog`, `SignalStripExpanded`, `QuestObjective`, `QuestLog`, `OpeningSequence`, `OpeningPhase`, `WorldViewRect`, `ForgeQuantity`, `ForgeSettings`, `ProcessingJob`, `ProcessingOperation`, `ProductionToggles`, `TutorialState`, `TutorialPopup`, `MapPanState`, `ContentState`, `CameraDelta`, `ProdTreeViewState`, `ScoutEnabled`
- Tests: `ScoutEnabled` unit test
- Status: Active

**c:\Github\VoidDrift\src\components\markers.rs** (100 lines)
- Purpose: Marker components for entity tagging
- Markers: `MapMarker`, `MainCamera`, `MenuCamera`, `DockedAt`, `ShipCargoBarFill`, `StarLayer`, `LastHeading`, `InOpeningSequence`, `ThrusterGlow`, `MiningBeam`, `AutonomousShipTag`, `StationVisualsContainer`, `StationHub`, `Berth`, `BerthType`, `BerthVisual`, `MapElement`, `MapIcon`, `MapLabel`, `MapConnector`, `DestinationHighlight`, `TutorialHighlight`, `AutopilotTarget`, `CargoOreLabel`, `CargoCountLabel`, `ActiveBottle`, `CarryingBottle`, `MenuStar`, `AsteroidBody`, `AsteroidBand`, `InnerRingAsteroid`
- Status: Active

**c:\Github\VoidDrift\src\components\utilities.rs** (72 lines)
- Purpose: Utility functions for ore config keys, names, laser requirements, berth position calculation
- Functions: `ore_config_key`, `ore_name`, `ore_laser_required`, `berth_world_pos`
- Tests: Ore lookup consistency tests
- Status: Active

**c:\Github\VoidDrift\src\components\queries.rs** (42 lines)
- Purpose: Type aliases for common Bevy query filters
- Type Aliases: `BaseShipFilter`, `BaseStationFilter`, `BaseCameraFilter`, `VisualsCameraFilter`, `VisualsStarFilter`, `VisualsStationFilter`, `VisualsContainerFilter`, `VisualsShipFilter`, `VisualsAutoShipFilter`
- Status: Active

**c:\Github\VoidDrift\src\components\events.rs** (58 lines)
- Purpose: Custom Bevy events for inter-system communication
- Events: `ShipDockedWithCargo`, `ShipDockedWithBottle`, `FulfillRequestEvent`, `RepairStationEvent`, `OpeningCompleteEvent`, `DroneDispatched`, `InsufficientLaserEvent`, `SignalFired`
- Status: Active

### Configuration (src/config/)

**c:\Github\VoidDrift\src\config\mod.rs** (9 lines)
- Purpose: Module declarations for config sub-modules
- Exports: `balance`, `visual`, `content`
- Status: Active

**c:\Github\VoidDrift\src\config\balance.rs** (176 lines)
- Purpose: Balance configuration structs and TOML loading
- Structs: `BalanceConfig`, `MiningConfig`, `RefineryConfig`, `ForgeConfig`, `DroneConfig`, `AsteroidConfig`, `StationConfig`, `NarrativeConfig`, `UiConfig`, `MapConfig`, `AsteroidSpawningConfig`, `RingsConfig`, `ScoutConfig`
- Tests: Asteroid rings loading test
- Status: Active, loads from `assets/balance.toml`

**c:\Github\VoidDrift\src\config\visual.rs** (386 lines)
- Purpose: Visual configuration structs and TOML loading
- Structs: `VisualConfig`, `StarfieldConfig`, `AsteroidVisualConfig`, `OreTypeConfig`, `OreConfig`, `AsteroidRingConfig`, `AsteroidRingsConfig`, `OreNodeConfig`, `IngotNodeConfig`, `ProductionTreeConfig`, `DepletionParticlesConfig`, `ParticlesConfig`, `IngotTypeConfig`, `IngotsConfig`, `BottleVisualConfig`, `StationVisualConfig`, `ZLayerConfig`, `MapColorsConfig`, `EguiConfig`, `DroneVisualEntry`, `DroneVisualConfig`, `ComponentThrusterConfig`, `ComponentHullConfig`, `ComponentCanisterConfig`, `ComponentAICoreConfig`, `ComponentDroneBayConfig`, `ShipDroneConfig`, `ShipOpeningConfig`, `ShipConfig`, `ComponentConfig`
- Tests: Visual config loading and drone visual field tests
- Status: Active, loads from `assets/visual.toml`

**c:\Github\VoidDrift\src\config\content.rs** (175 lines)
- Purpose: Content configuration structs and YAML loading
- Structs: `ContentConfig`, `OneShotLine`, `AmbientLine`, `EventPool`, `TutorialConfig`, `TutorialStep`, `TutorialPopup`, `QuestConfig`, `QuestObjectiveDef`, `ObjectiveTriggers`, `RequestConfig`, `RequestDef`, `ResourceRequirement`, `RewardDef`, `LogsConfig`, `LogEntry`
- Function: `read_yaml` (platform-specific for WASM/Android)
- Status: Active, loads from `assets/content/*.yaml`

### Scenes (src/scenes/)

**c:\Github\VoidDrift\src\scenes\mod.rs** (2 lines)
- Purpose: Module declaration for scene sub-modules
- Exports: `main_menu`
- Status: Active

**c:\Github\VoidDrift\src\scenes\main_menu.rs** (273 lines)
- Purpose: Main menu UI system and state management
- Functions: `setup_main_menu`, `main_menu_system`, `render_save_list`, `format_timestamp`, `spawn_menu_camera`, `cleanup_menu`
- Status: Active

**c:\Github\VoidDrift\src\scenes\menu_starfield.rs** (73 lines)
- Purpose: Procedural starfield for main menu background
- Functions: `spawn_menu_starfield`, `menu_star_drift_system`
- Status: Active

**c:\Github\VoidDrift\src\scenes\restore.rs** (201 lines)
- Purpose: Game state restoration from save data
- Functions: `ingame_startup_system`, `restore_save_state`, `spawn_saved_drones`, `apply_dev_mode_signal`
- Status: Active

**c:\Github\VoidDrift\src\scenes\save_overlay.rs** (100 lines)
- Purpose: Save/load overlay UI system
- Functions: `save_overlay_system`
- Status: Active

### Systems - Asteroid (src/systems/asteroid/)

**c:\Github\VoidDrift\src\systems\asteroid\mod.rs** (3 lines)
- Purpose: Module declaration for asteroid sub-modules
- Exports: `spawn`, `lifecycle`
- Status: Active

**c:\Github\VoidDrift\src\systems\asteroid\spawn.rs** (254 lines)
- Purpose: Asteroid spawning and procedural mesh generation
- Functions: `spawn_initial_asteroids`, `spawn_asteroid`
- Status: Active

**c:\Github\VoidDrift\src\systems\asteroid\lifecycle.rs** (34 lines)
- Purpose: Asteroid lifespan management
- Functions: `asteroid_lifecycle_system`
- Status: Active

### Systems - Game Loop (src/systems/game_loop/)

**c:\Github\VoidDrift\src\systems\game_loop\mod.rs** (6 lines)
- Purpose: Module declaration for game_loop sub-modules
- Exports: `mining`, `auto_process`, `autonomous`, `economy`, `scout_dispatch`
- Status: Active

**c:\Github\VoidDrift\src\systems\game_loop\mining.rs** (188 lines)
- Purpose: Ship mining system with laser tier checks
- Functions: `mining_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\game_loop\auto_process.rs** (145 lines)
- Purpose: Automatic resource processing (refining, forging, drone building)
- Functions: `auto_refine_system`, `auto_forge_system`, `auto_build_drones_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\game_loop\autonomous.rs** (128 lines)
- Purpose: Autonomous ship state machine and movement
- Functions: `autonomous_ship_system`, `autonomous_beam_system`, `docked_autonomous_ship_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\game_loop\economy.rs** (72 lines)
- Purpose: Event-driven economy updates
- Functions: `ship_docked_economy_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\game_loop\scout_dispatch.rs** (338 lines)
- Purpose: Scout Mk I automation - orbit, paint asteroids, dispatch miners
- Functions: `scout_spawn_system`, `scout_orbit_system`, `scout_paint_cleanup_system`
- Tests: 13 unit tests for scout behavior
- Status: Active

### Systems - Narrative (src/systems/narrative/)

**c:\Github\VoidDrift\src\systems\narrative\mod.rs** (2 lines)
- Purpose: Module declaration for narrative sub-modules
- Exports: (inline modules)
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\bottle.rs** (155 lines)
- Purpose: Bottle collection mechanic
- Functions: `bottle_spawn_system`, `bottle_input_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\content_router.rs** (152 lines)
- Purpose: Echo content routing (one-shots, ambient lines, event pools)
- Functions: `content_event_system`, `content_ambient_system`, `fire_one_shot`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\logs.rs** (51 lines)
- Purpose: Log unlock checking
- Functions: `check_log_unlocks`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\narrative_events.rs** (83 lines)
- Purpose: Narrative event handling (bottle collection, opening completion)
- Functions: `narrative_event_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\opening_sequence.rs** (139 lines)
- Purpose: Opening cinematic sequence
- Functions: `opening_sequence_system`, `opening_drone_move_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\quest.rs** (86 lines)
- Purpose: Quest objective state management
- Functions: `quest_signal_system`, `quest_update_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\narrative\signal.rs** (214 lines)
- Purpose: Signal log generation and firing
- Functions: `signal_system`, `emit`, `emit_refirable`
- Status: Active

### Systems - Persistence (src/systems/persistence/)

**c:\Github\VoidDrift\src\systems\persistence\mod.rs** (2 lines)
- Purpose: Module declaration for persistence sub-modules
- Exports: `save` (inline)
- Status: Active

**c:\Github\VoidDrift\src\systems\persistence\save.rs** (18 lines)
- Purpose: Persistence module entry point
- Exports: `schema`, `io`, `systems`, `SAVE_VERSION`
- Status: Active

**c:\Github\VoidDrift\src\systems\persistence\schema.rs** (143 lines)
- Purpose: Save data schema and events
- Structs: `SaveData`, `SaveCategory`, `DroneSaveData`
- Events: `AutosaveEvent`, `SaveRequestEvent`
- Tests: Save deserialization test
- Status: Active

**c:\Github\VoidDrift\src\systems\persistence\io.rs** (216 lines)
- Purpose: Platform-specific save/load I/O (WASM vs native)
- Functions: `get_save_base_dir`, `save_dir`, `autosave_path`, `save_game`, `load_game`, `list_saves`, `sanitize_filename`, `current_timestamp`
- Status: Active

**c:\Github\VoidDrift\src\systems\persistence\systems.rs** (164 lines)
- Purpose: Save collection and application systems
- Functions: `collect_save_data`, `autosave_system`, `save_request_system`
- Status: Active

### Systems - Setup (src/systems/setup/)

**c:\Github\VoidDrift\src\systems\setup\mod.rs** (10 lines)
- Purpose: Module declaration for setup sub-modules
- Exports: `world_spawn`, `entity_setup`, `mesh_builder`, `quest_init`
- Status: Active

**c:\Github\VoidDrift\src\systems\setup\world_spawn.rs** (136 lines)
- Purpose: World initialization and cleanup
- Functions: `cleanup_world_entities`, `reset_game_resources`, `setup_world`, `spawn_starfield`, `spawn_camera`
- Status: Active

**c:\Github\VoidDrift\src\systems\setup\entity_setup.rs** (316 lines)
- Purpose: Entity spawning (station, drones, highlights)
- Functions: `spawn_opening_drone`, `spawn_station`, `spawn_berths`, `spawn_destination_highlight`, `spawn_tutorial_highlight`, `spawn_rocket_part`, macro `spawn_drone_core_children`
- Status: Active

**c:\Github\VoidDrift\src\systems\setup\mesh_builder.rs** (207 lines)
- Purpose: Mesh building utilities
- Functions: `build_mesh_from_polygon`, `build_mesh_from_polygon_with_colors`, `build_mesh_from_quad`, `generate_ore_polygon_points`, `generate_ore_band_quads`, `generate_rocket_points`, `triangle_mesh`
- Status: Active

**c:\Github\VoidDrift\src\systems\setup\quest_init.rs** (28 lines)
- Purpose: Quest log initialization
- Functions: `init_quest_log`
- Status: Active

### Systems - Ship Control (src/systems/ship_control/)

**c:\Github\VoidDrift\src\systems\ship_control\mod.rs** (4 lines)
- Purpose: Module declaration for ship_control sub-modules
- Exports: `autopilot`, `asteroid_input`, `ship_spawn`
- Status: Active

**c:\Github\VoidDrift\src\systems\ship_control\autopilot.rs** (149 lines)
- Purpose: Ship autopilot and docking
- Functions: `autopilot_system`, `docked_ship_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\ship_control\asteroid_input.rs** (128 lines)
- Purpose: Asteroid tap-to-dispatch input handling
- Functions: `asteroid_input_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\ship_control\ship_spawn.rs** (175 lines)
- Purpose: Drone ship spawning
- Functions: `spawn_drone_ship`, `spawn_bottle_drone`, `ship_config_to_rocket_config`, `spawn_rocket_part`
- Status: Active

### Systems - Telemetry (src/systems/telemetry/)

**c:\Github\VoidDrift\src\systems\telemetry\mod.rs** (455 lines)
- Purpose: Telemetry system for analytics
- Resources: `TelemetryEvent`, `SessionId`, `TelemetryConsent`, `LoopStallTimer`, `LogTabState`, `TelemetryOptInPrompt`, `TelemetrySessionCounter`
- Functions: Platform-specific send functions, session management, loop stall detection
- Status: Active

### Systems - UI (src/systems/ui/)

**c:\Github\VoidDrift\src\systems\ui\mod.rs** (12 lines)
- Purpose: Module declaration for UI sub-modules
- Exports: `hud`, `station_tabs`, `tutorial`, various HUD systems
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\mod.rs** (437 lines)
- Purpose: Main HUD UI system
- Functions: `hud_ui_system`, `ship_cargo_display_system`, `cargo_label_system`, `station_visual_system`, `sync_max_drones_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\buttons.rs** (64 lines)
- Purpose: HUD button rendering (FLEET, PIPELINE, SAVE, FOCUS)
- Functions: `render_hud_buttons`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\content.rs** (551 lines)
- Purpose: Drawer tab content rendering (CARGO, HANGAR, FORGE, QUESTS, LOGS)
- Functions: `render_tab_content`, `render_ore_pipeline`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\overlays.rs** (214 lines)
- Purpose: Tutorial and telemetry opt-in overlays
- Functions: `render_overlays`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\prod_tree.rs** (450 lines)
- Purpose: Production tree viewport with zoom/pan
- Functions: `render_production_tree`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\hud\state_machine.rs** (20 lines)
- Purpose: Drawer state transitions
- Functions: `update_drawer_state`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\station_tabs.rs** (99 lines)
- Purpose: Station tab rendering and queue management
- Functions: `render_queue_card`, `add_log_entry`
- Status: Active

**c:\Github\VoidDrift\src\systems\ui\tutorial.rs** (204 lines)
- Purpose: Tutorial popup system with highlighting
- Functions: `tutorial_system`
- Status: Active

### Systems - Visuals (src/systems/visuals/)

**c:\Github\VoidDrift\src\systems\visuals\mod.rs** (11 lines)
- Purpose: Module declaration for visuals sub-modules
- Exports: `map`, `viewport`, `visuals`, `debug_log`, `ore_polygon`, `ingot_node`, `component_nodes`, `mesh_builder`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\map.rs** (214 lines)
- Purpose: Camera control and map element visibility
- Functions: `camera_follow_system`, `show_map_elements`, `hide_map_elements`, `map_highlight_system`, `pinch_zoom_system`, `map_pan_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\viewport.rs** (54 lines)
- Purpose: Camera viewport adjustment for UI
- Functions: `drawer_viewport_system`, `ui_layout_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\visuals.rs** (207 lines)
- Purpose: Visual effects (thruster glow, ship rotation, starfield scroll, station rotation, berth occupancy)
- Functions: `thruster_glow_system`, `ship_rotation_system`, `starfield_scroll_system`, `station_rotation_system`, `berth_occupancy_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\debug_log.rs** (43 lines)
- Purpose: Global debug logging system
- Functions: `log_debug_info`, `flush_debug_log_system`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\ore_polygon.rs** (64 lines)
- Purpose: Procedural ore node drawing (jagged polygon with star veins)
- Structs: `OrePolygonConfig`
- Functions: `draw_ore_polygon`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\ingot_node.rs** (68 lines)
- Purpose: Isometric ingot node drawing
- Structs: `IngotNodeConfig`
- Functions: `draw_ingot_node`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\component_nodes.rs** (465 lines)
- Purpose: Component node drawing (thruster, hull, canister, AI core, drone bay)
- Structs: `ThrusterConfig`, `HullConfig`, `CanisterConfig`, `AICoreConfig`, `DroneBayConfig`, `RocketConfig`
- Functions: `draw_thruster`, `draw_hull`, `draw_canister`, `draw_ai_core`, `draw_drone_bay`, `draw_rocket`
- Status: Active

**c:\Github\VoidDrift\src\systems\visuals\mesh_builder.rs** (207 lines)
- Purpose: Mesh building utilities (shared with setup module)
- Functions: `build_mesh_from_polygon`, `build_mesh_from_polygon_with_colors`, `build_mesh_from_quad`, `generate_ore_polygon_points`, `generate_ore_band_quads`, `generate_rocket_points`, `triangle_mesh`
- Status: Active

### Systems Root

**c:\Github\VoidDrift\src\systems\mod.rs** (10 lines)
- Purpose: Module declaration for all system sub-modules
- Exports: `setup`, `game_loop`, `ship_control`, `asteroid`, `ui`, `persistence`, `narrative`, `visuals`, `telemetry`
- Status: Active

### UI Kit (src/ui_kit/)

**c:\Github\VoidDrift\src\ui_kit\mod.rs** (3 lines)
- Purpose: Module declaration for UI kit sub-modules
- Exports: `styles`, `primitives`
- Status: Active

**c:\Github\VoidDrift\src\ui_kit\styles.rs** (37 lines)
- Purpose: UI button style definitions
- Structs: `ButtonStyle`, `HighlightKind`
- Status: Active

**c:\Github\VoidDrift\src\ui_kit\primitives.rs** (59 lines)
- Purpose: UI primitive components
- Functions: `vd_button`
- Status: Active

---

## Configuration Files

**c:\Github\VoidDrift\Cargo.toml** (89 lines)
- Purpose: Rust package configuration
- Sections: `[package]`, `[lib]`, `[[bin]]`, `[dependencies]`, platform-specific dependencies, `[profile.release]`
- Key Dependencies: Bevy 0.15.3, bevy_egui 0.33.0, rand, serde, serde_json, toml, serde_yaml, chrono, reqwest, uuid
- Platform Features: Android (android-game-activity), WASM (webgl2, webgpu, gloo-storage)
- Status: Active, Bevy pinned at 0.15.3 per ADR

**c:\Github\VoidDrift\assets\balance.toml** (94 lines)
- Purpose: Game balance parameters
- Sections: `[mining]`, `[refinery]`, `[forge]`, `[drone]`, `[asteroid]`, `[station]`, `[narrative]`, `[ui]`, `[map]`, `[asteroid_spawning]`, `[rings.inner]`, `[rings.middle]`, `[rings.outer]`, `[scout]`
- Status: Active

**c:\Github\VoidDrift\assets\visual.toml** (262 lines)
- Purpose: Visual configuration (colors, sizes, z-layers, component visuals)
- Sections: `[starfield]`, `[asteroid]`, `[bottle]`, `[station]`, `[z_layer]`, `[map_colors]`, `[egui]`, `[drone.opening]`, `[drone.mission]`, `[ore.metal]`, `[ore.h3_gas]`, `[ore.void_essence]`, `[asteroid.inner_ring]`, `[asteroid.middle_ring]`, `[asteroid.outer_ring]`, `[production_tree.ore_node]`, `[production_tree.ingot_node]`, `[production_tree]`, `[ingot.metal]`, `[ingot.crystal]`, `[ingot.void]`, `[particles.depletion]`, `[component.thruster]`, `[component.hull]`, `[component.canister]`, `[component.ai_core]`, `[component.drone_bay]`, `[ship.drone]`, `[ship.opening]`
- Status: Active

**c:\Github\VoidDrift\assets\content\echo.yaml** (61 lines)
- Purpose: Echo one-shot and ambient lines
- Sections: `one_shots`, `ambient`, `event_pools`
- Status: Active

**c:\Github\VoidDrift\assets\content\tutorial.yaml** (55 lines)
- Purpose: Tutorial step definitions
- Sections: `steps`
- Status: Active

**c:\Github\VoidDrift\assets\content\objectives.yaml** (56 lines)
- Purpose: Quest objective definitions
- Sections: `quest_objectives`
- Status: Active

**c:\Github\VoidDrift\assets\content\requests.yaml** (16 lines)
- Purpose: Faction request definitions
- Sections: `faction_requests`
- Status: Active

**c:\Github\VoidDrift\assets\content\logs.yaml** (51 lines)
- Purpose: Unlockable log entries
- Sections: `logs`
- Status: Active

---

## Document Files (docs/** and root *.md)

### Root Markdown Files

**c:\Github\VoidDrift\README.md** (197 lines)
- Purpose: Project overview, build instructions, architecture summary
- Sections: The Game, What's Working, Technical Architecture, Project Structure, Roadmap, Building for Android, Building for WASM, Publishing to itch.io, License
- Currency: Current (references v2.8.7-tutorial-4a)
- Status: Active

**c:\Github\VoidDrift\AGENT_CONTRACT.md** (9710 bytes)
- Purpose: Agent contract for AI development
- Status: Active

**c:\Github\VoidDrift\BUILDING.md** (4135 bytes)
- Purpose: Build instructions
- Status: Active

**c:\Github\VoidDrift\CHANGELOG.md** (7186 bytes)
- Purpose: Development history
- Status: Active

**c:\Github\VoidDrift\LICENSE** (1077 bytes)
- Purpose: MIT license
- Status: Active

**c:\Github\VoidDrift\TUTORIAL_POPUP_VERIFICATION.md** (2028 bytes)
- Purpose: Tutorial popup verification notes
- Status: Active

### docs/ Directory

**c:\Github\VoidDrift\docs\README.md** (4857 bytes)
- Purpose: Documentation index
- Status: Active

**c:\Github\VoidDrift\docs\ARCHITECTURE.md** (19169 bytes)
- Purpose: Deep technical architecture reference
- Status: Active

**c:\Github\VoidDrift\docs\CHANGELOG.md** (13037 bytes)
- Purpose: Full development history
- Status: Active

**c:\Github\VoidDrift\docs\DEVELOPER.md** (10227 bytes)
- Purpose: Developer onboarding guide
- Status: Active

**c:\Github\VoidDrift\docs\DEVELOPMENT_PIPELINE.md** (10540 bytes)
- Purpose: Development process documentation
- Status: Active

**c:\Github\VoidDrift\docs\FUTURE_DESIGN_NOTES.md** (12888 bytes)
- Purpose: Future design considerations
- Status: Active

**c:\Github\VoidDrift\docs\GDD_v1_ARCHIVED.md** (40946 bytes)
- Purpose: Archived Game Design Document v1
- Status: Archived

**c:\Github\VoidDrift\docs\GDD_v2.md** (2486 bytes)
- Purpose: Game Design Document v2
- Status: Active

**c:\Github\VoidDrift\docs\NARRATIVE_JUSTIFICATION.md** (7607 bytes)
- Purpose: Narrative design rationale
- Status: Active

**c:\Github\VoidDrift\docs\VOIDRIFT_REFACTOR_ANALYSIS.md** (15505 bytes)
- Purpose: Refactoring analysis
- Status: Active

**c:\Github\VoidDrift\docs\WASM_BUILD.md** (4269 bytes)
- Purpose: WASM build instructions
- Status: Active

**c:\Github\VoidDrift\docs\WASM_ITCH_SIZING.md** (1575 bytes)
- Purpose: WASM itch.io sizing notes
- Status: Active

**c:\Github\VoidDrift\docs\WINDSURF.md** (8297 bytes)
- Purpose: Windsurf IDE configuration
- Status: Active

**c:\Github\VoidDrift\docs\narrative_canon.md** (3890 bytes)
- Purpose: Locked narrative foundation
- Status: Active

**c:\Github\VoidDrift\docs\roadmap.md** (10084 bytes)
- Purpose: Project roadmap
- Status: Active

**c:\Github\VoidDrift\docs\codebase-audit-2026-05-10.md** (18050 bytes)
- Purpose: Codebase audit report
- Status: Active

### docs/adr/ (22 items)
- Purpose: Architectural Decision Records
- Status: Active

### docs/analysis/ (1 item)
- Purpose: Analysis documents
- Status: Active

### docs/archive/ (34 items)
- Purpose: Archived documentation
- Status: Archived

### docs/design/ (4 items)
- Purpose: Design documents
- Status: Active

### docs/directives/ (28 items)
- Purpose: Implementation directives (agent contracts)
- Status: Active

### docs/phases/ (14 items)
- Purpose: Phase-by-phase archival summaries
- Status: Active

### docs/state/ (1 item)
- Purpose: State documentation
- Status: Active

---

## Build and Deployment Scripts

### Root PowerShell Scripts

**c:\Github\VoidDrift\bake_android.ps1** (596 bytes)
- Purpose: Android APK baking
- Status: Active

**c:\Github\VoidDrift\bake_wasm.ps1** (580 bytes)
- Purpose: WASM binary baking
- Status: Active

**c:\Github\VoidDrift\build_android.ps1** (9800 bytes)
- Purpose: One-click Android build + deploy pipeline
- Status: Active

**c:\Github\VoidDrift\build_desktop.ps1** (1210 bytes)
- Purpose: Desktop build
- Status: Active

**c:\Github\VoidDrift\build_wasm.ps1** (5586 bytes)
- Purpose: WASM build
- Status: Active

**c:\Github\VoidDrift\capture_gate_evidence.ps1** (837 bytes)
- Purpose: Gate evidence capture
- Status: Active

**c:\Github\VoidDrift\publish.ps1** (5513 bytes)
- Purpose: itch.io publishing
- Status: Active

**c:\Github\VoidDrift\run.ps1** (230 bytes)
- Purpose: Quick run script
- Status: Active

**c:\Github\VoidDrift\setup_env.ps1** (306 bytes)
- Purpose: Environment setup
- Status: Active

**c:\Github\VoidDrift\verify.ps1** (1022 bytes)
- Purpose: Verification script
- Status: Active

### scripts/ Directory

**c:\Github\VoidDrift\scripts\gh_tools.ps1** (1331 bytes)
- Purpose: GitHub tools
- Status: Active

**c:\Github\VoidDrift\scripts\record_demo.ps1** (2423 bytes)
- Purpose: Demo recording
- Status: Active

**c:\Github\VoidDrift\scripts\trim_demo.ps1** (397 bytes)
- Purpose: Demo trimming
- Status: Active

**c:\Github\VoidDrift\scripts\serve_wasm.py** (993 bytes)
- Purpose: WASM local server
- Status: Active

**c:\Github\VoidDrift\scripts\OBS_SETUP.md** (1019 bytes)
- Purpose: OBS setup instructions
- Status: Active

**c:\Github\VoidDrift\scripts\local_itch_preview.html** (14791 bytes)
- Purpose: Local itch preview template
- Status: Active

**c:\Github\VoidDrift\scripts\obs_scene.json** (7246 bytes)
- Purpose: OBS scene configuration
- Status: Active

**c:\Github\VoidDrift\scripts\shot_guide.html** (8394 bytes)
- Purpose: Shot guide
- Status: Active

### rfd-telemetry/

**c:\Github\VoidDrift\rfd-telemetry\main.py** (Python)
- Purpose: Telemetry server
- Status: Active

---

## Test Coverage

### Files with `#[cfg(test)]` Modules

**c:\Github\VoidDrift\src\components\resources.rs**
- Test: `ScoutEnabled` unit test
- Coverage: Scout enabled state validation

**c:\Github\VoidDrift\src\components\game_state.rs**
- Tests: `Drone` component unit test, `AutonomousShip` component unit test
- Coverage: Drone and autonomous ship component validation

**c:\Github\VoidDrift\src\components\utilities.rs**
- Tests: Ore lookup consistency tests
- Coverage: Ore config key/name/laser mapping validation

**c:\Github\VoidDrift\src\config\balance.rs**
- Test: Asteroid rings loading test
- Coverage: Balance config loading validation

**c:\Github\VoidDrift\src\config\visual.rs**
- Tests: Visual config loading test, drone visual field presence test
- Coverage: Visual config loading and field validation

**c:\Github\VoidDrift\src\systems\game_loop\scout_dispatch.rs**
- Tests: 13 unit tests for scout behavior (spawn, orbit, paint, dispatch, cleanup)
- Coverage: Scout Mk I automation behavior validation

**c:\Github\VoidDrift\src\systems\persistence\schema.rs**
- Test: Save deserialization with missing fields
- Coverage: Save data backward compatibility validation

---

## Summary

- **Total Rust Source Files:** 73
- **Total Configuration Files:** 7 (Cargo.toml + 6 TOML/YAML)
- **Total Document Files:** 121+ (root + docs/)
- **Total Build Scripts:** 13 (PowerShell + Python)
- **Test Coverage:** 7 files with test modules, ~20 individual unit tests
- **Primary Platform:** Android (Moto G 2025) with WASM fallback
- **Engine:** Bevy 0.15.3 (pinned)
- **UI Framework:** bevy_egui 0.33.0
- **Status:** Active development, Phase 4a complete (v3.0.10-tutorial-complete)
