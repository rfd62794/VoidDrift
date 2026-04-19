# Project State
updated: 2026-04-19
agent: Antigravity

## Status
phase: Post-Phase 9 — Visual Polish Complete, Map Polish Next
test_floor: Moto G 2025 (API 35) — Build verified clean.
last_directive: Voidrift_Documentation_Update_Directive.md

## What Is Built
Full production chain economy running on Moto G 2025 (API 35). Two-track 
resource system (Magnetite/Carbon), autonomous fleet with smart routing, 
power economy with self-preservation, station AI telemetry. Visual polish 
complete through Step 5: parallax starfield, asteroid polygons, ship 
triangles with rotation, thruster glows, mining beams. Codebase modularized 
into 8 system files + dedicated constants/components. Ship does not yet stop 
short of asteroid — known issue, deferred.

## What Is Next
- [x] Step 7: Narrative Signal (Opening Sequence, Scripted Telemetry)
- [/] Step 8: Station Architecture (Rotation, Berths, Dual-Station model)
- [ ] Step 9: NPC Visitors (Traders, Wanderers, Berth 3 logic)
- [ ] Step 10: Drone Depot Construction (Infrastructure expansion)
After map polish: module-aware ADR documentation, then post-slice economy 
expansion (five-ore mineral map, laser tiers, sector progression).

## Known Issues
- Ship does not stop short of asteroid on arrival — overshoots slightly

## Open Questions
- SECTOR_3_POS confirmed as (-200.0, 300.0)? Verify against autonomous logic
- Refinery and power inline chunks in economy.rs — extraction named correctly?

## Economy Constants (Locked)
SHIP_SPEED: 180.0
CARGO_CAPACITY: 100
MINING_RATE: 20.0
REFINERY_RATIO: 10
HULL_REFINERY_RATIO: 5
REPAIR_COST: 25
AI_CORE_COST_CELLS: 50
HULL_PLATE_COST_CARBON: 5
SHIP_HULL_COST_PLATES: 3
POWER_COST_CYCLE_TOTAL: 4
POWER_COST_REFINERY: 1
POWER_COST_HULL_FORGE: 2
POWER_COST_SHIP_FORGE: 3
POWER_COST_AI_FABRICATE: 5
SHIP_POWER_MAX: 10.0
SHIP_POWER_FLOOR: 3.0
EGUI_SCALE: 3.0

## Post-Slice Roadmap (Captured — Not Yet Scoped)
Five-ore mineral map: Magnetite, Iron, Carbon, Tungsten, Titanite
Laser tiers: Basic (Steel) → Tungsten → Composite (Crystal Matrix)
Sector progression: Sectors 1-5, geographic gates
Asteroid cores: Laser-gated deeper material in existing fields
Crystal layer: TBD design session
Power Core: Power Cells + Crystal Matrix (concept, not specced)
Trader mechanic: First Tungsten Laser via trade (Autonomous Ship + Titanium Hull)
Blueprint system: Discovered/purchased recipes
Pause/Resume autonomous ships
Ship ceiling review (3 ships?)
Async background refinery
Hex map / procedural system generation (long term)

## Recent Decisions
- ADR-001: PresentMode::Fifo mandatory on Mali GPU
- ADR-002: Mesh2d for world-space primitives
- ADR-003: bevy_egui for all HUD and UI, EGUI_SCALE=3.0
- ADR-004: Bevy 0.15 pinned for Android stability
- ADR-005: Autonomous agents use dedicated systems
- ADR-006: Module structure — lib.rs split into systems/
- MINING_RATE tuned to 20.0, SHIP_SPEED to 180.0 for mobile RTT
- Five-ore economy design locked: Magnetite/Iron/Carbon/Tungsten/Titanite
