# VoidDrift — Phase 2: UI Refactor + Requests Framework
**Directive Version:** 1.0  
**Date:** April 27, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 1c + Phase 5 refactor complete, tagged `v1.2.2-stuck-ship-safety-complete`

---

## AGENT CONTRACT

You are implementing a UI restructure and Requests framework for VoidDrift, a Bevy/egui mobile-first space mining game targeting Android (Moto G 2025, 720×1604).

**You are NOT allowed to:**
- Add new game logic beyond what is specified here
- Modify mining, auto-process, autonomous, or save systems
- Remove or rename existing component fields without explicit instruction
- Introduce new dependencies without flagging for review

**You ARE responsible for:**
- Collapsing ore pipeline tabs into a single PRODUCTION tab with ComboBox
- Replacing the UPGRADES placeholder tab with a functional REQUESTS tab
- Implementing Faction ComboBox in REQUESTS tab
- Implementing per-request display with fulfillment logic
- Removing dead tab code (Station, Fleet) cleanly
- Ensuring all touch targets meet mobile minimums (44px height)

**Definition of Done:**
- Three tabs visible on dock: CARGO | PRODUCTION | REQUESTS
- PRODUCTION ComboBox cycles all four ore types correctly
- REQUESTS tab shows faction dropdown + request list
- Fulfillment button deducts resources and applies upgrade effect
- No regressions on existing CARGO tab or mining loop
- Tested on device (screenshot required before marking complete)

---

## Context: Current Tab Structure

The existing station drawer has 5 horizontal tabs:
1. CARGO — reserves grid, fleet count, drone builder progress
2. IRON — ore pipeline with auto-refine/auto-forge toggles
3. TUNGSTEN — ore pipeline with toggles
4. NICKEL — ore pipeline with toggles
5. UPGRADES — empty placeholder ("System upgrades offline")

Dead code exists for `Station` and `Fleet` tabs — not accessible but present in the loop.

**Problem:** Adding Aluminum as a 4th ore type would require a 6th tab, exceeding mobile touch target limits.

---

## Target Tab Structure

```
CARGO | PRODUCTION | REQUESTS
```

Three tabs only. All horizontal touch targets remain mobile-safe.

---

## Phase 2a: PRODUCTION Tab

### Goal
Collapse IRON, TUNGSTEN, NICKEL, and new ALUMINUM tabs into a single PRODUCTION tab with an `egui::ComboBox` ore selector.

### Aluminum Addition
Add Aluminum as a 4th ore type alongside Iron, Tungsten, Nickel:
- Ore: `AluminumOre`
- Ingot: `AluminumIngot`
- Part: `AluminumPlate` (placeholder name — confirm before implementing)
- Pipeline: Ore → Ingot → Part (same pattern as existing metals)
- Auto-refine and auto-forge toggles (same as existing)

### ComboBox Behavior
- Default selection: Iron (first in list)
- Options: Iron | Tungsten | Nickel | Aluminum
- Selecting an ore renders that ore's pipeline below the dropdown
- Pipeline UI is identical in structure across all four — only labels and resource references change
- ComboBox touch target minimum: 44px height, full tab width

### UI State
Add to `ui_state.rs`:
```rust
pub struct ProductionTabState {
    pub selected_ore: OreType,
}

pub enum OreType {
    Iron,
    Tungsten,
    Nickel,
    Aluminum,
}
```

### Cleanup
- Remove IRON, TUNGSTEN, NICKEL as standalone tabs
- Remove UPGRADES placeholder tab
- Remove dead Station and Fleet tab code from the tab loop
- Confirm no orphaned match arms remain

---

## Phase 2b: REQUESTS Tab

### Goal
Replace the UPGRADES placeholder with a functional Requests tab. Requests are the upgrade system — fulfilling a request grants a specific upgrade unlock.

### Faction ComboBox
- Rendered at top of REQUESTS tab
- Initially one faction available: `[Faction Name TBD — use "Outer Reach" as placeholder]`
- Additional factions appear here when unlocked (future phases)
- ComboBox touch target minimum: 44px height

### Request List
Each request renders as a card below the faction selector:

```
[ Request Name                          ]
[ Flavor text — one line, keep short   ]
[ Requires: 50 Aluminum Ingots          ]
[ Reward: Fleet Capacity +25%           ]
[ FULFILL ] ← button, grayed if insufficient resources
```

- Fulfill button active only when player has sufficient resources
- On fulfill: deduct resources, apply upgrade effect, mark request complete
- Completed requests remain visible but show "COMPLETE" state instead of button
- Requests do not expire

### Initial Request Set (Placeholder — confirm before finalizing)

| Request Name | Requires | Reward |
|---|---|---|
| Hull Reinforcement | 50 Iron Ingots | Cargo Capacity +25% |
| Thruster Upgrade | 50 Tungsten Ingots | Ship Speed +25% |
| Core Overclock | 50 Nickel Ingots | Power +25% |
| Fleet Expansion | 50 Aluminum Plates | Max Drones 5 → 10 |

Each request maps to one upgrade branch. One faction, four requests initially.

### Upgrade Application
Hook into existing upgrade fields on the `Station` or `Ship` component (confirm field names before implementing). Apply as a percentage multiplier or flat increment consistent with current upgrade architecture.

If no upgrade fields exist yet, add them as `f32` multipliers defaulting to `1.0`:
```rust
pub cargo_capacity_multiplier: f32,   // default 1.0
pub ship_speed_multiplier: f32,        // default 1.0
pub power_multiplier: f32,             // default 1.0
pub max_drones: u32,                   // default 5
```

### UI State
Add to `ui_state.rs`:
```rust
pub struct RequestsTabState {
    pub selected_faction: FactionId,
    pub completed_requests: Vec<RequestId>,
}

pub enum FactionId {
    OuterReach,
}

pub enum RequestId {
    HullReinforcement,
    ThrusterUpgrade,
    CoreOverclock,
    FleetExpansion,
}
```

---

## Phase 2c: Cleanup & Verification

### Dead Code Removal
- [ ] Station tab removed from tab loop
- [ ] Fleet tab removed from tab loop
- [ ] No unreachable match arms remaining
- [ ] Compiler warnings clean (`cargo check` passes with no warnings)

### Mobile Touch Target Audit
- [ ] All ComboBox elements ≥ 44px height
- [ ] All tab buttons ≥ 44px height
- [ ] FULFILL button ≥ 44px height
- [ ] Request cards readable at 720px width without horizontal scroll

### Regression Check
- [ ] CARGO tab displays correctly
- [ ] Mining loop unaffected
- [ ] Auto-process unaffected
- [ ] Save/load unaffected
- [ ] No crashes on dock/undock

---

## Test Anchors

Before marking Phase 2 complete, provide:

1. **Screenshot of PRODUCTION tab** with ComboBox open showing all 4 ore types
2. **Screenshot of PRODUCTION tab** with Aluminum pipeline visible
3. **Screenshot of REQUESTS tab** with faction dropdown and all 4 requests visible
4. **Screenshot of REQUESTS tab** after fulfilling one request (COMPLETE state visible)
5. **`cargo check` output** — zero warnings
6. **30-second play session** — dock, switch tabs, fulfill a request, undock, mine, redock — no crashes

All screenshots must be from physical device (Moto G), not emulator.

---

## File Touch Map

Expected files modified:
- `src/components/ui_state.rs` — add ProductionTabState, RequestsTabState, enums
- `src/components/game_state.rs` — add upgrade multiplier fields if absent
- `src/systems/ui/station_tabs.rs` — primary implementation target
- `src/constants.rs` — add Aluminum constants, request costs

Expected files added:
- None required — keep changes within existing module structure

---

## Out of Scope (Do Not Implement)

- Scanning system (Phase 3)
- Derelict spawning (Phase 3)
- First Contact event (Phase 3)
- Additional factions beyond OuterReach placeholder
- Request expiry or time limits
- Animated crystal farming visuals
- Any new asteroid or ship behavior

---

## Notes for Review

- Confirm `AluminumPlate` as the part name or provide alternate before implementing
- Confirm `Outer Reach` as faction placeholder name or provide alternate
- Confirm upgrade field names on existing components before adding new fields
- If `ComboBox` scroll behavior feels wrong on device, flag before shipping — do not self-correct silently
