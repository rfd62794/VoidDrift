# ADR-019: Stateless UI Component Hierarchy
**Date:** May 2026
**Status:** Accepted

## Context
Current VoidDrift UI implementation is ad-hoc: buttons are painter calls + interact rect written inline wherever needed. Panels are custom-drawn regions. No shared abstraction exists — changing button appearance requires manually finding every button. This pattern has become unwieldy as the UI grows (HUD, Production Tree, Main Menu, overlays).

A natural hierarchy emerges for VoidDrift's UI needs:

- **Level 1 — Primitives**: 
  ```rust
  pub fn vd_button(
      painter: &egui::Painter,
      ui: &mut egui::Ui,
      rect: egui::Rect,
      label: &str,
      style: &ButtonStyle,
  ) -> egui::Response
  ```
  `vd_panel(painter, rect, style) → ()`, `vd_overlay(ui, rect, dim) → ()`
  
  `ButtonStyle` is a struct in `components/styles.rs` — colors, corner radius, font size.
- **Level 2 — Components**: Drawer (uses Panel + Button primitives), Popup (uses Overlay + Button primitives), TabBar (uses Button primitives)
- **Level 3 — Screens**: HUD, ProductionTree, MainMenu (use Components)

## Decision
UI components must be implemented as **stateless functions** that draw and return a Response. State lives in Bevy resources, not in UI components.

## Rationale
- **Architectural fit**: VoidDrift already uses a resource-driven architecture (GameState, ViewState, ProductionToggles, etc.). Stateless functions align with this pattern.
- **Composability**: Stateless functions are simpler to compose. `Drawer` can be built from `vd_panel` + `vd_button` calls without managing internal state.
- **Painter pattern compatibility**: Current egui implementation uses painter calls. Stateless functions fit naturally — they receive painter + params, draw, return Response.
- **Simplicity**: No need to manage hover/press state per component. egui's input system handles this via `ui.interact()` and Response objects.
- **Testability**: Pure functions are easier to test than stateful structs.

Stateful structs would offer more power (each component owning its own hover/press state) but add complexity that conflicts with the existing resource-driven design.

## Consequences
- **Positive**: Consistent UI component library across all screens.
- **Positive**: Easy to modify button/panel styles globally by changing primitive functions.
- **Positive**: Clear separation: components draw, resources hold state.
- **Constraint**: All UI state must be explicitly modeled as resources (no implicit component state).
- **Implementation**: Create `src/systems/ui/components/` directory with `mod.rs`, `primitives.rs` (Level 1), `drawer.rs` (Level 2), etc. Avoid god class by separating concerns across files.
