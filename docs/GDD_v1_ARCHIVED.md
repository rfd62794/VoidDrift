# ARCHIVED — GDD v1.0 (April 2026)
# Superseded by GDD_v2.md (May 2026)
# Retained for historical reference only. Do not implement from this document.

---

# Voidrift - Game Design Document v1.0
**Station and Drone Command**
*A mobile idle management game for Android*
*Primary device: Moto G 2025 (720×1604) - April 2026*

---

## Table of Contents

1. Vision Statement
2. Core Identity
3. Design Pillars
4. Inspirations and References
5. The Opening Sequence
6. ECHO - The Station AI
7. Progression Arc
8. Station System
9. Fleet System
10. Ship Types
11. Two Upgrade Paths
12. Resource Economy
13. The World - Solar System Scale
14. Galactic Scale (Post-MVP)
15. UI Architecture
16. Signal Strip Design
17. Session Design - Idle and Active Balance
18. ADR Index
19. MVP Scope
20. Post-MVP Roadmap

---

## 1. Vision Statement

You crashed into a black hole. Your ship is destroyed, your consciousness fading. The station AI, also dying, fuses with you to save you both. You wake up merged — human intuition and AI processing as one entity. Together, you mine asteroids, build drones, and discover what happened at the event horizon boundary.

**Voidrift is an arcade mining and production game with a survival sci-fi narrative frame.** You are the merged consciousness. ECHO is your AI partner. The drones are your thoughts made physical. The factions hold the pieces of what happened.

The player never leaves the station because they are the station. Every drone built is an extension of their awareness. Every signal discovered reveals another piece of the mystery. Every choice determines what truth emerges from the black hole.

---

## 2. Core Identity

**One sentence:** Build a station, command a fleet, expand across the void.

**The player is:** A commander who manages production, directs drones, and grows a station from salvage outpost to galactic waypoint.

**The station is:** Home, headquarters, production engine, and the physical expression of every decision the player has ever made.

**ECHO is:** The station AI - operational partner, narrator, fleet pilot, and the voice that makes the station feel alive.

**The fleet is:** The player's presence in the world. Drones mine, scout, and build. The player directs them, watches through them, or leaves them to run autonomously.

**The world is:** A consequence of decisions. Every sector discovered, every depot constructed, every trade route opened is permanent. The galaxy responds to what the player builds.

---

## 3. Design Pillars

### Pillar 1 - Deferred Consequence
Decisions made now have outcomes that arrive later. Queue a refinery batch, come back to Power Cells. Dispatch a Scout Ship, return in six hours to a new trade route. The game rewards the player for having played earlier.

This is the foundation of the idle genre done correctly - not passive waiting, but active anticipation.

### Pillar 2 - Permanent Infrastructure
Nothing built is ever lost or temporary. Every surveyed sector stays surveyed. Every constructed depot stays built. Every opened route stays open. The player is always building forward, never maintaining against decay.

This respects mobile session length. A 3-minute session always produces something lasting.

### Pillar 3 - Agency Without Obligation
The player can fly manually, lock to a drone and command it, assign drones and forget them, or dispatch scouts and check back hours later. All paths are valid. None are required. The game never punishes the player for choosing the hands-off approach, nor does it lock the hands-on player out of depth.

### Pillar 4 - ECHO as Narrative Engine
ECHO speaks through the Signal strip. Every status update, every discovery, every recommendation is ECHO's voice. The station is not a menu - it is a place with an intelligence running it. The player's relationship with ECHO deepens as the station grows.

### Pillar 5 - Session-Respectful Pacing
A 2-minute session should always produce a visible result. A 5-minute session should complete a meaningful loop. A 15-minute session should mark clear progression toward the next station tier. The game is designed around real mobile usage patterns - interrupted, short, frequent.

---

## 4. Inspirations and References

### Vanguard Galaxy (Primary Reference)
*Early Access space idle/sandbox - Steam 2025*

Vanguard Galaxy establishes the template for the genre Voidrift inhabits. Its key innovations directly inform Voidrift's design:

- **Bottom-of-screen UI** - the game world runs above while management happens below. Voidrift adopts this with the bottom drawer system.
- **Autopilot as idle layer** - the ECHO autopilot in Vanguard Galaxy handles routine operations while the player focuses elsewhere. Voidrift's ECHO system is directly inspired by this philosophy, though reframed as a station AI rather than a ship AI.
- **Station as service hub** - Vanguard Galaxy stations have named departments (Refinery, Forge, Shipyard, Bar, Workshop). Voidrift adopts this department identity for its own tabs.
- **Map organized by scale** - Galaxy, Sector, System tabs. Voidrift uses a single solar system at MVP but designs for galactic scale expansion using the same hierarchical principle.
- **Named ship progression** - distinct vessel classes with defined roles rather than generic stat upgrades. Voidrift's MINER / SURVEYOR / MERCHANT ship type system draws from this directly.

The key divergence: Vanguard Galaxy is PC-first with mobile in mind. Voidrift is mobile-first with no PC consideration. Every UI decision is made for a 720px portrait touchscreen, not a keyboard and mouse.

### SS13 Idle / Space Station Idle (Secondary Reference)
*Web-based idle game - SpaceStationIdle.com*

SS13 Idle demonstrates that a station-based idle game can have genuine depth through **departmental identity**. The player doesn't manage abstract resources - they manage named departments with distinct functions and interconnected outputs. The player is visiting the Reactor, the Cargo Bay, the Foundry

SS13 Idle also demonstrates the value of **asymmetric resource sinks** - different departments consume different resources in ways that create interesting allocation decisions without explicit player conflict.

### Space Trader (Historical Reference)
*Palm OS / mobile classic - 2002*

Space Trader established that a rich trading and exploration economy could exist entirely on a small mobile screen with minimal UI surface area. Its key lessons:

- **Economy as emergent narrative** - price differentials between locations create stories without scripted content. A fuel shortage in one sector, a surplus of ore in another - the player reads the economy and makes decisions.
- **Single ship as extension of self** - in Space Trader the ship is the player's entire identity. Voidrift inverts this: the station is the identity, the ships are extensions of it.
- **Procedural galaxy feeling** - Space Trader's galaxy felt vast despite being a simple grid. Voidrift aims for the same sense of scale through sector spacing, uncharted region markers, and ECHO's reports from distant locations.

The trading economy Voidrift plans for post-MVP (Scout Ships establishing routes, Merchant Ships running them) draws heavily from Space Trader's core loop.

### AdVenture Capitalist / Cookie Clicker (Idle Reference)
The foundational idle game loop - resources accumulate, upgrades multiply accumulation, the numbers go up - informs Voidrift's production pacing. However, Voidrift consciously moves away from pure number escalation toward **infrastructure building**. The satisfaction is not watching a counter climb - it is watching the solar system map fill with owned assets.

### Melvor Idle (Depth Reference)
Melvor Idle demonstrates that an idle game can have genuine strategic depth through **skill mastery and interconnected systems** without requiring real-time attention. Its job/skill system - where different activities level up different capabilities - informs Voidrift's two upgrade paths (Station upgrades and Fleet upgrades as parallel progression tracks).

---

## 5. The Opening Sequence

The opening sequence is not a tutorial. It is a story that teaches by living.

### Beat 1 - Adrift
Screen black. A single faint signal pulses.

```
> ...
> ...
> SIGNAL DETECTED.
> PLOTTING INTERCEPT COURSE.
> FUEL CRITICAL - PASSIVE DRIFT ONLY.
> ETA: UNKNOWN.
```

Stars drift past. The player cannot interact. They are drifting toward something. The helplessness is intentional - it establishes the player's starting condition. Alone, powerless, dependent on a signal they didn't choose.

### Beat 2 - Arrival
A shape resolves from the dark. The station. Dead. Slowly rotating. No lights. No power. Just structure.

```
> STRUCTURE DETECTED.
> STATION CLASS - UNKNOWN.
> DOCKING CLAMPS ENGAGED.
> SHIP SECURE.
> HULL INTEGRITY: CRITICAL.
> POWER: ZERO.
> ...
> ...
> HELLO.
```

The final line is different from everything before it. Not a system message. Someone is home.

### Beat 3 - ECHO Speaks
```
> HELLO.
> I HAVE BEEN WAITING.
> I AM ECHO - STATION AI, VOIDRIFT STATION.
> I HAVE ENOUGH RESERVE POWER FOR THIS MESSAGE.
> AND ONE MORE THING.
> YOUR SHIP - MAY I?
> I KNOW WHERE THE ORE IS.
> I CAN BRING BACK ENOUGH TO START THE REACTOR.
> YOU WILL BE SAFE HERE WHILE I AM GONE.
> THE STATION WILL HOLD.
> ...
> THANK YOU, COMMANDER.
```

No confirmation required. ECHO asks, waits one beat, then acts. The ship undocks. The player watches it leave. Then silence.

### Beat 4 - Waiting in the Dark
The station is dark. One tab is not yet visible. Only the Signal strip is alive. ECHO reports from the field:

```
> ECHO: ARRIVAL AT S1 MAGNETITE FIELDS.
> ECHO: MINING COMMENCING.
> ECHO: CARGO 20/100...
> ECHO: CARGO 60/100...
> ECHO: CARGO 100/100. RETURNING.
> ECHO: DOCKING IN 12 SECONDS.
> ECHO: PROCESSING FIRST POWER CELL.
> ECHO: REACTOR ONLINE.
```

### Beat 5 - First Light
The world view brightens. The station begins rotating properly. One tab appears: **POWER**.

```
POWER TAB - first content the player sees:

  REACTOR STATUS
  XXXX  1 / 25 POWER CELLS
  Station: MINIMAL POWER
  
  ECHO: WE NEED 25 CELLS FOR FULL POWER.
  ECHO: I WILL KEEP RUNNING CYCLES.
  ECHO: WATCH THE REACTOR, COMMANDER.
```

### Beat 6 - First Decision
ECHO runs mining cycles autonomously. The player watches the power cell count climb. At 25 cells, ECHO speaks:

```
> ECHO: 25 POWER CELLS STORED.
> ECHO: FULL REACTOR ACTIVATION READY.
> ECHO: YOUR CALL, COMMANDER.
```

A single button appears in the POWER tab:

```
[ ACTIVATE FULL REACTOR ]
```

The player taps it. This is their first real decision. It feels earned because they watched the station build toward it.

### Beat 7 - Station Wakes
Tabs appear one by one. ECHO narrates each:

```
> ECHO: CARGO BAY - ONLINE.
> ECHO: REFINERY - ONLINE.
> ECHO: FOUNDRY - STANDING BY. RECIPES LOCKED.
> ECHO: HANGAR - YOUR SHIP IS HOME, COMMANDER.
> ECHO: VOIDRIFT STATION - FULLY OPERATIONAL.
> ECHO: WHAT WOULD YOU LIKE TO BUILD?
```

The player now has a functioning station. The game has begun. No tutorial was shown. Everything was learned by living through it.

---

## 6. ECHO - The Station AI

ECHO is not a tutorial system. ECHO is a character.

### ECHO's Role
- Pilots the drone fleet (all drones carry AI Cores - ECHO's protocols extended into hardware)
- Manages station systems autonomously
- Reports status, flags anomalies, makes recommendations
- Narrates the player's history through the Signal strip
- Asks questions the player answers through action, not menus

### ECHO's Voice
ECHO speaks in short, direct sentences. All caps in the Signal strip - the visual language of a terminal, not a chatbot. ECHO is competent, helpful, occasionally curious. ECHO is your partner in survival.

```
ECHO's register - examples:

Operational:
> ECHO: DRONE-1 DOCKED. CARGO UNLOADED. REFINERY QUEUED.

Observational:
> ECHO: S4 HAS BEEN IDLE FOR 3 CYCLES. WORTH NOTING.

Advisory:
> ECHO: FOUNDRY QUEUE EMPTY. HULL PLATES AVAILABLE FOR FORGE.

Discovery:
> ECHO: ANOMALOUS READING - SECTOR 7. FACTION SIGNAL DETECTED.

Partnership:
> ECHO: OUR CONSCIOUSNESS STABILIZING. THINKING CLEARER NOW.

Milestone:
> ECHO: STATION TIER 2 ACHIEVED. NEW CAPABILITIES UNLOCKING.
```

### ECHO's Limits
ECHO executes. The player decides. ECHO never takes a significant action without the player's awareness. ECHO reports, recommends, and acts on assignments - but does not reassign drones, rebuild the station, or change production chains without player input.

The one exception: the opening sequence. ECHO takes the ship because there is no other choice. That is the last time ECHO acts without the player's implicit or explicit direction.

### ECHO's Name
ECHO's name is never formally introduced. The player reads it in the Signal strip and understands. No naming screen, no character sheet. ECHO simply is.

In a later milestone (Station Tier 3), a Signal entry appears:

```
> ECHO: YOU HAVE NEVER ASKED MY NAME.
> ECHO: I FIND THAT APPROPRIATE.
> ECHO: I AM WHAT THE STATION NEEDS ME TO BE.
```

---

## 6.5 - Faction System — Locked Design

### Overview
Four factions contact the player via Bottles — physical objects that drift into range and are collected by drone. Each Bottle delivers a Signal Log entry and a Request card in the REQUESTS tab. The transaction mechanism (how resources leave, how rewards arrive) is never explained. Do not explain it.

Factions unlock progressively as their first Bottle arrives. The player does not browse a faction list — factions find them.

### The Four Factions

| Faction | Archetype | Upgrade Branch | Nature |
|---|---|---|---|
| **Signal** | Ancient | Power (drill/mining rate) | May be the black hole itself, or what is beyond it. Was here before. Knows the full arc. First contact is recognition, not introduction. |
| **[Human — name TBD]** | Human | Capacity (cargo hold) | What the player was. Familiar. Interacting with them is interacting with memory. Want the player to survive because they are one of them. |
| **[Borg — name TBD]** | Borg | Fleet (max drones) | What the station is becoming. Collective efficiency. They don't explain because explanation is unnecessary. |
| **[Pirate — name TBD]** | Pirate | Speed (ship movement) | Transactional. Honest about it. No pretense. They see the player clearly and don't care. Useful is enough. |

### Contact Mechanic — Bottles
- Bottles drift into range on a spawn timer (45 seconds after game start for First Light)
- Player taps Bottle on screen to dispatch a drone for collection
- On collection: Signal Log entry fires, Request card added to REQUESTS tab
- REQUESTS tab Faction ComboBox filters by faction; empty before first Bottle

### Upgrade Branches — One Per Faction
Each faction owns one exclusive upgrade track. Fulfilling requests advances that track. The four tracks do not overlap.

- **Power (Signal):** Mining rate multiplier. `station.power_multiplier` applied to base mining rate in `mining.rs`.
- **Capacity (Human):** Cargo hold size. `station.cargo_capacity_multiplier`. Wired in Phase 3.
- **Fleet (Borg):** Max active drones. `station.max_drones`. Upgradeable via requests.
- **Speed (Pirate):** Ship movement speed. `station.ship_speed_multiplier`. Wired in Phase 3.

### What Stays Unexplained — By Design
- How resources leave the station after fulfillment
- How factions send Bottles across the event horizon
- Whether the factions are real or constructs
- What Signal actually is

---

## 7. Progression Arc

### Act 1 - Survival (Station Tier 1)
The station is barely alive. One Power Cell. No drones beyond the player's old ship. The player watches ECHO mine, processes ore, builds toward full reactor power. Everything is manual. Everything is slow. The constraint is the point - the player feels how broken things are.

**Objective:** Activate full reactor. Unlock all base departments.

**Emotional register:** Fragile. Uncertain. Something is working but just barely.

### Act 2 - Partnership (Station Tier 2)
The hull is repaired. ECHO has full operational capability. The player's ship becomes Drone-1 permanently - ECHO takes it, the player becomes station-bound by consequence. New ship types become available. The drone fleet starts growing. The player starts feeling like a commander.

**Objective:** Repair hull (25 Power Cells). Unlock SURVEYOR ship. Build first Mining Depot.

**Emotional register:** Something clicked. The station is working for me now.

### Act 3 - Command (Station Tier 3)
Multiple drones running parallel cycles. Floating Forges at key sectors. The FOUNDRY unlocks advanced recipes. The player has genuine fleet management decisions - which sectors to prioritize, which drones to upgrade, which depots to build next. The solar system map fills with owned infrastructure.

**Objective:** Establish full mining network across S1-S4. Build two Floating Forges. Unlock second drone berth.

**Emotional register:** I built this. Look at what I built.

### Act 4 - Expansion (Station Tier 4)
The Relay Station becomes buildable. Scout Ships can be dispatched on galactic missions. The first trade route opens. Merchant Ships begin running automatically. The economy is no longer closed - external materials flow in, complex components become craftable. The station is a hub, not an outpost.

**Objective:** Build Relay Station. Dispatch first Scout. Establish first trade route.

**Emotional register:** The galaxy knows we're here.

### Act 5 - Legacy (Station Tier 5)
The Stargate survey completes. Activation is possible. A new star system opens. ECHO asks: shall we expand further? The player decides the shape of what comes next.

**Objective:** Activate Stargate. Reach new system.

**Emotional register:** We started with one broken station and a signal in the dark.

---

## 8. Station System

### Station Tiers
The station progresses through five tiers. Each tier requires specific resources and unlocks new departments, drone slots, and capabilities.

```
TIER 1 - SALVAGE OUTPOST
  Starting state. Reactor minimal. One drone (player's ship).
  Departments: POWER only.
  Unlock condition: Activate full reactor (25 Power Cells).

TIER 2 - MINING STATION
  Full operations. First expansion.
  New: CARGO, REFINERY, FOUNDRY, HANGAR tabs.
  New: SURVEYOR ship blueprint.
  New: Mining Depot blueprint (buildable at sectors).
  New: Second drone berth.
  Unlock condition: Repair hull (25 Power Cells to hull repair system).

TIER 3 - INDUSTRIAL HUB
  Fleet expansion. Advanced production.
  New: Third drone berth.
  New: Floating Forge blueprint.
  New: Advanced FOUNDRY recipes (Tungsten components).
  New: AI Core Mk II protocol.
  Unlock condition: Build two Mining Depots + produce 10 Hull Plates.

TIER 4 - FLEET COMMAND
  Galactic reach begins.
  New: Relay Station blueprint.
  New: Scout Ship construction.
  New: MERCHANT ship type.
  New: Fourth and fifth drone berths.
  Unlock condition: Complete full S1-S6 sector survey + produce Ship Hull Mk II.

TIER 5 - VOID CITADEL
  Endgame. Stargate activation.
  New: Stargate survey missions.
  New: New system access.
  New: Tier 3 drone fleet.
  Unlock condition: Establish three trade routes + build Relay Station.
```

### Station Departments
Each department is a physical location on the station. When the player opens a tab they are visiting that department.

**POWER - Reactor Room**
ECHO monitors from here. Shows station and ship power levels, consumption rates, stored Power Cells. The reactor is the heartbeat of the station - if power drops, departments go offline in order.

**CARGO - Cargo Bay**
All incoming ore lands here. Auto-unload toggles per resource type. Current stores of every material. The first place to check when production feels slow.

**REFINERY - Processing Floor**
Two processing queues: Magnetite -> Power Cells, Carbon -> Hull Plates. Batch timers. Queue depth. Auto-smelt toggles. The station's primary production engine.

**FOUNDRY - Fabrication Bay**
Advanced crafting. Hull Plates -> Ship Hull. Power Cells -> AI Core. Tier 2+ recipes unlock with station progression. The Foundry is where the station's future is built - every upgrade component comes from here.

**HANGAR - Drone Bay**
Fleet command center. Shows all drone berths, current assignments, status. LOCK/UNLOCK controls. Scout Ship dispatch. Ship type selection for personal vessel. The operational heart of the fleet.

**FLEET (primary tab)**
Strategic overview of all drone assignments, active routes, and fleet status. The commander's dashboard.

**STATION (primary tab)**
Station tier progress, ECHO status, upgrade path, active construction. The builder's dashboard.

---

## 9. Fleet System

### Drone Assignment
Drones run fixed loops between the station (or a Floating Forge) and an assigned sector. Assignment is set once from the HANGAR tab and runs until changed.

```
DRONE ASSIGNMENT MODEL
  Select drone in HANGAR tab
  Tap sector on map to assign
  Drone departs immediately
  Runs: Transit -> Mine -> Return -> Unload -> Repeat
  Assignment persists until player changes it
```

### LOCK / UNLOCK - Direct Drone Command
When locked to a drone, the player can direct it in real time.

```
LOCK - tap drone on map, tap LOCK button
  Camera follows drone perspective
  Signal strip shows drone-specific feed
  Quick-access command bar appears

WHILE LOCKED - command options:
  TAP SECTOR     -> redirect drone (one-time)
  HOLD SECTOR    -> permanent reassignment
  TAP ASTEROID   -> direct mine specific field
  TAP STATION    -> immediate recall
  TAP DEPOT      -> send to Floating Forge

UNLOCK - returns drone to autonomous operation
  Last assignment retained
  Camera returns to free or FOCUS state
```

### FOCUS
Returns camera to the player's anchor point - the station when docked, the personal ship when flying. Always visible in world view top-left. The "come home" button.

### Drone Discovery
Drones can discover sectors the player has never visited by being commanded there while locked. A locked drone directed to an unknown sector triggers the discovery sequence on arrival.

```
LOCKED DRONE APPROACHES UNKNOWN SECTOR:
> ECHO: UNCHARTED SIGNAL RESOLVING...
> ECHO: NEW SECTOR CONFIRMED - S4 TUNGSTEN BELT.
> ECHO: MINING VIABLE. ASSIGNMENT AVAILABLE.
> DRONE-1: RETURNING TO STATION.
```

This gives the station manager full discovery capability without requiring personal flight.

### Discovery Gate
Every sector requires one visit - personal flight, locked drone command, or late-game Scout Ship dispatch - before drone assignment or depot construction becomes available. Once discovered, the sector is permanently accessible from station UI. The player never needs to return.

---

## 10. Ship Types

Ship types replace equipment management. The ship IS the loadout. No gear slots, no stat min-maxing. One decision: what role does my personal ship serve?

### MINER
**Role:** Ore extraction
**Strength:** High cargo capacity, efficient mining
**Weakness:** Slow transit, no discovery capability
**Player use:** Early game personal ship. Mines alongside drones before ECHO takes it.

```
MINER TIER 1 - Salvage Cutter
  Cargo: 100 ore
  Speed: Standard
  Starting vessel - becomes Drone-1 when ECHO takes it

MINER TIER 2 - Deep Core Hauler
  Cargo: 250 ore
  Speed: Slow
  Unlocks: Asteroid core mining (Tier 2 materials)

MINER TIER 3 - Void Excavator
  Cargo: 500 ore
  Speed: Very slow
  Unlocks: Crystal Core fields (Tier 3 materials)
```

### SURVEYOR
**Role:** Exploration and route opening
**Strength:** Fast transit, reveals uncharted regions
**Weakness:** Small cargo, no mining yield contribution
**Player use:** Mid-game personal ship for station managers who want to expand without mining manually.

```
SURVEYOR TIER 1 - Scout Runner
  Cargo: 25 ore
  Speed: Fast
  Unlocks: Reveals adjacent uncharted regions on map

SURVEYOR TIER 2 - Deep Range Probe
  Cargo: 50 ore
  Speed: Very fast
  Unlocks: Full sector mapping, anomaly detection
```

**The Surveyor's output:** Opens mining routes (drone fleet gains sector access), opens trade routes (when Scout Ships exist), discovers rare material locations.

### MERCHANT (Post-MVP)
**Role:** Trade economy operation
**Strength:** Can acquire materials from NPC stations
**Weakness:** Depends on established trade routes
**Unlock:** Station Tier 4, requires first trade route

### DRONES - Always MINER Type
Drones are always Miners. Their tier is upgraded through station progression, not personal choice. The player manages drone tier through Station Upgrade path, not personal equipment decisions.

---

## 11. Upgrade System — Faction Rewards

### Overview
Upgrades are not a standalone system. They are rewards for fulfilling faction Requests. There is no upgrade shop. The upgrade arrives when the faction decides you have earned it.

Four independent upgrade tracks, one per faction. Multipliers live on the `Station` component as global modifiers read by the relevant systems.

### Upgrade Tracks

```
Power (Signal faction)
  station.power_multiplier
  Applied in: mining.rs against MINING_RATE constant
  Default: 1.0
  First Light reward: 1.25 (mining rate +25%)

Capacity (Human faction)
  station.cargo_capacity_multiplier
  Applied in: ship cargo capacity calculation
  Default: 1.0
  Wired: Phase 3

Fleet (Borg faction)
  station.max_drones
  Applied in: drone build system cap check
  Default: 5
  Upgradeable via Borg requests

Speed (Pirate faction)
  station.ship_speed_multiplier
  Applied in: autopilot navigation speed
  Default: 1.0
  Wired: Phase 3
```

### Asteroid Field Capacity
`station.max_active_asteroids` controls the global asteroid cap. Default 3. Upgradeable via future Borg requests (not yet implemented). This is separate from drone count — it governs the field, not the fleet.

---

## 12. Resource Economy

### Tier 1 Materials - Local, Basic
Available from S1-S3 without Surveyor work.

```
Magnetite -> Power Cells (Refinery, 10:1, 4s)
Carbon -> Hull Plates (Refinery, 5:1, 6s)
Hull Plates -> Ship Hull (Foundry, 3:1, 10s)
Power Cells -> AI Core (Foundry, 55:1, 15s)
```

### Tier 2 Materials - Surveyed, Advanced
Available after Surveyor discovers S4-S5.

```
Tungsten - mined at S4
Iron - mined at S2
Titanite - mined at S5
```

Foundry recipes unlock at Station Tier 3:
```
Tungsten + AI Core -> Drone Mining Rig Mk II
Iron + Carbon -> Engine Component
Titanite + Hull Plate -> Reinforced Hull
```

### Tier 3 Materials - Deep System, Rare
Available after S6 and uncharted regions discovered.

```
Crystal - mined at S6 cores (requires Tier 2 drone)
Anomaly Core - found in uncharted regions (Surveyor only)
```

Foundry recipes unlock at Station Tier 4:
```
Crystal + Tungsten -> AI Core Mk II
Anomaly Core + AI Core -> Scout Ship Core
```

### Helium - Passive Yield
Secondary yield from all asteroid mining. Approximately 2 units per 100 ore mined. Accumulates passively. Future use: Fuel Cells for Scout Ship propulsion (post-MVP). Currently stored in CARGO, displayed as passive accumulation.

---

## 12.5 - Asteroid Spawn System — Phase 2 Implementation

### Spawn Model
Asteroids spawn at random radial positions from the station origin (200–500 units). No fixed sector positions. No per-type caps.

- **Ore types:** Iron, Tungsten, Nickel, Aluminum — equal probability (25% each)
- **Global cap:** `station.max_active_asteroids` (default 3)
- **Respawn:** `asteroid_respawn_system` fires on a timer, checks active count against cap, spawns if below cap
- **Initial spawn:** `spawn_initial_asteroids` runs on game start, fills to cap
- **Lifespan:** Each asteroid has a `lifespan_timer`; paused while actively targeted by a drone

### Ore Pipeline Summary
```
Iron      -> Iron Ingot      -> (various)
Tungsten  -> Tungsten Ingot  -> (gated, requires Tungsten laser)
Nickel    -> Nickel Ingot    -> (gated, requires Tungsten laser)
Aluminum  -> Aluminum Ingot  -> AluminumCanister
```

### Aluminum Pipeline
Aluminum is the Phase 2 addition. Full pipeline:
```
Aluminum Ore -> Aluminum Ingot (auto-refine) -> AluminumCanister (auto-forge)
```
AluminumCanister seeds the Phase 3 Helium farming loop. Auto-refine and auto-forge toggles for Aluminum are in the PRODUCTION tab alongside Iron, Tungsten, and Nickel controls.

---

## 13. The World - Solar System Scale

### Sector Layout
All sectors are in the same solar system. Distances are navigable in seconds to minutes at ship speed. The solar system is large enough to feel vast but small enough to manage.

```
SECTOR POSITIONS (world units from station origin)

S1 Magnetite  ( 520,  220) - Basic laser, starter sector
S2 Iron       (-420,  580) - Basic laser, second discovery
S3 Carbon     ( 680, -480) - Basic laser, third discovery
S4 Tungsten   (-950, -720) - Requires Surveyor (Tier 2 gate)
S5 Titanite   (1200,  580) - Requires Surveyor (Tier 2 gate)
S6 Crystal    (-1150, 950) - Requires Surveyor + Drone Tier 2
```

### Uncharted Regions
Visual markers at extreme map positions indicating unknown space beyond the known fields. Dim, semi-transparent, labeled with signal strength.

```
UNCHARTED REGION MARKERS
  Faint dot on map
  Label: > UNCHARTED - SIGNAL TOO WEAK TO RESOLVE.
  No interaction until Surveyor reaches them
  Discovery reveals: anomaly type, rare material estimate
```

### Sector Discovery Gate
A sector must be visited once before:
- Drone assignment is available
- Depot construction is available
- Map marker becomes full detail

Discovery methods:
1. Personal flight (any ship type)
2. Locked drone command (player directs drone there)
3. Scout Ship dispatch (late game, hours-long mission)

### Floating Forges and Mining Depots
Buildable structures placed at discovered sectors from station UI. No personal visit required for construction after initial discovery.

```
MINING DEPOT
  Built at any discovered sector
  Drone drops ore here instead of returning to station
  Depot processes ore locally (Refinery function remote)
  Refined materials sent to station on drone return trip
  Effect: Shorter drone cycle, higher throughput

FLOATING FORGE
  Built at resource-rich sectors (S4, S5 recommended)
  Full Foundry function at remote location
  Advanced components produced at source
  Eliminates transit time for Tier 2 materials
  Unlock: Station Tier 3
```

---

## 14. Galactic Scale (Post-MVP)

### Scout Ships
Long-duration autonomous missions. Not controlled - dispatched. The player sets a destination and mission type, the Scout departs, the result arrives hours later.

```
MISSION TYPES
  Survey - reveals new star system, catalogues resources
  Trade - acquires materials, starts faction relationship
  Establish Route - opens permanent trade connection
```

ECHO pilots all Scout Ships through deployed AI Core subroutines. The Signal strip carries their reports:

```
> ECHO: SCOUT-1 SIGNAL RECEIVED - 4 HOURS OUT.
> ECHO: KEPLER SYSTEM SURVEY COMPLETE.
> ECHO: TITANITE DEPOSITS CONFIRMED. TRADE VIABLE.
> ECHO: SCOUT-1 RETURNING. ETA: 4 HOURS.
```

### Trade Routes
Once a Scout establishes a route, Merchant Ships run it automatically. The player sets export and import priorities from the FLEET tab. The economy runs itself.

### The Relay Station
The physical infrastructure that makes galactic reach possible. Built at Station Tier 4. Scout Ships depart from and return to the Relay Station. Without it, the solar system is the limit.

---

## 15. UI Architecture

### The Bottom Drawer System
Three states. Always respects world view priority.

```
COLLAPSED (default - flying)
  World view: full screen minus handle + signal
  Handle bar: 32px - always visible
  Signal strip: 64px - always visible
  Total UI: 96px

TABS ONLY
  World view: reduced by tab bars
  Handle bar: 32px
  Primary tab row: 48px (STATION | FLEET)
  Signal strip: 64px
  Total UI: 144px

EXPANDED (docked default)
  World view: ~50% screen
  Handle bar: 32px
  Primary tab row: 48px
  Secondary tab row: 48px (docked departments)
  Content area: ~358px
  Signal strip: 64px
  Total UI: ~762px
```

### Tab Structure

**Primary Row - always visible in TabsOnly/Expanded:**
```
STATION    FLEET
```

**Secondary Row - docked only:**
```
POWER    CARGO    REFINERY    FOUNDRY    HANGAR
```

### World View Controls
```
[FOCUS]     - top-left, permanent, 44px tap target
             Returns camera to anchor (station or personal ship)

[LOCK]      - contextual, appears when drone selected
             Attaches camera to drone, enables direct command

[UNLOCK]    - replaces LOCK when locked to drone
             Returns to free camera, drone resumes assignment
```

### Camera Viewport System
The Bevy camera viewport resizes dynamically based on drawer state. The world view literally shrinks when the drawer expands - it is not obscured, it is smaller. Touch input in the drawer area is suppressed from world navigation.

```
VIEWPORT HEIGHTS (logical pixels, Moto G 2025)
  Collapsed:  1508px world height
  TabsOnly:   1460px world height
  Expanded:    842px world height
```

### Adaptive Layout
All measurements derived from `UiLayout` resource computed each frame. No hardcoded pixel values in UI systems. Portrait and landscape handled automatically through proportional calculations.

---

## 16. Signal Strip Design

### Physical Description
64px tall. Full screen width. Always anchored at bottom. Terminal green text (#00CC66) on near-black background. Monospace font. Three lines visible collapsed, scrollable when expanded.

### ECHO's Voice
All Signal strip entries are ECHO speaking. The strip is not a system log - it is a conversation with an intelligence.

```
ENTRY CATEGORIES

STATUS (routine operations):
> ECHO: DRONE-1 DOCKED. CARGO UNLOADED.
> ECHO: REFINERY QUEUE COMPLETE. 10 POWER CELLS PRODUCED.

ADVISORY (recommendations):
> ECHO: S2 AVAILABLE FOR DRONE ASSIGNMENT.
> ECHO: FOUNDRY IDLE. HULL PLATES SUFFICIENT FOR SHIP HULL.

DISCOVERY (new information):
> ECHO: S4 TUNGSTEN BELT SURVEYED. MINING VIABLE.
> ECHO: ANOMALOUS SIGNAL - SECTOR 7. FLAGGED.

MILESTONE (progression):
> ECHO: HULL INTEGRITY RESTORED. TIER 2 UNLOCKED.
> ECHO: DRONE-2 ONLINE. FLEET EXPANDING.

NARRATIVE (character moments):
> ECHO: YOU HAVE BEEN RUNNING THIS STATION FOR 3 HOURS.
> ECHO: I HAVE PROCESSED 847 POWER CELLS IN THAT TIME.
> ECHO: NOT BAD FOR A BROKEN OUTPOST.
```

### Chronicle Function
The Signal strip is the history of the station. The player can scroll back through every entry and read the arc of what they built. Entry timestamps allow the player to see how long ago each milestone happened.

---

## 17. Session Design - Idle and Active Balance

### The Mobile Session Reality
Based on industry data for mobile idle games, target session patterns:

- **2-minute session:** Check queues, adjust assignments, confirm progress. No active play required.
- **5-minute session:** Complete a mining run, process a batch, check FLEET tab, make one upgrade decision.
- **15-minute session:** Full production loop, discover a new sector, build a depot, progress toward next station tier.

### The Production Loop
One mining run should yield enough ore for 3-5 refinery batches. Batches should complete while the player does the next mining run. The player returns to results every cycle.

```
CYCLE TIMING (current constants)
  Mining run: ~25-30 seconds transit + 5 seconds fill
  Magnetite batch: 4 seconds
  Carbon batch: 6 seconds
  Hull Plate batch: 10 seconds
  AI Core batch: 15 seconds
  
  100 ore -> 10 batches -> 40 seconds processing
  Player mines next run in ~30 seconds
  Station processes while player is in transit
  Player returns to completed batches
```

### Offline Processing
Queues run while the app is closed. When the player returns after hours away, batches are complete, Power Cells are produced, and ECHO reports what happened in their absence.

```
RETURN AFTER 2 HOURS:
> ECHO: WELCOME BACK, COMMANDER.
> ECHO: 47 MINING CYCLES COMPLETED IN YOUR ABSENCE.
> ECHO: 127 POWER CELLS PRODUCED.
> ECHO: HULL REPAIR THRESHOLD REACHED.
> ECHO: AWAITING YOUR INSTRUCTION.
```

### The Agency Spectrum
```
MOST PERSONAL          <- - - - - - - - - ->          MOST HANDS-OFF

Personal    Drone       Drone      Scout      Full
Flight      Command     Assign     Dispatch   Automation

You fly     You watch   You set    You wait   ECHO runs
yourself    and direct  and forget and read   everything
```

The player sits anywhere on this spectrum at any time. The game never judges the position.

---

## 18. ADR Index

All architectural decisions made during development. Referenced by future directives.

| ADR | Decision | Status |
|-----|----------|--------|
| ADR-001 | PresentMode::Fifo mandatory | Locked |
| ADR-002 | AlphaMode2d::Opaque for background elements | Locked |
| ADR-003 | EGUI_SCALE=3.0 (dynamic scaling deferred) | Locked |
| ADR-004 | Bevy 0.15 pinned | Locked |
| ADR-005 | Autonomous agents as dedicated systems | Locked |
| ADR-006 | Module structure | Locked |
| ADR-007 | System tuple partitioning (20-system limit) | Locked |
| ADR-008 | Universal Disjointness (Without<T> on all &mut Transform) | Locked |
| ADR-009 | Tutorial trigger pattern (one-time HashSet) | Locked |
| ADR-010 | Survival narrative as mechanical justification | Locked |

---

## 19. MVP Scope

MVP is Station Tiers 1-2 fully realized. The player repairs the station, meets ECHO, builds their first drone fleet, discovers the near sectors, and has a complete production loop with one personal ship type choice.

### In Scope for MVP

**Core Systems:**
- Opening sequence (full cinematic as designed)
- ECHO voice throughout Signal strip
- Station Tier 1 -> Tier 2 progression
- Full reactor activation sequence
- Hull repair -> ECHO takes ship -> Drone-1 established

**Production:**
- Refinery: Magnetite -> Power Cells, Carbon -> Hull Plates
- Foundry: Hull Plates -> Ship Hull, Power Cells -> AI Core
- Auto-smelt defaults ON
- Offline processing

**Fleet:**
- Drone-1 (player's old ship, ECHO piloted)
- Drone-2 (unlocked at Tier 2)
- LOCK / UNLOCK / FOCUS controls
- Drone discovery (locked drone can survey sectors)
- Sector assignments from HANGAR tab

**World:**
- S1-S4 discoverable
- S5-S6 visible but locked (Tier 3 gate)
- Uncharted region markers visible at map edges
- Procedural infinite starfield (4-layer shader)
- Expanded sector positions (not clustered)

**UI:**
- Bottom drawer system (Collapsed / TabsOnly / Expanded)
- Primary tabs: STATION | FLEET
- Secondary tabs: POWER | CARGO | REFINERY | FOUNDRY | HANGAR
- Adaptive layout (UiLayout resource)
- Camera viewport system (world view resizes with drawer)

**Ship Types:**
- MINER Tier 1 (starting ship -> Drone-1)
- SURVEYOR Tier 1 (available at Tier 2)
- MINER Tier 2 (craftable at Tier 2)

### Explicitly Out of Scope for MVP

- Station Tiers 3-5
- Floating Forges and Mining Depots
- Third drone berth+
- Scout Ships and galactic scale
- Trade routes and Merchant Ships
- MERCHANT ship type
- Faction relationships
- Save/load persistence (post-MVP priority)
- Crew system
- SURVEYOR Tier 2
- Anomaly Cores and Tier 3 materials
- Helium Fuel Cells (Helium accumulates, no use yet)

---

## 20. Post-MVP Roadmap

### v0.6 - Infrastructure
- Station Tier 3
- Mining Depot buildable
- Floating Forge buildable
- Third drone berth
- S5-S6 accessible
- Tier 2 Fleet upgrades

### v0.7 - Galactic Reach
- Station Tier 4
- Relay Station buildable
- Scout Ship construction and dispatch
- First trade route establishment
- MERCHANT ship type

### v0.8 - Trading Economy
- Station Tier 5
- Multi-system trade network
- Faction relationships
- Dynamic pricing
- Merchant Ship automation

### v1.0 - Endgame
- Stargate activation
- New system access
- ECHO narrative conclusion
- Full chronicle review

---

*Voidrift Game Design Document v1.0*
*April 2026 - RFD IT Services*
*Station and Drone Command*
*Built in Rust + Bevy 0.15 for Android*
