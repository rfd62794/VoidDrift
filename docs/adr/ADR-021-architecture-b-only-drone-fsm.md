# ADR-021: Architecture B Only — Single Drone FSM, No Architecture A Drones

**Date:** 2026-05-17
**Status:** Accepted

## Context

VoidDrift has two drone execution models in the codebase:

- **Architecture A** — `Ship` component, `ShipState` enum, autopilot-driven navigation
  via `AutopilotTarget`. Player-controlled. One entity exists: the opening sequence ship.
  Despawns on docking. No persistence after the opening cinematic.

- **Architecture B** — `AutonomousShip` component, five-state FSM:
  `Holding → Outbound → Mining → Returning → Unloading`. Cycles indefinitely.
  Spawned by `spawn_drone_ship` / `spawn_drone_ship_with_visuals`. Driven entirely
  by `autonomous_ship_system` in `drone/fsm.rs`. Fleet-managed via `FleetCount`.

During the Phase 3 → Phase 4 transition, Drone-1 was converted from Architecture A
to Architecture B in-place. As of the module reorganization (this session), all drone
lifecycle code lives under `src/drone/`. No Architecture A drone is built or managed
by any system after the opening sequence concludes.

## Decision

**Architecture B is the only drone architecture.** Permanently.

- `spawn_drone_ship` and `spawn_drone_ship_with_visuals` in `drone/spawn.rs` are the
  sole spawn paths for mining drones. No new spawn function may use `Ship` as the
  primary drone component.
- The `AutonomousShip` FSM is the contract. All drone behavior — dispatch, mining,
  return, unload — runs through `drone/fsm.rs`. No parallel Ship-based drone logic
  will be added.
- Architecture A (`Ship` + `AutopilotTarget`) remains in the codebase solely for the
  opening sequence ship and bottle-carrier drones. It is not extended to mining drones
  under any circumstance.
- Future drone tiers (Mk II, Mk III, Scout upgrades) are implemented as new
  `DroneClass` variants and new FSM branches inside `drone/fsm.rs`, not as new
  architecture variants.

## Consequences

- **Positive:** One system owns all drone state. `fleet_count_system`, `drone_visibility_system`,
  and the test suite in `src/tests/` have a single, stable contract to assert against.
- **Positive:** The `drone/` module boundary is permanent. No cross-contamination
  from ship_control into drone lifecycle.
- **Negative:** Bottle-carrier and opening-sequence ship remain Architecture A. They
  are isolated, non-fleet, and explicitly excluded from FleetCount queries via
  `Without<AutonomousShip>` / `Without<Drone>` filters. This split is accepted as
  permanent.

## Invariant

Any entity that enters `FleetCount` tracking MUST carry:
  `Drone { .. }` + `AutonomousShip { state: AutonomousShipState::Holding, .. }`

No entity carrying only `Ship` may be tracked by `fleet_count_system` or dispatched
by `scout_orbit_system` or `asteroid_input_system`.

## Known Debt

- Factory-built drones (`economy/process.rs :: auto_build_drones_system`) currently
  call `spawn_drone_ship` (no visuals). Must be changed to
  `spawn_drone_ship_with_visuals` to produce visible geometry. Filed for immediate
  next session.
- Empty `mod.rs` files in `systems/game_loop/`, `systems/setup/`,
  `systems/asteroid/` are benign dead files pending a cleanup pass.
