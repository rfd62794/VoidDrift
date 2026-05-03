# Voidrift — Phase 9 Directive: Production Chain Completion
**Status:** Approved — Ready for Execution  
**Gate Phase:** 9  
**Date:** April 2026  
**Depends On:** Phase 8b Gate PASSED ✅

---

## 1. Objective

Complete the factory line. The autonomous ship assembly chain is currently incomplete — the AI Core is a one-time station upgrade and the Ship Hull does not exist as a distinct manufactured item. This phase corrects both and establishes a repeatable, two-track production chain that converges at ship assembly.

When Phase 9 is complete, the player manages two parallel production lines simultaneously:

**Structural track:** Carbon → Hull Plate → Ship Hull  
**Energy track:** Magnetite → Power Cells → AI Core  
**Assembly:** Ship Hull + AI Core → Autonomous Ship

This is the factory line. It is the core of the commander fantasy.

---

## 2. Design Intent

Every stage in the chain should feel like a discrete manufacturing decision, not an automatic conversion. The player chooses when to refine, when to forge, when to fabricate, and when to assemble. The station executes. The player directs.

The two tracks are deliberately asymmetric:

- **Structural track** is longer (3 stages) and requires Carbon runs which are longer-range trips
- **Energy track** is shorter (2 stages) but consumes more of the base's primary resource

Managing both simultaneously under power constraints is the intended pressure.

---

## 3. Production Chain — Locked

| Stage | Input | Output | Ratio | Action | Power Cost |
|-------|-------|--------|-------|--------|-----------|
| Refinery | Carbon | Hull Plate | 5:1 | REFINE HULL | `POWER_COST_HULL_FORGE` (2) |
| Forge | Hull Plate | Ship Hull | 3:1 | FORGE HULL | `POWER_COST_SHIP_FORGE` (3) |
| Fabricator | Power Cells | AI Core | 50:1 | FABRICATE CORE | `POWER_COST_AI_FABRICATE` (5) |
| Assembly | Ship Hull + AI Core | Autonomous Ship | 1+1:1 | ASSEMBLE SHIP | 0 — assembly is free |

---

## 4. New Constants — All Named

| Constant | Value | Notes |
|----------|-------|-------|
| `HULL_PLATE_COST_CARBON` | 5 | Carbon per Hull Plate — existing, confirm unchanged |
| `SHIP_HULL_COST_PLATES` | 3 | Hull Plates per Ship Hull — NEW |
| `AI_CORE_COST_CELLS` | 50 | Power Cells per AI Core — was one-time, now repeatable |
| `POWER_COST_SHIP_FORGE` | 3 | Power Cells to forge one Ship Hull |
| `POWER_COST_AI_FABRICATE` | 5 | Power Cells to fabricate one AI Core |

> ⚠️ `POWER_COST_HULL_FORGE` (2) was established in Phase 8. Do not change it. The new `POWER_COST_SHIP_FORGE` (3) is a separate constant for the Hull → Ship Hull step.

---

## 5. Station Inventory Extensions

Add two new inventory fields to the `Station` component:

```rust
// Add to existing Station component
station.ship_hulls: u32    // manufactured Ship Hulls ready for assembly
station.ai_cores: u32      // fabricated AI Cores ready for assembly
```

The existing `station.hull_plate_reserves: u32` feeds into `ship_hulls` via the forge action.

**Full station inventory after Phase 9:**

```
station.magnetite_reserves: f32   // existing
station.carbon_reserves: f32      // existing
station.power_cells: u32          // existing
station.hull_plate_reserves: u32  // existing
station.ship_hulls: u32           // NEW
station.ai_cores: u32             // NEW
```

---

## 6. Action Definitions

### 6.1 FORGE HULL (New Action)

Converts Hull Plates into a Ship Hull.

**Trigger:** Player taps FORGE HULL button while docked  
**Condition:** `station.hull_plate_reserves >= 3` AND `station.power_cells >= POWER_COST_SHIP_FORGE`  
**Effect:**
- Deduct 3 Hull Plates from `hull_plate_reserves`
- Deduct 3 Power Cells from `power_cells`
- Add 1 to `ship_hulls`
- Log: `[STATION AI] Ship Hull fabricated. Structural assembly ready.`

**Button states:**

| Condition | Button State | Label |
|-----------|-------------|-------|
| Plates >= 3 AND cells >= 3 | Active (cyan) | FORGE HULL |
| Plates < 3 | Greyed | FORGE HULL (need 3 plates) |
| Cells < 3 | Greyed | FORGE HULL (insufficient power) |

### 6.2 FABRICATE CORE (Replaces one-time AI Core build)

Produces a repeatable AI Core item from Power Cells.

**Trigger:** Player taps FABRICATE CORE button while docked  
**Condition:** `station.power_cells >= AI_CORE_COST_CELLS (50)` AND `station.ai_cores < 2` (no point stockpiling more than 2)  
**Effect:**
- Deduct 50 Power Cells from `power_cells`
- Deduct 5 Power Cells as fabrication cost (`POWER_COST_AI_FABRICATE`)
- Add 1 to `ai_cores`
- Remove `AiCore` marker component pattern — AI Cores are now inventory items, not station markers
- Log: `[STATION AI] AI Core fabricated. Unit ready for assembly.`

**Button states:**

| Condition | Button State | Label |
|-----------|-------------|-------|
| Cells >= 55 AND cores < 2 | Active (cyan) | FABRICATE CORE |
| Cells < 55 | Greyed | FABRICATE CORE (need 55 cells) |
| Cores >= 2 | Greyed | CORE STOCKPILE FULL |

> ⚠️ Total cost is 55 Power Cells — 50 for the Core plus 5 fabrication power cost. Communicate this clearly in the UI label or tooltip.

### 6.3 ASSEMBLE SHIP (Replaces BUILD SHIP)

Combines Ship Hull and AI Core into an Autonomous Ship.

**Trigger:** Player taps ASSEMBLE SHIP button while docked  
**Condition:** `station.ship_hulls >= 1` AND `station.ai_cores >= 1` AND autonomous ship count < 2  
**Effect:**
- Deduct 1 from `ship_hulls`
- Deduct 1 from `ai_cores`
- Spawn Autonomous Ship entity with field assignment per Phase 8 routing rules
- Log: `[STATION AI] Ship assembly complete. Autonomous unit launched.`

**Button states:**

| Condition | Button State | Label |
|-----------|-------------|-------|
| Hull >= 1 AND Core >= 1 AND ships < 2 | Active (cyan) | ASSEMBLE SHIP |
| Hull < 1 | Greyed | ASSEMBLE SHIP (no hull) |
| Core < 1 | Greyed | ASSEMBLE SHIP (no core) |
| Ships >= 2 | Hidden | — ship ceiling reached |

---

## 7. AiCore Marker Refactor

The existing `AiCore` marker component on the Station entity must be removed and replaced with the `station.ai_cores: u32` inventory field.

**Current pattern (Phase 6):**
- Build AI Core → adds `AiCore` component to Station entity
- Station has AiCore → Sector 7 discovery triggers
- Station has AiCore → BUILD SHIP button appears

**New pattern (Phase 9):**
- Fabricate AI Core → increments `station.ai_cores`
- `station.ai_cores > 0` OR `station.ship_hulls > 0` → station has manufacturing capability (discovery trigger)
- `station.ai_cores >= 1` AND `station.ship_hulls >= 1` → ASSEMBLE SHIP available

**Sector 7 discovery trigger:** Change from `AiCore component present` to `station.ai_cores > 0`. First fabrication triggers discovery. Behaviour unchanged from player perspective.

> ⚠️ This is a breaking change to the AiCore marker pattern. Identify all locations that check for `AiCore` component before removing it.

---

## 8. Docking UI Layout

The docking panel now has more actions. Reorganise into clear sections:

**Section 1 — Station AI Log** (top, full width, 5 lines)

**Section 2 — Resource Status** (full width)
```
MAG: {n} | CAR: {n} | CELLS: {n} | PLATES: {n} | HULLS: {n} | CORES: {n}
STATION POWER: {n}/50
```

**Section 3 — Refinery Row** (two buttons)
```
[REFINE CELLS]    [REFINE HULL]
```

**Section 4 — Manufacturing Row** (two buttons)
```
[FORGE HULL]    [FABRICATE CORE]
```

**Section 5 — Assembly & Maintenance Row**
```
[ASSEMBLE SHIP]    [TOP UP SHIP]    [REPAIR]
```

> ⚠️ Five sections is a lot for mobile. Verify all sections are visible and tappable at EGUI_SCALE 3.0 before finalising layout. If cramped, collapse Section 3 and 4 into a single scrollable row.

---

## 9. Pre-Implementation Research

Before writing any code, answer these two questions:

**Q1:** List every location in `src/lib.rs` that references the `AiCore` component — queries, inserts, checks. This determines the full scope of the marker refactor.

**Q2:** Does the current `BUILD SHIP` button logic reference `AiCore` component directly or through a query result? This determines whether the button needs a full rewrite or a condition swap.

Report findings before implementation begins.

---

## 10. File Scope

Only these files may be modified in Phase 9:

| File | Change |
|------|--------|
| `src/lib.rs` | New inventory fields, FORGE HULL action, FABRICATE CORE action, ASSEMBLE SHIP refactor, AiCore marker removal, docking UI layout update |
| `Cargo.toml` | Only if new dependency required — justify before adding |
| `docs/state/current.md` | Update on phase completion |

**All other files are read-only for this phase.**

---

## 11. Test Anchors

All 8 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P9-01 | FABRICATE CORE produces ai_cores inventory item | Logcat: "AI Core fabricated." + ui shows cores: 1 |
| TB-P9-02 | FABRICATE CORE repeatable — second core producible | Logcat: second fabrication succeeds |
| TB-P9-03 | FORGE HULL consumes 3 Hull Plates, produces 1 Ship Hull | Logcat + UI: hull_plates -3, ship_hulls +1 |
| TB-P9-04 | ASSEMBLE SHIP consumes 1 Hull + 1 Core | Logcat: both inventory items deducted |
| TB-P9-05 | Autonomous Ship spawns and routes correctly after assembly | Visual: orange ship departs for assigned field |
| TB-P9-06 | Sector 7 discovery triggers on first AI Core fabrication | Logcat: discovery message fires once |
| TB-P9-07 | Ship ceiling still enforced at 2 autonomous ships | Visual: ASSEMBLE SHIP hidden with 2 ships active |
| TB-P9-08 | All button states correct — greyed/active/hidden per conditions | Visual: each button state verified on device |

---

## 12. Gate 9 Completion Criteria

All of the following must be true before Phase 9 is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Full production chain playable end-to-end: Carbon → Hull Plate → Ship Hull + Power Cells → AI Core → Autonomous Ship
- [ ] AI Core fabrication is repeatable — not a one-time action
- [ ] AiCore marker component fully removed — no orphaned references
- [ ] All button states correct across all conditions
- [ ] Docking UI readable and tappable at EGUI_SCALE 3.0
- [ ] All 8 test anchors TB-P9-01 through TB-P9-08 verified
- [ ] Gate screenshot shows full resource inventory visible in docking UI on device

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — full resource status bar visible, all 6 inventory items showing
3. Logcat showing fabrication, forge, and assembly log lines in sequence

---

## 13. Economy Sanity Check

With the full chain in place, building one autonomous ship requires:

**Structural track:**
- 15 Carbon → 3 Hull Plates → 1 Ship Hull
- Power cost: 6 cells (refinery) + 3 cells (forge) = 9 cells

**Energy track:**
- 50 Power Cells for AI Core + 5 fabrication cost = 55 cells total

**Total cost per autonomous ship:** 15 Carbon + 55 Power Cells + 9 operational power

At current production rates (MINING_RATE 20.0, SHIP_SPEED 180.0) this represents approximately 8-10 minutes of active play per ship. That's the intended weight — meaningful but not grinding.

Do not adjust these ratios without a full economy recalculation.

---

*Voidrift Phase 9 Directive | April 2026 | RFD IT Services Ltd.*  
*The factory line is the game. This phase completes it.*
