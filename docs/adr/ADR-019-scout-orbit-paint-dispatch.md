# ADR-019: Scout Mk I Orbit-Paint-Dispatch Pattern

**Status:** Accepted
**Date:** May 2026
**Context:** Scout Mk I Corrected Implementation (v3.5.14)

## Context

The original Scout Mk I implementation (v3.5.12) used a direct auto-dispatch pattern: when `ScoutEnabled.active == true`, idle Mining drones were immediately assigned to the nearest unoccupied Inner Ring asteroid without any visual feedback or dedicated Scout entity.

This approach had several issues:
- No visual representation of Scout automation
- No clear indication of which asteroids were targeted
- Difficult to debug and observe Scout behavior
- Did not match the intended design (Scout entity orbiting and painting targets)

## Decision

Implement a three-phase Scout Mk I system with a dedicated Scout entity:

1. **Scout Entity Spawning** (`scout_spawn_system`)
   - Spawns a single `DroneClass::Scout` entity when `ScoutEnabled.active` becomes true
   - Despawns Scout entity when toggled off
   - Uses `is_changed()` guard to prevent continuous respawning
   - Scout rendered as cyan-colored drone sprite (placeholder for future visual polish)

2. **Orbit-Paint-Dispatch** (`scout_orbit_system`)
   - Scout orbits Inner Ring at fixed circular path (radius from `RingConfig.inner_radius`)
   - Orbit speed and proximity threshold read from `balance.toml [scout]` section
   - On proximity to unpainted asteroid: attaches `Painted` component and spawns green Annulus ring
   - Immediately dispatches one idle Mining Mk I drone to painted asteroid
   - Tags dispatched miner with `DroneTarget` component for cleanup tracking
   - Skips already-painted asteroids (enforced by `Without<Painted>` query)
   - One paint per system tick (break after dispatch)

3. **Paint Cleanup** (`scout_paint_cleanup_system`)
   - Watches for Mining drones returning to Holding state with `DroneTarget` present
   - Despawns green Annulus ring
   - Removes `Painted` component from asteroid
   - Removes `DroneTarget` from miner (frees for reassignment)

## Components

### New Components

- **`Painted`**: Marker on asteroid entities that Scout has targeted. Stores the green Annulus ring entity for cleanup.
- **`ScoutOrbit`**: On Scout drone entity. Drives circular orbit math (angle, radius, speed).
- **`DroneTarget`**: On dispatched Mining drones. Stores the asteroid entity it was sent to.

### Configuration

Added to `balance.toml`:
```toml
[scout]
orbit_speed_rad_per_sec = 0.3
proximity_threshold = 50.0
```

## Consequences

### Positive

- Clear visual feedback: Scout entity visible on map, green rings indicate painted asteroids
- Easier to debug: can observe Scout orbit and paint behavior directly
- Matches intended design: Scout entity with circular orbit and painting behavior
- Extensible: foundation for future Scout Mk II (patrol behavior) and Scout upgrades

### Negative

- More complex than direct auto-dispatch (3 systems vs 1)
- Additional entity overhead (Scout entity, ring entities per painted asteroid)
- Requires additional config values in balance.toml

### Risks

- Scout entity could get out of sync with `ScoutEnabled` if systems are misconfigured
- Ring entity cleanup must be reliable to prevent memory leaks
- Orbit math must be robust (angle wrapping at TAU)

## Alternatives Considered

1. **Keep direct auto-dispatch**: Rejected due to lack of visual feedback and debugging capability
2. **Scout as UI-only overlay**: Rejected because Scout should be a world entity for future patrol behavior
3. **Multiple Scout entities**: Rejected for Mk I - reserved for future Scout Mk II or upgrades

## Implementation Notes

- Scout sprite uses cyan color placeholder (`Color::srgb(0.0, 1.0, 1.0)`) - shape polish deferred to future visual sprint
- Green Annulus rings mirror `DestinationHighlight` pattern from map.rs
- Scout orbit uses `Without<Painted>` query to automatically skip painted asteroids
- `is_changed()` guard on `ScoutEnabled` is critical to prevent continuous respawning
- Ring entities spawned with `MapElement` component for proper layering
- All three systems run in `Update` schedule with `run_if(in_state(AppState::InGame))`

## Related ADRs

- ADR-018: RingConfig for Spawning (provides inner_radius for Scout orbit)
- ADR-015: Autonomous Ship State Machine (unchanged - Scout writes to AutonomousAssignment and sets Outbound state)
