# VoidDrift

*Asteroids meets Event Horizon — a mobile arcade mining game.*

## Project Overview

VoidDrift is an arcade mining game where you manage a drone fleet to mine asteroid debris at the edge of a black hole. The original codebase is written in **Rust/Bevy**, targeting **Android** and **WASM**.

**The plan on Replit:** Rebuild as a **Native Web App in TypeScript** (browser-first, no Rust/WASM build chain required).

## Game Loop

- Drones mine asteroids autonomously (mine → return → unload)
- Ore refines and forges into components
- Faction "bottles" drift in from the event horizon — collect them, read the signal, fulfill requests
- Upgrades accelerate the loop
- No win condition. No escape.

## Original Stack (Rust/Bevy — for reference)

| Layer | Tech |
|-------|------|
| Language | Rust |
| Engine | Bevy 0.15.3 |
| UI | bevy_egui 0.33 |
| Target | Android API 35 / WASM |

## Target Stack (TypeScript Web Rebuild)

Browser-native TypeScript game. Stack TBD during rebuild planning — likely canvas/WebGL renderer with ECS-style architecture mirroring the original Bevy design.

## Key Source Docs (Original)

- `docs/ARCHITECTURE.md` — full system breakdown (Layer 1/2/3: Engine/Game/Presentation)
- `docs/DEVELOPER.md` — build and tooling notes
- `assets/balance.toml` — all game balance constants
- `assets/visual.toml` — visual config
- `assets/content/` — narrative content (signals, faction text, etc.)
- `src/` — full Rust source, organized by system

## User Preferences

- Rebuild target: Native Web App (TypeScript)
- Keep game design faithful to the original Rust/Bevy implementation
