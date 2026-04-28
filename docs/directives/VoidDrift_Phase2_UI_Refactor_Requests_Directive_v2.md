# VoidDrift — Phase 2: UI Refactor + Requests Framework
**Directive Version:** 2.0  
**Date:** April 27, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 1c + Phase 5 refactor complete, tagged `v1.2.2-stuck-ship-safety-complete`

---

## AGENT CONTRACT

You are implementing a UI restructure, Requests framework, and Bottle collection mechanic for VoidDrift, a Bevy/egui mobile-first space mining game targeting Android (Moto G 2025, 720×1604).

**You are NOT allowed to:**
- Add new game logic beyond what is specified here
- Modify mining, auto-process, autonomous, or save systems
- Remove or rename existing component fields without explicit instruction
- Introduce new dependencies without flagging for review
- Self-correct silently — flag unexpected behavior before proceeding

**You ARE responsible for:**
- Collapsing ore pipeline tabs into a single PRODUCTION tab with ComboBox
- Replacing the UPGRADES placeholder tab with a functional REQUESTS tab
- Implementing Faction ComboBox in REQUESTS tab
- Implementing Request cards as collected messages (not a browseable shop)
- Spawning drifting Bottle entities that player taps to collect
- Wiring collection event to Signal Log entry + Request card drop
- Removing dead tab code (Station, Fleet) cleanly
- Ensuring all touch targets meet mobile minimums (44px height)

**Definition of Done:**
- Three tabs visible on dock: CARGO | PRODUCTION | REQUESTS
- PRODUCTION ComboBox cycles all four ore types correctly
- REQUESTS tab shows only collected Request cards (empty until first Bottle collected)
- Faction ComboBox visible, initially shows Signal only
- Fulfillment button deducts resources and applies upgrade effect
- Bottles spawn, drift, player taps to dispatch drone, drone returns, log + card appear
- No regressions on existing CARGO tab or mining loop
- Tested on physical device — screenshots required before marking complete

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

## Faction & Upgrade Branch Design (LOCKED)

Four factions exist in the game world. Only **Signal** is available in Phase 2. Others arrive in future phases via Bottle collection events.

| Faction | Archetype | Upgrade Branch | Flavor |
|---|---|---|---|
| Signal | Ancient | Power | Was here first. Offers help you don't fully understand. |
| [Human Faction TBD] | Human | Capacity | Survivors. Familiar. Want you to endure. |
| [Borg Faction TBD] | Borg | Fleet / Drones | Collective efficiency. You're already becoming one of them. |
| [Pirate Faction TBD] | Pirate | Speed | Transactional. Honest about it. No pretense. |

**Phase 2 implements Signal only.** Faction ComboBox shows Signal as the sole option. Architecture must support future faction additions without structural changes.

**Upgrade multipliers live on the `Station` component** as global modifiers affecting all ships:
```rust
pub cargo_capacity_multiplier: f32,  // default 1.0
pub ship_speed_multiplier: f32,       // default 1.0
pub power_multiplier: f32,            // default 1.0
pub max_drones: u32,                  // default 5
```

---

## Phase 2a: PRODUCTION Tab

### Goal
Collapse IRON, TUNGSTEN, NICKEL, and new ALUMINUM tabs into a single PRODUCTION tab with an `egui::ComboBox` ore selector.

### Aluminum Addition
Add Aluminum as a 4th ore type alongside Iron, Tungsten, Nickel:
- Ore: `AluminumOre`
- Ingot: `AluminumIngot`
- Part: `AluminumCanister` — confirmed name (seeds Phase 3 Helium loop)
- Pipeline: Ore → Ingot → Canister (same pattern as existing metals)
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

## Phase 2b: Bottle Collection Mechanic

### Concept
Drifting Bottle entities spawn in space. The player taps a Bottle to dispatch a drone to collect it. On return, two things happen simultaneously:
1. A text entry drops into the existing Signal Log
2. A Request card drops into the REQUESTS tab

**Bottles are not mined.** They are tapped and fetched by drone. The player initiates collection deliberately.

### Bottle Entity
- Small visual — drifting slowly in space (simple distinct shape, not an asteroid)
- Tappable — same tap-to-target interaction pattern as asteroids
- On tap: nearest available drone dispatched to collect
- Drone returns to station, collection event fires
- Bottle despawns on collection
- Only one Bottle active at a time in Phase 2

### Spawn Behavior (Phase 2)
- One Bottle spawns after a short delay from game start (30–60 seconds — confirm timing before implementing)
- This Bottle triggers First Contact with Signal
- No further Bottle spawning logic required in Phase 2 — one Bottle, one event

### Collection Event
On drone return with Bottle:
```
→ Signal Log: append flavor text entry
→ REQUESTS tab: append Signal's first Request card
```

### Signal First Contact — Log Entry (placeholder text)
```
SIGNAL RECEIVED — ORIGIN UNKNOWN
Frequency matched. You were expected.
We have observed your work. It is... acceptable.
A proposal follows.
```

### Signal First Request Card
```
[ First Light                           ]
[ Something has been noticed about you. ]
[ Requires: 25 Iron Ingots              ]
[ Reward: Power +25%                    ]
[ FULFILL ] ← grayed until resources met
```

Costs and reward values are placeholders — adjust after initial playtest.

---

## Phase 2c: REQUESTS Tab

### Goal
Replace the UPGRADES placeholder with a REQUESTS tab that displays collected Request cards. Tab is empty until the player collects their first Bottle.

### Empty State
When no requests have been collected:
```
[ No signals received.                  ]
[ Something may be out there.           ]
```

### Faction ComboBox
- Rendered at top of REQUESTS tab
- Initially one faction available: Signal
- Filters displayed cards to selected faction
- Additional factions appear when their first Bottle is collected (future phases)
- ComboBox touch target minimum: 44px height

### Request Card Layout
```
[ Request Name                          ]
[ Flavor text — one line, keep short   ]
[ Requires: [resource] [amount]         ]
[ Reward: [upgrade description]         ]
[ FULFILL ] ← active only if resources met
```

- Cards appear in collection order (newest at bottom)
- Fulfilled cards remain visible, FULFILL button replaced by COMPLETE
- Cards do not expire
- Cards are not browseable before collection — they arrive, they do not pre-exist

### UI State
Add to `ui_state.rs`:
```rust
pub struct RequestsTabState {
    pub selected_faction: FactionId,
    pub collected_requests: Vec<CollectedRequest>,
}

pub struct CollectedRequest {
    pub id: RequestId,
    pub faction: FactionId,
    pub fulfilled: bool,
}

pub enum FactionId {
    Signal,
    // Future: Human, Borg, Pirate
}

pub enum RequestId {
    FirstLight,
    // Future requests added as Bottles arrive
}
```

### Upgrade Application
Apply multipliers to `Station` component fields on fulfillment. Confirm existing field names before implementing. If fields are absent, add them with defaults as specified in the Faction Design section above.

---

## Phase 2d: Cleanup & Verification

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
- [ ] Bottle tap target sufficient size for mobile finger

### Regression Check
- [ ] CARGO tab displays correctly
- [ ] Mining loop unaffected
- [ ] Auto-process unaffected
- [ ] Save/load unaffected
- [ ] No crashes on dock/undock

---

## Test Anchors

Before marking Phase 2 complete, provide **all of the following**:

1. **Screenshot: PRODUCTION tab** — ComboBox open, all 4 ore types visible
2. **Screenshot: PRODUCTION tab** — Aluminum pipeline selected and visible
3. **Screenshot: REQUESTS tab** — empty state before Bottle collected
4. **Screenshot: Bottle visible in space** — distinct from asteroids
5. **Screenshot: REQUESTS tab** — Signal first request card visible after collection
6. **Screenshot: Signal Log** — First Contact entry visible
7. **Screenshot: REQUESTS tab** — after fulfilling First Light (COMPLETE state visible)
8. **`cargo check` output** — zero warnings
9. **Play session** — mine, collect Bottle, dock, view log, view request, fulfill, undock — no crashes

All screenshots must be from physical device (Moto G), not emulator.

---

## File Touch Map

Expected files modified:
- `src/components/ui_state.rs` — ProductionTabState, RequestsTabState, enums
- `src/components/game_state.rs` — Station upgrade multiplier fields, Aluminum resource fields
- `src/components/resources.rs` — ProductionToggles Aluminum entries
- `src/systems/ui/station_tabs.rs` — primary UI implementation
- `src/systems/ui/hud/` — tab rendering updates
- `src/systems/narrative/signal.rs` — Signal Log entry on collection
- `src/constants.rs` — Aluminum constants, Bottle spawn timing

Expected files added:
- `src/systems/narrative/bottle.rs` — Bottle spawn, drift, tap-to-collect, collection event

---

## Out of Scope (Do Not Implement)

- Additional Bottle spawns beyond the first Signal contact
- Additional factions beyond Signal — architecture only, no implementation
- Scanning system
- Crystal farming
- Station visual growth
- Request expiry or time limits
- Any new asteroid or ship behavior beyond existing systems

---

## Confirmed Names & Values

| Item | Confirmed Value |
|---|---|
| Aluminum part | `AluminumCanister` |
| Initial faction | `Signal` |
| Signal archetype | Ancient |
| Upgrade host component | `Station` |
| Bottle spawn delay | 30–60s (confirm before implementing) |
| First request name | `First Light` (placeholder) |
| First request cost | 25 Iron Ingots (placeholder — adjust after playtest) |
| First request reward | Power +25% |
