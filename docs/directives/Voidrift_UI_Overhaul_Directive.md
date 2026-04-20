# Voidrift — UI Overhaul Directive: Signal Strip & Station Departments
**Status:** Approved — Ready for Execution  
**Type:** UI Architecture Overhaul  
**Date:** April 2026  
**Depends On:** Z-Layer Standardization COMPLETE ✅ | Signal Narrative Design LOCKED ✅

---

## 1. Objective

Replace the current monolithic bottom panel with a proper UI architecture:

1. **Signal Strip** — always visible ambient narrative, full width bottom
2. **Left Panel** — MAP toggle + station department tabs when docked, no visible border
3. **Tab Detail Panel** — context-sensitive bottom panel above Signal strip when docked

The current `hud_ui_system` bottom panel is completely replaced. The left panel separator line is removed. The Signal strip is implemented as a new persistent system.

---

## 2. Scope

**In scope:**
- Signal strip implementation (`SignalLog` resource, `signal_system`, opening sequence)
- Left panel: remove separator line, add department tabs when docked
- Tab detail panel: context-sensitive content above Signal strip
- Opening sequence: automatic autopilot to station, Signal lines S-001 through S-007
- Signal triggers S-001 through S-021 per the narrative design document
- Progressive tab unlock logic

**Explicitly out of scope:**
- Trader or beacon Signal sources (future)
- SHIP PORT tab full implementation (fleet list, pause/resume) — stub only
- Sound
- Any economy or gameplay logic changes
- Visual polish beyond UI layout

---

## 3. Signal Strip Specification

### 3.1 Visual

```
Position:    Bottom of screen, full width
Height:      2 lines of text at EGUI_SCALE 3.0
Background:  Color::rgba(0.05, 0.05, 0.05, 0.85) — near-black, slight transparency
Text color:  #00CC66 — terminal green
Font:        FiraSans-Bold (existing)
Prefix:      > on every line
Border:      None — Frame::NONE
```

### 3.2 Behavior

- Always visible — docked or flying, no exceptions
- When docked: shows last 3 signal lines
- When flying: shows last 2 signal lines
- New lines scroll upward — oldest line pushed off top
- Read-only — no interaction, no tap targets
- No duplicate lines — each trigger ID fires exactly once per session

### 3.3 Data Structures

Add to `components.rs`:

```rust
#[derive(Resource, Default)]
pub struct SignalLog {
    pub entries: VecDeque<String>,
    pub fired: HashSet<u32>,
}

#[derive(Resource)]
pub struct OpeningSequence {
    pub phase: OpeningPhase,
    pub timer: f32,
}

#[derive(PartialEq)]
pub enum OpeningPhase {
    Adrift,
    SignalIdentified,
    AutoPiloting,
    InRange,
    Docked,
    Complete,
}
```

### 3.4 Signal Triggers — Implementation Table

Implement each trigger in `signal_system`. Fire each ID exactly once.

| ID | Trigger Condition | Line |
|----|------------------|------|
| 1 | Game start (Startup) | `> SIGNAL RECEIVED.` |
| 2 | 2.0s after ID 1 | `> SOURCE IDENTIFIED. BEARING 047.` |
| 3 | OpeningPhase::AutoPiloting | `> MOVING TO INVESTIGATE.` |
| 4 | Station within 300 units of ship | `> STRUCTURE DETECTED. DERELICT CLASS.` |
| 5 | OpeningPhase::Docked | `> DOCKING COMPLETE.` |
| 6 | 1.0s after ID 5 | `> POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.` |
| 7 | RESERVES tab unlocks | `> REPAIRS POSSIBLE. MATERIALS REQUIRED.` |
| 8 | First Magnetite cargo unloaded | `> MAGNETITE ACQUIRED. REFINERY READY.` |
| 9 | First Power Cells produced | `> POWER CELLS PRODUCED. REPAIR THRESHOLD: 25.` |
| 10 | power_cells >= 25 | `> REPAIR THRESHOLD MET. INITIATE WHEN READY.` |
| 11 | station.online becomes true | `> POWER RESTORED. STATION ONLINE.` |
| 12 | 2.0s after ID 11 | `> AI CORE FABRICATION NOW AVAILABLE.` |
| 13 | First AI Core fabricated | `> AI CORE NOMINAL. SECTOR 7 SCAN INITIATED.` |
| 14 | 3.0s after ID 13 | `> CARBON SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 7.` |
| 15 | First Hull Plate produced | `> HULL PLATE FABRICATED. FORGE AVAILABLE.` |
| 16 | First Ship Hull forged | `> SHIP HULL COMPLETE. ASSEMBLY POSSIBLE.` |
| 17 | First autonomous ship assembled | `> AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.` |
| 18 | Second autonomous ship assembled | `> AUTONOMOUS UNIT LAUNCHED. SECTOR 7 ASSIGNED.` |
| 19 | station.power_cells < 5 (once per drop below) | `> POWER RESERVES CRITICAL. MINING RUN REQUIRED.` |
| 20 | Autonomous ship enters Holding state | `> AUTONOMOUS UNIT HOLDING. POWER INSUFFICIENT.` |
| 21 | Autonomous ship exits Holding state | `> AUTONOMOUS UNIT DISPATCHED.` |

IDs 19, 20, 21 may fire multiple times per session — remove from `fired` set on condition reset so they can refire when the condition recurs. All others fire exactly once.

---

## 4. Opening Sequence

### 4.1 Behavior

On game start, the player has no control. The ship is adrift. The opening sequence runs automatically:

| Phase | Duration | What Happens |
|-------|----------|-------------|
| Adrift | 0.5s | Ship spawns. Signal fires ID 1. |
| SignalIdentified | 2.0s | Timer runs. Signal fires ID 2. |
| AutoPiloting | Until station in range | Autopilot set to station automatically. Signal fires ID 3. |
| InRange | Until docked | Station enters range. Signal fires ID 4. |
| Docked | 1.5s | Auto-dock. Signal fires ID 5, then ID 6 after 1s. |
| Complete | — | Signal fires ID 7. Player has full control. UI unlocks. |

### 4.2 Player Input During Opening

All touch input is ignored during `OpeningPhase != Complete`. The player watches — they do not interact. This is the only time input is suppressed.

### 4.3 Opening Sequence System

New system: `opening_sequence_system` in `systems/narrative.rs` (new file).

Runs every tick until `OpeningPhase::Complete`. After complete, system can be removed from the schedule or continue running as a no-op.

---

## 5. Left Panel Specification

### 5.1 Fixed Changes

```rust
egui::SidePanel::left("left_panel")
    .frame(egui::Frame::NONE)
    .show_separator_line(false)  // REMOVES THE LINE
    .show(ctx, |ui| { ... });
```

### 5.2 Contents — Always

MAP / EXIT MAP toggle button at top. Same behavior as current.

### 5.3 Contents — When Docked Only

Department tabs below MAP toggle. Rendered as vertical button list, not egui tabs (tab bar has visual chrome — buttons are cleaner).

```
[MAP]
─────
[RESERVES]
[POWER]     ← greyed until station.online
[SMELTER]   ← greyed until station.online
[FORGE]     ← greyed until forge_unlocked
[SHIP PORT] ← greyed until autonomous_count > 0
```

Active tab: bright, full opacity
Locked tab: dim, 40% opacity, not tappable
Unlocked but inactive: normal opacity, tappable

### 5.4 Tab Unlock Conditions

| Tab | Unlock Condition |
|-----|-----------------|
| RESERVES | Always when docked |
| POWER | `station.online == true` |
| SMELTER | `station.online == true` |
| FORGE | `station.ai_cores > 0 \|\| station.ship_hulls > 0` |
| SHIP PORT | autonomous ship count > 0 |

### 5.5 Active Tab State

Track in a new resource:

```rust
#[derive(Resource, Default, PartialEq)]
pub enum ActiveStationTab {
    #[default]
    Reserves,
    Power,
    Smelter,
    Forge,
    ShipPort,
}
```

Default to RESERVES on dock. Persist selection while docked. Reset to RESERVES on undock.

---

## 6. Tab Detail Panel

Appears above the Signal strip when docked and a tab is active. Full width. Replaces current bottom panel entirely.

### 6.1 RESERVES Tab

```
MAGNETITE:  {n}     CARBON:  {n}
POWER CELLS: {n}    HULL PLATES: {n}
SHIP HULLS:  {n}    AI CORES: {n}

[REPAIR — 25 cells]   ← visible only if !station.online && power_cells >= 25
```

REPAIR button disappears permanently after repair. No other buttons in this tab.

### 6.2 POWER Tab

```
STATION POWER:  {n}/50  [████████░░]
SHIP POWER:     {n}/10  [██████░░░░]

CONSUMPTION:
  Autonomous Ships:  {n} cells/cycle
  Refinery:          1 cell/batch
  Forge:             2 cells/batch

AUTONOMOUS STATUS:
  Ship 1 (S1):  {Outbound / Mining / Returning / Holding}
  Ship 2 (S7):  {Outbound / Mining / Returning / Holding}
```

Read-only. No buttons. Pure status display.

### 6.3 SMELTER Tab

```
MAGNETITE → POWER CELLS  (10:1)
  Reserves: {n} Mag  →  {n} cells possible
  [REFINE CELLS]  ← greyed if mag < 10 or power_cells_cost insufficient

CARBON → HULL PLATES  (5:1)
  Reserves: {n} Car  →  {n} plates possible
  [REFINE HULL]  ← greyed if carbon < 5 or power insufficient
```

### 6.4 FORGE Tab

```
HULL PLATES → SHIP HULL  (3:1)
  Plates: {n}  →  {n} hulls possible
  [FORGE HULL]  ← greyed if plates < 3

POWER CELLS → AI CORE  (55 total)
  Cells: {n}
  [FABRICATE CORE]  ← greyed if cells < 55 or cores >= 2
```

### 6.5 SHIP PORT Tab (Stub)

```
ASSEMBLE SHIP
  Requires: 1 Ship Hull + 1 AI Core
  [ASSEMBLE]  ← greyed if hull < 1 or core < 1 or ships >= 2

FLEET STATUS:
  [stub — "Fleet management coming soon"]
```

Full fleet list and pause/resume deferred to next directive.

---

## 7. New File: systems/narrative.rs

Create this file for opening sequence and signal system:

```rust
// systems/narrative.rs
use bevy::prelude::*;
use crate::constants::*;
use crate::components::*;

pub fn opening_sequence_system(...) { ... }
pub fn signal_system(...) { ... }
```

Add to `systems/mod.rs`:
```rust
pub mod narrative;
```

Add to `lib.rs` system scheduling:
```rust
.add_systems(Update, (
    systems::narrative::opening_sequence_system,
    systems::narrative::signal_system,
    // ... existing systems
))
```

---

## 8. File Scope

| File | Change |
|------|--------|
| `src/components.rs` | Add SignalLog, OpeningSequence, OpeningPhase, ActiveStationTab |
| `src/systems/narrative.rs` | CREATE — opening sequence and signal system |
| `src/systems/mod.rs` | Add pub mod narrative |
| `src/systems/ui.rs` | Full rewrite of hud_ui_system — left panel, tab system, tab detail panels, signal strip rendering |
| `src/lib.rs` | Register new resources and narrative systems |
| `Cargo.toml` | READ-ONLY |

**All other files are read-only.**

---

## 9. Pre-Implementation Research

Before writing any code, answer:

1. What is the current egui version in use (bevy_egui 0.33 → egui version?)? Confirm `.show_separator_line(false)` is available on `SidePanel` in that version.
2. Does `VecDeque` require a specific import in the current codebase or is it already used?
3. What system currently fires when the ship docks? The opening sequence auto-dock must hook into that same transition.

Report findings before implementation begins.

---

## 10. Implementation Sequence

1. Add new components and resources to `components.rs`
2. Create `narrative.rs` with opening sequence stub — verify it compiles
3. Implement Signal strip rendering in `ui.rs` — deploy, verify visible on device
4. Implement opening sequence — deploy, verify S-001 through S-007 fire correctly
5. Implement left panel separator fix — deploy, verify line gone
6. Implement department tabs — deploy, verify unlock logic correct
7. Implement each tab detail panel one at a time — deploy after each
8. Wire all Signal triggers — deploy, verify each fires once

---

## 11. Completion Criteria

- [ ] Signal strip visible at bottom of screen always
- [ ] Signal strip shows correct lines when docked vs flying
- [ ] Opening sequence plays on game start — ship moves to station automatically
- [ ] S-001 through S-021 all trigger correctly on device
- [ ] Left panel separator line gone
- [ ] Department tabs visible when docked
- [ ] Tab unlock logic correct — locked tabs dim and non-tappable
- [ ] RESERVES tab shows all resource counts and REPAIR button pre-online
- [ ] POWER tab shows power levels and autonomous ship status
- [ ] SMELTER tab shows refinery actions with correct greying
- [ ] FORGE tab shows forge and fabricate actions with correct greying
- [ ] SHIP PORT tab stub visible when autonomous ship exists
- [ ] All existing gameplay functionality preserved
- [ ] No screen flicker introduced
- [ ] Gate screenshot showing Signal strip, left panel tabs, and tab detail simultaneously

---

*Voidrift UI Overhaul Directive | April 2026 | RFD IT Services Ltd.*  
*The Signal is always on. The UI reveals itself as the station wakes.*
