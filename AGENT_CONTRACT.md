# AGENT_CONTRACT
version: 1.0
repo: VoidDrift
updated: 2026-04-18

## STRUCTURE
src/lib.rs          : App setup and plugin registration only
src/constants.rs    : All game constants — single source of truth
src/components.rs   : All Component and Resource structs
src/systems/        : Modular system implementations (9 logic files)
  autopilot.rs      : Ship movement and navigation
  mining.rs         : Ore extraction and mining beam visuals
  economy.rs        : Refinery, forge, and power economy
  autonomous.rs     : AI drone state machine and routing
  visuals.rs        : Starfield, thruster glow, and rotation
  ui.rs             : egui HUD and world-space UI
  map.rs            : Input handling and camera control
  setup.rs          : Entity spawning and mesh generation
android/            : Gradle project for Android packaging
docs/adr/           : Architectural Decision Records
docs/phases/        : Phase archival summaries
docs/state/         : current.md (always current)

## FILE_REGISTRY
src/systems/*       | Feature logic        | agent  | every session
src/lib.rs          | App setup            | agent  | on setup change
src/components.rs   | Data structures      | agent  | on data change
src/constants.rs    | Game tuning          | both   | every session
docs/state/current.md| Project status       | both   | every session
docs/adr/ADR-NNN.md  | Decision records     | human  | on decision
docs/phases/phase_*.md| Phase summaries      | agent  | on phase complete

## INVARIANTS
hardware    : Physical device evidence required at every gate
scope       : Every directive lists explicit file scope
adrs        : No architectural decision without an ADR
phases      : No phase begins without prior gate passing on device
build       : PresentMode::Fifo mandatory — do not change
ui          : bevy_egui only for HUD — no Text2d, no camera-parented Mesh2d
modules     : lib.rs is app setup only — no logic, no components, no constants
constants   : All constants in constants.rs — never hardcode inline
