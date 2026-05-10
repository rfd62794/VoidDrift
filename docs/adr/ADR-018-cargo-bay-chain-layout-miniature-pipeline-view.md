# ADR-018: Cargo bay chain layout as miniature pipeline view
**Date:** May 2026
**Status:** Accepted

## Context
The cargo bay (Production tab) displays the player's resource chain: ore → ingots → components → drones. Initially, this was designed as a simple list of cards with counts and buttons. However, this failed to communicate the production pipeline relationship between resources.

Players struggled to understand:
- Which ore produces which ingot
- Which ingot produces which component
- How components combine to build a drone
- Where bottlenecks occur in the pipeline
- The flow of materials through the station

The game's theme is "station as production engine" — the UI should reflect this by showing the production chain visually as a pipeline.

## Decision
Implement the cargo bay as a miniature pipeline view using a chain layout:
- **4 rows** representing the production chain:
  - Row 0: Ore nodes (Iron, Tungsten, Nickel, Aluminum) — raw materials
  - Row 1: Ingot nodes (Iron Ingot, Tungsten Ingot, Nickel Ingot, Aluminum Ingot) — processed materials
  - Row 2: Component nodes (Hull Plate, Thruster, AI Core, Canister) — fabricated outputs
  - Row 3: Drone Bay node — final destination
- **Vertical alignment**: Each column represents a production chain (e.g., Iron → Iron Ingot → Hull Plate → Drone)
- **Visual states**: Locked (ghost outline, alpha 0.2), ActiveEmpty (dim color, alpha 0.5), ActivePopulated (full color, alpha 1.0)
- **Procedural rendering**: Use the same procedural visual language as world entities (ore polygons, ingot isometric, component geometry)
- **Miniature scale**: Small symbols (24-36px) to fit in the drawer's limited height (~162px available)
- **Dynamic sizing**: Symbol size clamps based on available content height

## Rationale
This approach provides:
- **Pipeline visualization**: Players see the production chain as a flow, not isolated resources
- **Relationship clarity**: Vertical columns show which materials feed into which
- **Bottleneck detection**: Empty cells in the pipeline are immediately visible
- **Theme alignment**: Miniature production engine fits the station-as-factory theme
- **Space efficiency**: 4 rows × 4 columns fits in the drawer's constrained height
- **Visual consistency**: Uses the same procedural rendering as world entities
- **Configurable**: All visual parameters (sizes, colors, states) in visual.toml

## Consequences
- **Positive**: Communicates production chain relationship clearly
- **Positive**: Players can identify bottlenecks at a glance
- **Positive**: Fits the station-as-production-engine theme
- **Positive**: Consistent visual language with world entities
- **Constraint**: Limited vertical space (~162px) requires miniature symbols
- **Constraint**: 4×4 grid may be dense on smaller screens
- **Constraint**: Requires procedural rendering in egui (not Bevy mesh)
- **Maintenance**: Visual parameters must be tuned for drawer height variations
