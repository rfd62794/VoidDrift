# ADR-016: Layer 1/2/3 engine/game/presentation architecture
**Date:** May 2026
**Status:** Accepted

## Context
As the codebase grew beyond 15,000 lines, the lack of clear architectural boundaries became a problem:
- God classes emerged (resources.rs 226 lines, hud/mod.rs 1040 lines, main_menu.rs 639 lines)
- Duplication spread across files (rocket spawning in entity_setup.rs and ship_spawn.rs)
- Hardcoded values scattered throughout (signal triggers in signal.rs, laser tiers in mining.rs)
- Tech debt accumulated (auto_forge processing inline instead of via queues)
- No clear dependency direction — everything depended on everything

A comprehensive codebase analysis revealed the need for layered architecture to:
- Establish clear dependency boundaries
- Enable scoped rework without scope bleed
- Make the codebase maintainable for a single developer
- Provide a roadmap for structural improvement

## Decision
Organize the codebase into three distinct layers with strict dependency rules:

**Layer 1: Engine (Infrastructure)**
- Directories: `src/lib.rs`, `src/config/`, `src/components/`, `src/systems/persistence/`, `src/systems/setup/`
- Responsibility: Core infrastructure — app setup, config loading, ECS components, save/load, entity spawning
- Dependencies: None (base layer)
- Dependents: Layer 2 and Layer 3

**Layer 2: Game (Mechanics)**
- Directories: `src/systems/game_loop/`, `src/systems/ship_control/`, `src/systems/asteroid/`, `src/systems/narrative/`
- Responsibility: Gameplay logic — mining, refining, autonomous ships, narrative, quest progression
- Dependencies: Layer 1 only
- Dependents: Layer 3

**Layer 3: Presentation (UI + Visuals)**
- Directories: `src/systems/ui/`, `src/systems/visuals/`, `src/scenes/`
- Responsibility: Rendering and interface — HUD, menus, visual effects, camera
- Dependencies: Layer 1 and Layer 2
- Dependents: None (top layer)

Dependency rule: Layer N can only depend on Layer < N. Layer 1 depends on nothing external to the layer.

## Rationale
This layered architecture provides:
- **Clear boundaries**: Each layer has a well-defined scope and responsibility
- **Scoped rework**: Issues can be organized by layer, preventing scope bleed
- **Dependency clarity**: No circular dependencies, clear direction of data flow
- **Maintainability**: A single developer can work on one layer without understanding the entire codebase
- **Testability**: Each layer can be tested independently
- **Parallel work**: Multiple developers could theoretically work on different layers simultaneously
- **Migration path**: Clear path for future refactoring (e.g., Bevy UI migration stays in Layer 3)

## Consequences
- **Positive**: Eliminates god classes through focused module boundaries
- **Positive**: Removes duplication through shared infrastructure (Layer 1)
- **Positive**: Enables config-driven design (Layer 1 config used by Layer 2)
- **Positive**: Provides clear roadmap for structural rework (Layer 1 → Layer 2 → Layer 3)
- **Positive**: Makes the codebase maintainable for long-term development
- **Constraint**: Strict dependency rules must be enforced (no Layer 3 depending directly on Layer 2 components)
- **Constraint**: Some refactoring required to move code into correct layers
- **Constraint**: Issue tracking must organize by layer to prevent scope bleed
- **Migration**: Existing code must be gradually reorganized to fit layer boundaries
