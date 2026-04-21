# Voidrift — UI Architecture Vision
**Locked:** April 2026
**Status:** Canonical — migration approved, not yet executed
**Source:** Design conversation April 2026 — Layer 2 document.

---

## Framework Migration

| | Current | Target |
| :--- | :--- | :--- |
| Framework | bevy_egui 0.33 / egui 0.31 | Bevy UI (native, built into Bevy 0.15) |
| Layout model | Immediate mode, pixel-based | Flexbox (Taffy), percentage-based |
| Touch handling | egui Sense::click() | `Pointer<Click>` observers (bevy_picking) |
| Mobile alignment | Drift on egui — known issue | Stable — percentage layout adapts to screen |
| Debug overlays | Keep egui for debug use if needed | Remove from game HUD entirely |

**Migration approach:** Panel by panel replacement, not big bang.
egui and Bevy UI can coexist during transition (confirmed by research, April 2026).
Remove egui entirely when all panels are migrated.

### Required Cargo.toml Feature Additions

When migration begins, add to the bevy feature list:
```toml
"bevy_ui",                    # Bevy UI layout system
"bevy_picking",               # Touch/pointer event system
"bevy_ui_picking_backend",    # Connects picking to UI nodes
"default_font",               # Embedded Fira Mono for prototyping
```

### Touch Interaction Pattern

Use `Pointer<Click>` observers on `Button` nodes — **not** the deprecated `Interaction` component.

```rust
parent.spawn(button_bundle())
    .observe(|_: On<Pointer<Click>>, ...| { /* handle tap */ });
```

- `Pointer<Click>` fires on touch-**release** — correct behavior for buttons
- `Interaction::Pressed` fires on touch-**down** — wrong for tap feel
- `Interaction` is functional but being deprecated in favor of `bevy_picking`
- All UI nodes are pickable by default — no `Pickable` component needed

### Text Rendering Note

Use `Text` inside `Node` hierarchies (Bevy UI text), not `Text2d` (world-space).
Known Bevy 0.15 bug: `Text2d` entities spawned in `Startup` at scale factor ≠ 1.0
get wrong size/position on first frame. This does not affect UI-embedded `Text`.
Spawn UI in state-enter systems (not `Startup`) to avoid any timing issues.

---

## Portrait Layout (Primary)

**Target device:** Moto G 2025, 720×1604 logical pixels

```
┌─────────────────────────────────────┐
│                                     │
│          WORLD VIEW                 │
│      (flex: fills remaining)        │
│                                     │
│                                     │
│                                     │
├──────────┬──────────────────────────┤
│          │                          │
│ LEFT NAV │   CONTEXT PANEL          │
│  30% w   │      70% w               │
│          │                          │
│  MAP     │  [tab content here]      │
│  QUEST   │                          │
│  ──────  │                          │
│  RESERVES│                          │
│  POWER   │                          │
│  FORGE   │                          │
│  CRAFTER │                          │
│  SHIP PT │                          │
│          │                          │
├──────────┴──────────────────────────┤
│  SIGNAL STRIP  (64px, always visible)│
└─────────────────────────────────────┘
```

**Bottom panel height:** ~45% of screen height when docked; collapses to just signal strip when undocked.

**Left nav width:** 30% of screen width
**Context panel width:** 70% of screen width (remainder)

---

## Landscape Layout (Secondary)

**Target device:** onn tablet, 1200×2000 logical pixels

```
┌───────────┬──────────────────┬──────────────┐
│           │                  │              │
│ LEFT NAV  │   WORLD VIEW     │  CONTEXT     │
│  20% w    │     60% w        │   PANEL      │
│           │                  │   20% w      │
│  MAP      │                  │              │
│  QUEST    │                  │  [tab content│
│  ──────   │                  │   here]      │
│  RESERVES │                  │              │
│  POWER    │                  │              │
│  FORGE    │                  │              │
│  CRAFTER  │                  │              │
│  SHIP PT  │                  │              │
│           │                  │              │
├───────────┴──────────────────┴──────────────┤
│        SIGNAL STRIP  (64px, always visible)  │
└──────────────────────────────────────────────┘
```

**Left nav:** 20% width, full height minus signal strip
**World view:** 60% width, full height minus signal strip
**Context panel:** 20% width, full height minus signal strip

---

## Orientation Detection

Single check at runtime — no separate code paths:

```rust
let is_landscape = window.physical_width() > window.physical_height();
```

Same `Node` components for both orientations. Different `Style` values inserted
based on orientation check. One layout system handles both.

Orientation re-check on window resize event (handles rotation mid-session).

---

## Context Panel States

| Condition | Context Panel Content |
| :--- | :--- |
| Not docked | Minimal ship status: power bar, cargo bar, engine tier |
| Docked — any tab | Active station tab content |
| QUEST tapped | Quest objectives panel (overlays context panel) |
| MAP tapped | Map view takes over world view area |

---

## Signal Strip

Always visible. Always at bottom. Never hidden by other panels.

| State | Height | Behavior |
| :--- | :--- | :--- |
| Collapsed | 64px | Shows 3 most recent signal lines. Tap anywhere to expand. |
| Expanded | ~240px | Shows 20 lines scrollable. Tap outside to collapse. |

Visual style:
- Background: near-black, alpha 200/255
- Text: terminal green `#00CC66`
- Font: monospace
- Prefix: `>` on each line

---

## Touch Targets

All interactive elements must meet minimum touch target size:
- Minimum height: **44px logical pixels**
- Minimum width: **44px logical pixels** where feasible
- Tab buttons: full nav width × 44px minimum

---

## Node Structure (Bevy UI)

Root structure (portrait, docked):

```rust
// Root: full screen, column layout
Node { width: 100%, height: 100%, flex_direction: Column, .. }
  // World view: fills remaining space
  Node { flex_grow: 1.0, .. }
  // Bottom panel: fixed height
  Node { height: ~45%, flex_direction: Row, .. }
    // Left nav
    Node { width: 30%, flex_direction: Column, .. }
    // Context panel
    Node { width: 70%, .. }
  // Signal strip: fixed height
  Node { width: 100%, height: 64px, .. }
```

---

## Migration Sequence

Suggested order for panel-by-panel replacement:

1. **Signal strip** — simplest, always visible, good first target
2. **Left nav buttons** (MAP, QUEST) — simple stateful buttons
3. **Station tabs** (RESERVES, POWER, FORGE, CRAFTER, SHIP PORT)
4. **Context panel** (tab content areas)
5. **Quest panel** (objective list)
6. **Tutorial popup** (modal overlay)
7. Remove `bevy_egui` dependency entirely

Each step: implement in Bevy UI → verify on device → remove egui equivalent.
Never remove egui panel before Bevy UI replacement is verified on hardware.
