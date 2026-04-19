# ADR-002: Mesh2d for World-Space Entities
**Date:** April 2026  
**Status:** Accepted  

## Context
Initial prototypes using Bevy's standard `SpriteBundle` (and `Sprite` components in 0.15) resulted in application crashes and severe rendering artifacts on the Moto G 2025. Logcat reported unsupported gralloc format errors (`0x38`, `0x3b`), indicating that the hardware compositor could not handle the specific texture formats being requested by the default sprite pipeline.

## Decision
All world-space entities (ship, asteroids, stations) must be rendered using `Mesh2d` primitives and `ColorMaterial` rather than `Sprite` components.

## Rationale
`Mesh2d` utilizes a simpler rendering path that relies on geometric vertex data rather than complex texture-atlas formats. Testing verified that `Mesh2d` rectangles avoid the gralloc format mismatches encountered with sprites on the Mali driver, ensuring stable rendering across different Android 15 security patches.

## Consequences
- **Positive**: Stable rendering of core game objects without driver crashes.
- **Constraint**: Restricts world-space visuals to geometric primitives or custom meshes until a Mali-compatible sprite format is confirmed.
- **Decision**: Visual complexity is deferred in favor of hardware stability for the MVP slice.
