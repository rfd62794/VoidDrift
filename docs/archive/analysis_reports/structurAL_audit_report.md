# Structural Audit Report — Voidrift (April 2026)

This report provides a comprehensive analysis of the current codebase structure, focusing on system distribution, file health, component isolation, and documentation status.

## 1. System & File Audit

Files exceeding **300 lines** are flagged as primary split candidates. Line counts for functions are approximated based on start/end ranges.

| File Path | Total Lines | System / Helper Function | Approx. Size | Status |
| :--- | :--- | :--- | :--- | :--- |
| `src/systems/setup.rs` | 641 | `setup_world` | **487 lines** | 🚨 **CRITICAL OVERSIZE** |
| | | `generate_ore_mesh` family | 150 lines | |
| `src/systems/ui.rs` | 405 | `hud_ui_system` | **254 lines** | ⚠️ **SPLIT CANDIDATE** |
| | | `cargo_label_system` | 15 lines | |
| `src/systems/narrative.rs` | 407 | `tutorial_system` | 46 lines | ⚠️ **SPLIT CANDIDATE** |
| | | `opening_sequence_system` | 96 lines | |
| | | `signal_system` | 260 lines | |
| `src/components.rs` | 372 | All Definitions | 372 lines | ⚠️ **SPLIT CANDIDATE** |
| `src/systems/economy.rs` | 220 | `processing_queue_system` | 66 lines | Healthy |
| `src/systems/visuals.rs` | 198 | `berth_occupancy_system` | 27 lines | Healthy |
| `src/systems/map.rs` | 162 | `map_pan_system` | 32 lines | Healthy |
| `src/systems/autonomous.rs` | 140 | `autonomous_ship_system` | 93 lines | Healthy |

---

## 2. Component & Resource Usage Study

List of structures in `components.rs` and their current usage profile. Items used by **only 1-2 systems** are candidates for localization or moving to specific modules.

### Components
*   **Ship (and related Enums)**: Core (Used everywhere)
*   **Station**: Core (Used everywhere)
*   **AutopilotTarget**: `autopilot.rs`, `setup.rs`, `narrative.rs`
*   **AsteroidField**: `mining.rs`, `setup.rs`, `ui.rs` (Total Lockdown filter)
*   **MapMarker / MapElement**: `map.rs`, `setup.rs`
*   **MainCamera**: `map.rs`, `setup.rs`
*   **AutonomousShip / AutonomousAssignment**: `autonomous.rs`, `ui.rs`, `lib.rs`
*   **DockedAt**: `autopilot.rs`, `autonomous.rs`, `visuals.rs`, `setup.rs` (Solid pattern)
*   **ShipCargoBarFill**: `ui.rs`, `setup.rs`
*   **Berth / BerthVisual**: `visuals.rs`, `setup.rs`
*   **CargoOreLabel / CargoCountLabel**: `ui.rs`, `setup.rs` (Tutorial UX Phase)
*   **MiningBeam**: `mining.rs`, `visuals.rs` (Foundational)
*   **ThrusterGlow**: `visuals.rs`, `setup.rs`

### Resources
*   **SignalLog**: `narrative.rs`, `ui.rs`, `lib.rs`
*   **QuestLog**: `quest.rs`, `ui.rs`, `lib.rs`
*   **TutorialState**: `narrative.rs`, `ui.rs`, `lib.rs` (Tutorial UX Phase)
*   **MapPanState**: `map.rs`, `lib.rs` (Tutorial UX Phase)
*   **StationQueues**: `economy.rs`, `ui.rs`
*   **OpeningSequence**: `narrative.rs`, `ui.rs`, `lib.rs`

> [!NOTE]
> `ShipCargoBarFill` and `CargoBarFill` (lines 127 vs 157 in `components.rs`) appear to be duplicates or near-duplicates causing potential confusion.

---

## 3. Post-ADR-006 Strategic Decisions

Significant architectural patterns established since ADR-006 that lack formal documentation:

1.  **System Tuple Partitioning**: The split of `Update` systems in `lib.rs` into "Gameplay & Logistics" and "Station, Narrative & UI" to bypass the Rust 20-tuple constraint.
2.  **Universal Disjointness (Total Lockdown)**: The mandatory use of `Without<Station>` and `Without<AsteroidField>` in Ship systems to maintain strict parallelism without query collision.
3.  **The Tutorial Trigger Pattern**: One-time fire logic in `narrative.rs` mapping to a central UI pop-up state.
4.  **World-Space Anchoring**: Child-parent relationship for HUD elements (cargo labels) and Docking berths.
5.  **Multi-Touch Conflict Resolution**: Suppression of single-touch map logic during pinch-zoom events.

---

## 4. Documentation Health Check

### phase_summaries (Missing)
The following development phases have no corresponding `docs/phases/` summary:
*   **Phase 07**: Signal Strip & Signal Narrative Overhaul
*   **Phase 08**: Resource Processing Queues & Deep Power Economy
*   **Phase 09**: Quest Tracker & Map Expansion (All 6 Asteroid Fields)
*   **Phase 10**: Contextual Tutorial & UX Legibility Pass

### ADRs (Missing)
*   **ADR-007**: System Partitioning (Tuple Limitation)
*   **ADR-008**: Universal Disjointness Architecture
*   **ADR-009**: Contextual Tutorial System Design

### Outdated Documents
*   `docs/state/current.md`: Stale since Phase B.
*   `ARCHITECTURE.md`: Large gaps regarding narrative and economy systems.
*   `AGENT_CONTRACT.md`: Invariants do not reflect the System Partitioning or Disjointness requirements.
