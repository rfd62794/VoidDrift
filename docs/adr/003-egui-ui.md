# ADR-003: bevy_egui for HUD and UI Rendering

## Status
Accepted

## Context
Initial HUD implementation used camera-parented `Mesh2d` and `Text2d` components. On the target hardware (Moto G 2025, Mali GPU, API 35), these rendering techniques proved unreliable, resulting in precision clipping, silent rendering failures, and illegible text.

Specifically, the driver struggled with the format/alpha blending of `Text2d` and the transform inheritance of camera-parented meshes in screenspace.

## Decision
We will use `bevy_egui` for all screenspace HUD and UI elements (Station panels, cargo readouts, map overlays, etc.). 

Key constraints:
- Unified UI architecture: No mixing of `bevy_ui` or camera-parented meshes with `egui`.
- Fixed Scaling: `EGUI_SCALE` locked at 3.0 for the Moto G 2025.
- Rationale for Scale: Initial device testing with 2.0 proved too small for reliable thumb interaction on the high-DPI (2400x1080 class) display. 3.0 ensures HUD elements and resource text are clearly readable and accessible in a handheld context.
- Rendering Layer: `egui` handles its own font atlas and render pass, effectively bypassing the driver issues encountered with `Text2d`.

## Consequences
- **Pros**: Stable text rendering, cross-platform UI consistency, simplified interaction logic (immediate mode), and robust font handling.
- **Cons**: UI is not "in-world" (though screen-space HUDs rarely need to be), and we overhead of a separate render pass (negligible for simple HUDs).
- **Future-proofing**: All future UI components (station panels, crew screens, map overlays) will follow this pattern.
