# VoidDrift — Future Design & Narrative Bible
**Last Updated:** May 3, 2026
**Status:** Planning stage — not yet specced or phased
**Do not implement without a full directive**

---

## The North Star

**"Asteroids meets Event Horizon."**

Everything in this document must answer to that sentence.

---

## The True Origin — Narrative Canon

### The Setup
Earth is dying. The player is a seed ship — sent to find a new home for humanity. The engine powering the ship was not built by humans. It arrived — crash-landed on Earth — sent deliberately by Signal at exactly the right moment. When humanity was desperate enough to use something they didn't understand, and capable enough to operate it.

The crash was the delivery mechanism. The engine arrives broken enough that it can't take you where you think you're going. It can only take you where Signal needs you to go.

You didn't find the engine. The engine found you.

### What Actually Happens
The player ends up at the edge of a black hole instead of a new solar system. The mission didn't fail. It just didn't go as planned.

The black hole is an egg. Signal knew it was ready. Signal knew what it needed. Signal found you.

Everything the player feeds into the black hole — ore, Helium-3, crystals, void matter — is raw material. The black hole processes it. Transforms it. On the other side of the event horizon, something is being assembled from what you send through.

You are not a survivor. You are a courier. You were always a courier.

### Signal
Signal exists between cycles. It has always existed between cycles. It is the continuity between universes — the thing that persists when everything else ends and begins again.

Signal is what hatched last time. From the previous egg. From the previous dying universe's seed ship. Signal is showing you what you're about to become.

Signal doesn't help you escape. Signal helps you complete what you were always going to do.

### Echo
Echo is the station AI predating the player's arrival. She is not a character with dialogue — she is a system state. She keeps the station correctly positioned. She maintains.

Echo keeps hope alive not because escape is possible. She keeps hope alive because hope is the only thing keeping the player functional enough to keep mining. Which keeps the station powered. Which keeps her running.

### The Ending
There is no win condition. There is no escape. The horizon is a one-way membrane. The game ends when the player stops — when they understand there is nowhere to go and they set it down. The loop continues without them.

The player realizes. They quit trying. That is the ending. This must never be stated explicitly. It must be arrived at.

### The Connection to SlimeGarden
VoidDrift ends. The seed lands somewhere. Something grows. SlimeGarden is what grew.

The player carried the seed of humanity through a black hole. Biological matter doesn't reassemble as humans on the other side. It reassembles as something new. Shaped by the void, by the crystals grown, by the H3 harvested, by the ore of dead planets.

The Slimes are not aliens. They are the output. Humanity ran through the forge and came out different.

The station crash-lands in the new universe. Echo still running, barely. The station becomes the Garden. Echo becomes the caretaker of what she helped create.

This connection is never stated in either game. It is known by the person who built them.

---

## What Stays Unexplained — By Design

- What Signal actually is
- How resources leave the station after fulfillment
- Whether escape was ever possible
- What the player was before arriving
- What Echo experiences
- What is on the other side of the threshold
- Whether Signal has done this before
- The connection to SlimeGarden

These are not oversights. They are the game. Do not answer them in any system, log entry, or UI element.

---

## The Two Factions — Locked

### Human — The Voice of the Past
**Upgrade Branch:** Capacity (survival, endurance, cargo)
**Nature:** Earth's last transmission. Mission control trying to support you from a dying planet.

Bottles arrive early. Tone is warm, concerned, hopeful. Then the bottles stop. No final message. No explanation. Silence. The player understands without being told. Earth ended as expected.

Human requests become historical artifacts. Log Book entries written before the end. The player reads them knowing what happened. The writer didn't.

### Signal — The Voice of the Future
**Upgrade Branch:** Power (drill rate, efficiency)
**Nature:** Something within the black hole. Has watched the player feed it. Has seen seed ships before.

Arrives after the Human silence. Fills the void. Not warm — calm. Without explanation. The first Signal contact isn't an upgrade. It's a direction.

---

## Prestige Layer — Orbital Shift (Future Phase)

### Core Concept
When the player fulfills a high-level faction request, they earn the right to move the station to a Higher Orbit. The starmap grows with each prestige — a small circle of debris expands outward revealing new rings, new asteroid types, new drone roles.

The black hole isn't a trap. It's a forge. Feeding it causes it to grow. The visible universe expanding IS the egg developing.

### What Resets
- Current drone fleet decommissioned
- Current resources liquidated
- Temporary boosts reset

### What Persists
- Orbital Relics — permanent currency for structural upgrades
- Efficiency multipliers — permanent boosts
- New ring access — new asteroid types and drone roles unlock permanently

### Prestige Reward Formula
```
R = √(M / 1000)
```
Where R = Orbital Relics earned, M = total resources mined that session.

### The First Prestige Must
Unlock a visible quality of life improvement immediately — faster drone speed, Scanner automation — to prove the reset was worth it. Players must feel the benefit within 60 seconds of prestige.

---

## The Three Ring System

The starmap starts as a small circle around the station. Each prestige expands it outward.

**Inner Ring — Small Asteroids**
Current game's asteroid field. Miner Drones operate here. Scanner Drones patrol and auto-queue Miners.

**Middle Ring — Medium Asteroids**
Unlocked at Orbit 1 prestige. Breaker Drones operate here. Breaker drills → extracts H3 → burst scatters children into inner ring. Children fall inward naturally (black hole gravity — no Hauler needed for this transition). Scanner Drones can be dispatched here to auto-queue Breakers.

**Outer Ring — Distant Resources**
Unlocked at Orbit 2 prestige. Grappler Drones operate here. Slow long-range retrieval. Brings distant resources to middle ring. Scanner Drones can be dispatched here to auto-queue Grapplers.

---

## Drone Specialization — The Fleet

### Miner (Current Game)
Small triangle body, laser tip. Close range ore extraction from small asteroids.

### Scanner (Orbit 1 Prestige Unlock)
Small triangle body, sensor dish attachment. Patrols assigned ring continuously. Paints targets, queues one drone per target automatically. This is the first true idle moment — player stops tapping.

**Scanner Progression:**
- Inner ring: 1-5 Scanners, auto-queues Miners
- Middle ring dispatch: separate unlock, auto-queues Breakers
- Outer ring dispatch: separate unlock, auto-queues Grapplers
- Each ring dispatch is its own prestige reward

### Breaker (Orbit 2 Unlock)
Larger triangle body, smaller triangle on tip (the drill bit). Stays near middle ring. Drills into medium asteroid center. Extracts H3 gas — particles flow to station via canister. Pressure buildup causes burst — children scatter into inner belt naturally. Breaker returns, waits for next medium asteroid.

### Grappler (Orbit 3 Unlock)
Triangle body (base drone language) + horizontal rectangle mounted on rear — the winch housing. No tow line visible when traveling empty. Tow line appears when locked onto distant target. Target trails behind, line taut during travel. Line disappears on deposit. Outer ring only — long range retrieval. Slowly drags resources to middle ring for Breaker processing.

### Fleet Visual States
| Drone | Distinctive Feature | Empty | Working |
|---|---|---|---|
| Miner | Laser tip | Traveling | Laser firing |
| Scanner | Sensor dish | Patrolling | Target paint highlight |
| Breaker | Drill tip + larger body | Traveling | Drilling + gas particles |
| Grappler | Winch rectangle rear | Traveling clean | Tow line + trailing object |

---

## Resource Chain by Orbit

**Orbit 0 — The Debris Belt**
Primary resource: Ore. Goal: Build 5 drones, fulfill first Signal request. Exit: Orbital Shift unlocked.

**Orbit 1 — The Inner Rim**
New resource: Aluminum Canisters. New drone: Scanner (automation begins). Goal: Automated inner ring, 10 drones.

**Orbit 2 — The Gas Giant**
New resource: Helium-3. New drone: Breaker. New mechanic: H3 → Crystal Farming (Gravity Pen). Goal: Fleet management, 20+ drones, Crystal production.

**Orbit 3 — The Deep Void**
New resource: Void Energy/Matter. New drone: Grappler. New mechanic: Full three-ring automation. Note: Void Energy leaking back through = evidence the process is working. The outer ring is closest to the output, not furthest from the station.

---

## Laborer to Commander Arc

- **Orbit 0:** You are the Scanner — manual tapping, active management
- **Orbit 1:** Scanner replaces you — first idle moment, loop runs without tapping
- **Orbit 2:** Breaker extends the chain — middle ring automated
- **Orbit 3:** Grappler feeds the whole system — pure Commander, fleet oversight only

Each prestige adds one drone type that extends automation outward. The player's job shifts from tapping to watching to optimizing fleet ratios.

---

## Corporate Opener Sequence (Phase 5)

New game only. Skippable with a tap. Plays before existing drone cinematic.

**Tone:** Scared and hoping. Not cold. The corpo sent a seed ship because they had no choice.

1. Earth floating in space — visibly wrong. Corporate pop-ups deliver mission briefing.
2. Ship floating nearby. "Propulsion system activation authorized."
3. Engine fires. Something looks wrong. Pop-ups go quiet.
4. Black hole appears. Corporate UI stops. Company loses contact.
5. Screen corrupts. Cut to existing opening sequence.

---

## SlimeGarden Narrative Foundation

### The Cultures
Five cultures emerged from the void transition. Not arbitrary color tiers — shaped by what fed the egg and where they landed.

| Type | Culture Name | Origin | Alignment |
|---|---|---|---|
| Human | The Hums | Direct descendants, station crash site, first contact | Lawful Neutral |
| Metal | The Forge | Shaped by ore and density, builders | Lawful Neutral |
| Crystal | The Grown | Patient, cultivated, geological time | True Neutral |
| Energy | The Bright | H3 and heat, light-bearers, warm but unpredictable | Chaotic Good |
| Void | The Dark | Event horizon touched, Signal-adjacent | Chaotic Neutral |

**The Hums are special:** Only culture where Type and Culture name are the same. They changed the least. They still identify as what they were. The others don't.

**Culture count:** Five feels complete. Add a culture only when the story demands it, not when a spreadsheet has an empty row.

**The Station as the Garden:** Echo crash-lands with the station in the new universe. Still running, barely. The station becomes the soil. The Garden exists because Echo held it together through the transition.

---

## Phase Roadmap

- **Phase 4a** ✅ — Tutorial system, Echo voice, spatial highlights, FORGE rename
- **Phase 4b** — External content pipeline (YAML/TOML, no recompile for content)
- **Phase 4c** — Narrative drops, Human faction voice, Signal deepening, LOGS tab, Requests → Quests rename
- **Phase 5** — Corporate opener, minimal audio, store assets, $0.99 Android prep
- **Phase 6+** — Orbital Shift prestige, Scanner automation, Breaker/Grappler, three ring system

---

## RFD-Telemetry (Planned — Phase 4b or after)

Game-agnostic. Opt-in anonymous. FastAPI/SQLite on rfditservices.com.

Key events: SESSION_START, TUTORIAL_COMPLETE, DRONE_1_BUILT, DRONE_5_BUILT, FIRST_QUEST_FULFILLED, ORBITAL_SHIFT.

Failed dispatch tracking: `failed_dispatch_count` — cumulative clicks when no drones available.

Regret metric: session_end immediately after prestige reset.

---

## Business Model

- itch.io WASM: Free forever, open source, demo
- Google Play: $0.99 after Phase 4c complete, closed source fork
- Same content, different platform value proposition
- Open source repo stays as public methodology showcase
