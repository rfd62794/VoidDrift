# VoidDrift

A mobile arcade mining game built in Rust/Bevy for Android. Mine asteroid debris at the edge of a black hole, build a drone fleet, and receive contact from factions you don't understand.

> **Status:** Phase 2 Complete — `v2.2.0-docs-phase2-complete` — Verified on Moto G 2025

---

## The Game

The asteroid fields are debris from what the black hole eats. You mine them. The station has been here longer than you. It should have been consumed. It has not been. You don't know why.

Factions send bottles across the event horizon. You don't know how. Resources deduct. Upgrades apply. What happens in between is outside the frame.

There is no win condition. There is no escape. The horizon is a one-way membrane.

**The loop:**
- Drones mine asteroids autonomously
- Ore refines and forges into components
- Bottles arrive — you collect them, read the signal, fulfill requests
- Upgrades accelerate the loop
- The loop continues

---

## What's Working

| System | State |
|--------|-------|
| Autonomous drone fleet (mine → return → unload) | ✅ Live |
| PRODUCTION tab — Iron / Tungsten / Nickel / Aluminum | ✅ Live |
| REQUESTS tab — Signal faction, First Light request | ✅ Live |
| Bottle collection mechanic — drift, tap, dual output | ✅ Live |
| Random radial asteroid spawning, global cap enforced | ✅ Live |
| `power_multiplier` wired to mining rate (+25% after First Light) | ✅ Live |
| Circular star map, station-centered, absolute parallax | ✅ Live |
| Save / load persistence including RequestsTabState | ✅ Live |
| Opening cinematic sequence | ✅ Live |

---

## Technical Architecture

Built for the **Moto G 2025 (720×1604, Mali-G57 GPU)**. Every architectural decision is made for that specific device.

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust | Performance, memory safety, `aarch64` target |
| Engine | Bevy 0.15.3 (pinned) | ECS-first, partitioned update schedule |
| UI | bevy_egui 0.33 | Stable text rendering on Mali GPU |
| Build | cargo-ndk r29 | Native Android binary generation |
| Target | Android API 35 | Moto G 2025 primary test device |

11 Architectural Decision Records govern the project. See [`docs/adr/`](docs/adr/).

Key ADRs:
- **ADR-003**: `bevy_egui` for all HUD (Mali GPU stabilization)
- **ADR-007**: System partitioning (Bevy's 20-tuple schedule limit)
- **ADR-008**: Universal Disjointness — `Without<T>` filter standard for all `&mut Transform` queries
- **ADR-010**: Narrative scope — what gets explained and what stays unexplained

---

## Project Structure

```
VoidDrift/
├── src/
│   ├── lib.rs                  # App entry & system scheduling
│   ├── constants.rs            # All game & balance constants
│   ├── components/             # ECS components & resources (split by domain)
│   │   ├── game_state.rs       # Station, Ship, AutonomousShip, Berth
│   │   ├── markers.rs          # Marker components (StarLayer, MapMarker, etc.)
│   │   ├── resources.rs        # ECS Resources (SignalLog, ShipQueue, etc.)
│   │   ├── ui_state.rs         # UI-only state (DrawerState, RequestsTabState)
│   │   └── utilities.rs        # Helper functions (ore_name, berth_world_pos)
│   └── systems/                # Modular logic by domain
│       ├── asteroid/           # Spawn, lifecycle, cap enforcement
│       ├── game_loop/          # Mining, auto-process, autonomous drones
│       ├── ship_control/       # Autopilot, asteroid input
│       ├── ui/                 # egui HUD, tabs, tutorial
│       ├── narrative/          # Opening sequence, signal, bottle, quest
│       ├── persistence/        # Save / load
│       └── visuals/            # Starfield, station rotation, map, viewport
├── android/                    # Gradle wrapper for Android APK packaging
├── assets/                     # Fonts
├── docs/
│   ├── adr/                    # 11 Architectural Decision Records
│   ├── directives/             # Implementation directives (agent contracts)
│   ├── phases/                 # Phase-by-phase archival summaries
│   ├── narrative_canon.md      # Locked narrative foundation
│   ├── ARCHITECTURE.md         # Deep technical reference
│   ├── GDD.md                  # Game Design Document
│   └── CHANGELOG.md            # Full development history
├── build_android.ps1           # One-click build + deploy pipeline
└── Cargo.toml
```

---

## Roadmap

| Phase | Status | Description |
|-------|--------|-------------|
| Phase 1 | ✅ Complete | Core mining → refining → drone building loop |
| Phase 1c | ✅ Complete | Asteroid lifecycle, lifespan timers, stuck-ship safety |
| Phase 2 | ✅ Complete | UI refactor, Requests framework, bottle mechanic, random spawn |
| Phase 3 | 🚧 Next | Architectural refactor — SRP / event bus, decouple narrative from simulation |
| Phase 4 | 🔮 Planned | Narrative drops — memory fragments, faction voice differentiation |

---

## Building for Android

### Prerequisites
- Rust 1.85+
- Android SDK + NDK r29
- `cargo-ndk` (`cargo install cargo-ndk`)
- Connected Android device with USB debugging enabled

### Build & Deploy

```powershell
.\build_android.ps1
```

Compiles, packages, and deploys directly to the connected device. Logcat output follows automatically.

*Applies `max-page-size=16384` for Android 15+ 16kb page compatibility.*

---

## License
MIT — *Built by [RFD IT Services Ltd.](https://rfditservices.com) — April 2026*
