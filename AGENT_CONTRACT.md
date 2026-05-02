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

### INV-004: Universal Disjointness (Total Lockdown)
Every system that queries `&mut Transform` MUST include explicit `Without<T>` 
filters for all major entity types that other queries in the same system 
might touch. This is non-negotiable on Mali-G57 GPU hardware.

Violating this causes runtime B0001 panics on Android that cargo check 
does not catch.

Required Without filters by entity type:
- Ship queries: Without<Station>, Without<AsteroidField>, Without<MiningBeam>
- Station queries: Without<Ship>, Without<AutonomousShipTag>  
- Beam queries: Without<Ship>, Without<Station>, Without<AsteroidField>
- Camera queries: Without<Ship>, Without<StarLayer>
- Star queries: Without<MainCamera>

### INV-005: System Tuple Partition
The Bevy Update schedule cannot hold more than 20 systems in a single tuple.
Systems are partitioned into two groups in lib.rs:
- Group 1 (Gameplay & Logistics): movement, mining, economy, autopilot, autonomous
- Group 2 (Station, Narrative & UI): visuals, narrative, tutorial, quest, ui, map

Never add a system without checking which group it belongs to and whether
the group is approaching the 20-system limit.

### INV-006: DockedAt Pattern
Ships that are docked at a berth MUST have a DockedAt(Entity) component
pointing to their berth entity. The docked_ship_system and 
docked_autonomous_ship_system use this to track rotating berth position
each tick. Never remove DockedAt without transitioning ship state to 
Navigating or Idle first.

### INV-007: One-Time Trigger Pattern
Signal triggers (SignalLog.fired HashSet) and Tutorial triggers 
(TutorialState.shown HashSet) are one-time only. Never clear these sets
during a session. IDs 19, 20, 21 in SignalLog are exceptions — they 
refire on state re-entry.

### INV-008: AlphaMode2d::Opaque for Background Elements
All background mesh entities (stars, station arms, connectors, asteroid 
boundary rings) MUST use AlphaMode2d::Opaque in their ColorMaterial.
Using Blend on Mali-G57 causes Z-sorting flicker that cannot be fixed 
by Z-layer adjustment alone. Achieve dimming through color values, 
not alpha transparency.

## WASM_BUILD

Build target : wasm32-unknown-unknown
Tool         : wasm-pack
Command      : wasm-pack build --target web --out-dir pkg
Output dir   : C:\Github\VoidDrift\pkg\
Entry point  : src/lib.rs → pub fn start() — gated #[cfg(target_arch = "wasm32")]
index.html   : pkg/index.html — hand-maintained, NOT owned by wasm-pack. Never overwrite it.
No script    : There is no build_wasm.ps1. Run the wasm-pack command directly.
wasm-opt     : Disabled in Cargo.toml [package.metadata.wasm-pack.profile.release]. Do not remove.

## DEPLOY

Pipeline repo  : C:\Github\RFD_IT_Publishing   (separate private repo, not VoidDrift)
Butler binary  : C:\Butler\butler.exe           (must be on system PATH)
Butler auth    : run `butler login` once — credentials stored locally, no .env key needed
Game config    : C:\Github\RFD_IT_Publishing\config\games.yaml → entry: voidrift
itch.io slug   : rdug627/voidrift
channel        : html5
build_dir      : C:\Github\VoidDrift\pkg

Deploy command (from RFD_IT_Publishing root):
    python publisher.py deploy voidrift --target itchio

Dry run (safe, prints command only):
    python publisher.py deploy voidrift --target itchio --dry-run

Raw Butler equivalent (what the pipeline runs):
    butler push C:\Github\VoidDrift\pkg rdug627/voidrift:html5

Full release sequence:
    1. wasm-pack build --target web --out-dir pkg          (from VoidDrift root)
    2. python publisher.py deploy voidrift --target itchio (from RFD_IT_Publishing root)
    3. git tag vX.Y.Z-<slug> && git push --tags            (from VoidDrift root)
