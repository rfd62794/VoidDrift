# ADR-020: Desktop Config Hot-Reload via Bevy AssetServer
**Date:** May 2026
**Status:** Proposed

## Context
VoidDrift configs load differently by target:

- **WASM / Android:** `include_str!` at compile time — already implemented in `src/config/visual.rs`, `src/config/balance.rs`, and `src/config/content.rs`. Configs are baked into the binary and require no filesystem access at runtime.
- **Desktop:** `Box::leak` + `std::fs::read_to_string` at startup, **no file watching**. Configs are read from `assets/visual.toml`, `assets/balance.toml`, and `assets/content/*.yaml` once at boot.

Desktop iteration on visual values (sizes, colors, spacing, balance numbers) currently requires a full binary restart per change. Empirically this is ~60 seconds per iteration cycle (compile, link, launch, navigate to the affected screen). The cost compounds across every UI polish session.

This is the only target where iteration speed is bottlenecked on restart. WASM and Android are not iteration environments — they are deployment targets reached via `bake_wasm.ps1` / `bake_android.ps1` after desktop work is done.

## Decision
Add filesystem watching to desktop config loading via Bevy's `AssetServer`. When `assets/visual.toml` or `assets/balance.toml` changes on disk, reload the corresponding config resource and broadcast a `ConfigChanged` event that affected systems can respond to.

**Scope:**
- **In scope:** desktop dev builds, `visual.toml` and `balance.toml`.
- **Out of scope:** WASM and Android (already baked at compile time, no change).
- **Out of scope:** desktop release embedding via `include_str!`. Deferred to ADR-021 if and when a self-contained desktop release becomes a shipping target.
- **Out of scope:** content YAMLs (`echo`, `tutorial`, `objectives`, `requests`, `logs`). Content changes infrequently and is not the iteration bottleneck this ADR addresses.
- **Out of scope:** Rust code hot-reload — not practical in Rust, and `cargo check` cycles are fast enough that restart-on-code-change is acceptable.

## Rationale
- **WASM and Android already bake configs.** No work is needed on those targets and their behavior is unchanged.
- **Desktop is the only iteration environment.** Optimizing it is high-leverage; optimizing the others is wasted work.
- **`AssetServer` is Bevy-native.** No external dependencies (`notify`, `hotwatch`, etc.) and no custom watch loop. Bevy's existing asset pipeline handles file change detection.
- **`ConfigChanged` event keeps systems decoupled** from the loading mechanism. Systems read the resource as they do today; they additionally subscribe to the event when they need to react to a change (e.g., regenerate a procedural mesh whose parameters live in `visual.toml`).
- **Substrate for future tooling.** A runtime config editor (dev-mode panel that mutates `visual.toml` and saves it) becomes trivial once watching is in place — the editor writes to disk, the watcher picks up the change, the same code path that handles external edits handles editor edits.

## Consequences
- **Positive:** `visual.toml` and `balance.toml` changes reflect in the running game within ~1–2 seconds. No restart, no recompile.
- **Positive:** Substrate for an in-game runtime config editor whenever that becomes worth building.
- **Positive:** Asymmetric cost — only desktop loading code changes. WASM/Android paths are untouched.
- **Constraint:** Systems that read `visual.toml` or `balance.toml` values **and cache derived state** (e.g., procedural meshes built from visual config) must subscribe to `ConfigChanged` and rebuild that state. Systems that re-query the config each frame need no changes.
- **Constraint:** Hot-reloaded changes must not corrupt save state. Config values that are referenced by saved data (e.g., balance numbers embedded in `SaveData`) need a strategy for handling mismatches, but this is a pre-existing concern not introduced by this ADR.
- **Out of scope (deferred):** Desktop release builds will continue to read from disk. If a desktop release ships, ADR-021 can introduce a `#[cfg(not(debug_assertions))]` `include_str!` path mirroring WASM/Android.
