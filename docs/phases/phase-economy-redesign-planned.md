# Phase: Economy Redesign — Planned
**Status:** Designed, not yet implemented
**Designed:** April 2026
**Depends on:** Bevy UI Migration + Code Refactor complete
**Source:** Design conversation April 2026 — Layer 2 document.

---

## Summary

The current economy (Magnetite → Power Cells → Repair) is a placeholder chain.
It teaches the processing queue mechanic but uses Power Cells as an early-game
repair resource, which conflicts with the intended mid-game Crystal-track role
for Power Cells.

This phase replaces the entire economy with a three-track system that teaches
resource chains sequentially and culminates in the Void Core / Stargate mechanic.

See `docs/design/ECONOMY.md` for the full specification.

---

## What Changes

### Department Renames
- **SMELTER / REFINERY** → **FORGE** (ore to ingots — five ore types, five queues)
- **FORGE** → **CRAFTER** (ingots to components, composites, and late-game items)

### Resource Changes
- **Power Cells** moved to Crystal track (mid-game, Crystal Matrix × 5 + Iron Plate × 2)
- **Repair Kits** added as early-game repair resource (Iron Plate × 2 + Magnetite Ingot × 3)
- **Helium** added as passive secondary yield from all asteroid mining (~2 per 100 ore)
- **Fuel Cell** added (Fuel Tank + Helium × 5) — Gas track consumable
- **Plasma Cell** added (Fuel Cell + Crystal Matrix × 2) — premium fuel
- **Void Core** added (Space Frame × 3 + Plasma Cell × 2 + Charged Plate × 2) — Stargate activation

### Ship Changes
- **Engine tier system** added: Mk I (180.0) through Mk V (500.0), permanent upgrades
- **Fuel Boost system** added: optional consumable speed burst (×1.8 or ×2.4 multiplier)
- `SHIP_SPEED` constant becomes Engine Mk I base — still 180.0

### Station Repair Change
- Repair cost changes from **25 Power Cells** to **5 Repair Kits**
- This makes the opening sequence: Mine → FORGE ingots → CRAFTER kits → Repair

---

## Quest Chain Changes

| Quest | Current | Revised |
| :--- | :--- | :--- |
| Q-001 | Locate signal | Unchanged |
| Q-002 | Dock at derelict | Unchanged |
| Q-003 | Repair station (25 Power Cells) | Craft 5 Repair Kits |
| Q-004 | Build AI Core | Repair station (uses Repair Kits) |
| Q-005 | Discover Sector 3 | Mine Helium (passive yield notification) |
| Q-006 | Mine Carbon | Craft 3 Fuel Cells |
| Q-007 | Assemble autonomous ship | Build Engine Mk II |
| Q-008 | (new) | Discover Sector 4 (Tungsten gate) |
| Q-009 | (new) | Build AI Core |
| Q-010 | (new) | Assemble autonomous ship |

---

## Signal Changes Required

Signals that reference the current repair chain need updating:

| Signal ID | Current Text | Required Change |
| :--- | :--- | :--- |
| 9 | `POWER CELLS PRODUCED. REPAIR THRESHOLD: 25.` | Update for Repair Kit threshold |
| 10 | `REPAIR THRESHOLD MET. INITIATE WHEN READY.` | Update trigger condition |
| 12 | `AI CORE FABRICATION NOW AVAILABLE.` | Sequence may shift |

New signals needed:
- Helium passive yield first detection
- Fuel Cell first production
- Engine upgrade confirmation
- Void Core completion
- Stargate activation sequence (multiple signals)

---

## Component Changes Required

### `Station` struct additions
- `iron_reserves: f32`
- `helium_reserves: f32`
- `crystal_reserves: f32`
- `iron_ingots: u32`
- `magnetite_ingots: u32`
- `carbon_rods: u32`
- `tungsten_bars: u32`
- `titanite_ingots: u32`
- `crystal_matrices: u32`
- `repair_kits: u32`
- `fuel_cells: u32`
- `plasma_cells: u32`
- `void_cores: u32`

### `Ship` struct additions
- `engine_tier: EngineTier` (replaces raw speed constant)
- `fuel_boost_active: bool`
- `fuel_boost_timer: f32`

### `StationQueues` expansion
- Current: 4 queues (magnetite_refinery, carbon_refinery, hull_forge, core_fabricator)
- Redesigned: up to 10+ queues across FORGE and CRAFTER departments

### `ProcessingOperation` expansion
- Current: 4 variants
- Redesigned: ~14 variants covering all FORGE and CRAFTER recipes

### `ActiveStationTab` changes
- `Smelter` → `Forge`
- `Forge` → `Crafter`
- Add `Market` and `Fleet` when those departments are implemented

---

## `OreType` Changes Required

Current `OreType` enum only has `Empty`, `Magnetite`, `Carbon`.
Iron, Tungsten, Titanite, CrystalCore all map to `OreType::Magnetite` in cargo logic.

Redesign requires distinct `OreType` variants for all six ore types so cargo
can be tracked and deposited to the correct reserve when unloaded.

---

## Execution Prerequisites

Before this phase can be executed:
1. Bevy UI migration complete — all egui panels replaced
2. Code refactor complete — `setup.rs`, `ui.rs`, `narrative.rs` split into focused modules
3. The new module structure provides clean insertion points for the expanded economy

---

## See Also

- `docs/design/ECONOMY.md` — full resource/department/recipe specification
- `docs/design/STARGATE.md` — Stargate design and Void Core activation
- `docs/design/VISION.md` — game identity and design pillars
- `docs/design/UI_VISION.md` — UI layout for new department tabs
