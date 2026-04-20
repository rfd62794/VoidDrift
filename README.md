# Voidrift

A mobile-first space mining and industrial management game built in Rust with Bevy 0.15, targeting Android hardware (API 35).

You are adrift. You find a derelict station. You mine, refine, and repair — watching your ship work while you direct it from above.

> **Status:** Core Gameplay Loop & Industrial Foundation Complete. Verified on Moto G 2024/2025.

---

## 🚀 The Gameplay Foundation

Voidrift is built around a tight industrial loop directed by tactical input:

- **Command & Control** — Set navigation targets via a strategic map. Autopilots handle precise approach and docking.
- **Extraction** — Reach asteroid fields and extract raw ores automatically. Advanced ores require upgraded mining lasers.
- **Parallel Processing** — Utilize the station's refinery, hull forge, and fabrication lab. Manage four independent production queues simultaneously.
- **Power Economy** — Maintain both ship and station power reserves. Use refined power cells to restore charge or complete massive structural repairs.
- **Narrative Telemetry** — Stay oriented through "The Signal," a low-frequency character voice reporting critical events and guidance.

---

## 🛠️ Technical Architecture

### Hardware-Hardened Design
Voidrift's architecture is a direct response to constraints discovered on physical Mali-G57 GPU hardware. We follow **Universal Disjointness** (ADR-008) to prevent runtime crashes and use dedicated egui render passes (ADR-003) for UI stability.

| Layer | Choice | Rationale |
|-------|--------|-----------|
| Language | Rust | Performance, memory safety, aarch64 target |
| Engine | Bevy 0.15 | ECS-first architecture, partitioned update schedule |
| UI | bevy_egui 0.33 | High-fidelity text/HUD rendering on mobile |
| Build | cargo-ndk r29 | Native Android binary generation |
| Target | API 35 | moto g play - 2024 (primary test device) |

### Key ADRs
Nine Architectural Decision Records govern the project. See [`docs/adr/`](docs/adr/) for detailed logic.

- **ADR-003**: `bevy_egui` for all HUD elements (Mali GPU stabilization).
- **ADR-007**: System Partitioning (bypassing Bevy's 20-tuple limit).
- **ADR-008**: Universal Disjointness (Total Lockdown pattern for Transform queries).
- **ADR-009**: Tutorial Trigger Pattern (one-time contextual instructional logic).

---

## 📦 Project Structure

```
VoidDrift/
├── src/
│   ├── lib.rs              # App entry & system partitioning groups
│   ├── constants.rs        # Centralized game & balance constants
│   ├── components.rs       # ECS components & global resources
│   └── systems/            # Modular logic by domain (mining, economy, etc.)
├── android/                # Gradle wrapper for Android APK packaging
├── assets/                 # Mesh data, materials, and fonts
├── docs/
│   ├── adr/                # 9 Structural Architectural Decisions
│   ├── directives/         # Past implementation blue-prints and SDDs
│   ├── phases/             # Detailed archival summaries per phase
│   ├── state/              # [current.md] Always-accurate codebase audit
│   ├── ARCHITECTURE.md     # Deep technical dive into data flow
│   └── CHANGELOG.md        # Reconstruction of all development cycles
├── build_android.ps1       # One-click build + deploy + logs pipeline
└── Cargo.toml
```

---

## 🚢 Development Roadmap

Voidrift is built in **Phases**, each gated by physical hardware verification.

| Phase | Milestone | Key Deliverable |
|-------|-----------|-----------------|
| 1-3 | Navigation | Touch destination input + Autopilot movement |
| 4-5 | Loop Completion | Core refinery loop + Station repair gate |
| 6-7 | Narrative | The Signal log + Cinematic opening sequence |
| 8 | Industrial Core | Parallel processing queues + Power Cell restore |
| 9 | Gated Galaxy | World-space text labels + Pinch zoom + 6 Ore Sectors |
| 10 | Tutorial UX | Contextual popups + Gated progression guidance |

---

## 🏗️ Building for Android

### Prerequisites
- Rust 1.95+
- Android SDK/NDK r26+
- `cargo-ndk` (`cargo install cargo-ndk`)

### Execution
Run the automated pipeline to compile, package, and deploy directly to a connected device:

```powershell
.\build_android.ps1
```

*Flags for Android 15+: The build automatically applies `max-page-size=16384` for 16kb page compatibility.*

---

## ⚖️ License
MIT — Dedicated to local mobile-first Rust engineering.  
*Built by [RFD IT Services Ltd.](https://rfditservices.com) — April 2026*
