# ADR-013: Bevy mesh for world entities, egui painter for UI layer
**Date:** May 2026
**Status:** Accepted

## Context
The project needs to render visual elements in two distinct contexts:
1. **World space**: Asteroids, ships, station, drones — entities that exist in the game world with 3D transforms
2. **UI space**: Production tree, cargo bay, HUD overlays — 2D interface elements drawn on top

Initially, both contexts used bevy_egui's painter for simplicity. However, this created problems:
- World entities rendered via egui don't benefit from Bevy's ECS transform hierarchy
- egui painter runs in UI phase, after world render phase — causes synchronization issues
- Performance overhead of converting world coordinates to egui screen coordinates every frame
- Cannot leverage Bevy's culling, batching, or z-sorting for world entities
- egui's immediate mode paradigm conflicts with Bevy's retained entity model

## Decision
Split the rendering pipeline by context:
- **World entities**: Use Bevy's native Mesh2D with ColorMaterial for all game world objects (asteroids, ships, station, drones, starfield)
- **UI elements**: Use bevy_egui's painter for all interface elements (production tree, cargo bay, HUD overlays, tutorial popups)

The boundary is clear: if it has a Transform in world space → Bevy mesh. If it's drawn on the HUD → egui painter.

## Rationale
This split provides:
- **Correct phase ordering**: World entities render in Bevy's 2D phase, UI renders in egui's UI phase
- **Transform hierarchy**: World entities benefit from Bevy's parent-child relationships
- **Performance**: Bevy's mesh batching and culling for world entities, egui's immediate mode for UI
- **Z-layering**: Bevy's z-axis for world depth, egui's painter stack for UI layering
- **Tooling fit**: Bevy's mesh inspector works for world entities, egui layout tools work for UI
- **Future migration**: Clear boundary for eventual Bevy UI migration (egui → bevy_ui)

## Consequences
- **Positive**: Eliminates phase synchronization issues between world and UI rendering
- **Positive**: Leverages Bevy's strengths (transforms, batching, culling) for world entities
- **Positive**: Leverages egui's strengths (layout, immediate mode) for UI
- **Positive**: Clear architectural boundary simplifies future refactoring
- **Constraint**: Two rendering codebases to maintain (Bevy mesh generation + egui drawing)
- **Constraint**: Some visual elements (like the production tree) need both: procedural generation in Bevy mesh for config preview, egui painter for HUD rendering
- **Migration path**: Future Bevy UI migration will replace egui painter but keep Bevy mesh for world entities
