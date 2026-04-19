# AGENT_CONTRACT
version: 1.0
repo: VoidDrift
updated: 2026-04-18

## STRUCTURE
src/           : Game source. lib.rs is the single source file for the slice.
android/       : Gradle wrapper project for Android APK packaging.
assets/        : Game assets. fonts/ contains FiraSans-Bold.ttf.
docs/adr/      : Architectural Decision Records. Locked after acceptance.
docs/phases/   : Phase summaries. Archival — never edited after creation.
docs/state/    : current.md only. Always current.
.cargo/        : NDK linker configuration. config.toml is build-critical.

## FILE_REGISTRY
src/lib.rs              | All game systems    | agent  | per phase
docs/state/current.md   | Project state       | both   | every session
docs/adr/ADR-NNN.md     | Decision records    | human  | on decision
docs/phases/phase_NN.md | Phase summaries     | agent  | on phase complete
AGENT_CONTRACT.md       | This file           | human  | on structural change

## INVARIANTS
hardware: Physical device evidence required at every gate. Agent summaries not accepted.
scope: Every directive lists explicit file scope. Unlisted files are read-only.
adrs: No architectural decision is made without an ADR. ADRs are locked after acceptance.
phases: No phase begins without the prior gate passing on device.
build: PresentMode::Fifo is mandatory. Do not change without hardware re-verification.
ui: bevy_egui only for HUD and screenspace UI. No Text2d, no camera-parented Mesh2d.
