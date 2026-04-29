# VoidDrift — Phase 3a: Pre-Refactor Cleanup Sprint
**Directive Version:** 1.0  
**Date:** April 27, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 2 complete, v2.2.0-docs-phase2-complete on main

---

## AGENT CONTRACT

This is a low-risk cleanup sprint. You are fixing three categories of pre-existing issues before the Phase 3 architectural refactor begins. These are isolated, surgical changes only.

**You are NOT allowed to:**
- Refactor any system logic or move ownership of any state
- Add new systems, components, or resources
- Change any game behavior — only logging and registration hygiene
- Touch any file not listed in the File Touch Map

**You ARE responsible for:**
- Replacing `despawn()` with `despawn_recursive()` in cleanup
- Removing the duplicate `station_visual_system` registration
- Adding `warn!()` log lines to all five silent fallback paths
- Verifying `cargo check` clean with zero warnings after each change

**Definition of Done:**
- `cleanup_world_entities` uses `despawn_recursive()` on all parent entities
- `station_visual_system` registered exactly once
- All five silent fallback paths emit `warn!()` before executing
- No behavior changes — game plays identically to before
- `cargo check` clean, zero warnings
- Physical device build verified — no regressions

---

## Fix 1: `despawn()` → `despawn_recursive()`

**File:** `src/systems/setup/world_spawn.rs`  
**Function:** `cleanup_world_entities`

**Problem:** Parent entities (ships, asteroids, station) are despawned with `commands.entity(entity).despawn()`. Child entities (meshes, labels, visual components) become orphans and persist until something else removes them. This leak gets significantly worse after the Phase 3 refactor introduces more spawned children.

**Fix:** Replace every `commands.entity(entity).despawn()` call in `cleanup_world_entities` with `commands.entity(entity).despawn_recursive()`.

**Scope:** This function only. Do not change `despawn()` calls elsewhere in the codebase without explicit instruction.

**Verify:** New game / load game cycle produces no orphaned entities. Check entity count before and after cleanup if possible.

---

## Fix 2: Remove Duplicate `station_visual_system` Registration

**File:** `src/lib.rs`  
**Lines:** `lib.rs:94` (visual chain) and `lib.rs:120` (UI/narrative group)

**Problem:** `station_visual_system` is registered in two separate system groups. It runs twice per frame, performing a redundant GPU material lookup and color write on the second pass. No behavior difference — pure waste.

**Fix:** Remove the second registration at `lib.rs:120`. Keep the registration in the visual chain at `lib.rs:94`.

**Verify:** Station visual color changes still work correctly after removal. No visual regression.

---

## Fix 3: Add `warn!()` to All Silent Fallback Paths

**Problem:** Five fallback paths silently recover ships into default states without logging. When things go wrong, there is no diagnostic signal. After the Phase 3 refactor moves state ownership, these silent paths will make regressions invisible.

Add `bevy::log::warn!()` to each path **before** the existing fallback logic executes. Do not change the fallback logic itself — only add logging.

### Path 1 — Stale target entity, no query match
**File:** `src/systems/ship_control/autopilot.rs:130–133`  
**Condition:** `target_entity` exists but matches none of the query arms (despawned asteroid)  
**Add:**
```rust
bevy::log::warn!(
    "autopilot: target entity {:?} matched no query arm — defaulting to Mining",
    target_ent
);
```

### Path 2 — No target entity
**File:** `src/systems/ship_control/autopilot.rs:135–137`  
**Condition:** `target.target_entity` is `None`  
**Add:**
```rust
bevy::log::warn!(
    "autopilot: ship {:?} has no target entity — setting Idle",
    entity
);
```

### Path 3 — Bottle collected but no berth found
**File:** `src/systems/ship_control/autopilot.rs:122–127`  
**Condition:** Bottle collected, re-targeting berth, but no berth entity found  
**Add:**
```rust
bevy::log::warn!(
    "autopilot: CarryingBottle ship {:?} found no berth — falling back to station hub",
    entity
);
```

### Path 4 — Mining target out of range
**File:** `src/systems/game_loop/mining.rs:86–89`  
**Condition:** Ship is Mining, has a target, but is > 80 units away  
**Add:**
```rust
bevy::log::warn!(
    "mining: ship {:?} too far from target {:?} — clearing mining target",
    entity,
    ship.current_mining_target
);
```

### Path 5 — No berth found on cargo return
**File:** `src/systems/game_loop/mining.rs:148–153`  
**Condition:** Ship returning to station, no berth found  
**Add:**
```rust
bevy::log::warn!(
    "mining: ship {:?} returning to station but no berth found — targeting hub position",
    entity
);
```

---

## Verification Checklist

### After Fix 1
- [ ] New game starts cleanly
- [ ] Load game restores correctly
- [ ] No visual orphans (station arms, ship meshes persisting after cleanup)

### After Fix 2
- [ ] Station visual color responds correctly to station state changes
- [ ] No duplicate color writes visible in frame profiling

### After Fix 3
- [ ] `cargo check` passes with zero warnings
- [ ] Run a play session — confirm no warn! lines fire under normal operation
- [ ] Force an edge case (let an asteroid despawn while targeted) — confirm warn! appears in logcat
- [ ] All five paths compile correctly with entity/component identifiers in scope

### Final
- [ ] `cargo check` clean across all three fixes
- [ ] Physical device build — game plays identically to Phase 2
- [ ] No new regressions in mining loop, bottle collection, or requests

---

## File Touch Map

Expected files modified:
- `src/systems/setup/world_spawn.rs` — Fix 1
- `src/lib.rs` — Fix 2
- `src/systems/ship_control/autopilot.rs` — Fix 3 (Paths 1, 2, 3)
- `src/systems/game_loop/mining.rs` — Fix 3 (Paths 4, 5)

Expected files added:
- None

---

## Out of Scope (Do Not Implement)

- Event bus implementation
- System splitting or responsibility redistribution
- New components, resources, or events
- Any changes to `autonomous.rs`, `opening_sequence_system`, `content.rs`, or `signal_system`
- Any UI changes
- Save system changes
- The LOGS tab or FORGE tab rename
- Any narrative additions
