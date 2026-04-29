# VoidDrift — Phase 3c: Event Bus Gaps & Spawn Deduplication
**Directive Version:** 1.0  
**Date:** April 28, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 3b complete, v2.4.0-phase3b-event-bus tagged

---

## AGENT CONTRACT

You are closing four remaining SRP violations identified in the post-Phase-3b audit. This directive covers the event bus gaps in `autonomous.rs` and `autopilot.rs`, and the duplicated ship spawn code in `bottle.rs` and `asteroid_input.rs`.

**You are NOT allowed to:**
- Change any game balancing values or constants
- Add new gameplay features or mechanics
- Touch any file not listed in the File Touch Map
- Change behavior — only ownership of that behavior moves

**You ARE responsible for:**
- Wiring `autonomous.rs` unloads through `ShipDockedWithCargo` event
- Removing `add_log_entry` call from `autonomous.rs`
- Closing the hub dock direct mutation in `autopilot.rs`
- Extracting a shared `spawn_drone_ship` function to eliminate duplicated ship spawn code in `bottle.rs` and `asteroid_input.rs`
- Removing queue decrement from input systems, moving to economy.rs

**Definition of Done:**
- `autonomous.rs` fires `ShipDockedWithCargo` instead of directly writing reserves
- `autonomous.rs` does not write to `station.log` or any narrative resource
- `autopilot.rs` hub dock path fires event instead of direct Station mutation
- Ship spawn code exists in exactly one place
- `bottle_input_system` and `asteroid_input_system` handle input and tap detection only
- Queue decrements happen in economy.rs, not input systems
- `cargo check` clean, zero warnings
- Behavioral parity verified on device

---

## Full Context: What Is Currently Wrong

### Finding #1 — `autonomous.rs` bypasses event bus
**File:** `src/systems/game_loop/autonomous.rs:77–82`

```rust
match assignment.ore_type {
    OreDeposit::Iron => station.iron_reserves += ship.cargo,
    OreDeposit::Tungsten => station.tungsten_reserves += ship.cargo,
    OreDeposit::Nickel => station.nickel_reserves += ship.cargo,
    OreDeposit::Aluminum => station.aluminum_reserves += ship.cargo,
}
```

Also at `autonomous.rs:63–64`: `dock_state` and `resume_timer` written directly.
Also at `autonomous.rs:21`: `add_log_entry(&mut station, ...)` — narrative write from transport system.

`economy.rs` owns all cargo unloading. This is a surviving violation of the exact pattern fixed in Phase 3b. Any future reaction to autonomous unloads (audio, autosave, analytics) has no hook.

### Finding #2 — `autopilot.rs` hub dock still mutates Station directly
**File:** `src/systems/ship_control/autopilot.rs:63–76`

Hub dock path (opening sequence cinematic only) writes:
- `station.*_reserves += ship.cargo` (no-op in practice — opening drone carries zero cargo)
- `station.dock_state = StationDockState::Resuming`
- `station.resume_timer = STATION_RESUME_DELAY`

These writes live outside `economy.rs`. Safe now, future trap if opening sequence gains cargo.

### Finding #3/#4 — Duplicated ship spawn code
**Files:** `src/systems/narrative/bottle.rs:51–124` and `src/systems/ship_control/asteroid_input.rs:53–102`

Both `bottle_input_system` and `asteroid_input_system` contain:
1. Input handling and touch detection
2. Full ship entity construction (mesh, material, child visuals) — identical boilerplate in both files
3. Queue mutation (`queue.available_count -= 1`)

Ship spawning is duplicated verbatim. Queue decrement belongs to economy, not input handling. As ship visuals evolve, both files must be updated in sync — they will diverge.

---

## Implementation Plan

### Fix 1 — Wire `autonomous.rs` through event bus

**File:** `src/systems/game_loop/autonomous.rs`

On autonomous ship dock (currently `autonomous.rs:63–82`):

**Remove:**
```rust
station.dock_state = StationDockState::Resuming;
station.resume_timer = STATION_RESUME_DELAY;
match assignment.ore_type {
    OreDeposit::Iron => station.iron_reserves += ship.cargo,
    // ...
}
```

Also remove: `add_log_entry(&mut station, ...)` call at `autonomous.rs:21`

**Add:**
```rust
docked_with_cargo_events.send(ShipDockedWithCargo {
    ship_entity: entity,
    ore_type: assignment.ore_type,
    amount: ship.cargo,
});
```

**Add to `autonomous.rs` params:**
- `EventWriter<ShipDockedWithCargo>`

**Remove from `autonomous.rs` params:**
- `station_query` mut (if only used for these mutations — confirm before removing)
- `signal_log` or `station.log` writes

**Verify:** `economy.rs` already handles `ShipDockedWithCargo` — no changes needed there unless autonomous ship entity lifecycle differs from autopilot ships.

Note: Autonomous ships may not despawn on dock the way autopilot ships do — confirm whether `economy.rs` should despawn the entity or just update reserves. If autonomous ships persist and re-dispatch, `economy.rs` needs a flag or the event needs a variant. Flag for review before implementing if this is the case.

---

### Fix 2 — Close hub dock direct mutation in `autopilot.rs`

**File:** `src/systems/ship_control/autopilot.rs:63–76`

The hub dock path fires for the opening sequence drone only. It currently writes `dock_state` and `resume_timer` directly.

**Replace** direct mutations with:
```rust
docked_with_cargo_events.send(ShipDockedWithCargo {
    ship_entity: entity,
    ore_type: ship.ore_type,
    amount: ship.cargo, // will be 0.0 — economy.rs handles gracefully
});
```

`economy.rs` already handles `dock_state` and `resume_timer` on `ShipDockedWithCargo` — no changes needed there.

---

### Fix 3/#4 — Extract shared ship spawn function

**New file:** `src/systems/ship_control/ship_spawn.rs`

Extract the duplicated ship entity construction into a single shared function:

```rust
pub fn spawn_drone_ship(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    start_pos: Vec2,
    target: AutopilotTarget,
    ore_type: OreDeposit, // or None for bottle fetch
) -> Entity {
    // Full ship entity construction here — mesh, material, child visuals
    // Extracted verbatim from current bottle.rs / asteroid_input.rs
    // Single source of truth for ship appearance and component bundle
}
```

**In `bottle_input_system`:**
- Keep: touch detection, world-space projection, bottle targeting
- Remove: ship entity construction boilerplate
- Remove: `queue.available_count -= 1`
- Add: call `spawn_drone_ship(...)` 
- Add: fire `DroneDispatched` event (see below) OR let economy.rs decrement on dispatch

**In `asteroid_input_system`:**
- Same pattern — keep input, remove spawn boilerplate, remove queue decrement
- Call `spawn_drone_ship(...)`

**Queue decrement — two options (flag for review before implementing):**

Option A: Add a `DroneDispatched` event, fire from both input systems, `economy.rs` decrements on receive.

Option B: Move queue decrement into `spawn_drone_ship` directly (simpler, slightly less pure).

Recommend Option A for consistency with the event bus pattern. Confirm before implementing.

**Add `ship_spawn.rs` to `src/systems/ship_control/mod.rs`.**

---

## Verification Checklist

### Behavioral parity
- [ ] Autonomous drone mines asteroid → docks → reserves update correctly
- [ ] Autonomous dock triggers station resume rotation
- [ ] Opening sequence drone docks → station comes online → gameplay begins
- [ ] Tap asteroid → drone dispatches → mines → returns → reserves update
- [ ] Tap bottle → drone dispatches → collects → Signal Log + First Light card appear
- [ ] Queue count correct after each dispatch and return
- [ ] `cargo check` clean, zero warnings

### Structural verification
- [ ] `autonomous.rs` has no direct writes to `Station` reserves, `dock_state`, or `resume_timer`
- [ ] `autonomous.rs` has no writes to `station.log` or `SignalLog`
- [ ] `autopilot.rs` hub dock path has no direct Station mutations
- [ ] Ship spawn code exists in exactly one place (`ship_spawn.rs`)
- [ ] `bottle_input_system` and `asteroid_input_system` contain no ship construction boilerplate
- [ ] Queue decrement does not happen in input systems

### Device verification
- [ ] Full play session — autonomous mining, manual dispatch, bottle collect — no regressions
- [ ] Opening sequence plays correctly, station comes online
- [ ] No warn! lines firing in logcat during normal operation

---

## File Touch Map

**Modified:**
- `src/systems/game_loop/autonomous.rs` — remove direct mutations, fire ShipDockedWithCargo, remove log write
- `src/systems/ship_control/autopilot.rs` — close hub dock direct mutation path
- `src/systems/narrative/bottle.rs` — remove ship spawn boilerplate, remove queue decrement, call spawn_drone_ship
- `src/systems/ship_control/asteroid_input.rs` — same as bottle.rs
- `src/systems/ship_control/mod.rs` — add ship_spawn module
- `src/systems/game_loop/economy.rs` — add DroneDispatched handler if Option A chosen

**Created:**
- `src/systems/ship_control/ship_spawn.rs`

---

## Out of Scope (Do Not Implement)

- Mining signal log write cleanup (Phase 3d)
- Signal/quest entanglement (Phase 3d)
- station_tabs.rs dead buttons (Phase 3d)
- world_spawn.rs bundling (Phase 3d)
- ingame_startup_system split (Phase 3d)
- Any new gameplay features
- UI layout changes
- Save system format changes
