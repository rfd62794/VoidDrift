# Voidrift — Phase 6 Directive: AI Core & Autonomous Drone
**Status:** Approved — Ready for Execution  
**Gate Phase:** 6  
**Date:** April 2026  
**Depends On:** Phase 5 Gate PASSED ✅ — Slice Complete

---

## 1. Objective

Implement the first post-slice feature: the AI Core module. The player builds an AI Core at the station using accumulated power cells. Building it spawns one autonomous drone that mines the asteroid field and returns to the station on a fixed loop — without player input.

This is the emotional shift the game is built around. The player stops being the ship and becomes the commander watching their operation run.

---

## 2. Design Intent

The AI Core is not a quality-of-life feature. It is the moment the game changes identity.

Before AI Core: you are a miner.  
After AI Core: you are building something.

The drone should feel alive — visibly moving, clearly working, returning with cargo. The player's job shifts to watching and planning the next upgrade while the drone handles the baseline.

---

## 3. Scope Boundaries

> ⚠️ HARD LIMIT: Phase 6 is AI Core build action and one autonomous drone on one fixed route only.

**In scope:**
- AI Core build button in station docking UI — costs 50 power cells
- Build button disabled when `power_cells < 50`, greyed with cost shown
- Build button hidden once AI Core is built — one only
- Drone entity spawned on build — visually distinct from player ship
- Drone autopilot: mine field → return to station → unload → repeat indefinitely
- Drone cargo bar visible above drone (same pattern as player ship)
- Drone unloads automatically on station arrival — no player input required
- Logcat confirms drone cycle: depart, arrive, mine complete, return, unload

**Explicitly out of scope — do not implement:**
- Second drone or fleet management
- Drone task assignment UI
- Second ore type or resource chain expansion
- Module slot UI or upgrade screen
- Save/load persistence
- Risk or degradation systems
- Any new sectors or destinations
- Player ship changes of any kind

---

## 4. Economy Constants — All Unchanged

Do not modify any existing constant. Add one:

| Constant | Value | Notes |
|----------|-------|-------|
| `AI_CORE_COST` | 50 | Power cells to build AI Core |
| `REPAIR_COST` | 25 | Existing — do not change |
| `REFINERY_RATIO` | 10 | Existing — do not change |
| `CARGO_CAPACITY` | 100 | Existing — do not change |
| `MINING_RATE` | 8.0 | Existing — do not change |
| `SHIP_SPEED` | 120.0 | Existing — do not change |
| `EGUI_SCALE` | 3.0 | Existing — do not change |

The drone uses the same `CARGO_CAPACITY`, `MINING_RATE`, and `SHIP_SPEED` as the player ship. No separate drone constants — they share the same physical rules.

---

## 5. Technical Specification

### 5.1 New Component: Drone

```rust
#[derive(Component)]
struct Drone {
    state: DroneState,
    cargo: f32,
}

#[derive(PartialEq)]
enum DroneState {
    Mining,
    Returning,
    Unloading,
}
```

The drone does not have an `Idle` state — it loops indefinitely once spawned. It begins in `Mining` state at spawn, positioned at the asteroid field.

### 5.2 New Component: AiCore

Marker component on the Station entity once built:

```rust
#[derive(Component)]
struct AiCore;
```

Check for presence of `AiCore` on the Station to determine whether build button should be shown.

### 5.3 DroneSystem

Runs every tick. Manages the full drone loop:

```
DroneState::Mining
  → Accumulate cargo at MINING_RATE * delta_seconds()
  → On cargo full: set DroneState::Returning, set autopilot target to station
  → Log: "[Voidrift Phase 6] Drone cargo full. Returning to station."

DroneState::Returning
  → Move toward station using same AutopilotSystem logic as player ship
  → On arrival: set DroneState::Unloading

DroneState::Unloading
  → Add drone.cargo to station ore reserves (same pool player refines from)
  → Reset drone.cargo to 0.0
  → Set DroneState::Mining, set position to asteroid field
  → Log: "[Voidrift Phase 6] Drone unloaded {X} ore. Returning to field."
```

**Important:** The drone does not use `AutopilotTarget`. It manages its own movement internally — the destination alternates between the asteroid field and the station on a fixed cycle. No player input ever affects the drone route.

### 5.4 Drone Spawning

When player taps Build AI Core button:
- Deduct 50 power cells from ship
- Add `AiCore` component to Station entity
- Spawn Drone entity at asteroid field position `(150.0, 100.0)`
- Spawn drone cargo bar as child of Drone entity (same pattern as player cargo bar)
- Drone color: `Color::srgb(1.0, 0.5, 0.0)` — orange, distinct from player cyan
- Log: `[Voidrift Phase 6] AI Core built. Drone deployed.`

### 5.5 Drone Cargo Bar

Same implementation pattern as player cargo bar:
- Two Mesh2d rectangles — background and fill — parented to Drone entity
- Offset: `(0.0, 24.0)` above drone
- Fill scales with `drone.cargo / CARGO_CAPACITY`
- Updates every tick

### 5.6 Station UI Extension

Extend `hud_ui_system` to include the AI Core build button in the docking panel:

| Element | Condition | Behaviour |
|---------|-----------|-----------|
| BUILD AI CORE (disabled) | `power_cells < 50` | Greyed, shows "AI CORE (50 cells)" |
| BUILD AI CORE (active) | `power_cells >= 50` and no AiCore on station | Cyan, tappable |
| (hidden) | AiCore already built | Button not shown — one AI Core only |

Display order in docking panel: REFINE button, then REPAIR button (if not repaired), then BUILD AI CORE button.

### 5.7 Ore Pool

The drone unloads ore into the same ore pool the player uses. This means:
- Drone returns ore → player can refine it without mining themselves
- The player's manual mining loop becomes optional once the drone is running
- This is intentional — the drone is the reward for the loop, not a parallel system

Implement as a new field on the Station component:

```rust
#[derive(Component)]
struct Station {
    repair_progress: f32,  // existing
    online: bool,          // existing
    ore_reserves: f32,     // NEW — shared pool for player + drone
}
```

When player docks: transfer ship cargo to `station.ore_reserves`, reset ship cargo.  
When player refines: consume from `station.ore_reserves`.  
When drone unloads: add to `station.ore_reserves`.

Update the docking UI to show `station.ore_reserves` as the ore count, not `ship.cargo`.

> ⚠️ This is a breaking change to the existing refinery flow. The player's cargo is now separate from the refinery input pool. Verify refinery still works correctly after this change before implementing drone logic.

---

## 6. File Scope

Only these files may be modified in Phase 6:

| File | Change |
|------|--------|
| `src/lib.rs` | Drone component, DroneState, DroneSystem, AiCore marker, drone spawn logic, Station ore_reserves field, docking UI build button, ore pool refactor |
| `Cargo.toml` | Only if a new dependency is required — justify before adding |
| `docs/state/current.md` | Update on phase completion |

**All other files are read-only for this phase.**

---

## 7. Pre-Implementation Research Required

Before writing any code, answer these two questions:

**Q1:** Does the existing `AutopilotSystem` query for all entities with `AutopilotTarget`, or is it scoped to the player ship specifically? The answer determines whether the drone can share the system or needs its own.

**Q2:** What is the current data flow for ore from ship cargo to refinery? Specifically — does the refinery read from `ship.cargo` directly, or from a separate storage component? This determines the scope of the ore pool refactor.

Report answers before implementation begins.

---

## 8. Test Anchors

All 8 must be verified before gate submission:

| ID | Behaviour | How to Verify |
|----|-----------|--------------|
| TB-P6-01 | Build button visible when docked with 50+ cells | Visual: button present and cyan |
| TB-P6-02 | Build button greyed below 50 cells | Visual: disabled state with cost shown |
| TB-P6-03 | Build deducts exactly 50 power cells | Logcat: cell count decreases by 50 |
| TB-P6-04 | Drone spawns at asteroid field in orange | Visual: orange rectangle at field position |
| TB-P6-05 | Drone mines and cargo bar fills | Visual: drone cargo bar grows |
| TB-P6-06 | Drone returns to station autonomously | Visual + logcat: "Drone cargo full. Returning." |
| TB-P6-07 | Drone unloads and ore reserves increase | Logcat: "Drone unloaded X ore." |
| TB-P6-08 | Drone resumes mining after unload — loop confirmed | Logcat: second mining cycle begins |

---

## 9. Gate 6 Completion Criteria

All of the following must be true before Phase 6 is marked complete:

- [ ] App launches on Moto G 2025 without crash
- [ ] Build AI Core button functional in docking UI
- [ ] Drone spawns visually distinct from player ship
- [ ] Drone completes at least two full cycles (mine → return → unload → mine) confirmed in logcat
- [ ] Player can still manually mine and refine while drone operates
- [ ] Ore reserves pool works correctly — drone and player both contribute
- [ ] Build button hidden after AI Core built — cannot build twice
- [ ] No screen flicker or buffer starvation (PresentMode::Fifo maintained)
- [ ] All 8 test anchors TB-P6-01 through TB-P6-08 verified
- [ ] Gate screenshot (TB-P6-GATE) shows player ship and drone both visible on device simultaneously

**Evidence required:**
1. Terminal output from `.\build_android.ps1`
2. Gate screenshot — both ships visible on device, drone cargo bar filling
3. Logcat showing at least two complete drone cycles

---

## 10. Known Risks

| Risk | Mitigation |
|------|-----------|
| Ore pool refactor breaks existing refinery | Test refinery before implementing drone. Explicit sequencing in §5.7. |
| Drone and player ship movement systems conflicting | Answer Q1 before coding. Drone movement may need to be fully independent of AutopilotSystem. |
| Two moving entities causing Mali GPU buffer starvation | Monitor logcat for `Can't acquire next buffer` after first drone deploy. PresentMode::Fifo should hold. |
| Drone cargo bar z-fighting with drone entity | Use drone z + 1 for cargo bar, same pattern as player. |
| Drone position reset on unload feels jarring | Instant teleport to field is acceptable for Phase 6. Smooth transit deferred. |

---

## 11. ADR Note

If the drone requires its own movement system separate from AutopilotSystem, document this as ADR-005 before implementing. The decision — shared system vs. dedicated drone system — has architectural consequences for the fleet management layer in future phases.

---

*Voidrift Phase 6 Directive | April 2026 | RFD IT Services Ltd.*  
*Spec and doc first. No code until pre-implementation research questions are answered.*
