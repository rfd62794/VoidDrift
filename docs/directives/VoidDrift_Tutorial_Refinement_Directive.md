# VoidDrift — Tutorial Refinement Directive
**Date:** May 2026  
**Status:** Ready for Agent Execution  
**Branch:** `feature/tutorial-refinement`  
**GitHub Issue:** #13  
**Builds Required:** WASM + Android (Moto G)

---

## Objective

Restore broken tutorial beats, add pipeline discovery nudge, build the Drawer Symbol Status Bar, and update popup visual styling to match current game aesthetic. No new mechanics. UI and content work only.

---

## Scope

### IN SCOPE
- Restore drawer button highlight (lost in prior sprint)
- New tutorial beat: Pipeline button nudge
- Drawer Symbol Status Bar using production node icons
- Three symbol states: ghost outline, dim, full
- ECHO nudge trigger for pipeline discovery
- Popup border and background style update to amber/dark
- Audit all six tutorial beats T-101 through T-106

### OUT OF SCOPE
- New tutorial content beyond pipeline nudge
- Production Tree changes
- Any mechanic or game logic changes
- Audio

---

## Part A: Tutorial Beat Audit

Before making any changes, audit all six existing beats and report which are broken:

Check each of T-101 through T-106:
- Does the highlight render correctly on the target element?
- Does the UNDERSTOOD button dismiss correctly?
- Does the sequence advance to the next beat?

Report findings as comments in the code before proceeding. Do not fix anything in this step — audit only.

---

## Part B: Restore Drawer Button Highlight

The drawer button highlight was lost in a prior sprint. Locate the tutorial system's highlight rendering code and restore it.

The highlight should:
- Render a pulsing amber border around the drawer button
- Same amber color as popup border: `Color32::from_rgb(180, 140, 50)`
- Pulse: alpha oscillates between 0.4 and 1.0 over 1.2 seconds using sine wave
- Renders on top of all other UI elements

---

## Part C: New Tutorial Beat — Pipeline Discovery

Add a new beat after the existing forge/production beat. Trigger: player has built at least one drone AND has never opened the Production Tree.

**Beat content:**
```
echo: pipeline status nominal. internal view available.
```

Highlight target: PIPELINE button in the HUD.

Same painted overlay style as existing tutorial beats. UNDERSTOOD button dismisses and sets `pipeline_nudge_shown = true` on save data — never shows again.

This beat does not block gameplay — it is a nudge, not a gate.

---

## Part D: Drawer Symbol Status Bar

Replace or supplement the current drawer resource display with a symbol bar using the production node icons already built in `component_nodes.rs`.

### D-1: Symbol states

Three states per symbol, driven by production state:

**Locked** — node not yet reachable in tech tree:
- Render using `draw_ore_polygon` / component draw function at `alpha = 0.2`
- Grey outline only — no fill color
- Communicates: "something exists here you haven't found yet"

**Active/Empty** — node unlocked but inventory at zero:
- Render at `alpha = 0.5`
- Solid color, dim
- Communicates: "this exists but isn't producing"

**Active/Populated** — node unlocked and inventory > 0:
- Render at `alpha = 1.0`
- Full color
- Optional: subtle glow using a larger same-color rect at low alpha behind the symbol
- Communicates: "flow active"

### D-2: Symbol bar layout

Row of symbols in the drawer, one per production output:
- Iron Ingot, Tungsten Ingot, Nickel Ingot, Aluminum Ingot
- Hull Plate, Thruster, AI Core, Canister
- Drone Bay (rocket silhouette)

Each symbol: 24×24px bounding box, 4px gap between symbols. Row centered horizontally in drawer.

Inventory count below each symbol in small text — same teal color as existing UI text. Zero counts render at 40% opacity.

### D-3: Ghost outline implementation

For locked symbols, render only the outline stroke of the shape — not the fill. Use `egui` stroke-only drawing where available, or draw filled shape at near-zero alpha then stroke at 0.2 alpha.

The ghost communicates "locked door" — player knows something is there without knowing what it becomes.

---

## Part E: Popup Style Update

Update the tutorial popup painted overlay to match current game aesthetic. The cyan border is a placeholder from before Sprint 5 — replace with amber.

**Updated spec:**
```rust
// Background
Color32::from_rgba_unmultiplied(5, 5, 10, 240)

// Border
Color32::from_rgb(180, 140, 50)  // desaturated amber
// Border width: 1.5px

// Title text
Color32::from_rgb(180, 140, 50)  // amber — matches border
// Size: 13px

// Body text  
Color32::from_rgb(220, 215, 210)  // warm off-white
// Size: 12px

// Button: UNDERSTOOD / allow
// Background: Color32::from_rgb(40, 35, 15)
// Text: Color32::from_rgb(180, 140, 50)
// Border: Color32::from_rgb(180, 140, 50) at 1px
```

Apply this style update to:
- Tutorial popup overlay
- Telemetry opt-in overlay (already uses amber — verify consistency)

---

## Part F: Acceptance Criteria

### F-1: Tutorial beats
- [ ] T-101 through T-106 all render highlights correctly
- [ ] All UNDERSTOOD buttons dismiss correctly
- [ ] Sequence advances without skipping or repeating
- [ ] Drawer button highlight restored — amber pulse visible

### F-2: Pipeline nudge
- [ ] Nudge appears after first drone built, pipeline never opened
- [ ] ECHO text renders correctly: lowercase, terse
- [ ] PIPELINE button highlighted with amber pulse
- [ ] Nudge never repeats after dismissal
- [ ] Does not block gameplay

### F-3: Symbol bar
- [ ] All nine symbols render in drawer
- [ ] Locked symbols show ghost outline at 0.2 alpha
- [ ] Empty symbols show dim color at 0.5 alpha
- [ ] Populated symbols show full color at 1.0 alpha
- [ ] Inventory counts visible below each symbol
- [ ] Zero counts at 40% opacity

### F-4: Popup styling
- [ ] Cyan border replaced with amber across all popups
- [ ] Tutorial and opt-in overlays visually consistent
- [ ] Text readable at amber/dark-background contrast ratio

### F-5: No regressions
- [ ] Mining loop functional
- [ ] Production Tree unchanged
- [ ] Save/load unaffected — `pipeline_nudge_shown` persists
- [ ] `.\verify.ps1` passes

**Screenshots required:**
- Drawer showing all three symbol states simultaneously
- Pipeline nudge beat active with PIPELINE button highlighted
- Tutorial popup with amber border

---

## Commit & Tag

```
git add -A
git commit -m "feat: Tutorial refinement — symbol bar, pipeline nudge, amber popups, beat restoration"
git tag v3.4.0-tutorial-refinement
git push origin feature/tutorial-refinement --tags
.\publish.ps1 -Build
```

Confirm itch.io build number changed post-publish.

---

## Notes for Agent

- Symbol draw functions already exist in `component_nodes.rs` — reuse them, do not duplicate
- Ghost outline: draw filled shape at alpha=0 then stroke at alpha=51 (0.2 * 255)
- Pipeline nudge saves to existing save struct — add `pipeline_nudge_shown: bool` field, increment SAVE_VERSION
- Amber color `Color32::from_rgb(180, 140, 50)` is used consistently across tutorial, opt-in, and symbol bar — define as a constant
- Do not modify Production Tree rendering
- Audit report from Part A must be completed before any fixes are applied

---

*VoidDrift Tutorial Refinement Directive*  
*May 2026 — RFD IT Services Ltd.*
