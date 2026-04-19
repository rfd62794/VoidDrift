# Voidrift — Phase 8 Directive: Power Economy & Smart Routing
**Status:** Approved — Ready for Execution  
**Gate Phase:** 8  
**Date:** April 2026  
**Depends On:** Phase 7 Gate PASSED ✅

---

## 1. Objective

Introduce power consumption as a living cost of automation, and fix autonomous ship routing so each ship covers a distinct field. The station AI becomes a competent dispatcher — it never sends a ship it can't afford to run, and it never leaves an operation unrecoverably stranded.

This phase makes the economy feel alive. Production and consumption are now in tension. The player's job is keeping the Magnetite loop funded while building toward Carbon.

---

## 2. Design Principles

**The station AI is competent, not fragile.**
- Ships are never dispatched without sufficient power for the full cycle
- All suspensions are automatic and recovery is automatic
- No player intervention required to resume a suspended operation
- No ship is ever stranded mid-field without power to return

**Power cost is a full cycle commitment, checked at dispatch.**
- The station calculates total cycle cost before departure
- If reserves are insufficient, the ship holds at station
- When reserves recover, dispatch resumes automatically

**Recovery is always logical.**
- A suspended ship waits at station — it does not sit in a field
- A suspended refinery resumes when power is available — no manual restart
- The player never has to "fix" a broken state, only maintain the supply

---

## 3. Power Cost Model — Locked

All constants defined here are final. Do not hardcode inline.

### Per-Cycle Costs

| Constant | Value | Covers |
|----------|-------|--------|
| `POWER_COST_TRANSIT_OUT` | 1 | Outbound flight to field |
| `POWER_COST_MINING` | 2 | Active ore extraction |
| `POWER_COST_TRANSIT_IN` | 1 | Return flight to station |
| `POWER_COST_CYCLE_TOTAL` | 4 | Sum — checked at dispatch gate |
| `POWER_COST_REFINERY` | 1 | Per Magnetite refinery batch |
| `POWER_COST_HULL_FORGE` | 2 | Per Carbon hull forge batch |

### Economy Verification (Pre-Implementation Check)

Verify these numbers hold before writing any code:

| Scenario | Production | Consumption | Net |
|----------|-----------|-------------|-----|
| 1 autonomous ship running | ~60 cells/min | ~24 cells/min | +36 cells/min |
| 2 autonomous ships running | ~60 cells/min | ~48 cells/min | +12 cells/min |
| 2 ships + both refineries active | ~60 cells/min | ~60 cells/min | ~0 cells/min |

Two ships plus active refining pushes toward breakeven — this is the intended pressure point. The player must keep the Magnetite loop funded or automation starts holding.

---

## 4. Autonomous Ship Routing — Locked

Ship assignment is determined at spawn. The station AI assigns fields intelligently — never two ships to the same field.

| Ship | Assigned Field | Ore |
|------|---------------|-----|
| First autonomous ship | Sector 1 | Magnetite |
| Second autonomous ship | Sector 7 | Carbon |

**Implementation rule:** On ship spawn, check how many autonomous ships already exist.
- If 0 existing ships → assign Sector 1
- If 1 existing ship → assign Sector 7
- If 2 existing ships → BUILD SHIP button hidden (two ship ceiling)

Station AI logs assignment on spawn:
```
[STATION AI] Autonomous unit assigned. Sector 1. Magnetite extraction.
[STATION AI] Autonomous unit assigned. Sector 7. Carbon extraction.
```

---

## 5. Technical Specification

### 5.1 Power Cost Application

Power is deducted at specific trigger points — not continuously per tick:

| Trigger | Cost Deducted | Timing |
|---------|--------------|--------|
| Ship dispatch gate | `POWER_COST_CYCLE_TOTAL` (4) | Before departure |
| Refinery batch completes | `POWER_COST_REFINERY` (1) | On completion |
| Hull forge batch completes | `POWER_COST_HULL_FORGE` (2) | On completion |

> ⚠️ Do NOT deduct power per tick. Deduct at discrete event points only. Per-tick deduction at 60 FPS would drain reserves in milliseconds.

### 5.2 Dispatch Gate Logic

Runs every tick for each autonomous ship in `Holding` state:

```
// Pseudocode
fn dispatch_gate(ship, station) {
    if ship.state == Holding {
        if station.power_cells >= POWER_COST_CYCLE_TOTAL {
            station.power_cells -= POWER_COST_CYCLE_TOTAL;
            ship.state = Outbound;
            log("[STATION AI] Power confirmed. Dispatching autonomous unit.");
        }
        // else: remain Holding, check again next tick
    }
}
```

### 5.3 New DroneState: Holding

Add `Holding` to the existing autonomous ship state machine:

```rust
#[derive(PartialEq)]
enum AutonomousShipState {
    Holding,    // NEW — at station, awaiting sufficient power
    Outbound,   // existing
    Mining,     // existing
    Returning,  // existing
    Unloading,  // existing
}
```

All autonomous ships spawn in `Holding` state. The dispatch gate transitions them to `Outbound` on the first tick that power conditions are met.

### 5.4 Refinery & Hull Forge Power Gate

Before processing any refinery or hull forge batch, check power:

```
// Pseudocode
fn refinery_gate(station, ship) {
    if station.power_cells >= POWER_COST_REFINERY {
        // process batch
        station.power_cells -= POWER_COST_REFINERY;
    } else {
        log("[STATION AI] Refinery offline. Insufficient power.");
        // grey button in UI
    }
}
```

The refinery button in the docking UI must reflect the power gate:
- Sufficient power: button active (cyan)
- Insufficient power: button greyed, label shows "REFINERY OFFLINE"

Same pattern for Hull Forge button.

### 5.5 Station AI Log Entries — Power System

| Trigger | Log Entry |
|---------|-----------|
| Ship holds at station — insufficient power | `[STATION AI] Insufficient power. Autonomous unit holding.` |
| Power restored — ship dispatches | `[STATION AI] Power confirmed. Dispatching autonomous unit.` |
| Refinery offline | `[STATION AI] Refinery offline. Insufficient power.` |
| Hull forge offline | `[STATION AI] Hull forge offline. Insufficient power.` |
| Power critically low (< 4 cells) | `[STATION AI] Power reserves critical. Reserve: {n} cells.` |

> ⚠️ Log entries must not fire every tick. Use a state-change trigger — log once when condition is entered, not continuously while it persists.

### 5.6 Routing Assignment

On autonomous ship spawn, assign field based on existing ship count:

```rust
// Pseudocode
fn assign_ship_field(existing_ship_count) -> SectorTarget {
    match existing_ship_count {
        0 => SectorTarget::Sector1,  // Magnetite
        1 => SectorTarget::Sector7,  // Carbon
        _ => panic!("Ship ceiling exceeded") // should never reach — BUILD SHIP hidden at 2
    }
}
```

Store `SectorTarget` as a component on the autonomous ship entity. The autonomous ship system reads this component for its fixed destination rather than hardcoding coordinates.

---

## 6. File Scope

Only these files may be modified in Phase 8:

| File | Change |
|------|--------|
| `src/lib.rs` | Power constants, Holding state, dispatch gate, refinery/forge power gates, routing assignment, Station AI log entries |
| `Cargo.toml` | Only if new dependency required — justify before adding |
| `docs/state/current.md` | Update on phase completion |

**All other files are read-only for this phase.**

---

## 7. Pre-Implementation Verification

Before writing any code, confirm two things:

**V1:** Run the economy math in §3 against current constants. Confirm net production at 2-ship scenario is positive but under pressure. If the numbers don't hold, report back before proceeding.

**V2:** Confirm the current autonomous ship state machine location in `src/lib.rs`. The `Holding` state must be added to the existing enum — do not create a parallel system.

---

## 8. Test Anchors

All 9 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P8-01 | Ship holds at station when power < 4 | Logcat: "Insufficient power. Autonomous unit holding." |
| TB-P8-02 | Ship dispatches automatically when power restored | Logcat: "Power confirmed. Dispatching autonomous unit." |
| TB-P8-03 | 4 Power Cells deducted on dispatch | Logcat confirms deduction |
| TB-P8-04 | Ship 1 assigned to Sector 1 | Visual: first ship mines Magnetite |
| TB-P8-05 | Ship 2 assigned to Sector 7 | Visual: second ship mines Carbon |
| TB-P8-06 | Refinery button greyed when power insufficient | Visual: "REFINERY OFFLINE" on button |
| TB-P8-07 | Refinery resumes automatically when power restored | Visual: button returns to active |
| TB-P8-08 | Station AI critical warning fires below 4 cells | Logcat: "Power reserves critical." |
| TB-P8-09 | BUILD SHIP button hidden when 2 ships exist | Visual: button absent with 2 autonomous ships active |

---

## 9. Gate 8 Completion Criteria

All of the following must be true before Phase 8 is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Ship holds correctly when power insufficient
- [ ] Ship dispatches automatically when power restored — no player input
- [ ] Correct power deducted per cycle — 4 cells confirmed in logcat
- [ ] Ship 1 mines Sector 1, Ship 2 mines Sector 7 — simultaneously on device
- [ ] Refinery power gate working — button states correct
- [ ] Hull forge power gate working — button states correct  
- [ ] Station AI log entries firing on state change only — not every tick
- [ ] No screen flicker or buffer starvation
- [ ] All 9 test anchors TB-P8-01 through TB-P8-09 verified
- [ ] Gate screenshot (TB-P8-GATE) shows both autonomous ships operating distinct fields simultaneously

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — both ships visible, distinct fields, power reserves visible in UI
3. Logcat showing dispatch gate, power deduction, and routing assignment

---

## 10. Known Risks

| Risk | Mitigation |
|------|-----------|
| Power deduction per tick at 60 FPS drains reserves instantly | Deduct at discrete event points only — §5.1 is explicit on this |
| Station AI log firing every tick for sustained conditions | Use state-change trigger — log once on entry, not while condition persists |
| Holding state causing ship to never dispatch if power gate logic is off-by-one | Gate checks `>=` not `>` — 4 cells exactly is sufficient |
| Two ships on same field if routing check uses wrong count | Count existing `AutonomousShip` entities before spawn, not after |

---

## 11. Balance Note

The pressure point is two ships plus active refining. At that state the economy runs near breakeven — the player must maintain the Magnetite loop or automation starts holding. This is intentional. The station AI reporting critical reserves is the signal to the player that they need to mine. The squeeze is the game.

Do not adjust power costs without a full economy recalculation. The constants in §3 were derived from session pacing on the Moto G 2025 and are balanced against the tuned MINING_RATE of 20.0 and SHIP_SPEED of 180.0.

---

*Voidrift Phase 8 Directive | April 2026 | RFD IT Services Ltd.*  
*Spec and doc first. Pre-implementation verification required before any code is written.*
