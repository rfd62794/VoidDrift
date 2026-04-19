# Project State
updated: 2026-04-18
agent: human

## Status
phase: Post-Slice — Phase 6 Specced, Not Started
test_floor: N/A — Bevy project. Gate evidence on physical device.
last_directive: docs/phases/phase_05_repair_slice_complete.md

## What Is Built
MVP slice complete and verified on Moto G 2025 (API 35). Full loop proven: navigate → mine → refine → repair → station online. Six gated phases. bevy_egui confirmed as stable HUD solution on Mali GPU. Full documentation suite in place. Repository is public.

## What Is Next
Phase 6: AI Core Module. First post-slice feature. Station produces an AI Core (cost: 50 power cells) which spawns one autonomous drone on a fixed mine → return route. This is the first step of the commander arc — the player stops being the ship and starts being the operation.

## Open Questions
- Does the autonomous drone use the same AutopilotSystem as the player ship, or a separate simplified system?
- Should the drone be visually distinct from the player ship (different color rectangle)?
- Does the drone's cargo unload automatically on station arrival, or does the player trigger it?
- Is the AI Core a one-time unlock or a consumable that could be built multiple times (second drone)?

## Economy Constants (Locked)
- SHIP_SPEED: 120.0
- CARGO_CAPACITY: 100
- MINING_RATE: 8.0
- REFINERY_RATIO: 10
- REPAIR_COST: 25
- EGUI_SCALE: 3.0
- AI_CORE_COST: 50  ← NEW — locked in design session

## Post-Slice Roadmap (Named Deferral — Do Not Scope Until Phase 6 Ships)
- Phase 7: Second ore type + second refined product. Economy texture.
- Phase 8: Second module slot. Two modules, two playstyles.
- Save/load persistence — required before any public release
- Sector map expansion — multiple fields, gate network
- Risk layer — ship degradation, breakdown probability
- Threat system — NPC competition, no direct combat
- Notifications — idle session alerts

## Recent Decisions
- ADR-001: PresentMode::Fifo mandatory on Mali GPU
- ADR-002: Mesh2d for world-space primitives
- ADR-003: bevy_egui for all HUD and UI, EGUI_SCALE=3.0
- ADR-004: Bevy 0.15 pinned for Android stability
- AI_CORE_COST=50 locked — design session 2026-04-18
- Two-of-everything ceiling locked for post-slice economy: two ores, two products, two modules
