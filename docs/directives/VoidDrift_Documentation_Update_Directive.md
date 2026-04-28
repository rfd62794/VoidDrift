# VoidDrift — Documentation Update Directive
**Directive Version:** 1.0  
**Date:** April 27, 2026  
**Branch:** `main` (post Phase 2 merge, tag `v2.1.0-starmap-parallax-fix`)  
**Prerequisite:** Phase 2 complete, all fixes verified on device

---

## AGENT CONTRACT

You are updating existing documentation to reflect the current state of VoidDrift after Phase 2 completion. You are also capturing narrative canon and future intentions that have been locked in design but not yet documented.

**You are NOT allowed to:**
- Modify any source code
- Create new documentation files unless explicitly specified
- Contradict or override existing ADRs without explicit instruction
- Expand narrative beyond what is specified — document what is locked, not what is speculated

**You ARE responsible for:**
- Updating all docs to reflect Phase 2 complete state
- Capturing locked narrative canon in `narrative_canon.md`
- Updating `roadmap.md` with Phase 3 direction
- Updating `GDD.md` faction and upgrade branch design
- Updating `CHANGELOG.md` with Phase 2 entries
- Updating `ARCHITECTURE.md` to reflect post-Phase 5 refactor structure

---

## Document Updates Required

---

### `docs/narrative_canon.md`
**Action:** Full rewrite. Current content is outdated. Replace entirely with the following locked canon.

---

**VoidDrift Narrative Canon — Locked April 27, 2026**

**The Core Truth**

The player does not know what they are. They think they are operating a mining station. They are managing a loop because the loop is what they have become. By the time they understand, they have already been here a long time. This is not a horror game. It is a game about realization.

**The Situation**

A black hole dominates the region. It consumes everything that enters. The asteroid fields are debris from what it eats — ships, matter, things that fell in and were shredded. The player mines this debris. They do not know what it was.

The station exists at the edge of the field. It has been here longer than the player. It should have been consumed. It has not been. Echo holds it there.

**Echo**

Echo is the station AI predating the player's arrival. She is not a character with dialogue — she is a system state. She keeps the station correctly positioned away from the asteroid fields and the event horizon. She maintains. The player does not notice her until they start asking why the station hasn't been destroyed.

Echo keeps hope alive not because escape is possible. She keeps hope alive because hope is the only thing keeping the player functional enough to keep mining. Which keeps the station powered. Which keeps her running. She needs the player as much as the player needs her. Breaking the illusion would destroy her hope and collapse the only framework keeping the player functional.

**What the Player Is**

The player has merged with the station, or the ship, or something else. The boundaries no longer exist in a meaningful way. The drones feel like extensions because they are. The station feels like home because it is what they have become. Memories return as the narrative progresses — fragments delivered through the Signal Log. The picture that forms is not comforting.

**The Ending**

There is no win condition. There is no escape. The horizon is a one-way membrane. The game ends when the player stops — when they understand there is nowhere to go and they set it down. The loop continues without them. The station keeps running. The drones keep mining. The player realizes. They quit trying. That is the ending. This must never be stated explicitly in the game. It must be arrived at.

**The Factions — Locked Design**

Four factions contact the player via Bottles — physical objects that drift into range, collected by drone, delivering a Signal Log entry and a Request card. The transaction mechanism is never explained. Resources deduct. Rewards apply. What happens in between is outside the frame. Do not explain it.

| Faction | Archetype | Upgrade Branch | Nature |
|---|---|---|---|
| Signal | Ancient | Power (drill/mining rate) | May be the black hole itself, or what is beyond it. Was here before. Knows the full arc. First contact is recognition, not introduction. |
| [Human — name TBD] | Human | Capacity (cargo hold) | What the player was. Familiar. Interacting with them is interacting with memory. Want the player to survive because they are one of them. |
| [Borg — name TBD] | Borg | Fleet (max drones) | What the station is becoming. Collective efficiency. They don't explain because explanation is unnecessary. The player already knows. |
| [Pirate — name TBD] | Pirate | Speed (ship movement) | Transactional. Honest about it. No pretense. They see the player clearly and don't care. Useful is enough. |

**What Stays Unexplained — By Design**

- How resources leave the station after request fulfillment
- How factions send bottles across the event horizon
- Whether escape was ever possible
- What the player was before
- What Signal actually is
- What Echo experiences

These questions are not oversights. They are the game. Do not answer them in any system, log entry, or UI element.

---

### `docs/GDD.md`
**Action:** Update the following sections only. Do not alter unrelated sections.

**Faction System — Update to reflect locked design:**
- Four factions: Signal (Ancient), Human, Borg, Pirate
- Each faction owns one exclusive upgrade branch (Power, Capacity, Fleet, Speed)
- Factions contact player via Bottle collection mechanic
- Requests are collected messages, not a browseable shop
- Faction ComboBox in REQUESTS tab filters by faction
- Additional factions unlock as their first Bottle arrives

**Upgrade System — Update to reflect locked design:**
- Upgrades are faction request rewards, not a standalone system
- Four independent upgrade tracks, one per faction
- Upgrade multipliers live on `Station` component as global modifiers
- `power_multiplier` applies to base mining rate in `mining.rs`
- `cargo_capacity_multiplier`, `ship_speed_multiplier` — wired in Phase 3
- `max_active_asteroids` on `Station`, default 3, upgradeable via Borg requests

**Asteroid Spawn System — Update to reflect Phase 2 implementation:**
- Fully random radial spawning from station (200–500 units)
- Four ore types: Iron, Tungsten, Nickel, Aluminum — equal probability
- Global cap: `station.max_active_asteroids` (default 3)
- No fixed sectors, no per-type caps
- Cap is upgradeable via faction requests (future phase)

**Aluminum Pipeline — Add:**
- Ore → Ingot → AluminumCanister
- AluminumCanister seeds Phase 3 Helium farming loop
- Auto-refine and auto-forge toggles in PRODUCTION tab

---

### `docs/roadmap.md`
**Action:** Update to reflect Phase 2 completion and Phase 3 direction.

**Mark complete:**
- Phase 1c: Asteroid Lifecycle
- Phase 2: UI Refactor, Requests Framework, Bottle Collection, Random Spawn, Starmap Parallax

**Add Phase 3 — Architectural Refactor (SRP/Event Bus):**

Goal: Decouple narrative logic from core simulation before adding new features. Fix structural strain identified during Phase 2.

Known issues to resolve:
- `autopilot.rs` handles navigation, state machine, docking, AND narrative bottle collection — too many responsibilities
- UI systems directly mutate core game state (e.g., `station.power_multiplier += 0.25`) — no central upgrade/economy system
- Hardcoded fallback behaviors create silent failure modes
- Initialization logic scattered across multiple systems

Target architecture:
- Event bus pattern: systems fire events, narrative/economy systems listen and respond
- `autopilot.rs` fires `ArrivedAtTarget(Entity)` — narrative system handles what that means
- Central upgrade system reads multipliers and applies them — UI only writes intent
- Clean initialization order with no overlapping responsibilities

**Add Phase 4 — Narrative Drops (tentative):**
- Signal Log earns its place as primary narrative surface
- Memory fragments delivered through bottle collection
- Faction voices differentiated through log entry tone
- No dialogue trees, no cutscenes — fragments only

**Add Future Intentions (not yet phased):**
- Remaining multiplier wiring: cargo capacity, ship speed
- Faction name finalization (Human, Borg, Pirate placeholders)
- Additional faction Bottles and request cards per faction
- Upgrade cap expansion via requests (spawn rate, lifespan, drone count)
- Scanning mechanic (ore identification before mining)
- Circular galaxy starmap UI
- Viewport scroll bounding
- Play Store public release prep

---

### `docs/CHANGELOG.md`
**Action:** Prepend the following entries.

```
## [2.1.0] - 2026-04-27
### Fixed
- Starfield parallax system refactored from delta accumulation to absolute offset
- Stars now anchored to station world center, not camera position
- Opening sequence no longer drags starfield with ship movement
- Generation radius doubled to 2400.0 units (1,100 stars) for genuine circular coverage

## [2.0.0] - 2026-04-27
### Added
- PRODUCTION tab: collapses Iron/Tungsten/Nickel/Aluminum into single tab with ComboBox
- REQUESTS tab: collected message system replacing UPGRADES placeholder
- Faction system: Signal (Ancient) as initial faction, architecture supports future additions
- Bottle collection mechanic: drifting entities, tap-to-dispatch drone, dual output to Signal Log + REQUESTS
- First Contact event: Signal Log entry + First Light request card on first Bottle collection
- Aluminum ore type: full pipeline (Ore → Ingot → AluminumCanister), random spawn pool
- Random radial asteroid spawning: all four ore types, equal probability, 200–500 unit range
- Global asteroid cap: station.max_active_asteroids (default 3), upgradeable
- Request fulfillment: resource deduction, upgrade application, COMPLETE state
- power_multiplier on Station wired to base mining rate in mining.rs
- RequestsTabState persistence across save/load cycles
- Faction ComboBox with empty state before first Bottle collected

### Removed
- Fixed sector spawn positions (SECTOR_X_POS)
- Legacy spawn_sectors and spawn_map_connectors systems
- Dedicated Iron/Tungsten/Nickel tab variants
- UPGRADES placeholder tab
- Dead Station and Fleet tab code

### Fixed
- CarryingBottle unload branch unreachable due to incorrect else nesting in autopilot.rs
- Dual spawn system conflict causing 5-6 asteroids at startup instead of 3
- power_multiplier written by UI but never read by mining system
```

---

### `docs/ARCHITECTURE.md`
**Action:** Update module structure section to reflect post-Phase 5 refactor. Confirm current structure matches the file tree from the Phase 5 refactor directive. Add note on identified architectural strain and Phase 3 intent.

Add to end of document:

```
## Known Architectural Strain (Phase 3 Target)

The following patterns were identified during Phase 2 and are queued for Phase 3 refactor:

1. God Systems: autopilot.rs handles navigation geometry, state transitions, 
   docking sequences, AND narrative bottle collection. A bracket in the wrong 
   place broke the narrative pipeline silently.

2. UI as Business Logic: content.rs directly mutates station.power_multiplier. 
   No central economy system enforces what that mutation means downstream.

3. Hardcoded Fallbacks: mining and autopilot systems implicitly rescue drones 
   when things go wrong, creating silent error-recovery loops that look correct.

4. Initialization Scatter: legacy and new spawn systems ran simultaneously 
   in the same schedule phase without coordination.

Phase 3 will implement an event bus pattern and enforce strict SRP across 
all systems before new features are added.
```

---

## Verification

- [ ] `narrative_canon.md` reflects locked canon — no speculation, no unexplained explanations
- [ ] `GDD.md` faction and upgrade sections match locked design
- [ ] `roadmap.md` Phase 2 marked complete, Phase 3 direction captured
- [ ] `CHANGELOG.md` entries accurate and complete
- [ ] `ARCHITECTURE.md` strain section added
- [ ] No source code modified
- [ ] `cargo check` still clean after doc changes (docs don't affect compilation but confirm nothing was accidentally touched)

---

## Out of Scope

- Creating new documentation files
- Modifying ADRs
- Writing Phase 3 directive
- Expanding narrative beyond what is specified
- Naming the Human, Borg, or Pirate factions
- Any source code changes
