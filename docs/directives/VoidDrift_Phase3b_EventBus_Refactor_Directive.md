# VoidDrift — Phase 3b: Event Bus & SRP Refactor
**Directive Version:** 1.0  
**Date:** April 28, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 3a complete, v2.3.0-phase3a-cleanup-complete on main

---

## AGENT CONTRACT

You are implementing an architectural refactor to decouple narrative, economy, and transport concerns that are currently entangled in `autopilot.rs` and the UI render systems. This is a structural change — game behavior must remain identical before and after.

**You are NOT allowed to:**
- Change any game balancing values or constants
- Add new gameplay features or mechanics
- Modify save file format or persistence logic
- Touch any file not listed in the File Touch Map
- Change behavior — only ownership of that behavior moves

**You ARE responsible for:**
- Defining five new Bevy events
- Refactoring `autopilot.rs` to fire events instead of directly mutating economy/narrative state
- Creating a new `economy.rs` system that owns cargo unload and upgrade application
- Creating a new `narrative_events.rs` system that owns signal log writes and request card drops
- Replacing direct UI mutations in `content.rs` with event fires
- Wiring `OpeningCompleteEvent` in `opening_sequence.rs`
- Registering all new events and systems in `lib.rs`
- Verifying zero behavior regressions on physical device

**Definition of Done:**
- `autopilot.rs` handles movement and arrival detection only — no direct Station mutation, no signal_log writes, no requests_tab writes, no active_tab forcing
- `content.rs` fires events on button click — no direct Station mutation
- Economy system owns all cargo unload and multiplier application
- Narrative system owns all signal log and request card writes
- `cargo check` clean, zero warnings
- Physical device verified — mining loop, bottle collection, request fulfillment all behave identically

---

## Full Context: What Is Currently Wrong

This context is provided so you can navigate the codebase without prior analysis. All citations are `file:line` against the current `src/` tree.

### Problem 1 — `autopilot.rs` is a domain hub

`autopilot_system` in `src/systems/ship_control/autopilot.rs` currently owns:

**Transport (correct — keep here):**
- Ship movement (`transform.translation += movement.extend(0.0)` — `autopilot.rs:140`)
- Arrival detection (distance threshold checks)
- State transitions: `ShipState::Navigating`, `ShipState::Mining`, `ShipState::Idle`, `ShipState::Docked`
- `AutopilotTarget` insertion for re-routing

**Economy (wrong — move to economy.rs):**
- `station.iron_reserves += ship.cargo` (`autopilot.rs:57,93`)
- `station.tungsten_reserves += ship.cargo` (`autopilot.rs:58,94`)
- `station.nickel_reserves += ship.cargo` (`autopilot.rs:59,95`)
- `station.aluminum_reserves += ship.cargo` (`autopilot.rs:60,96`)
- `station.dock_state → Resuming` (`autopilot.rs:63,101`)
- `station.resume_timer = STATION_RESUME_DELAY` (`autopilot.rs:64,101`)
- `ship.cargo = 0.0` (`autopilot.rs:98`)
- `queue.available_count += 1` (`autopilot.rs:83`)
- `commands.entity(entity).despawn_recursive()` (`autopilot.rs:86`)
- `autosave_events.send(AutosaveEvent)` (`autopilot.rs:85`)

**Narrative (wrong — move to narrative_events.rs):**
- `signal_log.entries.push_back(...)` (`autopilot.rs:70`)
- `signal_log.entries.pop_front()` (`autopilot.rs:72`)
- `requests_tab.collected_requests.push(...)` (`autopilot.rs:74`)
- `commands.entity(entity).remove::<CarryingBottle>()` (`autopilot.rs:79`)

**UI forcing (wrong — remove entirely):**
- `*active_tab = ActiveStationTab::Cargo` (`autopilot.rs:62`) — autopilot should never touch UI state

### Problem 2 — UI systems directly mutate game economy

In `src/systems/ui/hud/content.rs`:
- `content.rs:95–96` — REPAIR button: `station.repair_progress = 1.0; station.online = true` — no event, no validation, no log
- `content.rs:172` — FULFILL button: `station.iron_ingots -= 25.0` — direct deduction in render closure
- `content.rs:173` — FULFILL button: `station.power_multiplier += 0.25` — direct mutation in render closure
- `content.rs:174` — FULFILL button: `req.fulfilled = true` — direct state flip in render closure

### Problem 3 — Opening sequence writes economy state directly

In `src/systems/narrative/opening_sequence.rs`:
- `opening_sequence_system` writes `queue.available_count += 1` directly
- `opening_sequence_system` sets `station.dock_state` directly
- No event separates "opening sequence ended" from "economy should now initialize"

---

## Implementation Plan

### Step 1 — Define Events

Create or add to `src/components/resources.rs` (or a new `src/components/events.rs` if cleaner):

```rust
/// Fired by autopilot when a ship arrives at its berth with cargo
pub struct ShipDockedWithCargo {
    pub ship_entity: Entity,
    pub ore_type: OreType,
    pub amount: f32,
}

/// Fired by autopilot when a ship arrives at its berth carrying a bottle
pub struct ShipDockedWithBottle {
    pub ship_entity: Entity,
}

/// Fired by content.rs when player clicks FULFILL on a request card
pub struct FulfillRequestEvent {
    pub request_id: RequestId,
    pub faction_id: FactionId,
}

/// Fired by content.rs when player clicks REPAIR STATION button
pub struct RepairStationEvent;

/// Fired by opening_sequence_system when the cinematic ends and gameplay begins
pub struct OpeningCompleteEvent;
```

Register all five in `lib.rs` via `.add_event::<EventName>()`.

---

### Step 2 — Refactor `autopilot.rs`

**File:** `src/systems/ship_control/autopilot.rs`

**Goal:** Transport and arrival detection only. Fire events on arrival. Remove all direct economy and narrative mutations.

#### On berth arrival (currently `autopilot.rs:57–86`):

Replace direct mutations with event fires:

```rust
// REMOVE: station.iron_reserves += ship.cargo (and all other reserve mutations)
// REMOVE: station.dock_state = DockState::Resuming
// REMOVE: station.resume_timer = STATION_RESUME_DELAY
// REMOVE: queue.available_count += 1
// REMOVE: autosave_events.send(AutosaveEvent)
// REMOVE: commands.entity(entity).despawn_recursive()
// REMOVE: *active_tab = ActiveStationTab::Cargo
// REMOVE: signal_log / requests_tab writes

// ADD:
if carrying_query.get(entity).is_ok() {
    docked_with_bottle_events.send(ShipDockedWithBottle { ship_entity: entity });
} else {
    docked_with_cargo_events.send(ShipDockedWithCargo {
        ship_entity: entity,
        ore_type: ship.ore_type,  // confirm field name before implementing
        amount: ship.cargo,
    });
}
```

`autopilot.rs` params to **remove**: `station_query` (mut), `signal_log`, `requests_tab`, `active_tab`, `queue` (mut — keep read-only if needed for other logic, confirm), `autosave_events`

`autopilot.rs` params to **keep**: `time`, `asteroid_query`, `berth_query`, `bottle_query`, `carrying_query`, `commands`, `transform`, new event writers

#### Remove UI tab forcing:
Delete `*active_tab = ActiveStationTab::Cargo` entirely. The UI should respond to game state, not be commanded by the transport system.

---

### Step 3 — Create `economy.rs`

**File:** `src/systems/game_loop/economy.rs` (new file)

This system listens for `ShipDockedWithCargo` and `ShipDockedWithBottle` and owns all economy consequences.

```rust
pub fn ship_docked_economy_system(
    mut cargo_events: EventReader<ShipDockedWithCargo>,
    mut bottle_events: EventReader<ShipDockedWithBottle>,
    mut fulfill_events: EventReader<FulfillRequestEvent>,
    mut repair_events: EventReader<RepairStationEvent>,
    mut station_query: Query<&mut Station>,
    mut queue: ResMut<ShipQueue>,
    mut autosave_events: EventWriter<AutosaveEvent>,
    mut commands: Commands,
) {
    for event in cargo_events.read() {
        // Apply cargo to correct reserve
        // Set station.dock_state = Resuming
        // Set station.resume_timer
        // queue.available_count += 1
        // commands.entity(event.ship_entity).despawn_recursive()
        // autosave_events.send(AutosaveEvent)
    }

    for event in bottle_events.read() {
        // queue.available_count += 1
        // commands.entity(event.ship_entity).despawn_recursive()
        // autosave_events.send(AutosaveEvent)
        // (narrative consequences handled by narrative_events.rs)
    }

    for event in fulfill_events.read() {
        // Deduct resources for the request
        // Apply upgrade multiplier to Station
        // Mark request fulfilled in RequestsTabState
    }

    for _event in repair_events.read() {
        // station.repair_progress = 1.0
        // station.online = true
    }
}
```

Confirm exact field names (`ship.ore_type`, `ship.cargo`) before implementing.

---

### Step 4 — Create `narrative_events.rs`

**File:** `src/systems/narrative/narrative_events.rs` (new file)

This system listens for `ShipDockedWithBottle` and `OpeningCompleteEvent` and owns all narrative consequences.

```rust
pub fn narrative_event_system(
    mut bottle_events: EventReader<ShipDockedWithBottle>,
    mut opening_events: EventReader<OpeningCompleteEvent>,
    mut signal_log: ResMut<SignalLog>,
    mut requests_tab: ResMut<RequestsTabState>,
    mut commands: Commands,
    carrying_query: Query<Entity, With<CarryingBottle>>,
) {
    for event in bottle_events.read() {
        // signal_log.entries.push_back(FIRST_CONTACT_TEXT)
        // signal_log trim if > 10
        // requests_tab.collected_requests.push(CollectedRequest::FirstLight)
        // commands.entity(event.ship_entity).remove::<CarryingBottle>()
    }

    for _event in opening_events.read() {
        // Any narrative setup that should happen when gameplay begins
        // queue.available_count += 1 moves here from opening_sequence.rs
    }
}
```

---

### Step 5 — Refactor `content.rs`

**File:** `src/systems/ui/hud/content.rs`

Replace direct mutations with event fires:

```rust
// REPAIR button — REMOVE:
// station.repair_progress = 1.0;
// station.online = true;
// REPLACE WITH:
repair_events.send(RepairStationEvent);

// FULFILL button — REMOVE:
// station.iron_ingots -= 25.0;
// station.power_multiplier += 0.25;
// req.fulfilled = true;
// REPLACE WITH:
fulfill_events.send(FulfillRequestEvent {
    request_id: RequestId::FirstLight,
    faction_id: FactionId::Signal,
});
```

`content.rs` params to add: `EventWriter<RepairStationEvent>`, `EventWriter<FulfillRequestEvent>`
`content.rs` params to remove: `Query<&mut Station>` (if only used for these mutations — confirm)

---

### Step 6 — Refactor `opening_sequence.rs`

**File:** `src/systems/narrative/opening_sequence.rs`

When the opening cinematic reaches its final phase and gameplay begins:

```rust
// REMOVE: queue.available_count += 1 (direct mutation)
// REMOVE: station.dock_state = ... (direct mutation)
// REPLACE WITH:
opening_complete_events.send(OpeningCompleteEvent);
```

The `narrative_events.rs` system handles `OpeningCompleteEvent` and applies the economy consequences.

---

### Step 7 — Register in `lib.rs`

Add to `lib.rs`:
```rust
.add_event::<ShipDockedWithCargo>()
.add_event::<ShipDockedWithBottle>()
.add_event::<FulfillRequestEvent>()
.add_event::<RepairStationEvent>()
.add_event::<OpeningCompleteEvent>()
```

Add new systems to appropriate system groups:
- `ship_docked_economy_system` — after `autopilot_system` in the visual chain (must run same frame as event fires)
- `narrative_event_system` — in the UI/Narrative group
- Add `economy.rs` and `narrative_events.rs` to their respective `mod.rs` files

**Critical ordering:** `autopilot_system` fires events → `ship_docked_economy_system` reads them. Both must be in the same frame. Verify chain ordering in `lib.rs` after registration.

---

## Verification Checklist

### Behavioral parity (must match Phase 2 exactly)
- [ ] Mine asteroid → cargo fills → ship returns → reserves update correctly
- [ ] Bottle appears → tap → drone collects → Signal Log entry appears → First Light card appears
- [ ] FULFILL button → iron ingots deduct → power_multiplier increases → COMPLETE state shows
- [ ] REPAIR button → station comes online
- [ ] Opening sequence completes → gameplay begins normally → queue initialized correctly
- [ ] Save/load cycle → all state restores correctly

### Structural verification
- [ ] `autopilot.rs` has no direct writes to `Station`, `SignalLog`, `RequestsTabState`, or `ActiveStationTab`
- [ ] `content.rs` has no direct writes to `Station`
- [ ] `opening_sequence.rs` has no direct writes to `ShipQueue` or `Station.dock_state`
- [ ] All five events defined and registered
- [ ] `economy.rs` and `narrative_events.rs` exist and registered in mod.rs
- [ ] `cargo check` clean, zero warnings

### Device verification
- [ ] Full play session — mine, collect bottle, fulfill request — no regressions
- [ ] Opening sequence plays correctly
- [ ] No warn! lines firing in logcat during normal operation
- [ ] Screenshots provided for bottle collection and request fulfillment

---

## File Touch Map

**Modified:**
- `src/systems/ship_control/autopilot.rs` — remove economy/narrative mutations, add event writers
- `src/systems/ui/hud/content.rs` — replace direct mutations with event fires
- `src/systems/narrative/opening_sequence.rs` — fire OpeningCompleteEvent instead of direct mutations
- `src/lib.rs` — register events and new systems
- `src/systems/game_loop/mod.rs` — add economy module
- `src/systems/narrative/mod.rs` — add narrative_events module
- `src/components/resources.rs` OR new `src/components/events.rs` — event definitions

**Created:**
- `src/systems/game_loop/economy.rs`
- `src/systems/narrative/narrative_events.rs`

---

## Out of Scope (Do Not Implement)

- LOGS tab or FORGE tab rename
- New bottle spawning logic
- Additional faction requests
- Any new gameplay features
- Save system format changes
- UI layout changes
- Scanning mechanic
- Crystal farming
- Any system not listed in the File Touch Map
