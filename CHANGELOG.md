# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
