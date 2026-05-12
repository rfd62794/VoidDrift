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

## Amendment (May 2026)

Recorded after as-built implementation of Level 1 (`vd_button`) in commits `a61c45d` and `e28df67`. The deviations below are intentional and supersede the original spec where they conflict.

- **Path:** as-built location is `src/ui_kit/`, **not** `src/systems/ui/components/`. Reason: naming collision with the existing Bevy ECS `src/components/` directory would have created daily import-disambiguation friction. `ui_kit` is unambiguous.

- **`vd_button` signature (as-built):**
  ```rust
  pub fn vd_button(
      ui: &mut egui::Ui,
      label: &str,
      style: ButtonStyle,
      enabled: bool,
      highlight: Option<HighlightKind>,
  ) -> egui::Response
  ```
  Deviations from the spec in this ADR's Context section:
  - **No `painter` parameter.** The primitive uses `egui::Button` via `ui.add_enabled(...)`, which already handles painter, hit-testing, and disabled visuals. A separate painter parameter was unnecessary.
  - **`style` is passed by value, not by reference.** `ButtonStyle` is `Copy` (~32 bytes); pass-by-value avoids temporary-lifetime gymnastics at call sites and matches egui's own conventions for `Vec2`, `Color32`, `Stroke`.
  - **`enabled: bool` and `highlight: Option<HighlightKind>` are lifted out of `ButtonStyle`.** `ButtonStyle` describes appearance only; `enabled` and `highlight` are per-frame runtime state (e.g., the amber pulse fires when `drone_built && !production_tree_ever_opened`). Mixing them into the style struct was a category error caught in review.

- **`ButtonStyle` fields (as-built):**
  ```rust
  pub struct ButtonStyle {
      pub min_size: egui::Vec2,
      pub fill: Option<egui::Color32>,        // None = inherit ui.style()
      pub text_color: Option<egui::Color32>,  // None = inherit ui.style()
      pub stroke: egui::Stroke,
      pub corner_radius: u8,                  // not f32 — CornerRadius::same() requires u8
  }
  ```
  `Option<Color32>` for `fill`/`text_color` honestly represents the current state of the codebase: every existing button inherits the egui theme, so `None` is the correct default. `corner_radius` is `u8` to match `egui::CornerRadius::same()`'s parameter type — using `f32` would have lied about the API contract and silently truncated fractional values.

- **`HighlightKind`:** enum with one variant (`Amber`) at present. The amber pulse logic was extracted verbatim from `src/systems/ui/hud/buttons.rs` (now removed), with the previously-hardcoded 200px overlay width replaced by `response.rect.width() * 2.5` to preserve the visual extension ratio for any button size.
