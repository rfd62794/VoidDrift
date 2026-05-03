# Voidrift — Phase 3 Directive: Mining System
**Status:** Approved — Ready for Execution  
**Gate Phase:** 3  
**Date:** April 2026  
**Depends On:** Phase 2 Gate PASSED ✅

---

## 1. Objective

Implement the mining loop. When the ship arrives at the Asteroid Field, mining begins automatically. Ore accumulates in the ship's cargo each tick. A cargo bar displays fill state. Mining stops when cargo is full.

This phase delivers the first **active gameplay feedback** — the player watches something happen as a direct result of their navigation decision.

---

## 2. Scope Boundaries

> ⚠️ HARD LIMIT: Phase 3 is strictly mining accumulation and cargo display.

**In scope:**
- `Mining` state added to `ShipState` enum
- Ore accumulates in ship cargo each tick while in Mining state
- Mining begins automatically on arrival at Asteroid Field — no additional tap required
- Mining stops when cargo reaches capacity
- Cargo bar renders as a world-space rectangle near the ship
- Logcat output confirms ore accumulation per tick

**Explicitly out of scope — do not implement:**
- Sector grid or any grid rendering
- Mining laser or any visual beam/effect
- Asteroid depletion (asteroid does not change state in Phase 3)
- Refinery or ore conversion
- Station docking UI
- Any inventory screen or panel
- Multiple ore types
- Ship upgrade logic

---

## 3. Technical Specification

### 3.1 Ship Component Extension

Extend the existing `Ship` component:

```rust
#[derive(Component)]
struct Ship {
    state: ShipState,
    speed: f32,
    cargo: u32,
    cargo_capacity: u32,
}
```

**Constants — all named, none hardcoded inline:**

| Constant | Value | Notes |
|----------|-------|-------|
| `SHIP_SPEED` | 120.0 | Existing — do not change |
| `CARGO_CAPACITY` | 100 | Max ore units ship can carry |
| `MINING_RATE` | 8 | Ore units added per second |
| `ARRIVAL_THRESHOLD` | 8.0 | Existing — do not change |

### 3.2 ShipState Extension

Add `Mining` to the existing enum:

```rust
#[derive(PartialEq)]
enum ShipState {
    Idle,
    Navigating,
    Mining,
}
```

### 3.3 State Transition Logic

| From | To | Condition |
|------|----|-----------|
| Navigating | Mining | Arrived at Asteroid Field destination |
| Navigating | Idle | Arrived at any other destination (Station) |
| Mining | Idle | `cargo >= cargo_capacity` |

**Important:** The arrival destination must be distinguishable. Tag the Asteroid Field entity with a marker component so the AutopilotSystem can check what it arrived at:

```rust
#[derive(Component)]
struct AsteroidField;
```

On arrival, AutopilotSystem checks if the destination entity has `AsteroidField` — if yes, transition to `Mining`. If no, transition to `Idle`.

### 3.4 MiningSystem

Runs every tick when `ShipState == Mining`:

```rust
// Pseudocode — agent implements in Rust
fn mining_system(time, mut ship_query) {
    let ore_this_tick = MINING_RATE * time.delta_seconds();
    ship.cargo = (ship.cargo + ore_this_tick).min(CARGO_CAPACITY);
    if ship.cargo >= CARGO_CAPACITY {
        ship.state = ShipState::Idle;
        log("[Voidrift Phase 3] Cargo full — mining complete.");
    }
}
```

Log ore accumulation **once per second**, not every tick — use an accumulator or timer to avoid logcat flooding.

### 3.5 Cargo Bar

Render a cargo bar as two stacked world-space rectangles parented to the ship entity:

| Element | Description |
|---------|-------------|
| Background | Dark grey rectangle, fixed width 40.0, height 6.0 |
| Fill | Cyan rectangle, width scales with `cargo / cargo_capacity` |
| Position | Offset above ship: `(0.0, 24.0)` relative to ship transform |
| Z | Ship z + 1 to render on top |

The fill rectangle width must update every tick to reflect current cargo. No text label required in Phase 3.

### 3.6 Logcat Requirements

The following log lines must appear and are used for gate verification:

```
[Voidrift Phase 3] Mining started.
[Voidrift Phase 3] Cargo: X/100          ← once per second while mining
[Voidrift Phase 3] Cargo full — mining complete.
```

---

## 4. File Scope

Only these files may be modified in Phase 3:

| File | Change |
|------|--------|
| `src/lib.rs` | Ship component extension, MiningSystem, cargo bar spawn/update, AsteroidField marker component, state transition logic |
| `Cargo.toml` | Only if a new dependency is required — justify before adding |

**All other files are read-only for this phase.**

---

## 5. Test Anchors

All 6 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P3-01 | Ship transitions to Mining state on arrival at Asteroid Field | Logcat: "Mining started" |
| TB-P3-02 | Ship transitions to Idle on arrival at Station | No "Mining started" log when navigating to station |
| TB-P3-03 | Ore accumulates each second while mining | Logcat: "Cargo: X/100" incrementing |
| TB-P3-04 | Mining stops at cargo capacity | Logcat: "Cargo full — mining complete" |
| TB-P3-05 | Cargo bar fill scales correctly with cargo level | Visual: bar grows from empty to full on device |
| TB-P3-06 | Cargo bar remains attached to ship during movement | Visual: bar moves with ship, does not drift |

---

## 6. Gate 3 Completion Criteria

All of the following must be true before Phase 3 is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Ship navigates to Asteroid Field and begins mining automatically
- [ ] Ship navigates to Station and does NOT begin mining
- [ ] Ore accumulates — logcat confirms incrementing cargo per second
- [ ] Mining stops when cargo full — logcat confirms
- [ ] Cargo bar visible on device, fills correctly
- [ ] Cargo bar moves with ship — does not detach
- [ ] All 6 test anchors TB-P3-01 through TB-P3-06 verified
- [ ] Gate screenshot (TB-P3-GATE) shows ship at Asteroid Field with partially filled cargo bar

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — ship at asteroid field, cargo bar partially filled, raw binary PNG
3. Logcat showing all three required Phase 3 log lines

---

## 7. Known Risks

| Risk | Mitigation |
|------|-----------|
| Cargo bar child entity may not move with ship if parented incorrectly | Use Bevy's parent/child transform hierarchy — child Transform is relative to parent |
| `MINING_RATE * delta_seconds()` produces float, cargo is u32 | Use an f32 accumulator on Ship, cast to u32 only when updating cargo display |
| Logcat flooding if ore logged every tick at 60 FPS | Use a 1-second log timer — log accumulator pattern, not per-tick |
| Destination type check requires entity reference at arrival | Store destination entity (not just Vec2) in `AutopilotTarget` — check for `AsteroidField` component on arrival |

---

## 8. Note on AutopilotTarget

The existing `AutopilotTarget` stores a `Vec2` destination. Phase 3 requires knowing **what** was arrived at, not just **where**. Extend the component:

```rust
#[derive(Component)]
struct AutopilotTarget {
    destination: Vec2,
    target_entity: Option<Entity>,
}
```

On arrival, the AutopilotSystem checks if `target_entity` has the `AsteroidField` component to determine the correct state transition. This is a small breaking change to Phase 2 code — confirm the existing navigation still works after the extension before proceeding to mining implementation.

---

*Voidrift Phase 3 Directive | April 2026 | RFD IT Services Ltd.*  
*Each phase produces a directive. No phase begins without the prior gate passing.*
