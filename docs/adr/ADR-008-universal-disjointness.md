# ADR-008: Universal Disjointness Architecture

**Date:** April 2026  
**Status:** Accepted

## Context
Bevy's ECS detects conflicting component access at runtime on Android.
When two queries in the same system both access `Transform` (one mutably, one immutably), and Bevy cannot prove they target disjoint entity sets, it panics with error `B0001: Query conflict`. 

This panic does not appear in `cargo check` — it only occurs on device. The Mali-G57 GPU in the Moto G 2025 (primary test device) triggered this consistently, leading to unstable builds that appeared fine in local compilation.

## Decision
Every system that uses `&mut Transform` MUST include `Without<T>` filters proving disjointness from all other Transform queries in the same system. This is known as the "Total Lockdown" pattern.

Standard exclusion filters:
- **Ships**: Must exclude `Station`, `AsteroidField`, `MiningBeam`.
- **Station**: Must exclude `Ship`, `AutonomousShipTag`.
- **UI/Map Labels**: Must exclude primary world entities.

## Consequences
- All new systems must adopt this filtering from the start.
- Adding a new primary entity type requires auditing existing systems for potential query collisions.
- Query definitions become verbose but ensure 100% stability on mobile hardware.
- The pattern is easily verified via project-wide grep for `&mut Transform`.

## Alternatives Considered
- **ParamSet**: A valid alternative, but results in more complex borrow-checking code and decreased readability. Rejected in favor of explicit query filtering.
- **System Splitting**: Systems are split when filtering becomes unmanageable, but filtering remains the primary defense.
