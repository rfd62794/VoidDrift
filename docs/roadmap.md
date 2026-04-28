# Voidrift Development Roadmap

*Phases, milestones, and current tasks for the Voidrift development team*

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
