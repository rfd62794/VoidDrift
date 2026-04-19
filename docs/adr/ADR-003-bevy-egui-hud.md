# ADR-003: bevy_egui for HUD and UI Rendering
**Date:** April 2026  
**Status:** Accepted  

## Context
Initial HUD implementation used camera-parented `Mesh2d` and `Text2d` components. On the target hardware (Moto G 2025, Mali GPU, API 35), these rendering techniques proved unreliable. `Text2d` rendered as invisible or corrupted glyphs, and camera-parented meshes suffered from erratic clipping, likely due to transform precision issues on Android 15.

## Decision
All screenspace HUD and UI elements (station panels, map overlays, resource readouts) must be implemented using `bevy_egui`.

## Rationale
`bevy_egui` handles its own font atlas and render pass, effectively bypassing the Bevy `Text2d` pipeline that was failing on the Mali driver. Immediate-mode UI also simplifies the coordination between ship state and docking interactions. Scaling is locked at `EGUI_SCALE = 3.0` to ensure thumb-friendly readability on high-DPI (2400x1080 class) displays.

## Consequences
- **Positive**: Stable text rendering and reliable UI hit-detection.
- **Positive**: Unified UI architecture; no mixing of disparate UI frameworks.
- **Constraint**: Screenspace UI is strictly limited to the `egui` render pass.
- **Decision**: All future UI components (crew screens, sector maps) must follow this pattern.
