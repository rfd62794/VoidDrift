# ADR-009: Tutorial Trigger Pattern

**Date:** April 2026  
**Status:** Accepted

## Context
As gameplay systems grew more complex (parallel processing, laser tiers, world expansion), players required contextual guidance. Early attempts to use immediate narrative signals resulted in overlapping popups and information overload.

## Decision
Implement a formalized `TutorialState` resource with the following properties:
1.  **HashSet<u32> for shown IDs**: Ensures each tutorial popup appears exactly once per session.
2.  **Sequential Priority**: Tutorials are evaluated in ID order (T-001, then T-002).
3.  **Mutual Exclusion**: No two tutorial popups can be active at once.
4.  **Opening Sequence Guard**: All tutorial evaluation is suspended until `OpeningPhase::Complete`.

## Consequences
- Tutorial logic is centralized in `narrative.rs`.
- UI rendering for popups is centralized in a single `egui` window call in `ui.rs`.
- Transitions (e.g., "OPEN SMELTER" button) can be tied to the tutorial's dismissal.
- The system is easily extensible by adding new conditions to the `tutorial_system`.

## Alternatives Considered
- **Stateless Triggers**: Rejected as it causes popups to reappear every time the condition is met (e.g., every time cargo is 80%).
- **Hardcoded Cinematics**: Too rigid for a non-linear economy game. Contextual popups offer better UX for player agency.
