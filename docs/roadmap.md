# Voidrift Development Roadmap

*Phases, milestones, and current tasks for the Voidrift development team*

---

## Current Sprint

**Sprint:** Phase 1b Complete, Phase 1c Queued  
**Status:** Core loop working, narrative frame locked  
**Timeline:** April 18-25 (completed), April 26-May 3 (next)  

**What's Working Now:**
- ✅ Mining → Refining → Drone building loop
- ✅ Survival sci-fi narrative frame (not horror)
- ✅ Unified ship queue system
- ✅ Bottom drawer UI with station tabs
- ✅ Save/load persistence

---

## Phase Breakdown

### Phase 1: Core Loop Foundation ✅ COMPLETE
**Timeline:** April 18, 2026 - April 25, 2026  
**Goal:** Establish mining → refining → drone building loop  

**Deliverables:**
- [x] Mining system with laser beams
- [x] Auto-refining (phase 0b)
- [x] Drone building and fleet management
- [x] Unified ship queue (phase 1b)
- [x] Save/load persistence
- [x] Narrative frame establishment

### Phase 1c: Asteroid Lifecycle 🚧 QUEUED
**Timeline:** April 26 - May 3, 2026  
**Goal:** Finite ore per asteroid, respawn cycle  

**Deliverables:**
- [ ] Asteroid inventory system (ore count)
- [ ] Depletion mechanic (asteroid disappears when mined out)
- [ ] Respawn timer (new asteroids spawn periodically)
- [ ] Natural gameplay cycle (player manages resource flow)

**Estimated:** 1 week  
**Blocker:** None (ready to start)

### Phase 2: Station Expansion 📋 PLANNED
**Timeline:** May 4 - May 17, 2026  
**Goal:** Progression through station modules  

**Deliverables:**
- [ ] Module system (upgrades unlock new capabilities)
- [ ] Upgrade costs (escalating, meaningful progression)
- [ ] Unlock gates (module A enables asteroid type B)
- [ ] Visual feedback (station grows as upgraded)

**Estimated:** 2 weeks  
**Dependency:** Phase 1c complete

### Phase 3: Faction System 📋 PLANNED
**Timeline:** May 18 - May 31, 2026  
**Goal:** First NPC interaction, trade mechanics  

**Deliverables:**
- [ ] Faction appearance (unmanned drone at boundary)
- [ ] Trade board (specific resource requests)
- [ ] Reputation system (trades affect standing)
- [ ] Story hints (dialogue reveals mystery)

**Estimated:** 2 weeks  
**Dependency:** Phase 2 complete (need surplus production)

### Phase 4: Discovery & Narrative 🔮 FUTURE
**Timeline:** June 1 - June 21, 2026  
**Goal:** Uncover what happened, factions reveal conflicts  

**Deliverables:**
- [ ] Faction conflicts emerge
- [ ] Story logs discovered
- [ ] Multiple faction paths available
- [ ] Endgame direction clarifies

**Estimated:** 3 weeks  
**Dependency:** Phase 3 complete

### Phase 5: Codebase Refactor 🔮 FUTURE
**Timeline:** June 22 - July 5, 2026  
**Goal:** Organize code for maintainability  

**Deliverables:**
- [ ] Component cleanup (split bloated components)
- [ ] System organization (group related systems)
- [ ] Documentation (explain architecture)
- [ ] Test coverage (add missing tests)

**Estimated:** 2 weeks  
**Dependency:** Phase 4 complete (lock gameplay first)

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

| Milestone | Target Date | Status | Notes |
|-----------|------------|--------|-------|
| Phase 1c Complete | May 3, 2026 | On track | Asteroid lifecycle working |
| Phase 2 Complete | May 17, 2026 | Planned | Station modules |
| Phase 3 Complete | May 31, 2026 | Planned | Faction system |
| Phase 4 Complete | June 21, 2026 | Planned | Story reveals |
| Phase 5 Complete | July 5, 2026 | Planned | Codebase refactor |
| v1.0 Ready | July 1, 2026 | Planned | Feature complete |

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

*Last updated: April 26, 2026*  
*Next update: May 3, 2026 (Phase 1c completion)*
