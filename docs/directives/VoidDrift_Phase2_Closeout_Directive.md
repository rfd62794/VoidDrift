# VoidDrift — Phase 2 Close-Out Directive
**Directive Version:** 1.0  
**Date:** April 27, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 2 Bug Fix Sprint complete, asteroid cap fixed, `cargo check` clean

---

## AGENT CONTRACT

This is a single-concern directive. Wire `power_multiplier` on `Station` to the base mining rate in `mining.rs`. Nothing else changes. Do not refactor, rename, reorganize, or improve anything outside the two files specified.

**Definition of Done:**
- `power_multiplier` applied to base mining rate in `mining.rs`
- Mining rate visibly faster after fulfilling First Light (+25%)
- `cargo check` clean, zero warnings
- Physical device confirmation provided

---

## The Fix

### Context
`Station.power_multiplier` is written correctly by the UI fulfillment logic when First Light is completed. It is not read anywhere. The base mining rate (`12.0 ore/sec`) is hardcoded in `mining.rs` without reference to `Station`.

### What Power Means (LOCKED)
**Power = drill/mining rate.** Signal's Ancient archetype makes you more efficient at extraction — not faster, not bigger, just more effective at the thing you're already doing. This is intentionally distinct from:
- Speed (Pirate branch) — ship movement
- Capacity (Human branch) — cargo hold
- Fleet (Borg branch) — drone count

### Change Required
In `src/systems/game_loop/mining.rs`:

Find where the base mining rate is applied (currently `12.0` ore/sec or equivalent constant).

Replace the hardcoded rate with:
```rust
let effective_mining_rate = BASE_MINING_RATE * station.power_multiplier;
```

Where `BASE_MINING_RATE` is the existing constant in `constants.rs` (confirm name before implementing — do not rename it).

The `mining` system must query `Station` to read `power_multiplier`. If `Station` is not already available in the mining system's query or params, add it as a read-only query. Do not add it as mutable.

### Expected Result
- Default state: `power_multiplier = 1.0`, mining rate unchanged at base
- After First Light fulfilled: `power_multiplier = 1.25`, mining rate 25% faster
- Player perception: asteroids deplete noticeably faster after upgrade

---

## Test Anchors

1. **Before fulfilling First Light** — mine an asteroid, note how long it takes to deplete
2. **Fulfill First Light** — confirm COMPLETE state
3. **After fulfilling First Light** — mine same ore type asteroid, confirm it depletes faster
4. **`cargo check` output** — zero warnings

Physical device confirmation required. Emulator not acceptable.

---

## File Touch Map

Expected files modified:
- `src/systems/game_loop/mining.rs` — apply `power_multiplier` to mining rate
- `src/constants.rs` — confirm `BASE_MINING_RATE` constant name only, no changes

Expected files added:
- None

---

## Out of Scope (Do Not Implement)

- Any other multiplier wiring (cargo, speed, drones — Phase 3)
- UI changes
- Refactoring of any system
- Narrative changes
- Save system changes

---

## After This Directive

Phase 2 is complete. Tag `dev` branch:

```
v2.0.0-phase2-complete
```

Merge `dev` → `main`.

Phase 3 will be a dedicated architectural refactor (SRP/ECS event bus) before any new features are added. That directive comes separately.
