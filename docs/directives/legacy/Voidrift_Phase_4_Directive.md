# Voidrift — Phase 4 Directive: Station UI & Refinery
**Status:** Approved — Ready for Execution  
**Gate Phase:** 4  
**Date:** April 2026  
**Depends On:** Phase 3 Gate PASSED ✅

---

## 1. Objective

Implement the station docking UI and refinery action. When the ship arrives at the Station, a simple overlay displays the player's ore and power cell counts. The player taps Refine to convert ore into power cells. This is the first meaningful decision point in the game — the player chooses when to refine.

This phase also locks the two economic constants that govern the entire slice endgame.

---

## 2. Economic Constants — Lock These Now

These constants must be defined in `src/lib.rs` and never hardcoded inline anywhere:

| Constant | Value | Notes |
|----------|-------|-------|
| `REFINERY_RATIO` | 10 | Ore units consumed per power cell produced |
| `REPAIR_COST` | 25 | Power cells required to complete station repair (Phase 5) |
| `CARGO_CAPACITY` | 100 | Existing — do not change |
| `MINING_RATE` | 8.0 | Existing — do not change |
| `SHIP_SPEED` | 120.0 | Existing — do not change |

A full cargo hold (100 ore) produces exactly 10 power cells. The repair requires 25 power cells — approximately 3 full mining runs. This pacing is intentional and must not be adjusted without review.

---

## 3. Scope Boundaries

> ⚠️ HARD LIMIT: Phase 4 is station docking UI and refinery action only.

**In scope:**
- Station docking state — ship arrives at station, docking UI appears
- UI overlay showing: current ore count, current power cell count, Refine button
- Refine action: consumes ore at 10:1, produces power cells, updates display
- Ship component extended to track `power_cells: u32`
- Logcat confirms refinery output

**Explicitly out of scope — do not implement:**
- Repair action (Phase 5)
- Any repair progress bar or station visual change
- Trading or selling resources
- Crew or fleet UI
- Any screen beyond the single station docking panel
- Animations or transitions

---

## 4. Technical Specification

### 4.1 Ship Component Extension

Add `power_cells` to the existing Ship component:

```rust
#[derive(Component)]
struct Ship {
    state: ShipState,
    speed: f32,
    cargo: f32,          // ore accumulator (existing)
    cargo_capacity: u32, // existing
    power_cells: u32,    // NEW
}
```

### 4.2 ShipState Extension

Add `Docked` to the existing enum:

```rust
#[derive(PartialEq)]
enum ShipState {
    Idle,
    Navigating,
    Mining,
    Docked,   // NEW — at station
}
```

### 4.3 State Transition Logic

Extend the existing arrival logic in AutopilotSystem:

| From | To | Condition |
|------|----|-----------|
| Navigating | Docked | Arrived at Station entity |
| Navigating | Mining | Arrived at AsteroidField entity (existing) |
| Docked | Navigating | Player taps a map marker to depart |

When transitioning to `Docked`:
- Unload cargo (set `cargo = 0.0`) — same as current Station arrival behaviour
- Show docking UI overlay
- Log: `[Voidrift Phase 4] Docked at Station. Ore unloaded.`

### 4.4 Docking UI Overlay

Rendered as `Mesh2d` world-space entities parented to the camera (consistent with ADR-002 — no Sprite components). The overlay appears when `ShipState == Docked` and is hidden otherwise.

**Layout — logical screen coordinates, camera-parented:**

| Element | Description | Position |
|---------|-------------|----------|
| Background panel | Dark rectangle, 200w × 120h | Centre screen |
| Ore label | `Text2d`: "ORE: {cargo}/100" | Panel top-left offset |
| Power cells label | `Text2d`: "CELLS: {power_cells}" | Below ore label |
| Refine button | Cyan rectangle, 120w × 32h | Panel bottom-centre |
| Refine label | `Text2d`: "REFINE" | Centred on button |

> ⚠️ Text2d caused instability in Phase 3. Before adding labels, verify on device with the panel background only first. Add text as a second deploy if the background is stable. Do not add both in one build.

### 4.5 Refinery Action

Triggered when player taps the Refine button while `ShipState == Docked`:

```
// Pseudocode
fn refine_action(ship) {
    let cells_producible = (ship.cargo as u32) / REFINERY_RATIO;
    if cells_producible == 0 {
        log("[Voidrift Phase 4] Refinery: insufficient ore.");
        return;
    }
    let ore_consumed = cells_producible * REFINERY_RATIO;
    ship.cargo -= ore_consumed as f32;
    ship.power_cells += cells_producible;
    log("[Voidrift Phase 4] Refined {ore_consumed} ore → {cells_producible} cells. Total: {ship.power_cells}");
}
```

Integer division is intentional — partial batches are not refined. 95 ore produces 9 cells, consuming 90 ore, leaving 5 ore in cargo.

### 4.6 Departure from Station

When the player taps a map marker while Docked:
- Hide docking UI
- Set `ShipState::Navigating`
- Set autopilot target as normal

The map toggle button must remain accessible while docked. Do not hide it.

---

## 5. File Scope

Only these files may be modified in Phase 4:

| File | Change |
|------|--------|
| `src/lib.rs` | Ship component extension, Docked state, docking UI spawn/hide, refinery system, departure logic |
| `Cargo.toml` | Only if a new dependency is required — justify before adding |

**All other files are read-only for this phase.**

---

## 6. Test Anchors

All 7 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P4-01 | Ship transitions to Docked on Station arrival | Logcat: "Docked at Station" |
| TB-P4-02 | Cargo unloads on docking | Logcat confirms, cargo bar resets |
| TB-P4-03 | Docking UI panel appears on dock | Visual: panel visible on device |
| TB-P4-04 | Ore and power cell counts display correctly | Visual: correct values shown |
| TB-P4-05 | Refine button tap converts ore to power cells | Logcat: refined X ore → Y cells |
| TB-P4-06 | Insufficient ore produces no cells | Logcat: "insufficient ore" when cargo < 10 |
| TB-P4-07 | Departure hides UI and resumes navigation | Visual: panel gone, ship moves |

---

## 7. Gate 4 Completion Criteria

All of the following must be true before Phase 4 is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Ship docks at Station — UI panel appears
- [ ] Ore and power cell counts are accurate
- [ ] Refine action produces correct cell count at 10:1
- [ ] Partial batch (e.g. 95 ore → 9 cells, 5 ore remaining) handled correctly
- [ ] Departure from station works — UI hides, ship navigates
- [ ] No screen flicker or buffer starvation (PresentMode::Fifo maintained)
- [ ] All 7 test anchors TB-P4-01 through TB-P4-07 verified
- [ ] Gate screenshot (TB-P4-GATE) shows docking UI panel with ore and power cell counts visible on device

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — docking UI visible on device, raw binary PNG
3. Logcat showing dock, refine, and departure log lines

---

## 8. Known Risks

| Risk | Mitigation |
|------|-----------|
| Text2d instability (observed in Phase 3) | Deploy panel background first, add text in second deploy — do not combine |
| UI visible in wrong state | Gate all UI visibility on `ShipState == Docked` check — not on a boolean flag |
| Refine tap registering while not docked | Refinery system must check `ShipState == Docked` before processing |
| Integer division truncation | Expected and intentional — document in code comment so future agent doesn't "fix" it |

---

## 9. Pacing Note

At 10:1 ratio and 100 cargo capacity, one full mining run produces 10 power cells. The repair in Phase 5 costs 25 power cells. The player must complete approximately 3 mining runs to complete the slice. This pacing is intentional — do not adjust either constant without explicit approval.

---

*Voidrift Phase 4 Directive | April 2026 | RFD IT Services Ltd.*  
*Each phase produces a directive. No phase begins without the prior gate passing.*
