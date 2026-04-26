# Voidrift — Directive: Processing Queue System & Auto-Dock
**Status:** Approved — Ready for Execution  
**Type:** Gameplay System  
**Date:** April 2026  
**Depends On:** Quest Tracker Directive IN PROGRESS ✅

---

## 1. Objective

Replace the instant batch processing system with a time-based queue system. Each operation type has its own independent queue and timer. Materials are consumed on queue. Results are deposited when processing completes.

Additionally implement auto-unload and auto-smelt toggles that fire on docking.

---

## 2. Design Principles

**The station works while you fly.** Queue operations before departing, return to collect results. The idle loop is the game.

**Four parallel operation queues, all independent:**
- Magnetite Refinery — Magnetite → Power Cells
- Carbon Refinery — Carbon → Hull Plates  
- Hull Forge — Hull Plates → Ship Hull
- Core Fabricator — Power Cells → AI Core

**Materials consumed immediately on queue.** Not on completion. The station has accepted the job.

**Auto-dock features reduce repetitive tapping.** Toggles let the player choose their automation level.

---

## 3. Time Costs — Locked

All tunable later. Do not hardcode inline — all in `constants.rs`.

```rust
pub const REFINERY_MAGNETITE_TIME: f32 = 20.0;  // seconds per batch
pub const REFINERY_CARBON_TIME: f32    = 30.0;  // seconds per batch
pub const FORGE_HULL_TIME: f32         = 45.0;  // seconds per batch
pub const FORGE_CORE_TIME: f32         = 60.0;  // seconds per batch
```

---

## 4. Data Structures

### 4.1 ProcessingQueue

Add to `components.rs`:

```rust
#[derive(Clone)]
pub struct ProcessingJob {
    pub operation: ProcessingOperation,
    pub batches: u32,           // number of batches queued
    pub timer: f32,             // seconds remaining on current batch
    pub completed: u32,         // batches completed so far
}

#[derive(Clone, PartialEq)]
pub enum ProcessingOperation {
    MagnetiteRefinery,   // Magnetite → Power Cells
    CarbonRefinery,      // Carbon → Hull Plates
    HullForge,           // Hull Plates → Ship Hull
    CoreFabricator,      // Power Cells → AI Core
}

#[derive(Component)]
pub struct StationQueues {
    pub magnetite_refinery: Option<ProcessingJob>,
    pub carbon_refinery:    Option<ProcessingJob>,
    pub hull_forge:         Option<ProcessingJob>,
    pub core_fabricator:    Option<ProcessingJob>,
}
```

Add `StationQueues` to the Station entity on spawn. Initialize all queues as `None`.

### 4.2 AutoDockSettings

```rust
#[derive(Resource)]
pub struct AutoDockSettings {
    pub auto_unload: bool,           // default: true
    pub auto_smelt_magnetite: bool,  // default: false
    pub auto_smelt_carbon: bool,     // default: false
}
```

Register as a resource. Initialize with defaults above.

---

## 5. Queue Logic

### 5.1 Queuing an Operation

When player taps a queue button (ADD 1, ADD 10, ADD MAX):

```
fn queue_operation(operation, batches, station, queues) {
    // 1. Calculate resource cost for requested batches
    // 2. Check station has sufficient resources
    // 3. Consume resources immediately
    // 4. Add/extend the queue for that operation type
    // 5. If queue was None, start timer at full duration
    // 6. Log: "> [OPERATION] QUEUED. [N] BATCHES."
}
```

If a queue already has a job running, new batches are added to `batches` — they extend the existing job, they don't create a second one.

### 5.2 Processing System

New system: `processing_queue_system` in `systems/economy.rs`.

Runs every tick for each active queue:

```
fn processing_queue_system(time, mut station_query) {
    for each active queue (magnetite_refinery, carbon_refinery, etc.) {
        if job.batches > 0 {
            job.timer -= delta;
            if job.timer <= 0.0 {
                // Batch complete
                deposit_output(operation, station);
                job.completed += 1;
                job.batches -= 1;
                if job.batches > 0 {
                    // Reset timer for next batch
                    job.timer = operation_time(operation);
                } else {
                    // Queue empty
                    queue = None;
                    log("> [OPERATION] COMPLETE.");
                }
            }
        }
    }
}
```

### 5.3 Output Deposit

On batch completion, deposit output to station reserves:

| Operation | Output per batch |
|-----------|-----------------|
| MagnetiteRefinery | +`REFINERY_RATIO` (10) Power Cells |
| CarbonRefinery | +`HULL_PLATE_RATIO` (5) Hull Plates |
| HullForge | +1 Ship Hull |
| CoreFabricator | +1 AI Core |

### 5.4 Resource Costs

On queue (immediate deduction):

| Operation | Cost per batch |
|-----------|---------------|
| MagnetiteRefinery | `REFINERY_RATIO` (10) Magnetite |
| CarbonRefinery | `HULL_PLATE_COST_CARBON` (5) Carbon |
| HullForge | `SHIP_HULL_COST_PLATES` (3) Hull Plates |
| CoreFabricator | `AI_CORE_COST_CELLS` (55) Power Cells |

---

## 6. Auto-Dock System

New system: `auto_dock_system` in `systems/autopilot.rs` or `economy.rs`.

Triggers exactly once on `ShipState` transition to `Docked`. Not every tick.

```
fn auto_dock_system(
    mut ship_query,
    mut station_query,
    settings: Res<AutoDockSettings>,
) {
    // On transition to Docked:
    
    // AUTO UNLOAD (always if enabled)
    if settings.auto_unload {
        transfer ship.cargo to station reserves;
        log("> CARGO UNLOADED AUTOMATICALLY.");
    }
    
    // AUTO SMELT MAGNETITE
    if settings.auto_smelt_magnetite {
        let batches = station.magnetite_reserves / REFINERY_RATIO;
        if batches > 0 {
            queue_operation(MagnetiteRefinery, batches);
            log("> MAGNETITE QUEUED FOR REFINING.");
        }
    }
    
    // AUTO SMELT CARBON
    if settings.auto_smelt_carbon {
        let batches = station.carbon_reserves / HULL_PLATE_COST_CARBON;
        if batches > 0 {
            queue_operation(CarbonRefinery, batches);
            log("> CARBON QUEUED FOR REFINING.");
        }
    }
}
```

The auto-dock system must run AFTER the docking state transition is set. Use `.after(autopilot_system)` ordering.

---

## 7. UI — Processing Tab Layout

Both SMELTER and FORGE tabs share the same visual layout pattern. Each shows all operation types available on that tab, with queue controls and status.

### 7.1 Operation Card

Each operation type renders as a card:

```
┌─────────────────────────────┐
│ MAGNETITE → POWER CELLS     │
│ Stock: 240 Mag  Cost: 10/batch │
│ [▶ PROCESSING... 14s] ████░ │  ← if active
│ Queued: 3 batches remaining  │
│ [+1]  [+10]  [MAX]  [CLEAR] │
└─────────────────────────────┘
```

**Progress bar:** Shows time remaining on current batch. Fills left to right. Cyan color.

**Queue count:** Shows batches remaining including current.

**Buttons:**
- `[+1]` — queue 1 batch if resources available
- `[+10]` — queue up to 10 batches
- `[MAX]` — queue maximum possible batches from current reserves
- `[CLEAR]` — cancel queued batches (not current processing). Resources NOT refunded — materials already consumed.

> ⚠️ CLEAR does not refund resources. This is intentional. The station accepted the job. Add a small warning note in the UI: "Queued batches cannot be refunded."

**Greyed state:** If insufficient resources to queue even 1 batch, all queue buttons are greyed. Show reason: "Need 10 Magnetite" etc.

### 7.2 SMELTER Tab Contents

- Magnetite Refinery card
- Carbon Refinery card

### 7.3 FORGE Tab Contents

- Hull Forge card
- Core Fabricator card

### 7.4 Auto-Dock Settings

Add to RESERVES tab, below resource counts:

```
AUTO-DOCK SETTINGS
[✓] Auto-Unload Cargo
[ ] Auto-Smelt Magnetite  
[ ] Auto-Smelt Carbon
```

Checkboxes. Toggle immediately updates `AutoDockSettings` resource. Persist for session (no save system yet).

---

## 8. Signal Integration

Add to `narrative.rs` signal triggers:

| Trigger | Signal Line |
|---------|-------------|
| Any operation queued | `> [OPERATION NAME] QUEUED. [N] BATCHES.` |
| Batch completes | `> [OPERATION NAME] BATCH COMPLETE.` |
| Queue empty | `> [OPERATION NAME] PROCESSING COMPLETE.` |
| Auto-unload fires | `> CARGO UNLOADED AUTOMATICALLY.` |
| Auto-smelt queues | `> MAGNETITE QUEUED FOR REFINING.` |

Signal lines for processing use the repeating pattern (not one-time triggers). Use state-change approach — log on queue start and on queue empty, not every batch completion.

---

## 9. File Scope

| File | Change |
|------|--------|
| `src/constants.rs` | Add 4 processing time constants |
| `src/components.rs` | Add ProcessingJob, ProcessingOperation, StationQueues |
| `src/lib.rs` | Register AutoDockSettings resource, register new systems |
| `src/systems/setup.rs` | Add StationQueues to Station spawn, initialize AutoDockSettings |
| `src/systems/economy.rs` | Add processing_queue_system |
| `src/systems/autopilot.rs` | Add auto_dock_system, trigger on docking transition |
| `src/systems/ui.rs` | Replace instant batch UI with queue card UI, auto-dock toggles in RESERVES tab |
| `src/systems/narrative.rs` | Add processing signal lines |
| `Cargo.toml` | READ-ONLY |

---

## 10. Implementation Sequence

1. Add constants, data structures, `StationQueues` spawn — verify compile
2. Add `processing_queue_system` — no UI yet, verify in logcat that timers tick
3. Add queue UI cards to SMELTER and FORGE tabs — deploy, verify queue buttons work
4. Add `auto_dock_system` — deploy, verify auto-unload fires on dock
5. Add auto-smelt toggles to RESERVES tab — deploy, verify toggles work
6. Add Signal integration for queue events — deploy, verify Signal lines appear

---

## 11. Completion Criteria

- [ ] Materials consumed immediately on queue tap
- [ ] Processing timer visible and counting down in UI
- [ ] Batch completes and deposits output after correct time
- [ ] Multiple queued batches process in sequence automatically
- [ ] CLEAR cancels queued batches without refund
- [ ] Auto-unload fires on dock when enabled
- [ ] Auto-smelt queues correct operation when enabled
- [ ] Auto-dock toggles in RESERVES tab functional
- [ ] Signal lines fire for queue and completion events
- [ ] No B0001 crashes — all new queries follow Universal Disjointness pattern
- [ ] Gate: queue Magnetite refinery, depart, return — Power Cells produced on arrival

---

## 12. Balance Note

Time costs are intentionally generous for mobile sessions. One mining run (~10 seconds) completes while 2 Magnetite batches process. The player always has something to do. Constants are named and adjustable without code changes.

---

*Voidrift Processing Queue Directive | April 2026 | RFD IT Services Ltd.*  
*The station works while you fly. Queue it. Go mine. Come back to results.*
