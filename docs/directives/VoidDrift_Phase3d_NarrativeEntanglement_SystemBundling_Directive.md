# VoidDrift — Phase 3d: Narrative Entanglement & System Bundling
**Directive Version:** 1.0  
**Date:** April 28, 2026  
**Branch:** `dev`  
**Prerequisite:** Phase 3c complete and verified on device

---

## AGENT CONTRACT

You are resolving five remaining SRP violations identified in the post-Phase-3b audit. This directive covers narrative writes in physics systems, quest state entangled with signal firing, dead UI code, and oversized initialization systems.

**You are NOT allowed to:**
- Change any game balancing values or constants
- Add new gameplay features or mechanics
- Touch any file not listed in the File Touch Map
- Change behavior — only ownership and organization of that behavior moves

**You ARE responsible for:**
- Moving the signal log write out of `mining.rs`
- Separating quest state mutation from signal firing in `signal.rs`
- Removing dead production queue buttons from `station_tabs.rs`
- Splitting `setup_world` resource reset into a separate system
- Splitting `ingame_startup_system` into load and new-game paths

**Definition of Done:**
- `mining.rs` contains no writes to `SignalLog` or any narrative resource
- `signal.rs` fires signals — quest state updates happen in a separate system
- Dead queue buttons removed from `station_tabs.rs`
- `setup_world` handles entity spawning only — resource reset is a separate system
- `ingame_startup_system` is split into focused, readable systems
- `cargo check` clean, zero warnings
- Behavioral parity verified on device

---

## Full Context: What Is Currently Wrong

### Finding #5 — `mining.rs` writes to SignalLog
**File:** `src/systems/game_loop/mining.rs:38`

```rust
signal_log.entries.push_front("> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.".to_string());
```

`mining_system` owns ore extraction physics. Writing to `SignalLog` is a narrative concern. `narrative_events.rs` is the designated owner of signal log writes from world events. This write has no event — it fires inline inside the physics loop.

### Finding #6 — `signal.rs` owns quest state mutation
**File:** `src/systems/narrative/signal.rs:19–78`

`signal_system` directly mutates `quest_log.objectives` — setting `ObjectiveState::Active` and `ObjectiveState::Complete` based on signal IDs. Signal firing and quest progression are two concerns in one system with no separation point. Adding quest logic requires touching the signal system and vice versa.

### Finding #7 — `station_tabs.rs` writes narrative resource via transport utility
**File:** `src/systems/ui/station_tabs.rs:5–11`

```rust
pub fn add_log_entry(station: &mut Station, entry: String) {
    if station.log.back() == Some(&entry) { return; }
    station.log.push_back(entry);
}
```

`station.log` is a narrative/audit log embedded in the Station component. This utility is called from `autonomous.rs` (a transport system). After Phase 3c, `autonomous.rs` will no longer call this — but the function and `station.log` still represent a conceptual mismatch. `station.log` should be unified with `SignalLog` or explicitly separated.

**Decision required before implementing:** Should `station.log` entries be routed through `SignalLog` (unified) or remain a separate station-internal log? Flag this to the user before proceeding. For now, if `autonomous.rs` no longer calls `add_log_entry` after Phase 3c, the immediate violation is resolved — document the remaining conceptual debt in an ADR note.

### Finding #8 — Dead production queue buttons in `station_tabs.rs`
**File:** `src/systems/ui/station_tabs.rs:83–85`

```rust
if ui.add_enabled(max_possible >= 1, egui::Button::new("+1").min_size(btn_size)).clicked() { 
    /* crate::systems::economy::queue_job(...) */ 
}
```

Buttons render but do nothing. The commented code references `crate::systems::economy::queue_job` which does not exist. Dead UI with no backing logic — misleading to any reader of the codebase.

### Finding #11 — `world_spawn.rs` bundles setup and reset
**File:** `src/systems/setup/world_spawn.rs:38–72`

`setup_world` handles both:
1. Resource reset (`*queue = ShipQueue::default()`, `*signal_log = SignalLog::default()`, etc.)
2. World entity spawning (starfield, camera, station, berths, etc.)

A `reset_game_resources` function exists but is unused — `setup_world` does both jobs. These are separate concerns and should be separate systems in the `OnEnter(InGame)` chain.

### Finding #12 — `ingame_startup_system` is a 150-line mega-system
**File:** `src/scenes/main_menu.rs:256–413`

One system handles:
- Save load path (restore all state from disk)
- New game path (initialize fresh state)
- Opening phase restore
- Queue restore
- Station state restore
- Tab restore
- Request restore
- Signal log restore
- Drone entity spawning (full child construction)

Load path and new-game path are a branch decision inside one 150-line function. Entity spawning inside a state-restore system is a separate concern.

---

## Implementation Plan

### Fix 5 — Move signal log write out of `mining.rs`

**File:** `src/systems/game_loop/mining.rs`

**Define a new event:**
```rust
pub struct InsufficientLaserEvent {
    pub ship_entity: Entity,
}
```

Add to `src/components/events.rs`.

**In `mining.rs`:** Replace the inline signal log write with:
```rust
insufficient_laser_events.send(InsufficientLaserEvent { ship_entity: entity });
```

**In `narrative_events.rs`:** Add handler:
```rust
for event in insufficient_laser_events.read() {
    signal_log.entries.push_front(
        "> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.".to_string()
    );
}
```

Register `InsufficientLaserEvent` in `lib.rs`.

---

### Fix 6 — Separate quest state from signal firing

**File:** `src/systems/narrative/signal.rs`

**Goal:** `signal_system` fires signals and records which signals have fired. Quest state updates happen in a new dedicated system that reacts to signals.

**Step 1:** Define a `SignalFired` event:
```rust
pub struct SignalFired {
    pub signal_id: String, // or SignalId enum if one exists — confirm
}
```

Add to `src/components/events.rs`.

**Step 2:** In `signal.rs`, when a signal condition is met:
- Continue recording to `signal_fired_ids` (existing behavior)
- Fire `SignalFired { signal_id }` event
- Remove direct `quest_log.objectives` mutations from this system

**Step 3:** Create `src/systems/narrative/quest.rs` (or add to existing `quest.rs` if it exists — confirm):
```rust
pub fn quest_update_system(
    mut signal_events: EventReader<SignalFired>,
    mut quest_log: ResMut<QuestLog>,
) {
    for event in signal_events.read() {
        // Match signal_id to quest objective updates
        // ObjectiveState::Active, ObjectiveState::Complete
        // Exact logic moved verbatim from signal.rs
    }
}
```

Register `SignalFired` in `lib.rs`. Register `quest_update_system` in narrative group after `signal_system`.

---

### Fix 7 — `station.log` / `add_log_entry` cleanup

**File:** `src/systems/ui/station_tabs.rs`

After Phase 3c, `autonomous.rs` no longer calls `add_log_entry`. 

**If `add_log_entry` has no remaining callers:** Remove the function entirely. If `station.log` has no remaining writers, document in an ADR note that `station.log` is conceptual debt — either unify with `SignalLog` in a future phase or remove the field from `Station`.

**If callers remain:** List them and flag for review. Do not remove without confirming all callers are accounted for.

---

### Fix 8 — Remove dead queue buttons from `station_tabs.rs`

**File:** `src/systems/ui/station_tabs.rs:83–85`

Remove the `+1`, `+10`, and `MAX` button UI blocks entirely. They render but do nothing — `crate::systems::economy::queue_job` does not exist.

Do not implement replacement logic. If production queue management is desired in a future phase, it will be specced separately. For now, remove the dead code cleanly.

Verify the FORGE tab (formerly PRODUCTION) still renders correctly after removal.

---

### Fix 11 — Split `setup_world` into two systems

**File:** `src/systems/setup/world_spawn.rs`

**Step 1:** Extract resource reset into its own system:
```rust
pub fn reset_game_resources(
    mut queue: ResMut<ShipQueue>,
    mut signal_log: ResMut<SignalLog>,
    mut camera_delta: ResMut<CameraDelta>,
    // ... all other resources currently reset in setup_world
) {
    *queue = ShipQueue::default();
    *signal_log = SignalLog::default();
    // etc — move verbatim from setup_world
}
```

**Step 2:** `setup_world` handles entity spawning only — remove all resource reset code.

**Step 3:** In `lib.rs`, update the `OnEnter(InGame)` chain:
```rust
.add_systems(OnEnter(AppState::InGame), 
    (
        cleanup_world_entities,
        reset_game_resources,
        setup_world,
        spawn_initial_asteroids,
        // ...
    ).chain()
)
```

Order matters: cleanup → reset → spawn. Confirm this matches current chain ordering before modifying.

---

### Fix 12 — Split `ingame_startup_system`

**File:** `src/scenes/main_menu.rs`

Split the 150-line system into focused systems:

**`detect_save_and_branch`** — reads save file, sets a local resource/flag indicating load vs new game. Does not spawn entities or mutate game state.

**`restore_save_state`** — runs if save exists: restores station, queue, tabs, requests, signal log. No entity spawning.

**`spawn_initial_drone`** — handles drone entity construction (the ship spawned during load restore). Calls `spawn_drone_ship` from Phase 3c's shared function.

**`initialize_new_game`** — runs if no save: sets initial state. Minimal — most initialization already handled by `reset_game_resources` and `setup_world`.

These can be registered as a chain within `OnEnter(InGame)` after `setup_world`, or as a single system with clearly separated internal functions if the chain ordering is complex. Flag for review if Bevy's run condition system makes the branch difficult to express cleanly.

---

## Verification Checklist

### Behavioral parity
- [ ] Mining an asteroid without sufficient laser rating → signal log message appears
- [ ] Signal fires → quest objectives update correctly
- [ ] New game starts cleanly — all resources initialized
- [ ] Load game restores all state — station, queue, requests, signal log, power_multiplier
- [ ] Drone spawns correctly on load
- [ ] FORGE tab renders correctly with queue buttons removed
- [ ] `cargo check` clean, zero warnings

### Structural verification
- [ ] `mining.rs` has no writes to `SignalLog` or narrative resources
- [ ] `signal.rs` has no direct writes to `quest_log.objectives`
- [ ] `quest_update_system` exists and handles all objective mutations
- [ ] `add_log_entry` removed if no callers remain
- [ ] Dead queue buttons removed from `station_tabs.rs`
- [ ] `setup_world` contains no resource reset code
- [ ] `reset_game_resources` exists as separate system
- [ ] `ingame_startup_system` split into focused systems
- [ ] All new events registered in `lib.rs`

### Device verification
- [ ] Full play session — new game, mine, collect bottle, fulfill request, save, reload
- [ ] Signal log messages appear correctly
- [ ] Quest objectives update correctly
- [ ] No warn! lines firing during normal operation

---

## File Touch Map

**Modified:**
- `src/systems/game_loop/mining.rs` — remove SignalLog write, fire InsufficientLaserEvent
- `src/systems/narrative/signal.rs` — remove quest_log mutations, fire SignalFired event
- `src/systems/ui/station_tabs.rs` — remove dead queue buttons, remove add_log_entry if no callers
- `src/systems/setup/world_spawn.rs` — extract resource reset into separate system
- `src/scenes/main_menu.rs` — split ingame_startup_system
- `src/components/events.rs` — add InsufficientLaserEvent, SignalFired
- `src/lib.rs` — register new events and systems

**Created:**
- `src/systems/narrative/quest.rs` (if not already exists — confirm)

---

## Out of Scope (Do Not Implement)

- hud/mod.rs camera/save concerns (#9) — deferred
- WorldViewRect render side-effect (#10) — documented in ADR, leave
- Aluminum mesh (#13) — visual, needs art decision
- constants.rs reorganization (#14) — cosmetic pass
- Station component split (#15) — touches everything, needs dedicated planning
- Any new gameplay features
- LOGS tab or FORGE tab implementation
- Any save format changes
