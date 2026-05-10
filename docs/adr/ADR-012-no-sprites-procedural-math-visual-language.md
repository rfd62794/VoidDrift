# ADR-012: No-sprites procedural math visual language
**Date:** May 2026
**Status:** Accepted

## Context
During Sprint 5 (Visual Overhaul), the project needed a visual language for ore, ingots, components, and the drone bay. The traditional approach would use sprite assets (PNG textures), but this presents several problems:
- Asset management overhead (versioning, file size, build pipeline)
- Limited scalability (different resolutions require multiple asset sets)
- Inability to procedurally vary visuals (all instances look identical)
- Asset bloat for the 4 ore types × 4 ingots × 4 components = 12+ sprites minimum

The game's aesthetic is "technical schematic meets sci-fi interface" — precise, mathematical, deterministic. This aligns with procedural generation rather than hand-drawn sprites.

## Decision
All visual elements in the game use procedural math-based rendering instead of sprite assets:
- **Ore nodes**: Procedural polygons with jagged edges and 5-pointed star vein patterns
- **Ingot nodes**: 3-face isometric parallelograms with depth-based shading
- **Component nodes**: Geometric primitives (trapezoids, circles, schematic lines) drawn mathematically
- **Drone bay node**: Rocket silhouette composed of geometric shapes
- **World entities**: Bevy Mesh2D with vertex colors for banding and procedural shapes

All rendering uses deterministic seeding for stable visuals across frames and sessions.

## Rationale
Procedural math-based rendering provides:
- **Asset-light**: No sprite files to manage, version, or bundle
- **Resolution-independent**: Scales perfectly to any screen size (720px mobile to 4K desktop)
- **Configurable**: All visual parameters (jaggedness, colors, dimensions) in visual.toml
- **Deterministic**: Same seed produces identical visuals every time
- **Aesthetic alignment**: Matches the technical/sci-fi interface theme
- **Performance**: Mesh generation happens once at spawn, not every frame

## Consequences
- **Positive**: Eliminates asset pipeline complexity
- **Positive**: Visuals are fully tunable via config without rebuilding assets
- **Positive**: Supports infinite visual variation (ore shapes, component details) without asset bloat
- **Positive**: Fits the game's deterministic, mathematical aesthetic
- **Constraint**: Requires procedural generation code (ore_polygon.rs, ingot_node.rs, component_nodes.rs)
- **Constraint**: Mesh generation happens at spawn time (small upfront cost, negligible after)
- **Learning curve**: Team must understand procedural generation patterns instead of sprite workflows
