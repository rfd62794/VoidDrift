# Voidrift

A mobile-first space mining game built in Rust with Bevy 0.15, targeting Android (API 35).

You are adrift. You find a derelict station. You mine, refine, and repair — watching your ship work while you direct it from above.

> **Status:** MVP Slice complete. Core loop proven on hardware. Active development.

---

## What's Been Built

The MVP slice is a fully playable, gated loop verified on a physical Moto G 2025 (API 35):

- **Navigate** — tap destinations on a map overlay, autopilot executes
- **Mine** — ship arrives at asteroid field, ore accumulates automatically
- **Refine** — dock at station, convert ore to power cells at 10:1
- **Repair** — spend 25 power cells to bring the derelict station online
- **Slice complete** — station changes state, overlay confirms

![Map View](screenshots/gate2_screenshot.png)
![Mining / Refinery](screenshots/gate4_map.png)
![Slice Complete](screenshots/p5_active.png)

Five development phases, each gated by physical device evidence before proceeding.

---

## Technical Stack

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust | Performance, safety, cross-platform target |
| Engine | Bevy 0.15 | ECS architecture, 2D sprite rendering, Android support |
| UI | bevy_egui 0.33 | Stable font rendering on Mali GPU — Text2d and camera-parented Mesh2d both proven unreliable on Android 15 |
| Build | cargo-ndk + Gradle | Modern Android pipeline — cargo-apk is deprecated |
| Activity | GameActivity | Bevy 0.15 default, requires API 31+ |
| Target | aarch64-linux-android (API 35) | Moto G 2025 primary test device |

---

## Architecture Decisions

Four ADRs govern the build. Each was made in response to real hardware evidence, not assumption.

**ADR-001 — PresentMode::Fifo is mandatory**  
The Mali GPU on the Moto G 2025 causes buffer starvation (`Can't acquire next buffer`) with any other present mode. Fifo is a hard requirement, not a preference.

**ADR-002 — Mesh2d for world-space primitives**  
Sprite components trigger unsupported gralloc format errors (`0x38`, `0x3b`) on the Mali driver. All world-space entities use Mesh2d.

**ADR-003 — bevy_egui for all HUD and UI**  
Text2d and camera-parented Mesh2d both fail silently on this hardware — text renders invisible, panels clip out of existence. bevy_egui handles its own render pass and font atlas, bypassing the driver entirely. Scale factor: `EGUI_SCALE = 3.0` for the Moto G 2025 high-DPI display.

**ADR-004 — Bevy 0.15 pinned, not latest**  
0.15 has the most mature Android community documentation. The GameActivity + cargo-ndk pipeline was established at 0.15. Upgrading to a newer version is deferred until the slice is complete.

Full ADR documents are in [`docs/adr/`](docs/adr/).

---

## Key Constants

```rust
const SHIP_SPEED: f32       = 120.0;  // world units per second
const CARGO_CAPACITY: u32   = 100;    // ore units
const MINING_RATE: f32      = 8.0;    // ore per second
const REFINERY_RATIO: u32   = 10;     // ore per power cell
const REPAIR_COST: u32      = 25;     // power cells to complete slice
const EGUI_SCALE: f32       = 3.0;    // Mali GPU / Moto G 2025
```

---

## Building for Android

### Prerequisites

- Rust (1.95.0+)
- Android SDK with NDK r26+
- `cargo-ndk` (`cargo install cargo-ndk`)
- ADB with USB debugging enabled on target device

### NDK Configuration

`.cargo/config.toml` targets `aarch64-linux-android35-clang`. Update the linker path to match your NDK installation:

```toml
[target.aarch64-linux-android]
linker = "path/to/ndk/toolchains/llvm/prebuilt/windows-x86_64/bin/aarch64-linux-android35-clang.cmd"
rustflags = [
    "-C", "link-arg=-lc++_shared",
    "-C", "link-arg=-Wl,-z,max-page-size=16384",
]
```

The `max-page-size=16384` flag is required for Android 15+ physical devices.

### Build & Deploy

```powershell
.\build_android.ps1
```

The script runs: prerequisite check → cargo-ndk compile → Gradle package → ADB install → logcat.

### Keystore

A signing keystore is required for release builds. Generate one and update `build_android.ps1` with the path. Do not commit the keystore to the repository.

---

## Project Structure

```
VoidDrift/
├── src/
│   ├── lib.rs              # App setup and plugin registration only
│   ├── constants.rs        # All game constants
│   ├── components.rs       # All ECS components and resources
│   └── systems/
│       ├── mod.rs          # Module declarations
│       ├── setup.rs        # World setup and entity spawning
│       ├── autopilot.rs    # Ship movement and docking
│       ├── mining.rs       # Ore extraction and mining beam
│       ├── economy.rs      # Refinery, forge, power economy
│       ├── autonomous.rs   # Autonomous ship state machine
│       ├── visuals.rs      # Starfield, thruster glow, effects
│       ├── ui.rs           # egui HUD and docking panel
│       └── map.rs          # Input, camera, map view transitions
├── android/                # Gradle wrapper for Android APK
├── assets/fonts/           # FiraSans-Bold.ttf
├── docs/
│   ├── adr/                # 6 Architectural Decision Records
│   ├── phases/             # Phase summaries (archival)
│   └── state/current.md    # Always-current project state
├── build_android.ps1       # Full build + deploy pipeline
└── Cargo.toml
```

---

## Development Approach

Voidrift is built using **Spec-Driven Development** — every phase is designed before implementation, gated by physical device evidence before proceeding.

The phase structure:

| Phase | Deliverable | Gate |
|-------|------------|------|
| 0 | Bevy hello world on Android | Screenshot + touch logcat on device |
| 1 | Static scene — ship, asteroid, station | Three entities visible simultaneously |
| 2 | Map + navigation | Ship navigates to tapped destination |
| 3 | Mining system | Cargo fills, stops at capacity |
| 4 | Station UI + refinery | Ore converts to power cells via egui |
| 5 | Repair + slice complete | Station comes online, overlay confirms |

Agent summaries are not accepted as gate evidence. Physical device screenshots and raw logcat output are required at every gate.

---

## What This Demonstrates

- **Rust + Bevy on Android** — a non-trivial pipeline involving NDK configuration, GameActivity, cargo-ndk, and Gradle, solved from scratch and documented
- **Hardware-driven architecture** — three ADRs emerged from real device failures, not theory. The Mali GPU constraints are real and the solutions are proven
- **ECS design discipline** — components, systems, and state machines designed before implementation
- **Constraint-based scope management** — five explicit exclusion lists across five phases kept the slice from becoming six games at once
- **Governance artifacts** — SDD, ADRs, and phase directives as a public engineering record

---

## License

MIT — see [LICENSE](LICENSE).  
Commercial rights retained by RFD IT Services Ltd.

---

*Built by [RFD IT Services Ltd.](https://rfditservices.com) — April 2026*
