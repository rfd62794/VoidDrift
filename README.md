# Voidrift

An arcade mining and production game where you've crashed into a black hole and merged with a dying station AI. Build a drone army, discover faction secrets, and determine your fate.

You're stranded. The station AI fused with your consciousness to save you. Now you mine asteroids, build drones, and uncover what happened at the event horizon boundary.

> **Status:** Core Gameplay Loop & Industrial Foundation Complete. Verified on Moto G 2024/2025.

---

## 🚀 The Premise & Loop

**The Narrative Frame:** You crashed into a black hole. The station AI merged with your consciousness to survive. Now you're extensions of each other, working together to understand what happened.

**The Gameplay Loop:** Mine asteroids → Refine resources → Build drones → Discover faction secrets

- **Mining** — Extract ore to power the station and your merged consciousness. Survival depends on resource flow.
- **Auto-Processing** — The station AI handles routine refining and production while you focus on strategic decisions.
- **Drone Fleet** — Build autonomous drones that are literally your thoughts made physical. They explore where you cannot.
- **Faction Discovery** — Uncover signals from other trapped ships at the event horizon boundary. Each has secrets about what happened.
- **Black Hole Boundary** — The map edge isn't arbitrary — it's the point of no return. Stars cut off visually where physics breaks down.

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
| 6-7 | Narrative Frame | The Signal log + Cinematic opening sequence (stranded + AI fusion) |
| 8 | Industrial Core | Parallel processing queues + Drone fleet expansion |
| 9 | Discovery | Faction signals + Black hole boundary visualization |
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
