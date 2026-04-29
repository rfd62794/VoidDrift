# ADR-011: Event Bus & Single Responsibility Refactor (Phase 3b)

**Date:** April 28, 2026  
**Status:** Accepted  
**Tag:** v2.4.0-phase3b-event-bus

## Context

Prior to Phase 3b, `autopilot.rs` was a domain hub that mixed three distinct concerns:

1. **Transport** — ship movement, arrival detection, state transitions
2. **Economy** — cargo unloading into station reserves, dock state resets, queue increments, autosave triggers, entity despawn
3. **Narrative** — signal log writes, request card insertion, `CarryingBottle` component removal

Additionally, `content.rs` (a UI render function) directly mutated `Station` fields on button click — no event, no validation, no separation from the render pass. And `opening_sequence.rs` directly wrote to `ShipQueue` and `Station.dock_state` at cinematic completion.

This violated SRP and created two concrete problems:
- Any system that needed to react to a ship docking had to be added inside `autopilot.rs`
- UI render code owned game state transitions, making future button feedback (audio, animation) impossible without coupling those systems back into egui closures

## Decision

Introduce a five-event bus as the contract between transport, economy, and narrative:

| Event | Fired by | Consumed by |
|---|---|---|
| `ShipDockedWithCargo` | `autopilot_system` | `ship_docked_economy_system` |
| `ShipDockedWithBottle` | `autopilot_system` | `ship_docked_economy_system`, `narrative_event_system` |
| `FulfillRequestEvent` | `render_tab_content` (UI) | `ship_docked_economy_system` |
| `RepairStationEvent` | `render_tab_content` (UI) | `ship_docked_economy_system` |
| `OpeningCompleteEvent` | `opening_sequence_system` | `narrative_event_system` |

**New files:**
- `src/components/events.rs` — event type definitions
- `src/systems/game_loop/economy.rs` — owns all economy consequences of ship docking, fulfillment, repair
- `src/systems/narrative/narrative_events.rs` — owns signal log writes, request card push, `OpeningCompleteEvent` handling

**Ownership after refactor:**
- `autopilot.rs` — movement and arrival detection only; no `Station` writes, no `SignalLog` writes, no `ActiveStationTab` forcing
- `economy.rs` — all `Station` reserve mutations, `dock_state` resets, queue increments, entity despawn, autosave
- `narrative_events.rs` — all `SignalLog` entries, `RequestsTabState` mutations, `CarryingBottle` removal, opening queue init
- `content.rs` — reads station state for display; fires events on button click only

## System Ordering

`autopilot_system` fires events → `ship_docked_economy_system` and `narrative_event_system` read them in the same frame. All three are registered in the same `.chain()` group in `lib.rs`:

```
autopilot_system
  → ship_docked_economy_system
  → narrative_event_system
  → camera_follow_system
  → ...
```

Bevy's event system guarantees events fired in frame N are readable in the same frame by systems that run after the writer in the same schedule. The chain enforces this ordering.

## Known Constraint: One-Frame Deferral on UI Events

`FulfillRequestEvent` and `RepairStationEvent` are fired inside the egui `Update` render pass (via `hud_ui_system`), but `ship_docked_economy_system` runs in a separate `Update` system group that executes **before** `hud_ui_system` in the current schedule.

**Consequence:** When a player clicks FULFILL or REPAIR, the event is fired this frame but the economy system does not read it until the **next frame**. The state change (ingot deduction, `station.online = true`) is applied one frame after the click.

**Why this is not currently observable:** The game renders at 60fps+ and there is no frame-synchronous feedback (no audio, no particle effect, no button state change) tied to the moment of state application. The UI reflects the new state on the very next render pass, which is imperceptible.

**Why this will matter:** The moment any of the following are added, this deferral becomes a visible artifact:
- Button click sound effect (plays one frame after click)
- Button press animation (springs one frame late)
- Immediate ingot count update on the fulfill button (count shows stale value for one frame)

**Resolution path when it matters:** Move `ship_docked_economy_system` to run after `hud_ui_system` within the same frame, or use `apply_deferred` to flush the event queue mid-frame. Do not work around it by moving economy logic back into the render closure.

## Alternatives Considered

### Keep economy logic in autopilot.rs
- **Rejected:** Prevents any other system from reacting to ship docking without modifying autopilot.rs. Creates a maintenance trap.

### Move economy logic into a Bevy observer (trigger/observer pattern)
- **Deferred:** Bevy 0.15 observers are available but add complexity. Event readers are sufficient for the current scale and are consistent with the rest of the codebase.

### Direct function calls from autopilot to economy functions
- **Rejected:** Eliminates the decoupling benefit. Economy system would still need to be imported and called from within autopilot's system parameter context.

## Consequences

### Positive
- `autopilot.rs` is now a single-concern system — transport only
- New reactions to ship docking (e.g. audio, achievement triggers) are added in `economy.rs` or `narrative_events.rs` without touching autopilot
- UI mutations no longer occur inside egui render closures
- `opening_sequence.rs` no longer has a direct dependency on `ShipQueue`

### Trade-offs
- `FulfillRequestEvent` / `RepairStationEvent` have a one-frame deferral (documented above)
- Five new types in the event namespace — small increase in compile surface

---

**Related files:**
- `src/components/events.rs`
- `src/systems/game_loop/economy.rs`
- `src/systems/narrative/narrative_events.rs`
- `src/systems/ship_control/autopilot.rs`
- `src/systems/ui/hud/content.rs`
- `src/systems/narrative/opening_sequence.rs`
- `src/lib.rs`
