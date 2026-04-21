# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Phase 10] — Tutorial UX & Map Pan — 2026-04-20
### Added
- **Contextual Tutorial System**: 6 instructional popups (T-001 to T-006) triggered by game state.
- **Map Panning**: Single-touch map dragging in MapView.
- **World-Space Labels**: `CargoOreLabel` and `CargoCountLabel` children on ship; `OreNameLabel` on asteroids.
### Changed
- **Cargo UI**: Added pulsing feedback at 95% capacity; smelter cards now show explicit conversion chains.
### Architecture
- **Trigger Pattern**: `TutorialState` resource with fired ID tracking.

## [Phase 09] — World Expansion & Pinch Zoom — 2026-04-20
### Added
- **Strategic Map Expansion**: All 6 asteroid sectors placed (Magnetite, Iron, Carbon, Tungsten, Titanite, Crystal).
- **Shape Families**: Generation logic for distinct asteroid geometries per ore type.
- **Multi-Touch Zoom**: Two-finger pinch-to-zoom on Space and Map views.
- **Gated Mining**: `LaserTier` system enforcing extraction requirements.
### Architecture
- **Inverted Input**: Multi-touch suppression for single-touch navigation and panning.

## [Phase 08] — Processing Queues & Auto-Dock — 2026-04-19
### Added
- **Industrial Queues**: Parallel production for refinement, forging, and fabrication.
- **Auto-Docking**: Configurable auto-unload and auto-smelt logic upon station arrival.
- **Power Economy**: Restored consumption/restoration loops for ship and station.
### Architecture
- **Refinement State**: `StationQueues` component managing parallel `ProcessingJob` entities.

## [Phase 07] — Signal Strip & Narrative Overhaul — 2026-04-19
### Added
- **Signal Log**: Bottom-screen narrative log with expanded scrollback view.
- **Opening Flow**: Scripted cinematic sequence from ADR-specified adrift state to successful docking.
- **Refirable Signals**: State-based telemetry (e.g., "VESSEL DEPARTED") that re-triggers on condition re-entry.
### Architecture
- **Narrative Telemetry**: `SignalLog` resource with time-stamped fired IDs.

## [Station Phase B] — Berth Architecture — 2026-04-19
### Added
- **Logical Berths**: Entities representing fixed docking slots with per-berth metadata.
- **DockedAt Component**: Persistence layer for ship-to-station locking.
### Architecture
- **Rotation Sync**: Ships now correctly inherit station rotation velocity via positional tracking.

## [Station Phase A] — Visual Overhaul — 2026-04-19
### Added
- **Procedural Hub**: Hub-and-spoke station mesh with active/inactive arm visualization.
- **Unified Meshes**: Migration from simple icons to complex procedural shapes for player and drones.

## [Directive A] — Stabilization & Disjointness — 2026-04-19
### Fixed
- **B0001 Panics**: Resolved 100% of runtime query conflicts on Mali GPU hardware.
### Architecture
- **Universal Disjointness**: Established project-wide `Without<T>` filter standard for all mutable Transform queries.

---

## [Bevy UI Migration + Code Refactor] — Planned — NOT YET IMPLEMENTED
### Planned
- Replace all bevy_egui panels with native Bevy UI nodes (flexbox layout)
- Add `bevy_ui`, `bevy_picking`, `bevy_ui_picking_backend` feature flags to Cargo.toml
- Percentage-based layout for portrait (720×1604) and landscape (1200×2000)
- Signal strip as persistent bottom panel (64px, always visible)
- Left nav (30% portrait / 20% landscape) and context panel
- `Pointer<Click>` observers replacing deprecated `Interaction` component
- Split `setup.rs` (676 lines) into focused spawn modules
- Split `narrative.rs` (450 lines) into opening_sequence, signal, tutorial
- Split `ui.rs` (454 lines) into hud, station_tabs, quest
- Remove `CargoBarFill` dead component (never queried)
- Fix `quest_update_system` not registered in Update schedule
- Fix `autopilot_system` double-registration
### Architecture
- Portrait and landscape handled by single layout system with orientation check
- egui removed entirely after all panels migrated and verified on device

## [Economy Redesign] — Planned — NOT YET IMPLEMENTED
### Planned
- Three resource tracks: Metal (Magnetite/Iron/Carbon/Tungsten/Titanite), Gas (Helium), Crystal
- FORGE department (replaces SMELTER/REFINERY) — ore to ingots, five parallel queues
- CRAFTER department (replaces FORGE) — ingots to components and composites
- Repair Kit as opening repair resource (5 kits, replaces 25 Power Cells)
- Power Cells moved to Crystal track (mid-game resource)
- Helium as passive secondary yield from all asteroid mining
- Engine tier system: Mk I (180.0) through Mk V (500.0), permanent upgrades
- Fuel Boost system: optional consumable speed burst (×1.8 Fuel Cell / ×2.4 Plasma Cell)
- Void Core as three-material MacGuffin requiring all three resource tracks
- Stargate as Precursor artifact requiring Void Core to activate
- Orbital stations above planetary bodies; player never lands
- Revised 10-objective quest chain teaching resource tracks sequentially
### Design Canon
- Full specification in `docs/design/ECONOMY.md`
- Stargate and galaxy design in `docs/design/STARGATE.md`

---

## [0.1.0] — 2026-04-18
Initial Prototype pass. Features navigation, basic mining, egui HUD, and repair loop.
### Technical
- Bevy 0.15 pinned.
- Mali GPU bypass (Fifo, Mesh2d).
- EGUI_SCALE = 3.0.
