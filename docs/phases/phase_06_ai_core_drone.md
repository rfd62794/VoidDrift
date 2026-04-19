# Phase 06: AI Core & Autonomous Drone
**Status:** PASSED ✅  
**Date:** April 2026  
**Hardware Verification:** Moto G 2025 (API 35)

## 1. Objective
Transition the game from a manual mining loop to an autonomous operation. The player builds an AI Core (50 power cells) to deploy a drone that automates the ore collection loop.

## 2. Technical Summary
- **ADR-005**: Formalized dedicated AI systems over shared autopilot logic.
- **Ore Pool Refactor**: Implemented `Station::ore_reserves` as a shared input for the refinery.
- **Drone State Machine**: Implemented `Outbound`, `Mining`, `Returning`, and `Unloading` states.
- **UI Logic**: Dynamic Build button and shared reserve status in the refinery panel.
- **UI Cleanup**: Intentionally removed the Phase 5 "STATION ONLINE" overlay to prevent viewport clutter during multi-agent operations.

## 3. Results
- **Loop Confirmation**: Drone successfully completes full mine → return → unload cycles without player input.
- **Shared Pool Logic**: Player ship and Drone both contribute to `ore_reserves`; player refines the combined total.
- **Visual Distinction**: Orange drone color and orange cargo bar provide clear agent differentiation.
- **Spawning Deviation**: The Drone now spawns at the Station and flies to the field (Outbound) rather than appearing at the field. This provides a better "departure" payoff for the build action.

## 4. Evidence
- **Logcat**: `[Voidrift Phase 6] AI Core built. Drone deployed.`
- **Logcat**: `[Voidrift Phase 6] Drone unloaded 100.0 ore. Departing for field.`
- **Screenshot**: `p6_gate.png` shows player ship and orange drone operating together.
