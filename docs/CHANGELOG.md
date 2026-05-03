# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-04-27
### Fixed
- Starfield parallax system refactored from delta accumulation to absolute offset
- Stars now anchored to station world center, not camera position
- Opening sequence no longer drags starfield with ship movement
- Generation radius doubled to 2400.0 units (1,100 stars) for genuine circular coverage
- `StarLayer` component extended with `orig_pos: Vec2` to enable absolute positioning

---

## [2.8.18] - Drone Queue HUD - 2026-05-03
### Changed
- Renamed MaxDrones resource to MaxDispatch (soft dispatch limit, not fleet cap)
- Renamed station.max_drones to station.max_dispatch
- Updated HUD to display "Fleet: {available}/{total}" (ready/total fleet size)
- Available = queue.available_count, Total = available + deployed (autonomous ship entities)
- Removed misleading cap display — no hard fleet limit exists
### Fixed
- Save data field name changed to max_dispatch for clarity

## [2.8.15] - Bottle Asteroid Overlap Fix - 2026-05-03
### Fixed
- Bottle entities no longer spawn overlapping asteroids
- Improved spawn position validation

## [2.8.14] - Tutorial Input Fixes - 2026-05-03
### Fixed
- Tutorial popups no longer block game input when dismissed
- Fixed tutorial state persistence across save/load

## [2.8.13] - Tutorial Popup Fixes - 2026-05-03
### Fixed
- Tutorial popup positioning corrected for all screen sizes
- Fixed popup rendering overlap with HUD elements

## [2.8.12] - Tutorial Highlight Fix 3 - 2026-05-03
### Fixed
- TutorialHighlight ring visibility state corrected
- Fixed highlight persistence after tutorial completion

## [2.8.11] - Tutorial Highlight Fix 2 - 2026-05-03
### Fixed
- Tutorial highlight targeting improved for nearest asteroid
- Fixed highlight flickering during state transitions

## [2.8.10] - Tutorial Highlight Fix - 2026-05-03
### Fixed
- TutorialHighlight component rendering corrected
- Fixed highlight Z-order relative to game elements

## [2.8.9] - Tutorial egui Highlights - 2026-05-03
### Added
- egui-based tutorial highlighting system
- Visual indicators for tutorial targets

## [2.8.8] - Tutorial Fixes & Forge Rename - 2026-05-03
### Changed
- Renamed "Production" tab to "FORGE" in UI display
- Tutorial system fixes for Phase 4a
### Fixed
- Tutorial popup timing corrected
- Fixed tutorial state tracking

## [2.8.7] - Tutorial 4a - 2026-04-XX
### Added
- Phase 4a tutorial system (T-101 to T-106 Echo voice)
- TutorialHighlight component (cyan ring, distinct from DestinationHighlight)
- Tutorial position driver system (highlights asteroid, then bottle)
- New game guard (tutorial resets on new game, skips on load)
### Preserved
- Legacy T-001 to T-006 system (non-functional, requires InOpeningSequence ship)

## [2.8.6] - Balance Speed & Mining - 2026-04-XX
### Changed
- Increased MINING_RATE from 18.0 to 22.0 ore/sec
- Increased SHIP_SPEED from 180.0 to 210.0 units/sec
- Asteroid depletion time: ~4.5s at current mining rate

## [2.8.5] - Balance Forge & Mining - 2026-04-XX
### Changed
- Halved FORGE_HULL_TIME from 10.0s to 5.0s
- Increased MINING_RATE from 12.0 to 18.0 ore/sec

## [2.8.4] - Balance Drone Spawn Weights - 2026-04-XX
### Changed
- Reduced DRONE_BUILD_TIME from 30.0s to 18.0s per drone
- Changed Aluminum spawn weight from 25% to 10%
- Iron/Tungsten/Nickel each at 30% spawn weight

## [2.8.3] - EventBus Complete - 2026-04-XX
### Added
- Phase 3b event bus refactor complete
- 8 Bevy events for decoupled systems
- economy.rs and narrative_events.rs systems
### Refactored
- autopilot.rs to fire events instead of direct mutations
- OpeningCompleteEvent, ShipDockedWithCargo, ShipDockedWithBottle, FulfillRequestEvent, RepairStationEvent, DroneDispatched, InsufficientLaserEvent, SignalFired

## [2.8.2] - UI Refactor v2 - 2026-04-XX
### Added
- Phase 2 UI Refactor v2 complete
- PRODUCTION tab: collapsed ore pipeline tabs into single tab with ComboBox
- REQUESTS tab: replaced UPGRADES placeholder
- Aluminum ore type (10% spawn weight)
- Bottle collection mechanic (spawn, drift, tap, dual output)
- Faction ComboBox to REQUESTS tab
- Request cards with fulfillment logic

## [2.8.1] - Cleanup Complete - 2026-04-XX
### Fixed
- Phase 3a pre-refactor cleanup complete
- Replaced despawn() with despawn_recursive() in cleanup_world_entities
- Removed duplicate station_visual_system registration in lib.rs
- Added warn!() log lines to silent fallback paths in autopilot.rs and mining.rs

## [2.8.0] - Power Multiplier - 2026-04-XX
### Added
- Phase 2 closeout complete
- Station.power_multiplier wired to base mining rate
- Effective mining rate: BASE_MINING_RATE * station.power_multiplier
- power_multiplier increases by 0.25 after First Light request fulfillment

---

## [2.0.0] - 2026-04-27
### Added
- PRODUCTION tab: collapses Iron/Tungsten/Nickel/Aluminum into single tab with ComboBox
- REQUESTS tab: collected message system replacing UPGRADES placeholder
- Faction system: Signal (Ancient) as initial faction; architecture supports future additions
- Bottle collection mechanic: drifting entities, tap-to-dispatch drone, dual output to Signal Log + REQUESTS
- First Contact event: Signal Log entry + First Light request card on first Bottle collection
- Aluminum ore type: full pipeline (Ore → Ingot → AluminumCanister), included in random spawn pool
- Random radial asteroid spawning: all four ore types, equal probability, 200–500 unit range from station
- Global asteroid cap: `station.max_active_asteroids` (default 3), enforceable via future requests
- Request fulfillment: resource deduction, upgrade application, COMPLETE state persists
- `power_multiplier` on `Station` wired to base mining rate in `mining.rs`
- `RequestsTabState` persistence across save/load cycles
- Faction ComboBox in REQUESTS tab with empty state before first Bottle collected
- `narrative_canon.md`: locked narrative foundation document

### Removed
- Fixed sector spawn positions (SECTOR_1_POS, SECTOR_2_POS, SECTOR_3_POS)
- Legacy `spawn_sectors` and `spawn_map_connectors` systems
- Dedicated Iron/Tungsten/Nickel tab variants
- UPGRADES placeholder tab
- Dead Station and Fleet tab code

### Fixed
- `CarryingBottle` unload branch unreachable due to incorrect `else` nesting in `autopilot.rs`
- Dual spawn system conflict causing 5–6 asteroids at startup instead of 3
- `power_multiplier` written by UI but never read by any downstream system

---

## [Narrative Realignment] — Survival Sci-Fi Frame — 2026-04-26
### Changed
- **Narrative Scope:** Clarified that survival sci-fi is narrative justification for mechanics, not horror genre
- **Documentation Alignment:** Updated README, GDD, and created NARRATIVE_JUSTIFICATION.md
- **ECHO AI:** Reframed as helpful partner, not threatening presence
- **Faction System:** Added comprehensive faction design to GDD
### Added
- **ADR-010:** Survival narrative as mechanical justification (locks scope decision)
- **NARRATIVE_JUSTIFICATION.md:** Explains why each mechanic exists narratively
### Design Philosophy
- **Frame, Not Genre:** Black hole setting explains why mechanics exist
- **Mystery over Horror:** Focus on discovery, not dread or terror
- **Mechanical Coherence:** Every system has narrative justification
- **No Code Changes:** This was documentation realignment only

---

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
