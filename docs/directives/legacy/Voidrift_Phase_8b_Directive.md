# Voidrift — Phase 8b Directive: Power vs Power Cells
**Status:** Approved — Ready for Execution  
**Gate Phase:** 8b (extends Phase 8 — gate does not close until this passes)  
**Date:** April 2026  
**Depends On:** Phase 8 Migration PASSED ✅

---

## 1. Context

Phase 8 migrated `power_cells` to the Station and implemented autonomous ship dispatch gating. That work is complete and verified.

This directive completes Phase 8 by implementing the Power vs Power Cells distinction across all entities. Without this, the player ship has no power resource of its own and can be locked out of play with no recovery path. That violates the core design principle: **the game must always be logically recoverable.**

---

## 2. The Distinction — Locked

**Power** is a continuous energy state. Every ship and the station has it. It depletes as operations run. It is restored by consuming Power Cells.

**Power Cells** are crafted inventory items. They are produced by the refinery from Magnetite. They are stored, transferred, and consumed to restore Power.

These are two separate values on every entity that operates autonomously.

---

## 3. Data Model

### 3.1 Player Ship

```rust
// Add to existing Ship component
ship.power: f32         // current energy — range 0.0 to SHIP_POWER_MAX
ship.power_cells: u32   // onboard Power Cell inventory
```

### 3.2 Station

```rust
// Add to existing Station component  
station.power: f32      // base energy state — range 0.0 to STATION_POWER_MAX
// station.power_cells already exists from Phase 8 migration
```

### 3.3 Autonomous Ships

```rust
// Add to existing AutonomousShip component
autonomous_ship.power: f32  // energy state — same model as player ship
```

---

## 4. Constants — All New, All Named

| Constant | Value | Notes |
|----------|-------|-------|
| `POWER_CELL_RESTORE_VALUE` | 3.0 | Power restored per cell consumed |
| `SHIP_POWER_MAX` | 10.0 | Player ship maximum power |
| `SHIP_POWER_FLOOR` | 3.0 | Self-preservation threshold |
| `SHIP_POWER_COST_TRANSIT` | 1.0 | Per transit leg (out or in) |
| `SHIP_POWER_COST_MINING` | 2.0 | Per mining operation |
| `STATION_POWER_MAX` | 50.0 | Station maximum power |
| `STATION_POWER_FLOOR` | 10.0 | Station critical threshold |
| `STATION_POWER_RESTORE_VALUE` | 5.0 | Power restored per cell consumed at station |
| `EMERGENCY_REFINE_COST` | 10 | Magnetite units consumed for emergency 1-cell production |

---

## 5. Player Ship Power Logic

### 5.1 Power Deduction

Power is deducted at discrete event points — never per tick:

| Event | Power Deducted |
|-------|---------------|
| Ship departs station (outbound) | `SHIP_POWER_COST_TRANSIT` (1.0) |
| Mining operation completes | `SHIP_POWER_COST_MINING` (2.0) |
| Ship arrives at station (inbound) | `SHIP_POWER_COST_TRANSIT` (1.0) |
| Total per full cycle | 4.0 |

### 5.2 Self-Preservation Logic

Runs on cycle completion, before depositing to station:

```
// Pseudocode — runs after each mining cycle
fn self_preservation(ship) {
    if ship.power < SHIP_POWER_FLOOR {
        if ship.power_cells > 0 {
            // Consume onboard Power Cell
            ship.power_cells -= 1;
            ship.power = (ship.power + POWER_CELL_RESTORE_VALUE).min(SHIP_POWER_MAX);
            log("[SHIP] Power Cell consumed. Power: {ship.power}");
        } else if ship.cargo_magnetite >= EMERGENCY_REFINE_COST {
            // Emergency onboard refine
            ship.cargo_magnetite -= EMERGENCY_REFINE_COST;
            ship.power_cells += 1;
            ship.power_cells -= 1; // immediately consume
            ship.power = (ship.power + POWER_CELL_RESTORE_VALUE).min(SHIP_POWER_MAX);
            log("[SHIP] Emergency refine. Power restored.");
        } else {
            // Critically low — ship returns to station immediately
            ship.state = ShipState::Navigating; // force return
            log("[SHIP] Power critical. Returning to station.");
        }
    }
}
```

### 5.3 Onboard Power Cell Inventory

The player ship carries Power Cells it produces or picks up at station:

- On docking: station deposits up to 3 Power Cells to ship inventory (if station reserves > 10)
- Player can also manually top up from station in docking UI
- Ship never carries more than 5 Power Cells onboard (weight consideration — pre-inventory system placeholder)

---

## 6. Station Power Logic

### 6.1 Station Power Restore

Station consumes Power Cells from its stockpile to maintain its own power level:

```
// Runs on station tick — once per second, not per frame
fn station_power_maintenance(station) {
    if station.power < STATION_POWER_FLOOR {
        if station.power_cells > 0 {
            station.power_cells -= 1;
            station.power = (station.power + STATION_POWER_RESTORE_VALUE).min(STATION_POWER_MAX);
            log("[STATION AI] Power Cell consumed. Base power restored.");
        } else {
            log("[STATION AI] Power Cell stockpile empty. Base power critical.");
        }
    }
}
```

### 6.2 Station Power Priority

Station power gates automation in priority order:

1. Station self-maintenance (floor: 10.0) — always first
2. Autonomous ship dispatch (costs 4 Power Cells from stockpile)
3. Refinery operation (costs 1 Power Cell from stockpile)
4. Hull Forge operation (costs 2 Power Cells from stockpile)

If station power is below floor, automation suspends regardless of Power Cell stockpile.

---

## 7. Autonomous Ship Power

Autonomous ships follow the same Power model as the player ship but simpler — no onboard refiner, no self-preservation logic beyond returning to station:

```rust
autonomous_ship.power: f32  // starts at SHIP_POWER_MAX on spawn
```

Power deducted at same event points as player ship. If autonomous ship power drops critically (below 2.0), it returns to station immediately and enters `Holding` state. Station restores its power on docking by consuming Power Cells from stockpile.

---

## 8. Docking UI Updates

Add to the existing docking panel:

**Ship status section:**
```
SHIP POWER: {ship.power}/{SHIP_POWER_MAX}
SHIP CELLS: {ship.power_cells}
```

**Station status section (existing resource bar — extend):**
```
MAGNETITE: {n} | CARBON: {n} | CELLS: {n} | HULLS: {n}
STATION POWER: {station.power}/{STATION_POWER_MAX}
```

**New action — TOP UP SHIP:**
- Button: deposits up to 3 Power Cells from station stockpile to ship inventory
- Condition: station.power_cells > 3 and ship.power_cells < 5
- Cost: 3 Power Cells from station stockpile

---

## 9. Station AI Log Entries — New

| Trigger | Log Entry |
|---------|-----------|
| Ship self-preservation fires | `[SHIP] Power Cell consumed. Power: {n}.` |
| Emergency refine fires | `[SHIP] Emergency refine initiated. Power restored.` |
| Ship forced return — critical power | `[SHIP] Power critical. Returning to station.` |
| Station power below floor | `[STATION AI] Base power critical. Suspending automation.` |
| Station power restored | `[STATION AI] Base power nominal. Resuming automation.` |
| Autonomous ship returns low power | `[STATION AI] Autonomous unit returned. Low power. Recharging.` |

> ⚠️ All log entries fire on state change only. Use the 30-second throttle pattern established in Phase 8 for sustained condition warnings.

---

## 10. File Scope

Only these files may be modified in Phase 8b:

| File | Change |
|------|--------|
| `src/lib.rs` | Power fields on Ship/Station/AutonomousShip, self-preservation logic, station power maintenance, docking UI updates, new constants |
| `Cargo.toml` | Only if new dependency required — justify before adding |
| `docs/state/current.md` | Update on phase completion |

**All other files are read-only.**

---

## 11. Test Anchors

All 8 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P8b-01 | Ship power depletes correctly per cycle | Logcat: power values decreasing at correct event points |
| TB-P8b-02 | Self-preservation consumes onboard Power Cell | Logcat: "[SHIP] Power Cell consumed." |
| TB-P8b-03 | Emergency refine fires when no cells available | Logcat: "[SHIP] Emergency refine initiated." |
| TB-P8b-04 | Ship returns to station on critical power | Visual: ship navigates home without player input |
| TB-P8b-05 | Station power maintains above floor automatically | Logcat: station power restoration firing |
| TB-P8b-06 | Automation suspends when station power critical | Logcat: "Base power critical. Suspending automation." |
| TB-P8b-07 | TOP UP SHIP button deposits cells correctly | Visual: ship.power_cells increases on tap |
| TB-P8b-08 | Autonomous ship returns on low power | Logcat: "Autonomous unit returned. Low power." |

---

## 12. Gate 8b Completion Criteria

All of the following must be true before Phase 8 fully closes:

- [ ] Player ship power depletes and restores correctly
- [ ] Self-preservation fires without player input
- [ ] Emergency refiner fires as last resort — player never locked out
- [ ] Station power distinct from Power Cell stockpile
- [ ] Automation priority order respected — station self-maintenance first
- [ ] Autonomous ships return on low power — never stranded
- [ ] Docking UI shows ship and station power clearly
- [ ] All 8 test anchors TB-P8b-01 through TB-P8b-08 verified
- [ ] Gate screenshot shows power values visible in docking UI on device

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — docking UI with ship power and station power both visible
3. Logcat showing self-preservation and station power maintenance firing

---

## 13. Design Invariant — Never Locked Out

The player must always have a recovery path. Priority:

1. Ship has Power Cells onboard → consume one
2. Ship has Magnetite cargo → emergency refine one cell
3. Ship has neither → return to station (station will restore on docking)
4. Station has no cells → player mines manually, refines manually, loop restarts

Step 4 is the floor. It requires player action but it is always available. There is no game state from which recovery is impossible.

---

*Voidrift Phase 8b Directive | April 2026 | RFD IT Services Ltd.*  
*Phase 8 gate does not close until 8b passes on device. These are one gate, not two.*
