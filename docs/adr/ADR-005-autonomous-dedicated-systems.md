# ADR-005: Autonomous Agents Use Dedicated Systems, Not AutopilotTarget
**Date:** April 2026  
**Status:** Accepted  

## Context
As the project expands to include non-player agents (e.g., autonomous drones), a decision was required on whether to reuse the existing `AutopilotTarget` system or implement dedicated logic for autonomous flight. Reusing `AutopilotTarget` would require expanding `AutopilotSystem` to handle complex state transitions (mine-return-loop) for multiple entity types.

## Decision
Autonomous agents (drones, future ships) will manage their own movement and state logic within dedicated Bevy systems rather than utilizing the `AutopilotTarget` component.

## Rationale
Decoupling autonomous agents from the player-controlled `AutopilotTarget` system prevents architectural drift and "God System" bloat. Dedicated systems (e.g., `drone_system`) can be optimized for specific loops (mining, hauling) without risking regressions in the player's core navigation experience. This allows for cleaner fleet management and diverse AI behaviors in future phases.

## Consequences
- **Positive**: Clean separation of concerns between player-driven and AI-driven navigation.
- **Positive**: Easier to implement entity-specific logic (e.g., drone-specific unloading) without branching logic in the core autopilot.
- **Constraint**: Requires a dedicated system for every primary agent class, slightly increasing the system count in `src/lib.rs`.
