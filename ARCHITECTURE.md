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
    mod.rs            — pub mod declarations
    setup.rs          — setup_world, entity spawning, mesh generators
    autopilot.rs      — autopilot_system
    mining.rs         — mining_system
    economy.rs        — station_status_system, power_maintenance, self_preservation
    autonomous.rs     — autonomous_ship_system, AI state machine
    visuals.rs        — starfield_scroll_system, thruster_glow_system, ship_rotation
    ui.rs             — hud_ui_system, cargo bars, station visuals
    map.rs            — handle_input, camera_follow_system, map view transitions

## System Inventory

- `systems::setup::setup_world`: Main startup system. Spawns camera, player ship, station, and asteroid fields.
- `systems::visuals::thruster_glow_system`: Controls Visibility of ThrusterGlow entities based on ship navigation/mining states.
- `systems::visuals::ship_rotation_system`: Smoothly rotates ship meshes to face their current destination or preserves last heading.
- `systems::visuals::starfield_scroll_system`: Parallax-scrolls StarLayer entities relative to camera movement with wrap-around logic.
- `systems::autopilot::autopilot_system`: Handles player navigation logic, proximity detection for docking, and resource unloading.
- `systems::mining::mining_system`: Manages player mining ray physics, resource extraction rate, and asteroid depletion visuals.
- `systems::autonomous::autonomous_ship_system`: Handles high-level state machine for AI drones (Outbound, Mining, Returning, etc.).
- `systems::economy::station_status_system`: Monitors station power reserves and issues low-power AI log warnings.
- `systems::economy::ship_self_preservation_system`: Monitors player ship power; triggers emergency refine or auto-return.
- `systems::economy::station_maintenance_system`: Periodic timer-based consumption of station power cells to maintain base systems.
- `systems::ui::hud_ui_system`: Main egui dashboard for docking, refinery, ship fabrication, and drone deployment.
- `systems::ui::station_visual_system`: Updates the world-space color of the station based on its online status.
- `systems::ui::ship_cargo_display_system`: Updates player ship's cargo fill bar scale and color.
- `systems::ui::autonomous_ship_cargo_display_system`: Updates AI drone cargo fill bar scale.
- `systems::map::camera_follow_system`: Smoothes camera positioning relative to player ship or centers it on MapView.
- `systems::map::handle_input`: Manages touch gesture detection for setting navigation targets or toggling map.
- `systems::map::enter_map_view` / `systems::map::exit_map_view`: Transition systems triggered by GameState changes to update camera projection.

## Component Inventory

- `Ship`: Main player vehicle data (state, speed, cargo, power).
- `AutonomousShip`: AI drone data and state.
- `AutonomousAssignment`: Navigation target and goal for AI drones.
- `Station`: Base infrastructure data (reserves, power, logs, timers).
- `AsteroidField`: Resource source data (ore type, depletion status).
- `AutopilotTarget`: Directional goal for navigation systems.
- `StarLayer`: Parallax factor for starfield entities.
- `LastHeading`: Persistent rotational state for ships.
- `CameraDelta`: Resource tracking per-tick camera movement.
- `MainCamera` / `PlayerShip` / `AutonomousShipTag`: Marker components.
- `ThrusterGlow` / `MiningBeam` / `ShipCargoBarFill`: Visual marker components.
- `MapMarker`: Marks entities visible/targetable on the map.

## ShipState Machine

Idle → Navigating → Mining → Docked → Idle
Idle → Navigating → Idle (station arrival)

## AutonomousShipState Machine

Holding → Outbound → Mining → Returning → Unloading → Holding

## Narrative Architecture
- **The Signal**: A unified narrative voice governed by strict telemetry-style rules.
- **SignalLog**: A central resource for persistent, character-driven reports.
- **OpeningSequence**: A dedicated state machine for the game's scripted introduction.
- Design Spec: [Signal: Narrative Design Document](file:///c:/Github/VoidDrift/docs/Voidrift_Signal_Narrative_Design.md)

## Station Systems
The station system manages the structural health, inventory, and departmental logic of the player's base. It uses a **Berth-based Docking** model (Main Station) and a **Static Depot** model (Drone Depot) to manage ship capacity.
- See [Station Architecture](file:///c:/Github/VoidDrift/docs/Voidrift_Station_Architecture.md) for details on rotation and dynamic berths.

## Hardware Constraints (ADRs)

ADR-001: PresentMode::Fifo — mandatory, Mali GPU buffer starvation
ADR-002: Mesh2d only — Sprite triggers gralloc format errors
ADR-003: bevy_egui — Text2d and camera-parented Mesh2d both fail on device
ADR-004: Bevy 0.15 pinned — most stable Android documentation
ADR-005: Dedicated systems for autonomous agents
ADR-006: Module structure — lib.rs app setup only
