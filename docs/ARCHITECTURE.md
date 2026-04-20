# Voidrift Architecture

## Technology Stack
- **Engine**: Bevy 0.15.2
- **UI**: egui 0.31, bevy_egui 0.33
- **Platform**: Android API 35 (Moto G 2025)
- **Tooling**: cargo-ndk r29

## Module Structure
Responsibilities are partitioned into modular system files:
- `src/systems/autopilot.rs`: Ship movement interpolation and berth synchronization.
- `src/systems/mining.rs`: Resource extraction logic and beam visuals.
- `src/systems/economy.rs`: Refinery, forge, and station power management.
- `src/systems/autonomous.rs`: AI drone state machines and fleet routing.
- `src/systems/visuals.rs`: Starfield, thruster effects, and station rotation.
- `src/systems/ui.rs`: Egui HUD implementation and world-space labels.
- `src/systems/map.rs`: Touch input, pinch zoom, and camera control.
- `src/systems/setup.rs`: Startup orchestration and mesh generation.
- `src/systems/narrative.rs`: Cinematic intro, signals, and tutorial triggers.
- `src/systems/quest.rs`: Quest state and objective progression.

## System Execution Order
The `Update` schedule is partitioned into two groups to bypass the Bevy 20-system tuple limit:
1.  **Group 1 (Gameplay & Logistics)**: Handles physical movement, mining, and autonomous drones.
2.  **Group 2 (Station, Narrative & UI)**: Handles visual updates, narrative signals, tutorial popups, and the Egui HUD pass.

Chaining is used within Group 1 to ensure stability (e.g., `autonomous_beam_system` runs `.after(autonomous_ship_system)`).

## ECS Architecture Constraints

### Universal Disjointness (Total Lockdown)
Every system querying `&mut Transform` must use `Without<T>` filters to target disjoint entity sets.
- Ship systems: `Without<Station>`, `Without<AsteroidField>`.
- Station systems: `Without<Ship>`, `Without<AutonomousShipTag>`.
This prevents runtime B0001 panics on Mali-G57 GPU hardware.

### DockedAt Pattern
Ships docked at the station are assigned a `DockedAt(Entity)` component linking them to a specific `Berth` entity. The `docked_ship_system` uses this parent-link to synchronize the ship's world position with the rotating station arm every tick, preventing drift.

## Entity Hierarchy

### Station
- `Station` (Root: Z_ENVIRONMENT)
  - `StationVisualsContainer` (Rotates)
    - `StationHub` (Mesh)
    - `Arm` (Mesh)
      - `BerthVisual` (Mesh)
  - `MapElement` (Z_MAP_MARKERS: Only visible in map mode)

### Ship
- `Ship` (Root: Z_SHIP)
  - `ThrusterGlow` (+0.1)
  - `MiningBeam` (Z_BEAM - Z_SHIP)
  - `CargoBarBack` (+0.1)
    - `ShipCargoBarFill` (+0.05)
  - `CargoOreLabel` (World-space Text2d)
  - `CargoCountLabel` (World-space Text2d)

### Asteroid
- `AsteroidField` (Root: Z_ENVIRONMENT)
  - `MapIcon` (Map Mode)
  - `MapLabel` (Map Mode)
  - `OreNameLabel` (World-space)

## Key Data Flows

### Signal Pipeline
`Game Logic Condition` → `signal_system` fires ID → `SignalLog` resource updated → `ui.rs` renders log entries.

### Tutorial Pipeline
`Instructional Condition` → `tutorial_system` checks ID → `TutorialState.active` set → `ui.rs` renders centered popup.

### Quest Progression
`Signal Fired` → `quest_update_system` checks ID → `QuestLog` objective state/progress updated → `ui.rs` renders objective list.

### Processing
`UI Button` → `queue_job` (deducts resources, adds `ProcessingJob`) → `processing_queue_system` ticks timer → `output deposited` in Station reserves.

### Docking
`Autopilot arrival` → `DockedAt` inserted → `docked_ship_system` tracks berth rotation → `auto_dock_system` handles cargo offload.

## Rendering Architecture

| Layer | Z-Constant | Purpose |
| :--- | :--- | :--- |
| **HUD** | 2.0 | World-space labels and egui pass. |
| **Cargo Bar** | 1.1 | Visual feedback above ships. |
| **Ship** | 1.0 | Player and autonomous vessels. |
| **Beam** | 0.8 | Mining laser effect. |
| **Map Markers** | 0.6 | Icons visible only in map mode. |
| **Environment** | 0.5 | Station and Asteroids. |
| **Connectors** | -5.0 | Map routes. |
| **Stars (Near)** | -50.0 | Parallax layer 1. |
| **Stars (Far)** | -100.0 | Parallax layer 2. |

> [!IMPORTANT]
> **Mali GPU Constraint**: All background elements must use `AlphaMode2d::Opaque`. Using `Blend` on these layers causes depth-sorting flicker on the Moto G 2025.

## Input Architecture
- **Space View**: Single-touch to set navigation target on Map Markers.
- **Map View**: Single-touch marker selection + Single-touch Pan.
- **Pinch Zoom**: Two-finger interaction; suppresses single-touch map pan/navigation inputs while active.
