# Voidrift — Directive: Documentation Refactor (Part 1)
**Status:** Approved — Ready for Execution  
**Type:** Documentation Only — Zero Code Changes  
**Date:** April 2026  
**Depends On:** Structural Audit COMPLETE ✅  
**Blocks:** Code Refactor Directive (Part 2)

---

## 1. Objective

Bring all project documentation into alignment with the current codebase. Every document listed in this directive is either stale, missing, or incomplete. No code changes. No new features. Documentation only.

This directive exists so that the code refactor in Part 2 is built against accurate specifications, and so that any future agent session begins with correct context.

---

## 2. Strict Rules for This Directive

- **Zero code changes.** If any file in `src/` is modified, that is a violation.
- **Read the code, then write the docs.** Every document must reflect actual current implementation, not intended or planned behavior.
- **No speculation.** If a system's behavior is unclear from reading the code, document it as unclear and flag for Part 2.
- **Concrete and specific.** No vague statements like "handles rendering." Name the systems, name the components, name the files.

---

## 3. Documents to Create or Overwrite

### 3.1 `docs/state/current.md` — OVERWRITE

Current state of the project as of this directive. The agent must read the entire codebase before writing this document.

Required sections:

```markdown
# Voidrift — Current State
**Date:** April 2026
**Build:** v0.4.x (post Phase B + Tutorial UX)

## Test Floor
[list passing/failing/skipped if tests exist, otherwise note: no automated tests]

## Completed Systems
[list every system function, which file it lives in, what it does in one sentence]

## Active Resources
[list every Resource registered in lib.rs with its purpose]

## Active Components  
[list every Component in components.rs with its purpose]

## Known Issues
[list any duplicate components, stale references, or flagged issues from audit]

## Open Directives
[list any directive that is approved but not yet executed]

## Next Queued Work
[Code Refactor Part 2, Directive B component decomposition, multi-device scaling]
```

### 3.2 `docs/ARCHITECTURE.md` — OVERWRITE

Full architectural description of the current system. Must reflect post-Tutorial UX state.

Required sections:

```markdown
# Voidrift Architecture

## Technology Stack
[Bevy 0.15, egui 0.31, bevy_egui 0.33, cargo-ndk, NDK r29, Android API 35]

## Module Structure
[Every file in src/systems/ with its responsibility]

## System Execution Order
[List Update schedule groups — Gameplay & Logistics, Station/Narrative/UI]
[Note the 20-tuple partition constraint and why it exists]

## ECS Architecture Constraints
[Universal Disjointness requirement — every &mut Transform query needs Without<> filters]
[DockedAt pattern — how docked ships track berth position]
[System tuple partition — two groups, why, what goes where]

## Entity Hierarchy
[Station entity tree — hub, arms, berths, visual container]
[Ship entity tree — mesh, cargo bars, cargo labels, thruster glow]
[Asteroid entity tree — mesh, map icon, map label, ore label]

## Key Data Flows
[Signal: trigger condition → signal_system → SignalLog → ui.rs renders]
[Tutorial: condition → tutorial_system → TutorialState.active → ui.rs renders popup]
[Quest: signal fires → quest state update → quest_update_system progress → ui.rs renders]
[Processing: queue button → materials consumed → StationQueues job → processing_queue_system ticks → output deposited]
[Docking: ship arrives at berth → DockedAt inserted → docked_ship_system tracks berth each tick → station rotation carries ship]

## Rendering Architecture
[Z-layer system — full table of Z constants]
[AlphaMode2d::Opaque for all background elements — why]
[Mali GPU considerations]

## Input Architecture
[Touch input — single finger space view vs map view]
[Two-finger pinch zoom — suppresses single touch]
[Map pan — single finger in MapView only]
```

### 3.3 `docs/CHANGELOG.md` — APPEND

Add entries for all phases since the last entry. Agent must read git log or walkthrough docs to reconstruct accurate dates and changes.

Format each entry:

```markdown
## [Phase N] — Title — Date
### Added
- [list new systems, components, features]
### Changed  
- [list modifications to existing systems]
### Fixed
- [list bugs resolved]
### Architecture
- [list architectural decisions made]
```

Required entries to add:
- Phase 07: Signal Strip & Narrative Overhaul
- Phase 08: Processing Queues & Auto-Dock
- Phase 09: Quest Tracker, World Expansion, Pinch Zoom
- Phase 10: Tutorial UX & Map Pan
- Station Phase A: Rotating Hub-and-Spoke Visual
- Station Phase B: Berth Navigation & DockedAt Architecture
- Directive A: Structural Stabilization (Universal Disjointness)

### 3.4 `docs/AGENT_CONTRACT.md` — APPEND INVARIANTS SECTION

The AGENT_CONTRACT.md defines what agents must never violate. Add a new section:

```markdown
## Invariants Added April 2026

### INV-004: Universal Disjointness (Total Lockdown)
Every system that queries `&mut Transform` MUST include explicit `Without<T>` 
filters for all major entity types that other queries in the same system 
might touch. This is non-negotiable on Mali-G57 GPU hardware.

Violating this causes runtime B0001 panics on Android that cargo check 
does not catch.

Required Without filters by entity type:
- Ship queries: Without<Station>, Without<AsteroidField>, Without<MiningBeam>
- Station queries: Without<Ship>, Without<AutonomousShip>  
- Beam queries: Without<Ship>, Without<Station>, Without<AsteroidField>
- Camera queries: Without<Ship>, Without<StarLayer>
- Star queries: Without<MainCamera>

### INV-005: System Tuple Partition
The Bevy Update schedule cannot hold more than 20 systems in a single tuple.
Systems are partitioned into two groups in lib.rs:
- Group 1 (Gameplay & Logistics): movement, mining, economy, autopilot, autonomous
- Group 2 (Station, Narrative & UI): visuals, narrative, tutorial, quest, ui, map

Never add a system without checking which group it belongs to and whether
the group is approaching the 20-system limit.

### INV-006: DockedAt Pattern
Ships that are docked at a berth MUST have a DockedAt(Entity) component
pointing to their berth entity. The docked_ship_system and 
docked_autonomous_ship_system use this to track rotating berth position
each tick. Never remove DockedAt without transitioning ship state to 
Navigating or Idle first.

### INV-007: One-Time Trigger Pattern
Signal triggers (SignalLog.fired HashSet) and Tutorial triggers 
(TutorialState.shown HashSet) are one-time only. Never clear these sets
during a session. IDs 19, 20, 21 in SignalLog are exceptions — they 
refire on state re-entry.

### INV-008: AlphaMode2d::Opaque for Background Elements
All background mesh entities (stars, station arms, connectors, asteroid 
boundary rings) MUST use AlphaMode2d::Opaque in their ColorMaterial.
Using Blend on Mali-G57 causes Z-sorting flicker that cannot be fixed 
by Z-layer adjustment alone. Achieve dimming through color values, 
not alpha transparency.
```

---

## 4. ADRs to Create

Create these files in `docs/adr/`:

### 4.1 `ADR-007-system-partitioning.md`

```markdown
# ADR-007: Update Schedule System Partitioning

**Date:** April 2026  
**Status:** Accepted

## Context
Rust's type system limits tuple size to 20 elements. Bevy's 
`.add_systems(Update, (sys1, sys2, ...))` uses tuples internally.
As Voidrift grew beyond 20 systems, the compiler rejected the single 
registration tuple.

## Decision
Partition the Update schedule into two named groups:
- Group 1: Gameplay & Logistics systems
- Group 2: Station, Narrative & UI systems

Each group is registered as a separate `.add_systems(Update, (...))` call.

## Consequences
- New systems must be assigned to a group on registration
- Groups must be monitored for size — stop at 18 to leave headroom
- System ordering between groups relies on Bevy's default scheduling
- If cross-group ordering is needed, use .after() explicitly

## Alternatives Considered
- SystemSet labeling: more complex, rejected for simplicity
- Single flat tuple: rejected, hits compiler limit
```

### 4.2 `ADR-008-universal-disjointness.md`

```markdown
# ADR-008: Universal Disjointness Architecture

**Date:** April 2026  
**Status:** Accepted

## Context
Bevy's ECS detects conflicting component access at runtime on Android.
When two queries in the same system both access Transform (one mutably),
and Bevy cannot prove they target disjoint entity sets, it panics with
error B0001. This panic does not appear in cargo check — only on device.

The Mali-G57 GPU in the Moto G 2025 (primary test device) triggers
this consistently. Multiple B0001 crashes were traced to systems with
unfiltered Transform queries.

## Decision
Every system that uses &mut Transform MUST include Without<T> filters
proving disjointness from all other Transform queries in the same system.
This is called "Total Lockdown" in project documentation.

A project-wide grep for "&mut Transform" is the verification tool.
Every result must have at least one Without<> filter.

## Consequences
- All new systems must follow this pattern from the start
- Adding a new entity type requires auditing all existing Transform queries
- The pattern is verbose but mechanical — easy to apply correctly

## Alternatives Considered
- ParamSet: valid alternative, more complex syntax, rejected for readability
- System splitting: used alongside filtering for the largest systems
```

### 4.3 `ADR-009-tutorial-trigger-pattern.md`

```markdown
# ADR-009: Contextual Tutorial Trigger Pattern

**Date:** April 2026  
**Status:** Accepted

## Context
Voidrift needs a tutorial system that fires exactly once per condition,
never repeats, shows one message at a time, and doesn't block gameplay.
The Signal system already handles narrative telemetry — tutorials need
a separate channel for longer explanatory text.

## Decision
Implement TutorialState resource with:
- shown: HashSet<u32> — IDs of already-shown tutorials
- active: Option<TutorialPopup> — currently displayed popup

tutorial_system checks conditions in ID order (lowest first).
If a condition is met and ID not in shown: set active.
On dismiss: insert ID into shown, clear active.

Tutorials are suppressed during OpeningPhase != Complete.

## Consequences
- Tutorial content is defined in code (narrative.rs), not data files
- Adding new tutorials requires modifying tutorial_system
- Session-only: shown set resets on app restart (no persistence yet)
- One popup at a time — if multiple conditions met, lowest ID wins

## Alternatives Considered
- Data-driven tutorial files: better long-term, too complex for current scale
- Signal-integrated tutorials: Signal lines are too brief for tutorial text
```

---

## 5. Phase Summaries to Create

Create these files in `docs/phases/`:

### 5.1 `phase-07-signal-narrative.md`

Document the Signal system implementation:
- SignalLog resource structure
- OpeningSequence state machine
- All 21+ Signal trigger IDs and their conditions
- Signal voice rules
- Opening sequence cinematic flow

### 5.2 `phase-08-processing-queues.md`

Document the processing queue system:
- StationQueues component structure
- Four parallel operation types with time constants
- Materials consumed on queue (not on completion)
- CLEAR behavior (active batch completes, future cancelled)
- AutoDockSettings and auto-unload/smelt behavior
- Processing signal integration

### 5.3 `phase-09-world-expansion.md`

Document the world expansion:
- All 6 sector positions and ore types
- OreDeposit and LaserTier component design
- Shape family generator functions
- Ore label child entity pattern
- Laser gate enforcement in mining_system
- Pinch zoom implementation and constants
- Map pan implementation

### 5.4 `phase-10-tutorial-ux.md`

Document the tutorial system:
- TutorialState resource structure
- T-001 through T-006 trigger conditions and messages
- Pop-up rendering and dismissal
- Cargo bar clarity changes (CargoOreLabel, CargoCountLabel)
- Cargo pulse at 95% capacity
- Smelter card conversion chain display
- Map pan system

---

## 6. Implementation Sequence

1. Read entire codebase — `src/` top to bottom before writing anything
2. Write `current.md` — current state from code reading
3. Write `ARCHITECTURE.md` — architecture from code reading
4. Write `CHANGELOG.md` entries — from walkthrough docs and git log
5. Write ADR-007, ADR-008, ADR-009
6. Write phase summaries 07-10
7. Append invariants to `AGENT_CONTRACT.md`

Do not write any document from memory or from previous conversation context. Read the actual code, then document what is actually there.

---

## 7. Completion Criteria

- [ ] `current.md` reflects actual current system state
- [ ] `ARCHITECTURE.md` covers all systems including post-Phase B additions
- [ ] `CHANGELOG.md` has entries for all 7 missing phases
- [ ] `AGENT_CONTRACT.md` has 5 new invariants (INV-004 through INV-008)
- [ ] ADR-007, ADR-008, ADR-009 created
- [ ] Phase summaries 07-10 created
- [ ] Zero code changes — no file in `src/` modified
- [ ] Every document based on actual code reading, not memory

**Gate:** A new agent session beginning with only the documentation (no conversation history) should be able to understand the full architecture, all invariants, and current project state without asking clarifying questions.

---

*Voidrift Documentation Refactor Directive | April 2026 | RFD IT Services Ltd.*  
*The code is what the system does. The documentation is what the next agent understands.*
