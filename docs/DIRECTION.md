# VoidDrift — Direction

*Drafted 2026-09-22 from repo evidence only. Where a claim could drift, the source is named.*

## Purpose

VoidDrift (*"Asteroids meets Event Horizon"*) is a mobile arcade mining game in
Rust/Bevy: mine asteroid debris at the edge of a black hole, build an autonomous
drone fleet, and receive contact from factions you don't understand
(README.md). The design vision frames the player as Fleet Commander and Station
Manager — Survivor → Station Founder → Fleet Commander → Galaxy Builder — with
permanent choices and no win condition (docs/design/VISION.md). It is sold as a
$4.99 premium title on itch.io, no microtransactions or ads (ADR-017,
docs/state/current.md "Pricing Strategy").

## Current state

- Live on itch.io, tagged `v2.8.7-tutorial-4a` — "Phase 4a Complete"
  (README.md status line). docs/state/current.md records build
  `v3.1.0-sprint5-visual-overhaul` (May 2026); git history since then is newer
  than both docs.
- Working per README.md "What's Working": autonomous drone fleet
  (mine → return → unload), PRODUCTION and REQUESTS tabs, bottle collection,
  radial asteroid spawning with global cap, `power_multiplier` wired to mining,
  star map, save/load including RequestsTabState, opening cinematic, tutorial
  T-101..T-107 (git log `ae9ba42`, `928e426`).
- Architecture: Layer 1 Engine / Layer 2 Game / Layer 3 Presentation
  (ADR-016, docs/state/current.md). Drone logic is Architecture B only — one
  `AutonomousShip` FSM under `src/drone/`; `Ship`+`AutopilotTarget` survives
  only for the opening ship and bottle carriers (ADR-021).
- Economy as implemented: four ores (Iron, Tungsten, Nickel, Aluminum) → ingots
  → components → drones, 8 parallel processing queues, station repair at 25
  Iron Ingots (docs/state/current.md "Current Economy").
- Test floor: `src/tests/` holds integration suites (dispatch, economy, fleet,
  fsm, invariants). `verify.ps1` is the documented harness: `cargo check` then
  `cargo test`. Note: current.md still claims "No automated tests" — stale;
  `src/tests/` landed in commit `51fd829`.
- A second codebase lives in `web/`: a browser-native TypeScript rebuild
  (Vite + canvas-style renderer) merged in from the Replit line
  (replit.md, git log `f139d55`, `3664d7f`). It has constants, state, save,
  hud, renderer, and asteroids/drones/production systems — early, no test
  script; `npm run build` (vite build) is its only checkable gate.
- CI/CD was moved off GitHub Actions to local scripts (git log `8f7e776`,
  `34eb511`).
- The prose roadmap `docs/roadmap.md` is an April 2026 snapshot — it lists
  Phase 3 as "next" although Phase 3 and Phase 4a have since shipped. Treat its
  phase table as history, not plan.

## Next steps

Ordered by the repo's own stated urgency:

1. **Launch blockers — "ship before $4.99"** (docs/state/current.md "Open
   Directives"): #9 audio pass, #7 production tree zoom/scroll, #18 asteroid
   density (`max_per_field = 1` in assets/balance.toml is "too sparse"), #13
   tutorial refinement / symbol-bar remainder, #5 Android store assets, #11
   Bevy community post.
2. **Structural rework** (current.md "Known Technical Issues" + issues
   #16/#19/#23–#32, ADR-021 "Known Debt"): split the god classes
   (`hud/mod.rs`, `resources.rs`, `save.rs`, `main_menu.rs`,
   `component_nodes.rs`), shared `rocket_spawner`/`ore_mesh_builder`,
   config-driven signal triggers, laser-tier validation to config, dead-code
   removal, factory drones via `spawn_drone_ship_with_visuals`, empty `mod.rs`
   cleanup.
3. **Phase 4b — narrative drops** (README.md roadmap table, docs/roadmap.md
   Phase 4): memory fragments via bottles, faction voice differentiation,
   First Human and Pirate faction bottles/requests, no dialogue trees or
   cutscenes.
4. **Economy redesign** (docs/phases/phase-economy-redesign-planned.md,
   docs/design/ECONOMY.md, STARGATE.md): three-track economy, engine tiers
   Mk I–V, fuel boost, Helium passive yield, Void Core / Stargate. The phase
   doc predates the current four-ore economy (it describes a Magnetite/Power
   Cell chain) — reconcile the spec before implementing.
5. **TypeScript web rebuild** (replit.md, `web/`): continue toward parity with
   the Rust/Bevy game. If this becomes the primary line, it likely stops the
   Rust roadmap — see `stop_if` in docs/roadmap.md.

## Definition of done

- `cargo check` and `cargo test` pass from the repo root — the verify.ps1 gate
  (verify.ps1).
- Physical device evidence at every gate on the Moto G 2025
  (AGENT_CONTRACT.md INVARIANTS `hardware`; capture via
  `capture_gate_evidence.ps1`, BUILDING.md).
- No phase begins without the prior gate passing on device
  (AGENT_CONTRACT.md INVARIANTS `phases`).
- Ship via `publish.ps1` / Butler to the `html5` channel; tag the release
  (AGENT_CONTRACT.md DEPLOY, BUILDING.md).

## Do not

From AGENT_CONTRACT.md INVARIANTS, Cargo.toml comments, and ADRs:

- Do not upgrade Bevy — pinned at 0.15.3 (ADR-004, Cargo.toml comment).
- Do not change `PresentMode::Fifo` — mandatory on Mali-G57 (ADR-001).
- HUD is `bevy_egui` only — no Text2d, no camera-parented Mesh2d (ADR-003).
- Respect Universal Disjointness: every `&mut Transform` query needs explicit
  `Without<T>` filters, or Android panics B0001 (ADR-008, INV-004).
- Update schedule tuples cap at 20 systems — check group before adding
  (ADR-007, INV-005).
- Never remove `DockedAt` without first leaving the docked state (INV-006).
- Signal/tutorial `fired`/`shown` sets are one-time; never clear them (INV-007).
- Background meshes use `AlphaMode2d::Opaque`; dim via color, not alpha
  (INV-008).
- `lib.rs` is app setup only; constants live in `constants.rs` / config
  (INVARIANTS `modules`, `constants`).
- Architecture A (`Ship`+`AutopilotTarget`) must not be extended to mining
  drones (ADR-021).
- `wasm-opt = false` in Cargo.toml stays; `pkg/index.html` is hand-maintained
  (BUILDING.md, AGENT_CONTRACT.md WASM_BUILD).
- `.publish.env` is gitignored — never commit it (BUILDING.md).
- No architectural decision without an ADR (AGENT_CONTRACT.md `adrs`).

## Sources of truth

| File | Owns |
| :--- | :--- |
| `AGENT_CONTRACT.md` | Invariants, file registry, WASM/deploy contract |
| `docs/state/current.md` | Implemented-system inventory, open issues, pricing |
| `docs/adr/` | Architectural decisions (ADR-001..ADR-021) |
| `docs/design/VISION.md`, `ECONOMY.md`, `STARGATE.md`, `UI_VISION.md` | Design intent (Layer 2 docs — do not overwrite with code observations) |
| `docs/narrative_canon.md` | Locked narrative foundation |
| `docs/roadmap.md` | Phase history (April 2026) + machine-readable swarm roadmap |
| `replit.md`, `web/` | TypeScript web rebuild line |
| `verify.ps1`, `build_android.ps1`, `build_wasm.ps1`, `publish.ps1` | Build/verify/deploy pipeline |
| `assets/balance.toml`, `assets/visual.toml`, `assets/content/*.yaml` | Game balance, visuals, narrative content |
