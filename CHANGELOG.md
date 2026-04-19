# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] — 2026-04-19 — Module Refactor

### Changed
- lib.rs split into constants.rs, components.rs, and src/systems/ with 9 logic files
- Each system in its own file: autopilot, mining, economy, autonomous, visuals, ui, map, setup
- lib.rs now contains app setup and plugin registration only

### Technical
- Progressive one-file-at-a-time migration with compile verification at each step
- Zero logic changes — pure structural reorganisation
- Import pattern established: use crate::constants::*, use crate::components::*
- Unified add_log_entry logic under systems::ui

---

## [0.2.0] — 2026-04-19 — Visual Polish

### Added
- Parallax starfield — 150 far stars + 50 near stars, two-speed scroll
- Asteroid irregular polygon — 8-vertex seeded random shape, depleted state
- Ship triangle mesh — directional, rotates to face travel direction
- Thruster glow — state-based visibility, player cyan / autonomous orange
- Mining beam — dynamic line from ship to asteroid while mining

### Fixed
- Subpixel shimmer on stars — integer sizes eliminate GPU anti-aliasing flicker
- Camera stutter — chained autopilot → camera → starfield execution order
- Autonomous ship beam visibility — enforced on every tick not just state exit

### Technical
- StdRng::seed_from_u64 used for asteroid shape generation
- CameraDelta resource tracks world-space camera movement per tick
- LastHeading component preserves ship facing when idle or docked

---

## [0.1.0] — 2026-04-18

### Added
- **Navigation**: Touch-based Map View with automated autopilot traversal.
- **Mining**: Automated resource accumulation when ship is stationary at asteroids.
- **Refinery**: Docking interaction for converting ore into power cells.
- **Repair**: Final narrative payoff loop; dock at station and spend power cells to restore it.
- **Unified HUD**: High-DPI compliant `bevy_egui` implementation for all screenspace UI.
- **Visual Payoff**: Dynamic station visual state change using `ColorMaterial` mutation.

### Technical
- **Engine**: Bevy 0.15 pinned for mobile stability.
- **Rendering**: Hardware-driven bypass for Mali GPU driver issues (`PresentMode::Fifo`, `Mesh2d` world entities).
- **UI Scaling**: `EGUI_SCALE = 3.0` lock for Moto G 2025.
- **Build**: Consolidated `cargo-ndk` + Gradle pipeline.

### Known Constraints
- **Text2d**: Proven unreliable/invisible on target Mali hardware.
- **Sprite Components**: Trigger gralloc format errors; world-rendering restricted to `Mesh2d` primitives.
- **Camera Parented UI**: Triggers erratic clipping on Android 15; all HUD migrated to egui render pass.
