# Voidrift — Stargate & Galaxy Design
**Locked:** April 2026
**Status:** Canonical — not yet implemented in code
**Source:** Design conversation April 2026 — Layer 2 document.

---

## The Stargate

A Precursor artifact present in the solar system from game start.
Visible on the map from day one. Inert. Cannot be activated without a Void Core.

The Signal acknowledges it early but cannot explain it. The player sees it
before they understand what it is.

Visual design: geometric, alien, dark, with occasional power flickers suggesting
it is not completely dead. Distinct from all other structures — not human-built,
not station architecture.

The Stargate is not built by the player. It is found and restored.
Building a Void Core proves mastery of all three resource tracks.
Activation is the payoff for completing the full economy chain.

---

## Activation Sequence

1. Player completes and installs a Void Core at the Stargate
2. Signal: `> STARGATE POWER THRESHOLD MET. DESTINATION LOCK REQUIRED.`
3. Multiple signals detected simultaneously — player sees partial information for each
4. Player selects one signal — gate locks to that destination permanently
5. Other signals are gone for this gate. New signals appear for the next gate
6. The selected system loads. The galaxy map gains a new node

---

## Signal Selection (Blue Prince influence)

Each destination signal shows partial information only:

| Field | What Player Sees |
| :--- | :--- |
| Signal strength | Strong / Moderate / Weak |
| Resource signature | Partial — e.g. "DENSE METALLIC SIGNATURE" |
| Anomaly flags | Precursor tech / biological / high density / dormant gate |
| Chain indicator | Whether another gate is detected at the destination |

Player chooses with incomplete information. The choice is **permanent**.
No two players have the same galaxy — each signal choice shapes what systems
exist in that player's universe.

Regret is possible. That is the design intent.

---

## Procedural System Generation

Each gate destination contains:
- 2–6 asteroid fields (ore types weighted by system class)
- 0–3 orbital stations with unique planetary resources
- 0–2 derelict structures (Precursor or unknown origin)
- 0–1 dormant Stargate pointing further out
- Unique Signal flavor text derived from system characteristics

The first gate destination is hand-crafted for MVP. Full procedural generation is v2.0.

---

## Orbital Stations & Planetary Bodies

Each orbital station is associated with a planetary body.

**Visual treatment:**
- Planet: large dim background circle, visual only, never interactive
- Orbital station: rotates slowly around the planet at fixed radius
- Player docks at the orbital station using the existing Berth system

**Gameplay treatment:**
- Each planet type produces one unique resource unavailable in the solar system
- That resource unlocks one new CRAFTER recipe and one new station department
- The player never lands on the planet surface

**Planet types (designed):**

| Planet Type | Visual | Unique Resource | Unlocks |
| :--- | :--- | :--- | :--- |
| Volcanic moon | Dark red | Thermal Compound | Heat Forge department |
| Ice giant | Pale blue | Cryo Crystal | Cooling systems |
| Organic world | Green-brown | Bio Extract | Chemistry department |
| Dense metallic | Grey-silver | Refined Iridium | Advanced hull recipes |
| Gas giant platform | Amber | Atmospheric Gas | Thruster upgrades |
| Ancient derelict | Dark purple | Precursor Tech | Research department |

---

## The Personal Galaxy Map

Every visited system is permanently on the player's galaxy map.
The map is a record of every gate choice ever made.
It cannot be reset without starting over.

The galaxy is a personal artifact — a record of consequence.

Two players who made different gate choices at different points will have
different galaxies, different resources available, and different late-game options.

---

## MVP Scope for Stargate

The full procedural system is not required to ship Phase 1.

**MVP requirements:**
- Stargate world object visible from game start (inert)
- Signal strip acknowledgement when Stargate is first approached
- Void Core as craftable item (even if Stargate activation is stub)
- One hand-crafted destination through the first gate
- Galaxy map expands to show new system after activation

**Deferred to v2.0:**
- Full procedural system generation
- Multiple gate destination choices with partial information
- Orbital station docking at planetary bodies
- Unique planetary resources and department unlocks

---

## Connection to Economy

The Stargate is the capstone of the three-resource economy.

```
Metal Track  ──┐
Gas Track    ──┼──► Void Core ──► Stargate Activation ──► New System
Crystal Track──┘
```

A player cannot activate the Stargate without having engaged seriously with
all three resource tracks. This is the intended late-game gate.
