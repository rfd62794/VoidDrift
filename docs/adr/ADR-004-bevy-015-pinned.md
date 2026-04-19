# ADR-004: Bevy 0.15 Pinned for Android Stability
**Date:** April 2026  
**Status:** Accepted  

## Context
During the initial project spike (Phase 0), multiple Bevy versions were evaluated. While newer versions offered experimental features, Bevy 0.15 was identified as having the most mature and battle-tested documentation for the Android `GameActivity` + `cargo-ndk` pipeline.

## Decision
The project will remain pinned to Bevy 0.15.3 and `bevy_egui` 0.33 for the duration of the MVP slice.

## Rationale
The primary risk for Voidrift is hardware-specific regression on Android 15. Maintaining a stable, well-documented engine version eliminates one major variable in the troubleshooting process. Migration to newer Bevy versions (e.g., 0.16+) is deferred until the core mechanical loop is complete and its performance baseline is established.

## Consequences
- **Positive**: Reliable build pipeline and predictable asset management.
- **Positive**: Access to a wealth of community-proven patterns for mobile ECS.
- **Constraint**: Newer engine features (e.g., improved post-processing or lighting) are inaccessible until a strategic migration is approved.
