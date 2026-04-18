# ADR 002: Prefer Mesh2d Over Sprite for World Primitives

## Context
During Phase 3 development, the Moto G 2025's Mali GPU driver reported `Invalid base format` and `mali_gralloc` unsupported format errors when using standard `bevy_sprite` components. These errors correlate with how `bevy_sprite` handles texture formats or quad-generation on this specific hardware.

## Decision
All world-space primitives (Ship, Markers, Cargo Bars) must be implemented using `Mesh2d` (via `Rectangle::new`) and `MeshMaterial2d`. The `bevy_sprite` / `Sprite` path should be avoided for primary geometric gameplay objects on Android/Mali.

## Consequences
- **Positive**: Eliminates gralloc format errors (`0x38`, `0x3b`).
- **Positive**: Direct control over the geometry and material without relying on quad-sheet batching that may be incompatible with the driver's texture unit expectations.
- **Negative**: Adds slight complexity to child-parenting coordinate math (e.g., manual center-alignment for bar scaling), as `Mesh2d` lacks the `Anchor` convenience of the `Sprite` component.
