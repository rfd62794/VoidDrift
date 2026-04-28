# VoidDrift — Phase 2 Bug Fix Sprint
**Directive Version:** 1.0  
**Date:** April 27, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 2 UI implementation complete, `cargo check` clean

---

## AGENT CONTRACT

You are fixing two Phase 2 blockers. Do not touch any system not listed in this directive. Do not refactor, rename, or improve anything outside the specified scope. Fix only what is broken.

**Definition of Done:**
- Asteroids spawn randomly across all four ore types within radial boundary
- Maximum 3 asteroids active simultaneously, cap stored on `Station`
- `CarryingBottle` unload branch fires correctly on drone return
- Signal Log entry appears after bottle collection
- First Light request card appears in REQUESTS tab after bottle collection
- All fixes survive dock/undock cycle
- `cargo check` clean, zero warnings
- Physical device screenshots provided for all test anchors

---

## Fix 1: Asteroid Spawn Refactor

### Problem
Current system uses fixed/dedicated spawn locations and ore type assignments. Aluminum was never added to the spawn pool. System needs to be replaced with fully random radial spawning.

### Target Behavior
- Any ore type (Iron, Tungsten, Nickel, Aluminum) can spawn at any valid location
- Spawn location: random radial position from station, within existing boundary range
- No weighting — all four ore types equally likely
- Maximum 3 asteroids active simultaneously (`max_active_asteroids` on `Station`, default 3)
- Spawn system checks active count before spawning — if at cap, skip
- When an asteroid despawns (depleted or lifespan expired), spawn system fills back up to cap naturally
- Existing asteroid lifespan, depletion, and retargeting logic unchanged

### Station Component Changes
Add to `Station` in `game_state.rs`:
```rust
pub max_active_asteroids: u32,  // default 3
```

This field is upgradeable in future phases via faction requests. Do not hardcode 3 anywhere else — always read from `Station`.

### Spawn Logic Changes
In `src/systems/asteroid/spawn.rs`:

Replace fixed spawn logic with:
1. Count currently active asteroids
2. If count >= `station.max_active_asteroids`, return early
3. Otherwise, pick a random ore type from `[Iron, Tungsten, Nickel, Aluminum]`
4. Pick a random radial position within existing boundary range from station
5. Spawn asteroid with selected ore type and position
6. Apply existing lifespan and ore amount logic unchanged

Random ore type selection — use existing Bevy `Res<Random>` or equivalent already in the codebase. Do not introduce a new RNG dependency.

### Aluminum Spawn Confirmation
Aluminum must appear in the random ore type pool. Verify by running long enough to observe an Aluminum asteroid spawn naturally.

---

## Fix 2: CarryingBottle Collection Event

### Problem
Drone dispatches to bottle and returns correctly. The `CarryingBottle` unload branch in `autopilot.rs` is not firing. Signal Log entry and First Light request card never appear.

### Diagnosis Step (Do This First)
Before making any changes, add a single debug log line at the entry of the `CarryingBottle` branch in `autopilot.rs`:

```rust
bevy::log::info!("CarryingBottle unload branch reached");
```

Build and run. Tap the bottle, wait for drone return. Check logcat output.

**If the log line appears:** The branch is reached but the downstream writes are failing. Trace the Signal Log write and `RequestsTabState` push — confirm they are writing to the correct persistent resource, not a local copy.

**If the log line does not appear:** The `CarryingBottle` component is not present on the drone at return time. Trace back to `bottle.rs` where `CarryingBottle` is attached — confirm the component is added to the correct entity after the bottle is collected in space, not a different entity.

### Fix Requirements
- `CarryingBottle` must be present on the returning drone entity when it reaches the station berth
- On berth arrival with `CarryingBottle`:
  1. Append Signal Log entry (First Contact flavor text)
  2. Push `CollectedRequest { id: RequestId::FirstLight, faction: FactionId::Signal, fulfilled: false }` to `RequestsTabState.collected_requests`
  3. Remove `CarryingBottle` component from drone
  4. Continue normal drone despawn cycle (do not skip or replace existing unload logic)
- `RequestsTabState` must be the persistent global resource — confirm the system has `ResMut<RequestsTabState>`, not a local variable

### Signal Log Entry Text
```
SIGNAL RECEIVED — ORIGIN UNKNOWN
Frequency matched. You were expected.
We have observed your work. It is... acceptable.
A proposal follows.
```

---

## Save File Integration

### Problem
`RequestsTabState` (collected cards, fulfilled status) is not persisted. Cards vanish on game reload.

### Fix
Add `RequestsTabState` serialization to the existing save system in `src/systems/persistence/save.rs`:

- Serialize `collected_requests: Vec<CollectedRequest>` — each entry needs `id`, `faction`, `fulfilled`
- Deserialize on load and restore to `RequestsTabState` resource
- If save file predates this field, default to empty `Vec` (no cards)
- Bottle already collected + game reloaded = cards still present, fulfilled state preserved

Do not change save file version unless the existing architecture requires it — flag for review if so.

---

## Test Anchors

Before marking this sprint complete, provide **all of the following**:

1. **Screenshot: Aluminum asteroid visible in space** — confirms it entered the random spawn pool
2. **Logcat output or screenshot** — `CarryingBottle` debug log line visible (can be removed after fix confirmed)
3. **Screenshot: Signal Log** — First Contact entry visible after bottle collection
4. **Screenshot: REQUESTS tab** — First Light card visible after bottle collection
5. **Screenshot: REQUESTS tab after reload** — card still present after save/load cycle
6. **Screenshot: First Light fulfilled** — COMPLETE state, resources deducted
7. **`cargo check` output** — zero warnings

All screenshots from physical device (Moto G), not emulator.

---

## File Touch Map

Expected files modified:
- `src/systems/asteroid/spawn.rs` — replace fixed spawn with random radial + ore type selection
- `src/components/game_state.rs` — add `max_active_asteroids` to `Station`
- `src/systems/ship_control/autopilot.rs` — fix `CarryingBottle` unload branch
- `src/systems/persistence/save.rs` — add `RequestsTabState` serialization
- `src/constants.rs` — remove any hardcoded fixed spawn positions if present

Expected files added:
- None

---

## Out of Scope (Do Not Implement)

- Spawn rate upgrades
- Lifespan upgrades  
- Inventory amount upgrades
- Scanning or ore identification
- Starfield looping / circular galaxy
- Scroll bounding fix
- Any UI changes beyond what is required to surface the fixed collection event
