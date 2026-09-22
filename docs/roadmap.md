# Voidrift Development Roadmap

*Phases, milestones, and current tasks for the Voidrift development team*

---

## Machine-Readable Roadmap (swarm)

Why these milestones: the repo's own queue orders launch blockers first — they
gate the $4.99 itch.io release (docs/state/current.md "Open Directives",
ADR-017). Structural rework comes second: current.md names it "dev branch,
between sprints", and both Phase 4b content and the economy redesign land
cleaner on split modules. Phase 4b follows README.md's roadmap table; the
economy redesign is last because its own phase doc requires the refactor to
finish first (docs/phases/phase-economy-redesign-planned.md). The phase tables
below are the April 2026 snapshot — kept as history; this block is the live
plan.

```yaml roadmap
status: approved
approved: 2026-09-22 robert-claude
reviewed: '2026-09-22'
replan_after_days: 14
stop_if: The TypeScript rebuild in web/ replaces the Rust/Bevy game as the shipping line, or Robert pulls the game from sale.
milestones:
- id: M1
  title: Launch blockers cleared for the $4.99 itch.io release
  status: active
  exit:
  - test: cargo test
  - grep:
      path: assets/balance.toml
      pattern: max_per_field = [2-9]
  - grep:
      path: Cargo.toml
      pattern: bevy_audio|kira|rodio
  steps:
  - id: M1.1
    title: 'Audio pass — sound effects and ambient (issue #9)'
    kind: feature
    size: M
    value: 5
    status: pending
    directive: ''
    detail: current.md lists "#9 Audio Pass" as a launch blocker. Cargo.toml enables no audio feature today, so the step picks an approved backend (bevy_audio or an ADR'd alternative), adds sound effects and ambient, and keeps Mali-G57 constraints in mind.
    accept:
    - grep:
        path: Cargo.toml
        pattern: bevy_audio|kira|rodio
    - test: cargo test
  - id: M1.2
    title: 'Production tree zoom/scroll (issue #7)'
    kind: feature
    size: M
    value: 3
    status: pending
    directive: ''
    detail: 'The production tree rendered in src/systems/ui/hud/prod_tree.rs has no zoom or scroll; issue #7 is a launch blocker. Add zoom/scroll input consistent with the existing pinch_zoom_system pattern in src/systems/visuals/map.rs.'
    accept:
    - grep:
        path: src/systems/ui/hud/prod_tree.rs
        pattern: zoom|scroll
    - test: cargo test
  - id: M1.3
    title: 'Increase asteroid density (issue #18)'
    kind: fix
    size: S
    value: 3
    status: pending
    directive: ''
    detail: assets/balance.toml sets max_per_field = 1, flagged "too sparse for gameplay" in current.md. Raise it (and station.max_active_asteroids headroom if needed) and sanity-check against the respawn cap logic in src/world/asteroid.rs.
    accept:
    - grep:
        path: assets/balance.toml
        pattern: max_per_field = [2-9]
    - test: cargo test
  - id: M1.4
    title: 'Finish tutorial refinement and symbol bar logic (issue #13)'
    kind: feature
    size: M
    value: 2
    status: pending
    directive: ''
    detail: 'Part of #13 landed (symbol status bar in drawer, ae9ba42; T-107 added then removed in 7183c34). Close out the remaining tutorial refinement and symbol-bar logic still flagged pending in current.md; tutorial content lives in assets/content/tutorial.yaml.'
    accept:
    - test: cargo test
  - id: M1.5
    title: 'Android store assets for Google Play submission (issue #5)'
    kind: docs
    size: M
    value: 2
    status: pending
    directive: ''
    detail: Produce the store listing assets (icon, feature graphic, screenshots, listing copy) under android/store/ needed for Google Play submission. capture_gate_evidence.ps1 exists for device-correct screenshots on the Moto G 2025.
    accept:
    - file: android/store
- id: M2
  title: Structural rework — god classes split, drone spawn debt paid
  status: pending
  exit:
  - test: cargo test
  - grep:
      path: src/economy/process.rs
      pattern: spawn_drone_ship_with_visuals
  - file: src/config/visual_utils.rs
  steps:
  - id: M2.1
    title: Pay ADR-021 debt — factory drones spawn with visuals; delete empty mod.rs stubs
    kind: fix
    size: S
    value: 4
    status: pending
    directive: ''
    detail: 'ADR-021 "Known Debt": auto_build_drones_system in src/economy/process.rs calls spawn_drone_ship, producing invisible drones — switch to spawn_drone_ship_with_visuals. Also delete the empty mod.rs stubs left in src/systems/game_loop/, src/systems/setup/, src/systems/asteroid/.'
    accept:
    - grep:
        path: src/economy/process.rs
        pattern: spawn_drone_ship_with_visuals
    - test: cargo test
  - id: M2.2
    title: 'Split hud/mod.rs god class (TD-001 / issue #16)'
    kind: refactor
    size: M
    value: 5
    status: pending
    directive: ''
    detail: hud/mod.rs (~1040 lines per current.md) still mixes cargo display, station visuals, production tree orchestration, and tab logic. Extract the remaining cohesive systems into focused hud/ submodules alongside the existing buttons.rs, content.rs, overlays.rs, prod_tree.rs, state_machine.rs. No behavior change.
    accept:
    - test: cargo test
  - id: M2.3
    title: 'Split Layer 1 god files — resources.rs, save.rs; move color utils (issues #23, #24, #25)'
    kind: refactor
    size: M
    value: 4
    status: pending
    directive: ''
    detail: 'Per current.md: components/resources.rs splits into states/resources/station/narrative modules; persistence/save.rs into save_data/save_system/save_paths; color conversion functions move out of config/visual.rs into src/config/visual_utils.rs. Mechanical moves only.'
    accept:
    - file: src/systems/persistence/save_data.rs
    - file: src/config/visual_utils.rs
    - test: cargo test
  - id: M2.4
    title: 'Split Layer 3 god files and remove dead code (issues #30, #31, #32)'
    kind: refactor
    size: M
    value: 3
    needs:
    - M2.3
    status: pending
    directive: ''
    detail: Split scenes/main_menu.rs into menu_ui/save_load/menu_setup and visuals/component_nodes.rs into per-component files; remove the ui_layout_system no-op in viewport.rs and legacy tutorial beats T-001..T-006 that can never fire. Needs M2.3 first so the save.rs split and the menu save/load extraction do not collide on the same files.
    accept:
    - file: src/scenes/menu_setup.rs
    - test: cargo test
  - id: M2.5
    title: 'Config-driven signal triggers and laser tier validation (issues #28, #19)'
    kind: refactor
    size: M
    value: 3
    status: pending
    directive: ''
    detail: src/systems/narrative/signal.rs carries 30+ hardcoded triggers and src/economy/mining.rs hardcodes laser tier validation. Move both into config — the assets/content/*.yaml and assets/balance.toml loading pattern already exists — so content and balance changes stop requiring code edits.
    accept:
    - test: cargo test
- id: M3
  title: Phase 4b — narrative drops and faction voices
  status: pending
  exit:
  - test: cargo test
  - grep:
      path: assets/content/requests.yaml
      pattern: Pirate
  - grep:
      path: assets/content/echo.yaml
      pattern: faction
  steps:
  - id: M3.1
    title: Memory fragments delivered through bottle collection
    kind: feature
    size: M
    value: 4
    status: pending
    directive: ''
    detail: 'Phase 4b per README.md and docs/roadmap.md: memory fragments arrive through the existing bottle mechanic (bottle_spawn_system / bottle_input_system) and surface in the Logs tab via check_log_unlocks. Add fragment entries under assets/content/ and wire their unlock triggers — fragments only, no dialogue trees or cutscenes (ADR-010).'
    accept:
    - test: cargo test
  - id: M3.2
    title: Faction voice differentiation in log and Echo content
    kind: feature
    size: M
    value: 3
    needs:
    - M3.1
    status: pending
    directive: ''
    detail: Differentiate faction voices through log entry tone (docs/roadmap.md Phase 4). Tag or group content by faction in assets/content/echo.yaml and logs.yaml so content_router and check_log_unlocks can select per-faction voice; keep the established YAML content-loading pattern.
    accept:
    - grep:
        path: assets/content/echo.yaml
        pattern: faction
    - test: cargo test
  - id: M3.3
    title: First Human and Pirate faction bottles and requests
    kind: feature
    size: M
    value: 3
    needs:
    - M3.2
    status: pending
    directive: ''
    detail: Add First Human and Pirate faction bottles plus additional Signal requests with escalating narrative weight (docs/roadmap.md Phase 4). assets/content/requests.yaml already defines the faction_request schema (see FirstLight) — extend it rather than inventing a new format. Needs M3.2 so the new factions ship with distinct voices.
    accept:
    - grep:
        path: assets/content/requests.yaml
        pattern: Pirate
    - test: cargo test
- id: M4
  title: Economy redesign groundwork — reconcile spec, then engine tiers and Helium
  status: pending
  exit:
  - test: cargo test
  - grep:
      path: src/components/game_state.rs
      pattern: engine_tier|EngineTier
  - grep:
      path: assets/balance.toml
      pattern: helium|Helium
  steps:
  - id: M4.1
    title: Reconcile the economy redesign spec with the shipped four-ore economy
    kind: design
    size: M
    value: 3
    needs:
    - M2.3
    status: pending
    directive: ''
    detail: docs/phases/phase-economy-redesign-planned.md describes a Magnetite/Power-Cell economy that no longer exists — the shipped game runs Iron/Tungsten/Nickel/Aluminum (current.md "Current Economy"). Update the phase doc and docs/design/ECONOMY.md so the three-track plan maps onto the live economy before any code changes. Needs the Layer 1 splits (M2.3) done so the spec targets the settled module layout.
    accept:
    - grep:
        path: docs/phases/phase-economy-redesign-planned.md
        pattern: Iron
    - file: docs/design/ECONOMY.md
  - id: M4.2
    title: Engine tier enum and fuel boost groundwork on Ship
    kind: feature
    size: M
    value: 3
    needs:
    - M4.1
    status: pending
    directive: ''
    detail: The redesign adds EngineTier Mk I–V as permanent upgrades plus an optional Fuel Boost speed multiplier; SHIP_SPEED becomes the Mk I base (phase doc "Ship Changes"). Add the enum, the Ship fields (engine_tier, fuel_boost_active, fuel_boost_timer), and balance.toml entries without changing dispatch behavior yet.
    accept:
    - grep:
        path: src/components/game_state.rs
        pattern: engine_tier|EngineTier
    - test: cargo test
  - id: M4.3
    title: Helium passive yield from asteroid mining
    kind: feature
    size: M
    value: 2
    needs:
    - M4.1
    status: pending
    directive: ''
    detail: Helium is a passive secondary yield (~2 per 100 ore) feeding the Gas track toward Fuel Cells (phase doc "Resource Changes"). Add the reserve field, balance.toml entries, mining-system accrual, and a first-detection signal.
    accept:
    - grep:
        path: assets/balance.toml
        pattern: helium|Helium
    - test: cargo test
```

---

## Current State

**Sprint:** Phase 3 Planning  
**Status:** Phase 2 complete and tagged. Codebase stable. Physical device verified.  
**Last Tag:** `v2.1.0-starmap-parallax-fix` — April 27, 2026

**What's Working Now:**
- ✅ Mining → Refining → Drone building loop
- ✅ Autonomous drone fleet with bottle collection
- ✅ PRODUCTION tab (Iron / Tungsten / Nickel / Aluminum with ComboBox)
- ✅ REQUESTS tab (Signal faction, First Light, fulfillment logic)
- ✅ Random radial asteroid spawning, global cap enforced
- ✅ `power_multiplier` wired to mining rate
- ✅ Star map: circular, station-centered, proper parallax
- ✅ Save/load persistence including RequestsTabState

---

## Phase Breakdown

### Phase 1: Core Loop Foundation ✅ COMPLETE
**Completed:** April 18–25, 2026  
**Goal:** Establish mining → refining → drone building loop  

- [x] Mining system with laser beams
- [x] Auto-refining
- [x] Drone building and fleet management
- [x] Unified ship queue
- [x] Save/load persistence
- [x] Narrative frame establishment

### Phase 1c: Asteroid Lifecycle ✅ COMPLETE
**Completed:** April 26, 2026  
**Goal:** Finite ore per asteroid, respawn cycle, stuck-ship safety  

- [x] Asteroid inventory system (ore_remaining)
- [x] Lifespan timer paused while drone is targeting
- [x] Respawn cycle with cap enforcement
- [x] Stuck-ship safety system

### Phase 2: UI Refactor + Requests + Arcade Loop ✅ COMPLETE
**Completed:** April 27, 2026  
**Tag:** `v2.0.0-phase2-complete`, `v2.1.0-starmap-parallax-fix`  
**Goal:** Arcade loop hardened, faction contact system live, UI restructured  

- [x] PRODUCTION tab: all four ore types under single ComboBox
- [x] REQUESTS tab: replaces UPGRADES placeholder
- [x] Signal faction (Ancient): First Light request and fulfillment
- [x] Bottle collection mechanic: spawn, tap-to-collect, dual output
- [x] Aluminum full pipeline: Ore → Ingot → AluminumCanister
- [x] Random radial asteroid spawning (200–500 units), 4 ore types, equal probability
- [x] Global asteroid cap (`station.max_active_asteroids = 3`) enforced
- [x] `power_multiplier` wired to mining rate in `mining.rs`
- [x] RequestsTabState persistence across save/load
- [x] Starfield: circular generation, station-centered, absolute parallax (no delta drift)
- [x] Legacy sector spawn systems removed

### Phase 3: Architectural Refactor (SRP / Event Bus) 🚧 NEXT
**Goal:** Decouple narrative logic from core simulation before adding new features. Fix structural strain identified during Phase 2.  
**Prerequisite:** Phase 2 physical device verification complete.

**Known issues to resolve:**
- `autopilot.rs` handles navigation geometry, state machine transitions, docking sequences, AND narrative bottle collection — too many responsibilities
- UI systems directly mutate core game state (e.g. `station.power_multiplier += 0.25`) — no central upgrade/economy system
- Hardcoded fallback behaviors create silent failure modes
- Initialization logic scattered across multiple systems (legacy dual-spawn bug)

**Target architecture:**
- Event bus pattern: systems fire events, narrative/economy systems listen and respond
- `autopilot.rs` fires `ArrivedAtTarget(Entity)` — separate narrative system handles what that means
- Central upgrade system reads multipliers and applies them — UI only writes intent
- Clean initialization order, no overlapping responsibilities

**Deliverables:**
- [ ] `ArrivedAtTarget` event type and handler system
- [ ] Bottle collection extracted from `autopilot.rs` into `narrative/bottle.rs` event handler
- [ ] Central upgrade application system (reads `Station` multipliers, applies downstream)
- [ ] UI fulfillment writes intent only — upgrade system applies
- [ ] Initialization order audit: `spawn_initial_asteroids` is sole asteroid spawner
- [ ] `cargo check` clean, zero warnings
- [ ] Physical device verification

### Phase 4: Narrative Drops 🔮 PLANNED
**Goal:** Signal Log earns its place as the primary narrative surface.  

- [ ] Memory fragments delivered through bottle collection
- [ ] Faction voices differentiated through log entry tone
- [ ] Additional Signal requests with escalating narrative weight
- [ ] First Human and Pirate faction bottles
- [ ] No dialogue trees, no cutscenes — fragments only

### Future Intentions (not yet phased)
- Remaining multiplier wiring: cargo capacity, ship speed
- Faction name finalization (Human, Borg, Pirate placeholders)
- Additional faction Bottles and request cards per faction
- Upgrade cap expansion via requests (spawn rate, lifespan, drone count)
- Scanning mechanic (ore identification before mining)
- Circular galaxy starmap UI overlay
- Viewport scroll bounding
- Play Store public release prep

---

## Current Tasks (Phase 1c)

### Task 1: Asteroid Inventory System
**Assigned:** Antigravity  
**Status:** Ready to start  
**Estimate:** 2 hours  

**Description:** Add ore count to asteroids, deplete as mined

**Acceptance Criteria:**
- [ ] ActiveAsteroid component has ore_remaining field
- [ ] Mining system decrements ore_remaining
- [ ] Asteroid despawns at 0 ore
- [ ] Player sees visual feedback (ore depleting)

**Implementation Notes:**
- Add ore_remaining to AsteroidField component
- Update mining_system to decrement count
- Add despawn logic when ore_remaining <= 0
- Visual feedback through color/material changes

### Task 2: Respawn Cycle
**Assigned:** Antigravity  
**Status:** Blocked until Task 1 complete  
**Estimate:** 1.5 hours  

**Description:** New asteroids spawn after timer

**Acceptance Criteria:**
- [ ] Respawn timer constant (configurable)
- [ ] New asteroid spawns at random location
- [ ] Player notified (visual or audio cue)
- [ ] Cycle repeats smoothly

**Implementation Notes:**
- Add ASTEROID_RESPAWN_TIME to constants.rs
- Timer system tracks elapsed time since depletion
- Spawn new asteroid at random valid position
- Signal strip notification when asteroid respawns

### Task 3: Test & Balance
**Assigned:** Robert (you)  
**Status:** Blocked until Tasks 1-2 complete  
**Estimate:** 1 hour  

**Description:** Play Phase 1c, tune constants

**Acceptance Criteria:**
- [ ] Play for 30 minutes continuously
- [ ] Asteroid lifecycle feels natural
- [ ] No crashes or bugs
- [ ] Feedback to adjust timing if needed

**Testing Focus:**
- Does depletion feel too fast/slow?
- Is respawn timing appropriate?
- Any performance issues with spawning/despawning?
- Does resource flow feel balanced?

---

## Milestones

| Milestone | Date | Status |
|-----------|------|--------|
| Phase 1 Complete | April 25, 2026 | ✅ Done |
| Phase 1c Complete | April 26, 2026 | ✅ Done |
| Phase 2 Complete | April 27, 2026 | ✅ Done — `v2.0.0-phase2-complete` |
| Starmap Parallax Fix | April 27, 2026 | ✅ Done — `v2.1.0-starmap-parallax-fix` |
| Phase 3 Complete | TBD | 🚧 Next |
| Phase 4 Complete | TBD | 🔮 Planned |
| Play Store Release | TBD | 🔮 Future |

---

## Dependencies & Blockers

### Current Blockers
**None** - Phase 1c is ready to start

### Upcoming Dependencies
- Phase 2 depends on Phase 1c (stable resource flow)
- Phase 3 depends on Phase 2 (need surplus production)
- Phase 4 depends on Phase 3 (need faction relationships)
- Phase 5 depends on Phase 4 (lock gameplay before refactor)

### Technical Dependencies
- Bevy UI migration planned but not blocking
- Android stability maintained through Universal Disjointness
- Performance monitoring required for each phase

---

## What Changed This Sprint

### April 18-25, 2026 - Narrative Pivot
- **Frame Change:** From mining sim to survival sci-fi
- **Justification:** Black hole setting explains mechanics
- **Documentation:** Created NARRATIVE_JUSTIFICATION.md
- **ADR-010:** Locked narrative scope decision

### April 26, 2026 - Documentation Infrastructure
- **Reorganization:** Clear docs hierarchy
- **Developer Guide:** DEVELOPER.md for onboarding
- **Roadmap:** This document with phase/task breakdown
- **Navigation:** docs/README.md explains structure

### Phase 1b Complete - Unified Ship Queue
- **Ship System:** All ships identical, queue-based dispatch
- **Fleet Management:** Ships available count, auto-assemble
- **UI Streamlined:** Fleet status at a glance
- **Save/Load Fixed:** Fleet persists correctly

---

## What's Next

### Immediate (This Week)
1. **Start Phase 1c:** Implement asteroid inventory system
2. **Complete depletion:** Add despawn logic
3. **Add respawn:** New asteroids spawn after timer
4. **Test balance:** Play and tune constants

### Near Future (Next Sprint)
1. **Plan Phase 2:** Design module system
2. **Architecture review:** Prepare for station expansion
3. **Performance testing:** Ensure Phase 1c is stable
4. **Documentation updates:** Reflect Phase 1c changes

### Long-term Goals
1. **Faction system:** Interactive NPCs at boundary
2. **Story reveals:** Uncover black hole mystery
3. **Endgame:** Player choice determines fate
4. **Codebase health:** Maintainable, documented architecture

---

## Development Guidelines

### Sprint Planning
- Each phase has clear deliverables and acceptance criteria
- Tasks assigned to specific developers
- Estimates based on previous experience
- Blockers identified and tracked

### Quality Standards
- All features tested on target hardware (Moto G 2025)
- No regressions in existing functionality
- Documentation updated for each phase
- Performance monitored and optimized

### Communication
- Daily progress updates in development channel
- Blockers raised immediately
- Design decisions documented as ADRs
- Cross-team coordination for dependencies

---

*Last updated: April 27, 2026 — Phase 2 complete, Phase 3 planning begins.*
