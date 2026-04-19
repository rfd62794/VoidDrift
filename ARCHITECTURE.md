# Architecture — Voidrift

This document provides a high-level overview of the Voidrift system architecture and the hardware-driven constraints governing its design.

## ECS & System Design

Voidrift is built on the **Bevy 0.15** Entity Component System (ECS). Game logic is decoupled into discrete systems that operate on queryable components:

- **AutopilotSystem**: Calculates vector math for ship navigation and handles arrival logic for asteroids and stations.
- **MiningSystem**: Accumulates resources at a fixed rate when the ship is in the `Mining` state.
- **Refinery Interaction**: Handled via `bevy_egui` callbacks that mutate the `Ship` component directly.
- **State Management**: The application uses Bevy `States` (`SpaceView`, `MapView`) to control camera projection and input interpretation.

## Hardware Constraints (Mali GPU / Moto G 2025)

Every architectural choice follows the "Physical Evidence First" rule. Three primary failures define the current stack:

| Failure Mode | Symptom | Solution | Evidence |
| :--- | :--- | :--- | :--- |
| **Buffer Starvation** | Rapid flicker, `Can't acquire next buffer` logs. | Mandatory `PresentMode::Fifo`. | ADR-001 |
| **Sprite Gralloc Errors** | App crash, `0x38`/`0x3b` format errors. | Mandatory `Mesh2d` for world primitives. | ADR-002 |
| **Silent UI Failure** | Invisible `Text2d`, clipping `Mesh2d` panels. | Mandatory `bevy_egui` for all HUD elements. | ADR-003 |

## System Inventory (`src/lib.rs`)

| System | Responsibility |
| :--- | :--- |
| `setup_world` | Spawns camera, ship, station, and asteroid field. |
| `autopilot_system` | Vector steering and arrival state transitions. |
| `mining_system` | Resource accumulation logic. |
| `cargo_display_system` | Updates world-space Mesh2d child (cargo bar) to reflect ship cargo. |
| `hud_ui_system` | Unified egui HUD (Navigation panel, Refinery bottom bar). |
| `camera_follow_system` | Interpolates camera position to ship (SpaceView) or origin (MapView). |
| `handle_input` | Unified touch-to-world coordinate mapping and entity selection. |
| `station_visual_system` | Mutates ColorMaterial asset when station repair is complete. |
| `slice_completion_system` | Centered egui window for end-of-slice confirmation. |

## Component Inventory

- **Ship**: Core state (Idle, Navigating, Mining, Docked), speed, cargo, and power cells.
- **Station**: Repair progress (0.0-1.0) and online status.
- **AsteroidField**: Marker for mining interaction.
- **AutopilotTarget**: Move-to destination and optional target entity reference.
- **MapMarker**: Identifies entities selectable from the Map overlay.
- **MainCamera**: Camera tag with EguiContextSettings attached.

## State Machine: ShipState

| From | To | Trigger |
| :--- | :--- | :--- |
| `Idle` | `Navigating` | User taps MapMarker |
| `Navigating` | `Mining` | Arrival at AsteroidField |
| `Navigating` | `Docked` | Arrival at Station |
| `Mining` | `Idle` | Cargo capacity reached |
| `Docked` | `Idle` / `Navigating` | User departs via Map |

---
*Reference: See [docs/adr/](docs/adr/) for detailed decision rationale.*
