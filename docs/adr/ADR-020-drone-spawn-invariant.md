# ADR-020: Drone Spawn Invariant — No DroneTarget at Creation

**Date:** 2026-05-17
**Status:** Accepted

## Context

Scout Mk I integration surfaced a persistent bug: `drone_targets=1` on the
converted Drone-1 entity despite no confirmed DroneTarget insertion in the
conversion path. Source traced across five files (autonomous.rs,
narrative_events.rs, entity_setup.rs, save/restore schema, scout_dispatch.rs)
without identification. Likely origin: autopilot.rs or asteroid_input.rs
inserting DroneTarget during opening cinematic navigation.

## Decision

DroneTarget is a dispatch-lifecycle component, not a spawn-lifecycle component.

- All drones are created in Holding state with NO DroneTarget, regardless of
  spawn path (component insert or future spawn_idle_drone).
- DroneTarget is inserted ONLY by `scout_orbit_system` at dispatch.
- DroneTarget is removed ONLY by `scout_paint_cleanup_system` on return to Holding.
- The Drone-1 conversion handler explicitly removes DroneTarget after component
  insert as a hardened invariant, independent of upstream source.

## Invariant

Any entity entering:
  `Drone { class: DroneClass::Mining }` + `AutonomousShip { state: Holding }` 
MUST NOT carry DroneTarget.

Applies to: Drone-1 conversion, any future spawn_idle_drone path, all future
drone tiers and classes.

## Deferred

- Integration test: spawn opening ship → fire OpeningCompleteEvent → assert
  no DroneTarget on resulting entity. Filed for v4.1 test sprint.
- Identify and fix actual DroneTarget source in autopilot.rs or
  asteroid_input.rs. Filed for v4.1 debt sprint.

## Fork Invariant (AntColony)

All Worker/Scout/Soldier units spawned in idle state must carry no dispatch
components. This ADR is inherited by all VoidDrift chassis forks.
