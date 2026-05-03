# VoidDrift: Documentation Reorganization + Developer Workflow Directive
**Objective:** Reorganize docs for clarity. Establish developer workflow (phases, goals, tasks, milestones). Prepare for codebase refactor analysis.  
**Status:** Documentation infrastructure phase  
**Estimated time:** 3-4 hours (docs reorganization + workflow setup)  
**Deliverable:** Clean docs structure + developer onboarding guide + milestone roadmap

---

## Part 1: Documentation Reorganization

### Current State
```
docs/
├── adr/ (9 files)
├── directives/ (multiple subdirs)
├── phases/ (historical)
├── GDD, ARCHITECTURE, CHANGELOG, etc (root level)
├── VISION, ECONOMY, STARGATE (design docs)
└── (scattered, inconsistent naming)
```

### Target State
```
docs/
├── README.md                           # Docs overview & navigation
├── GDD.md                             # Game Design Document
├── ARCHITECTURE.md                    # Technical architecture
├── CHANGELOG.md                       # Git-style changelog
├── NARRATIVE_JUSTIFICATION.md         # Why survival frame exists
├── DEVELOPER.md                       # Developer onboarding (NEW)
├── ROADMAP.md                         # Phase/milestone/task breakdown (NEW)
│
├── adr/                               # Architectural Decision Records
│   ├── ADR-001-present-mode-fifo.md
│   ├── ADR-002-...
│   └── ADR-010-narrative-scope.md
│
├── design/                            # Design deep-dives
│   ├── economy.md
│   ├── stargate.md
│   ├── ui-vision.md
│   └── vision.md
│
├── phases/                            # Historical phase summaries
│   ├── phase-0-arcade-loop-mvp.md
│   ├── phase-1-power-deletion.md
│   ├── phase-1b-unified-queue.md
│   └── ... (archived, reference only)
│
└── directives/                        # Implementation directives (archival)
    ├── analysis/
    ├── narrative/
    ├── systems/
    └── legacy/
```

### Key Changes

1. **Root level:** Only core docs (GDD, ARCHITECTURE, CHANGELOG, DEVELOPER, ROADMAP)
2. **Subdirectories:** adr/, design/, phases/, directives/
3. **New files:** DEVELOPER.md, ROADMAP.md
4. **Naming:** Lowercase, hyphens, consistent
5. **Navigation:** README.md explains structure

---

## Part 2: DEVELOPER.md (Developer Onboarding Guide)

**File:** `docs/DEVELOPER.md`

### Structure

```markdown
# VoidDrift Developer Guide

## Quick Start (Monday Morning)
- How to build
- How to deploy to device
- How to run tests
- Where to check current status

## Current State (What's Working)
- Core loop: Mining → Refining → Drone building
- UI: Bottom drawer with tabs (Iron/Tungsten/Nickel/Fleet/Upgrades)
- Save/Load: SQLite persistence working
- Narrative frame: Event Horizon survival sci-fi (justifies mechanics, not tone)

## Last Changes (What Changed Since You Left)
- April 18-25: Major narrative pivot + system unification
- Opening sequence: Falling into black hole → wake up fused with station AI
- Unified ship queue: All ships identical, queue-based dispatch
- UI streamlined: Removed redundant tabs, one-glance fleet status

## Next Phase (Where It's Going)
- Phase 1c: Asteroid inventory + respawn cycle
- Phase 2: Station modules + upgrades
- Phase 3: First faction + trade system
- Phase 4: Discovery/story reveals

## How to Contribute
- Read ROADMAP.md for current sprint
- Check /src/systems/ for logic
- All constants in constants.rs
- Follow ADRs (docs/adr/) for architectural decisions

## Key Files
- src/lib.rs - App setup, system registration
- src/constants.rs - All magic numbers
- src/components.rs - ECS components
- src/systems/ - Game logic by domain
- docs/NARRATIVE_JUSTIFICATION.md - Why each mechanic exists

## Build & Deploy
```bash
./build_android.ps1  # One-click build + deploy
```

## Testing
```bash
cargo test           # Run tests
cargo check          # Check compilation
```

## Emergency Contacts
- Antigravity (coding agent)
- Claude (architecture/directives)
- Check CHANGELOG.md for recent changes
```

---

## Part 3: ROADMAP.md (Phase/Milestone Structure)

**File:** `docs/ROADMAP.md`

### Structure

```markdown
# VoidDrift Development Roadmap

## Current Sprint
**Sprint:** Phase 1b Complete, Phase 1c Queued  
**Status:** Core loop working, narrative frame locked  
**Timeline:** April 18-25 (completed), April 26-May 3 (next)  

## Phases & Milestones

### Phase 1: Core Loop Foundation ✅ COMPLETE
**Goal:** Establish mining → refining → drone building loop  
**Deliverables:**
- [x] Mining system
- [x] Auto-refining (phase 0b)
- [x] Drone building
- [x] Unified ship queue (phase 1b)

### Phase 1c: Asteroid Lifecycle (QUEUED)
**Goal:** Finite ore per asteroid, respawn cycle  
**Deliverables:**
- [ ] Asteroid inventory (ore count)
- [ ] Depletion mechanic (asteroid disappears when mined out)
- [ ] Respawn timer (new asteroids spawn)
- [ ] Natural gameplay cycle

**Estimated:** 1 week  
**Blocker:** None (ready to start)

### Phase 2: Station Expansion (PLANNED)
**Goal:** Progression through station modules  
**Deliverables:**
- [ ] Module system (upgrades that unlock capability)
- [ ] Upgrade costs (escalating, meaningful progression)
- [ ] Unlock gates (module A unlocks asteroid type B)
- [ ] Visual feedback (station grows)

**Estimated:** 2 weeks  
**Dependency:** Phase 1c complete

### Phase 3: Faction System (PLANNED)
**Goal:** First NPC interaction, trade mechanics  
**Deliverables:**
- [ ] Faction appearance (unmanned drone at boundary)
- [ ] Trade board (faction requests specific resources)
- [ ] Reputation system (trades affect standing)
- [ ] Story hints (dialogue reveals mystery)

**Estimated:** 2 weeks  
**Dependency:** Phase 2 complete (need surplus production)

### Phase 4: Discovery & Narrative (FUTURE)
**Goal:** Uncover what happened, factions reveal conflicts  
**Deliverables:**
- [ ] Faction conflicts emerge
- [ ] Story logs discovered
- [ ] Multiple faction paths available
- [ ] Endgame direction clarifies

**Estimated:** 3 weeks  
**Dependency:** Phase 3 complete

### Phase 5: Codebase Refactor (FUTURE)
**Goal:** Organize code for maintainability  
**Deliverables:**
- [ ] Component cleanup (split bloated components)
- [ ] System organization (group related systems)
- [ ] Documentation (explain architecture)
- [ ] Test coverage (add missing tests)

**Estimated:** 2 weeks  
**Dependency:** Phase 4 complete (lock gameplay first)

## Current Tasks (Phase 1c)

### Task 1: Asteroid Inventory System
**Description:** Add ore count to asteroids, deplete as mined  
**Acceptance Criteria:**
- [ ] ActiveAsteroid component has ore_remaining field
- [ ] Mining system decrements ore_remaining
- [ ] Asteroid despawns at 0 ore
- [ ] Player sees visual feedback (ore depleting)

**Estimate:** 2 hours  
**Assigned:** Antigravity  
**Status:** Ready to start

### Task 2: Respawn Cycle
**Description:** New asteroids spawn after timer  
**Acceptance Criteria:**
- [ ] Respawn timer constant (configurable)
- [ ] New asteroid spawns at random location
- [ ] Player notified (visual or audio cue)
- [ ] Cycle repeats smoothly

**Estimate:** 1.5 hours  
**Assigned:** Antigravity  
**Status:** Blocked until Task 1 complete

### Task 3: Test & Balance
**Description:** Play Phase 1c, tune constants  
**Acceptance Criteria:**
- [ ] Play for 30 minutes
- [ ] Asteroid lifecycle feels natural
- [ ] No crashes or bugs
- [ ] Feedback to adjust timing

**Estimate:** 1 hour  
**Assigned:** Robert (you)  
**Status:** Blocked until Tasks 1-2 complete

## Milestones

| Milestone | Target Date | Status |
|-----------|------------|--------|
| Phase 1c Complete | May 3, 2026 | On track |
| Phase 2 Complete | May 17, 2026 | Planned |
| Phase 3 Complete | May 31, 2026 | Planned |
| Phase 4 Complete | June 21, 2026 | Planned |
| v1.0 Ready | July 1, 2026 | Planned |

## Dependencies & Blockers

**None currently.** Phase 1c ready to start.

## What Changed This Sprint

- Narrative pivot: Event Horizon survival sci-fi frame
- Ship queue unified: All ships identical, dispatch-based
- UI streamlined: Fleet status at a glance
- Save/load fixed: Fleet persists on load
- Documentation realigned: Survival frame clarified

## What's Next

Start Phase 1c: Asteroid inventory + respawn cycle
```

---

## Part 4: Implementation Checklist

### Antigravity Tasks

**1. Reorganize docs structure**
```bash
# Create new directories
mkdir -p docs/design
mkdir -p docs/directives/analysis
mkdir -p docs/directives/narrative
mkdir -p docs/directives/systems
mkdir -p docs/directives/legacy

# Move existing files
mv docs/ECONOMY.md docs/design/
mv docs/STARGATE.md docs/design/
mv docs/UI_VISION.md docs/design/
mv docs/VISION.md docs/design/

# Rename phase files (if needed)
# Move old directives to directives/legacy/
```

**2. Create docs/README.md**
- Explain docs structure
- Navigation guide
- Quick links to core docs

**3. Create DEVELOPER.md**
- Quick start (build, deploy, test)
- Current state (what works)
- Last changes (what changed)
- Next phase (where going)
- Key files & how to contribute

**4. Create ROADMAP.md**
- Phase breakdown (1-5)
- Current sprint tasks
- Milestone timeline
- Blockers & dependencies

**5. Verify all links**
- Update cross-references in docs
- Ensure README navigation works
- Test all relative paths

**6. Commit**
```bash
git add docs/
git commit -m "docs: reorganize structure + add developer workflow

- Reorganize docs into clear hierarchy (root core, subdirs for adr/design/phases/directives)
- Create DEVELOPER.md for developer onboarding
- Create ROADMAP.md with phases, milestones, current tasks
- Create docs/README.md explaining navigation
- Move design docs to docs/design/
- Move old directives to docs/directives/legacy/

No code changes. Documentation infrastructure only."

git tag v1.0-docs-reorganized
```

---

## Part 5: Codebase Refactor Analysis (NEXT)

After docs are clean, Antigravity should analyze codebase for refactor scope:

1. **Component organization** - Which are bloated? Which should split?
2. **System coupling** - Which systems depend on each other? Can we decouple?
3. **Code organization** - Should systems be grouped differently?
4. **Test coverage** - What's missing?
5. **Tech debt** - What's messy and why?

Output: Refactor options (A/B/C) with implementation roadmap.

**This becomes Phase 5 scope.**

---

## Why This Structure Works

### For You (Developer)
- **DEVELOPER.md:** Monday morning clarity (what changed, what's next)
- **ROADMAP.md:** Sprint/phase/task visibility
- **Clean structure:** Easy to find what you need
- **Habit building:** Same pattern for every repo

### For Next Person
- **Clear entry point:** docs/README.md
- **Quick start:** DEVELOPER.md
- **How we work:** Phases, tasks, milestones
- **Decisions locked:** ADRs explain why

### For AI Agents
- **Context available:** DEVELOPER.md summarizes state
- **Roadmap visible:** Know what's next
- **Decisions documented:** ADRs explain constraints
- **Directives have home:** docs/directives/ organized by domain

---

## Pattern to Repeat

This structure works for **every project**:

1. **docs/README.md** - Explain the docs
2. **docs/DEVELOPER.md** - Developer onboarding
3. **docs/ROADMAP.md** - Phases, milestones, tasks
4. **docs/adr/** - Decisions
5. **docs/design/** - Deep dives
6. **docs/directives/** - Implementation blueprints (archival)

---

## Success Criteria

- [ ] Docs reorganized with clear hierarchy
- [ ] DEVELOPER.md created (quick start + current state)
- [ ] ROADMAP.md created (phases + tasks + milestones)
- [ ] docs/README.md explains structure
- [ ] All cross-references work
- [ ] Next sprint (Phase 1c) is clear and ready
- [ ] Codebase refactor analysis queued for after

---

**This is your developer workflow foundation. Build this habit.**

**Go.**
