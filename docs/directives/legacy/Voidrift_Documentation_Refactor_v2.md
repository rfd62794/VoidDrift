# Voidrift — Documentation Refactor Directive v2
**Status:** Approved — Ready for Execution  
**Type:** Documentation Only — Zero Code Changes  
**Date:** April 2026  
**Supersedes:** Documentation Refactor Directive v1 (now obsolete — design vision has changed)

---

## 1. Why This Directive Exists

The v1 documentation directive was written to document the current codebase accurately. That work is still needed. But the design vision discussed in the April 2026 session has fundamentally changed the economy, resource system, department naming, and long-term direction of the game.

Documentation now needs two layers:

**Layer 1 — Current State:** What the code actually does right now. Accurate, technical, for the coding agent. The foundation for the Bevy UI migration and code refactor directives.

**Layer 2 — Design Canon:** Where the game is going. The full vision locked in conversation. The north star for the economy redesign and all future directives.

Both layers are required. Neither is optional.

---

## 2. Strict Rules

- **Zero code changes.** No file in `src/` is touched.
- **Layer 1 must be read from actual code.** Not from memory, not from conversation history. Read `src/` then write.
- **Layer 2 must be written from this directive.** It documents design decisions made in conversation that are not yet in the codebase.
- **Conflicts between layers are expected and correct.** Layer 1 says what exists. Layer 2 says what replaces it. The conflict is the work to be done.

---

## 3. Layer 1 — Current State Documents

### 3.1 `docs/state/current.md` — OVERWRITE

Read the entire `src/` directory before writing. Document what is actually there.

Required sections:

```markdown
# Voidrift — Current State
**Date:** April 2026
**Build:** v0.4.x

## System Inventory
[Every system function, file location, one-sentence purpose]

## Component Inventory  
[Every Component and Resource in components.rs, one-sentence purpose]

## Known Technical Issues
[CargoBarFill/ShipCargoBarFill duplicate, any other flags from structural audit]

## Current Economy (as implemented)
[Power Cells as repair resource, instant batch → queue system, current dept names]

## Current Quest Chain (as implemented)
[Q-001 through Q-007 as they exist in code]

## Current Signal Triggers (as implemented)
[S-001 through S-021+ as they exist in narrative.rs]

## Current Department Structure (as implemented)
[RESERVES, POWER, SMELTER, FORGE, SHIP PORT — what each does now]

## Open Directives (approved but not executed)
[Bevy UI Migration + Code Refactor, Economy Redesign, Three-Resource System]
```

### 3.2 `docs/ARCHITECTURE.md` — OVERWRITE

Read from code. Document actual architecture.

Required sections:
- Technology stack (Bevy 0.15, egui 0.31, bevy_egui 0.33, cargo-ndk, NDK r29, API 35)
- Module structure (every file in src/systems/)
- System execution order (two groups, tuple partition constraint)
- ECS constraints (Universal Disjointness, DockedAt pattern)
- Entity hierarchies (station, ship, asteroid)
- Key data flows (Signal, Tutorial, Quest, Processing, Docking)
- Rendering architecture (Z-layer table, AlphaMode2d::Opaque rationale)
- Input architecture (touch, pinch zoom, map pan)

### 3.3 `docs/AGENT_CONTRACT.md` — APPEND

Add invariants INV-004 through INV-008 as specified in v1 directive. These are still valid and needed regardless of design changes.

### 3.4 `docs/adr/ADR-007.md`, `ADR-008.md`, `ADR-009.md` — CREATE

As specified in v1 directive. Still valid.

---

## 4. Layer 2 — Design Canon Documents

These documents describe the intended future state. They do not describe current code. They are written from this directive, not from the codebase.

### 4.1 `docs/design/VISION.md` — CREATE

The full game vision locked April 2026.

```markdown
# Voidrift — Design Vision
**Locked:** April 2026  
**Status:** Canonical — supersedes all previous vision documents

## Identity
Voidrift is a space station management and fleet command game for mobile.
The player is a Fleet Commander and Station Manager, not a pilot.
The ship is a tool of command. The station is home base.
The galaxy is what you build gate by gate through permanent choices.

## Core Fantasy
Survivor → Station Founder → Fleet Commander → Galaxy Builder

## Design Pillars

### 1. Station Management (drawn from SS13 Idle)
The station has genuine department depth. Each department has inputs,
outputs, upgrade paths, and bottlenecks. Systems are interdependent.
Engineering feeds Fabrication. Mining feeds both.
The station runs itself once configured. The player makes strategic
decisions, not routine ones.

### 2. Fleet Command (drawn from Vanguard Galaxy)
Autonomous ships are strategic assets, not just drones on fixed routes.
Base Speed + Fuel Boost model. Engine tiers are permanent upgrades.
The fleet extends the player's reach beyond the station.

### 3. Exploration and Discovery
The galaxy is not pre-built. It is assembled gate by gate.
Each Stargate activation opens one new system permanently.
Discovery funds station depth — new resources unlock new departments.

### 4. Planetary Layer
Orbital stations above planetary bodies.
Player docks at the orbital station — never lands on the surface.
Each planet produces a unique resource unavailable in the solar system.
The orbital station is a specialized trading post.
Visually: large dim background circle, orbital station rotating around it.

## Pacing Model
Early game (0-30 min): Station management is mostly manual. Small galaxy.
Mid game: Automation handles routine ops. Galaxy expands through first gate.
Late game: Station runs itself. Player is explorer and commander.
The station grows through what the fleet discovers, not time spent at station.
```

### 4.2 `docs/design/ECONOMY.md` — CREATE

The full economy redesign locked April 2026.

```markdown
# Voidrift — Economy Design Canon
**Locked:** April 2026

## Three Resource Tracks

All resources belong to one of three tracks. Each track is self-sufficient.
Combining tracks unlocks composites. All three combined unlocks the Void Core.

### Metal Track
Primary ores: Magnetite, Iron, Carbon, Tungsten, Titanite
Each ore refines to its own ingot in the FORGE.

| Ore | Ingot | Notes |
|-----|-------|-------|
| Magnetite | Magnetite Ingot | Power pathway, Repair Kits |
| Iron | Iron Ingot | Core structural material |
| Carbon | Carbon Rod | Lightweight structures |
| Tungsten | Tungsten Bar | Advanced tooling, mid-game |
| Titanite | Titanite Ingot | Advanced lightweight, mid-game |

### Gas Track
Primary resource: Helium
Passive secondary yield from all asteroid mining (~2 per 100 ore).
Never mined directly — always a byproduct.

### Crystal Track
Primary resource: Crystal Core (S6, Composite Laser required)
Rare surface trace (0.1 rate), concentrated in asteroid cores.
Mid-to-late game material.

---

## Department Structure (Renamed)

### FORGE — Ore to Ingots
Replaces current SMELTER.
Each ore type has its own queue. Five parallel Forge queues at full build.

| Input | Output | Time |
|-------|--------|------|
| Magnetite × 10 | Magnetite Ingot × 1 | 12s |
| Iron × 8 | Iron Ingot × 1 | 12s |
| Carbon × 6 | Carbon Rod × 1 | 15s |
| Tungsten × 5 | Tungsten Bar × 1 | 20s |
| Titanite × 5 | Titanite Ingot × 1 | 20s |
| Crystal × 3 | Crystal Matrix × 1 | 30s |

### CRAFTER — Components and Composites
Replaces current FORGE.
Ingots and components become usable items.

#### Basic Components (early game)
| Input | Output | Time |
|-------|--------|------|
| Iron Ingot × 3 | Iron Plate × 1 | 18s |
| Carbon Rod × 4 | Carbon Tube × 1 | 18s |
| Iron Plate × 2 + Magnetite Ingot × 3 | Repair Kit × 1 | 15s |
| Carbon Tube × 3 + Iron Plate × 2 | Space Frame × 1 | 25s |

#### Fuel System (Metal + Gas track)
| Input | Output | Time |
|-------|--------|------|
| Iron Plate × 2 | Fuel Tank × 1 | 20s |
| Fuel Tank × 1 + Helium × 5 | Fuel Cell × 1 | 20s |

#### Ship Components
| Input | Output | Time |
|-------|--------|------|
| Iron Plate × 3 | Hull Plate × 1 | 25s |
| Hull Plate × 3 | Ship Hull × 1 | 35s |

#### Engine Tiers
| Input | Output | Base Speed |
|-------|--------|-----------|
| Starting | Engine Mk I | 180.0 |
| Iron Plate × 5 + Carbon Tube × 3 | Engine Mk II | 240.0 |
| Tungsten Bar × 3 + Space Frame × 2 | Engine Mk III | 310.0 |
| Charged Plate × 2 + Titanite Ingot × 4 | Engine Mk IV | 400.0 |

#### Power System (Crystal track — mid game)
| Input | Output | Time |
|-------|--------|------|
| Crystal Matrix × 5 + Iron Plate × 2 | Power Cell × 1 | 30s |

#### Two-Material Composites
| Input | Output | Purpose |
|-------|--------|---------|
| Iron Plate × 2 + Helium × 3 | Pressurized Hull × 1 | Better ship hull |
| Crystal Matrix × 2 + Iron Plate × 3 | Charged Plate × 1 | Engine Mk IV, Void Core |
| Fuel Cell × 1 + Crystal Matrix × 2 | Plasma Cell × 1 | Premium fuel |

#### Late Game — AI Core
| Input | Output | Time |
|-------|--------|------|
| Power Cell × 10 + Space Frame × 2 | AI Core × 1 | 60s |

#### The MacGuffin — Void Core
| Input | Output | Purpose |
|-------|--------|---------|
| Space Frame × 3 + Plasma Cell × 2 + Charged Plate × 2 | Void Core × 1 | Stargate activation |

---

## Ship Propulsion Model

### Base Speed
Determined by Engine tier. Permanent upgrade. No fuel cost.
Current constant SHIP_SPEED = 180.0 becomes Engine Mk I base.

### Fuel Boost
Optional. Consumes one Fuel Cell or Plasma Cell.
Duration: 8 seconds (Fuel Cell), 14 seconds (Plasma Cell)
Multiplier: ×1.8 (Fuel Cell), ×2.4 (Plasma Cell)
Player decision — never required, always advantageous.

---

## Station Repair

Opening quest target: craft 5 Repair Kits.
Repair Kit replaces Power Cells as the repair resource.
Power Cells are mid-game Crystal-track items — not for early repair.

Quest Q-003: Craft 5 Repair Kits to restore station systems.

---

## Opening Quest Chain (Revised)

| Quest | Objective | System Taught |
|-------|-----------|--------------|
| Q-001 | Locate the signal | Navigation |
| Q-002 | Dock at the derelict station | Docking |
| Q-003 | Craft 5 Repair Kits | FORGE + CRAFTER chain |
| Q-004 | Repair the station | RESERVES tab |
| Q-005 | Mine Helium (passive yield) | Gas track awareness |
| Q-006 | Craft 3 Fuel Cells | Gas track production |
| Q-007 | Build Engine Mk II | Ship Port, first upgrade |
| Q-008 | Discover Sector 4 | Laser gate, exploration |
| Q-009 | Build AI Core | Crystal track preview |
| Q-010 | Assemble autonomous ship | Fleet Commander identity |

---

## Resource Hierarchy

| Tier | Resources | Unlocks |
|------|-----------|---------|
| Early | Magnetite, Iron, Carbon, Helium | Repair, basic ships, fuel |
| Mid | Tungsten, Titanite, Crystal | Advanced ships, engines, power |
| Late | Crystal composites | Plasma Cells, Void Core |
| Endgame | Void Core | Stargate activation |
```

### 4.3 `docs/design/STARGATE.md` — CREATE

```markdown
# Voidrift — Stargate & Galaxy Design
**Locked:** April 2026

## The Stargate

A Precursor artifact present in the solar system from game start.
Visible. Inert. The Signal acknowledges it early but cannot explain it.
Visually distinct from all other structures — geometric, alien, dark,
occasional power flickers suggesting it is not completely dead.

The Stargate is not built by the player. It is found and restored.
Activation requires one Void Core — the three-material MacGuffin.
Building a Void Core proves mastery of all three resource tracks.

## Activation Sequence

1. Void Core crafted
2. Signal: STARGATE POWER THRESHOLD MET. DESTINATION LOCK REQUIRED.
3. Multiple signals detected simultaneously — player sees partial info
4. Player selects one signal — gate locks to that destination permanently
5. Others are gone for this gate. New signals appear for the next gate.

## Signal Selection (Blue Prince influence)

Each signal shows partial information only:
- Signal strength (strong/moderate/weak)
- Partial resource signature
- Anomaly indicators (Precursor tech / biological / high density)
- Whether another gate is detected at destination

Player chooses with incomplete information. The choice is permanent.
No two players have the same galaxy — each signal choice shapes
what systems exist in that player's universe.

## Procedural System Generation

Each gate destination contains:
- 2-6 asteroid fields (ore types weighted by system class)
- 0-3 orbital stations with unique resources
- 0-2 derelict structures (Precursor or unknown)
- 0-1 dormant Stargate pointing further out
- Unique Signal flavor text from system characteristics

## Orbital Stations

Each orbital station is associated with a planetary body.
The planet is a large background circle — visual only, never landed on.
The orbital station orbits the planet at a fixed radius.
Player docks at the orbital station using the existing Berth system.

Each planet type produces one unique resource unavailable in the solar system.
That resource unlocks one new CRAFTER recipe and one new station department.

| Planet Type | Visual | Unique Resource | Unlocks |
|-------------|--------|----------------|---------|
| Volcanic moon | Dark red | Thermal Compound | Heat Forge |
| Ice giant | Pale blue | Cryo Crystal | Cooling systems |
| Organic world | Green-brown | Bio Extract | Chemistry dept |
| Dense metallic | Grey-silver | Refined Iridium | Advanced hulls |
| Gas giant platform | Amber | Atmospheric Gas | Thruster upgrades |
| Ancient derelict | Dark purple | Precursor Tech | Research dept |

## The Personal Galaxy Map

Every visited system is permanently on the player's map.
The map is a record of every gate choice ever made.
It cannot be reset without starting over.
The galaxy is a personal artifact.

## MVP Scope for Stargate

The full procedural system is not required to ship.
MVP: one hand-crafted destination through the first gate.
The Stargate exists as a world object from day one.
Full procedural generation is v2.0.
```

### 4.4 `docs/design/UI_VISION.md` — CREATE

```markdown
# Voidrift — UI Architecture Vision
**Locked:** April 2026

## Framework Migration

Current: bevy_egui (egui 0.31) — causes alignment drift on mobile
Target: Bevy UI (native, flexbox layout, percentage-based sizing)

Migration approach: panel by panel replacement, not big bang.
egui and Bevy UI coexist during transition.
Remove egui entirely when all panels migrated.

## Portrait Layout (Primary — Moto G 2025, 720×1604)

World view: 100% width, flex fills remaining space after bottom panel
Bottom panel: 100% width, ~45% of screen height when docked
  - Left nav: 30% width — MAP, QUEST, separator, station tabs
  - Context panel: 70% width — tab content
Signal strip: 100% width, 64px height, always visible

## Landscape Layout (Secondary — onn tablet, 1200×2000)

Left nav: 20% width, full height minus signal
World view: 60% width, full height minus signal  
Context panel: 20% width, full height minus signal
Signal strip: 100% width, 64px height

## Orientation Detection

Single check at runtime: window.physical_width() > window.physical_height()
Same components, different Style values. No separate code paths.

## Context Panel States

When not docked: shows minimal ship status (power, cargo, engine)
When docked: shows active station tab content
Quest panel: overlays context panel when QUEST tapped
Map view: overlays world view when MAP tapped

## Touch Targets

All interactive elements: minimum 44px height
Pointer<Click> observers (not deprecated Interaction component)
Single finger: tap to navigate (space view), pan (map view)
Two finger: pinch zoom (both views), suppresses single touch

## Signal Strip

Always visible. Always at bottom. Never hidden.
Collapsed: 3 lines, non-interactive except tap to expand
Expanded: 20 lines scrollable, tap outside to collapse
Terminal green #00CC66, monospace font, > prefix on each line
```

---

## 5. Phase Summaries

Create or update these in `docs/phases/`:

### 5.1 Phases 07-10

Create summaries as specified in v1 directive. These document
what was built and are still needed for continuity.

### 5.2 `docs/phases/phase-economy-redesign-planned.md` — CREATE

Document the planned economy changes as a forward-looking phase summary:

```markdown
# Phase: Economy Redesign — Planned
**Status:** Designed, not yet implemented
**Depends on:** Bevy UI Migration + Code Refactor complete

## What Changes
- SMELTER renamed to FORGE (ore to ingots)
- FORGE renamed to CRAFTER (ingots to components)
- Power Cells moved to Crystal-tier (mid-game resource)
- Repair Kits added as early-game repair resource
- Helium added as passive secondary yield from all mining
- Engine tiers added (Mk I through Mk V, permanent upgrades)
- Fuel Boost added (optional consumable speed burst)
- Void Core added as three-material MacGuffin
- Stargate added as Precursor artifact requiring Void Core to activate

## Quest Chain Changes
Q-003 target changes from "collect 25 Power Cells" to "craft 5 Repair Kits"
Q-005 through Q-010 revised to teach resource tracks sequentially

## Signal Changes Required
S-007 through S-012 need updating for new repair chain
New Signal triggers for Helium detection, fuel boost, engine upgrade

## See Also
docs/design/ECONOMY.md — full economy specification
docs/design/STARGATE.md — Stargate and galaxy design
```

---

## 6. CHANGELOG Updates

Append to `docs/CHANGELOG.md`:

```markdown
## [Economy Redesign] — Designed April 2026 — NOT YET IMPLEMENTED
### Designed
- Three resource tracks: Metal, Gas (Helium), Crystal
- FORGE department (replaces SMELTER) — ore to ingots
- CRAFTER department (replaces FORGE) — ingots to composites  
- Repair Kit as opening repair resource (replaces Power Cells)
- Engine tier system (Mk I-V, permanent upgrades)
- Fuel Boost system (optional consumable speed burst)
- Void Core as three-material MacGuffin
- Stargate as Precursor artifact and galaxy builder
- Orbital Station + planetary body visual system
- Revised 10-objective quest chain

## [Bevy UI Migration] — Planned
### Planned
- Replace bevy_egui panels with native Bevy UI nodes
- Percentage-based layout for portrait and landscape
- Pointer<Click> observers replacing Interaction component
- Portrait primary (720×1604), landscape secondary (1200×2000)
```

---

## 7. Implementation Sequence

1. Read entire `src/` directory — top to bottom, no skipping
2. Write `current.md` from code reading
3. Write `ARCHITECTURE.md` from code reading
4. Append invariants to `AGENT_CONTRACT.md`
5. Write ADR-007, ADR-008, ADR-009
6. Write phase summaries 07-10 from walkthrough docs and git log
7. Write `VISION.md` from this directive
8. Write `ECONOMY.md` from this directive
9. Write `STARGATE.md` from this directive
10. Write `UI_VISION.md` from this directive
11. Write `phase-economy-redesign-planned.md` from this directive
12. Update `CHANGELOG.md`

Steps 1-6 are Layer 1 — read from code.
Steps 7-12 are Layer 2 — write from this directive.

Do not mix the layers. Do not write Layer 2 documents from code.
Do not write Layer 1 documents from this directive.

---

## 8. Completion Criteria

**Layer 1:**
- [ ] `current.md` reflects actual current system state from code reading
- [ ] `ARCHITECTURE.md` covers all systems including post-Phase B additions
- [ ] `AGENT_CONTRACT.md` has INV-004 through INV-008
- [ ] ADR-007, ADR-008, ADR-009 created
- [ ] Phase summaries 07-10 created

**Layer 2:**
- [ ] `VISION.md` created with full game identity and pillars
- [ ] `ECONOMY.md` created with complete resource/department/recipe tables
- [ ] `STARGATE.md` created with galaxy design and MVP scope
- [ ] `UI_VISION.md` created with layout specs and migration plan
- [ ] `phase-economy-redesign-planned.md` created
- [ ] `CHANGELOG.md` updated with planned phases

**Gate:**
A new agent session reading only the docs/ directory — no conversation
history, no src/ access — should be able to answer:
1. What does the game do right now?
2. What is the economy being redesigned to?
3. What are the architectural invariants that must never be violated?
4. What is the next directive to execute?

If all four are answerable from docs/ alone, the documentation is complete.

---

*Voidrift Documentation Refactor Directive v2 | April 2026 | RFD IT Services Ltd.*  
*Layer 1: what the code does. Layer 2: what replaces it. Both are true simultaneously.*
