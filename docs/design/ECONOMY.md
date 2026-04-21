# Voidrift — Economy Design Canon
**Locked:** April 2026
**Status:** Canonical — not yet implemented in code
**Source:** Design conversation April 2026 — Layer 2 document.
**Depends on:** Bevy UI Migration + Code Refactor complete before implementation.

---

## Three Resource Tracks

All resources belong to one of three tracks. Each track is self-sufficient.
Combining tracks unlocks composite resources. All three combined unlocks the Void Core.

### Metal Track
Primary ores: Magnetite, Iron, Carbon, Tungsten, Titanite
Each ore refines to its own ingot in the FORGE.

| Ore | Ingot | Notes |
| :--- | :--- | :--- |
| Magnetite | Magnetite Ingot | Power pathway, Repair Kits |
| Iron | Iron Ingot | Core structural material |
| Carbon | Carbon Rod | Lightweight structures |
| Tungsten | Tungsten Bar | Advanced tooling, mid-game |
| Titanite | Titanite Ingot | Advanced lightweight, mid-game |

### Gas Track
Primary resource: **Helium**
Passive secondary yield from all asteroid mining (~2 per 100 ore mined).
Never mined directly — always a byproduct of metal ore extraction.
Accumulates automatically during normal play.

### Crystal Track
Primary resource: **Crystal Core** (S6, Composite Laser required)
Rare surface trace rate (0.1), concentrated in asteroid cores.
Mid-to-late game material. Requires Tungsten Laser upgrade before Composite.

---

## Department Structure (Redesigned)

### FORGE — Ore to Ingots
Replaces current SMELTER / REFINERY department.
Each ore type has its own queue. Five parallel Forge queues at full build.

| Input | Output | Time | Notes |
| :--- | :--- | :--- | :--- |
| Magnetite × 10 | Magnetite Ingot × 1 | 12s | Opening material |
| Iron × 8 | Iron Ingot × 1 | 12s | Opening material |
| Carbon × 6 | Carbon Rod × 1 | 15s | Opening material |
| Tungsten × 5 | Tungsten Bar × 1 | 20s | Mid-game, Tungsten Laser gated |
| Titanite × 5 | Titanite Ingot × 1 | 20s | Mid-game, Tungsten Laser gated |
| Crystal × 3 | Crystal Matrix × 1 | 30s | Late game, Composite Laser gated |

### CRAFTER — Components and Composites
Replaces current FORGE department.
Ingots and components become usable items. All crafting happens here.

#### Basic Components (early game)

| Input | Output | Time |
| :--- | :--- | :--- |
| Iron Ingot × 3 | Iron Plate × 1 | 18s |
| Carbon Rod × 4 | Carbon Tube × 1 | 18s |
| Iron Plate × 2 + Magnetite Ingot × 3 | **Repair Kit** × 1 | 15s |
| Carbon Tube × 3 + Iron Plate × 2 | Space Frame × 1 | 25s |

#### Fuel System (Metal + Gas track)

| Input | Output | Time |
| :--- | :--- | :--- |
| Iron Plate × 2 | Fuel Tank × 1 | 20s |
| Fuel Tank × 1 + Helium × 5 | **Fuel Cell** × 1 | 20s |

#### Ship Components

| Input | Output | Time |
| :--- | :--- | :--- |
| Iron Plate × 3 | Hull Plate × 1 | 25s |
| Hull Plate × 3 | Ship Hull × 1 | 35s |

#### Engine Tiers (permanent ship upgrades)

| Input | Output | Base Speed |
| :--- | :--- | :--- |
| Starting | Engine Mk I | 180.0 |
| Iron Plate × 5 + Carbon Tube × 3 | Engine Mk II | 240.0 |
| Tungsten Bar × 3 + Space Frame × 2 | Engine Mk III | 310.0 |
| Charged Plate × 2 + Titanite Ingot × 4 | Engine Mk IV | 400.0 |
| Plasma Cell × 3 + AI Core × 1 | Engine Mk V | 500.0 |

Engine upgrades are permanent. Installed at SHIP PORT. No reverting.
`SHIP_SPEED` constant becomes Engine Mk I base. Each tier overwrites ship speed.

#### Power System (Crystal track — mid game)

| Input | Output | Time |
| :--- | :--- | :--- |
| Crystal Matrix × 5 + Iron Plate × 2 | Power Cell × 1 | 30s |

Power Cells are **mid-game resources** in the redesign. They are not the opening repair resource.

#### Two-Material Composites

| Input | Output | Purpose |
| :--- | :--- | :--- |
| Iron Plate × 2 + Helium × 3 | Pressurized Hull × 1 | Better ship hull |
| Crystal Matrix × 2 + Iron Plate × 3 | Charged Plate × 1 | Engine Mk IV input, Void Core input |
| Fuel Cell × 1 + Crystal Matrix × 2 | Plasma Cell × 1 | Premium fuel, Engine Mk V input |

#### Late Game — AI Core

| Input | Output | Time |
| :--- | :--- | :--- |
| Power Cell × 10 + Space Frame × 2 | AI Core × 1 | 60s |

#### The MacGuffin — Void Core

| Input | Output | Purpose |
| :--- | :--- | :--- |
| Space Frame × 3 + Plasma Cell × 2 + Charged Plate × 2 | **Void Core** × 1 | Stargate activation |

The Void Core requires all three resource tracks. Building it proves mastery of the full economy.

---

## Ship Propulsion Model

### Base Speed
Determined by Engine tier. Permanent upgrade. No fuel cost.
`SHIP_SPEED = 180.0` in current code becomes Engine Mk I base speed.

### Fuel Boost
Optional consumable. Never required. Always advantageous.

| Fuel | Duration | Multiplier |
| :--- | :--- | :--- |
| Fuel Cell | 8 seconds | ×1.8 |
| Plasma Cell | 14 seconds | ×2.4 |

Activation: player manually triggers boost from Ship Port or HUD button.
One cell consumed per activation. Cells are not auto-consumed.

---

## Station Repair (Redesigned)

**Opening quest target: craft 5 Repair Kits.**

Repair Kit replaces Power Cells as the early-game repair resource.
Power Cells are mid-game Crystal-track items — not for early repair.

Quest Q-003 (revised): Craft 5 Repair Kits to restore station systems.
Each kit costs: Iron Plate × 2 + Magnetite Ingot × 3.
So the actual unlock chain is: Mine Iron + Magnetite → FORGE ingots → CRAFTER plates → CRAFTER kits.

This teaches both the FORGE and CRAFTER pipeline before the player reaches any quest gate.

---

## Opening Quest Chain (Revised — 10 objectives)

| Quest | Objective | System Taught |
| :--- | :--- | :--- |
| Q-001 | Locate the signal | Navigation |
| Q-002 | Dock at the derelict station | Docking |
| Q-003 | Craft 5 Repair Kits | FORGE + CRAFTER chain (Metal track introduction) |
| Q-004 | Repair the station | RESERVES tab |
| Q-005 | Mine Helium (passive yield notification) | Gas track awareness |
| Q-006 | Craft 3 Fuel Cells | Gas track production |
| Q-007 | Build Engine Mk II | SHIP PORT, first permanent upgrade |
| Q-008 | Discover Sector 4 (Tungsten gate) | Laser gate, mid-game expansion |
| Q-009 | Build AI Core | Crystal track preview |
| Q-010 | Assemble autonomous ship | Fleet Commander identity |

---

## Resource Hierarchy

| Tier | Resources | Unlocks |
| :--- | :--- | :--- |
| Early | Magnetite, Iron, Carbon, Helium | Repair Kits, basic ships, first fuel |
| Mid | Tungsten, Titanite, Crystal | Advanced ships, Engine Mk III–IV, Power Cells |
| Late | Crystal composites (Charged Plate, Plasma Cell) | Engine Mk V, Void Core inputs |
| Endgame | Void Core | Stargate activation, galaxy expansion |

---

## Current vs. Designed Comparison

| Item | Current Code | Designed |
| :--- | :--- | :--- |
| Repair resource | Power Cells (25) | Repair Kits (5) |
| Ore-to-ingot dept | SMELTER / REFINERY | FORGE |
| Ingot-to-item dept | FORGE | CRAFTER |
| Power Cells | Early game (Magnetite → Cell) | Mid game (Crystal → Cell) |
| Engine | Single constant (180.0) | Mk I–V tiers with permanent upgrades |
| Fuel | Not implemented | Fuel Cell + Plasma Cell with Boost mechanic |
| Helium | Not implemented | Passive yield from all mining |
| Void Core | Not implemented | Three-track MacGuffin for Stargate |
| Quest chain | 7 objectives | 10 objectives (revised targets) |
