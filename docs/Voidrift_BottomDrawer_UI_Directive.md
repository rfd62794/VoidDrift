# Voidrift — Directive: Comprehensive UI Redesign (Bottom Drawer System)
**Status:** Approved — Ready for Execution  
**Type:** UI Architecture Redesign  
**Date:** April 2026  
**Branch:** feature/bottom-drawer-ui  
**Depends On:** Adaptive UI Directive COMPLETE ✅

---

## 1. Objective

Replace the current side panel + context panel architecture with a
bottom drawer system. Three drawer states. Full-width content. Single-column
queue cards. No side panel. No floating subpanels.

This is a complete UI restructure. All egui panel calls in `hud.rs` and
`station_tabs.rs` are replaced. The game world gets maximum screen space
when flying. The station UI is fully accessible when docked.

---

## 2. New Resource: DrawerState

Add to `components.rs`:

```rust
#[derive(Resource, PartialEq, Clone, Default)]
pub enum DrawerState {
    #[default]
    Collapsed,   // handle bar only — world view dominant
    TabsOnly,    // handle + tab bar — intermediate
    Expanded,    // handle + tabs + content — full access
}
```

Register in `lib.rs`:
```rust
.insert_resource(DrawerState::default())
```

---

## 3. Three Drawer States

### State 1 — Collapsed (default when flying)

```
┌─────────────────────────────────┐
│                                 │
│         WORLD VIEW              │
│         (full screen minus      │
│          handle + signal)       │
│                                 │
├─────────────────────────────────┤  ← handle bar, 32px
│ [≡] ════════════════════════   │
├─────────────────────────────────┤
│ > SIGNAL LINE 1                 │  ← signal strip, 64px
│ > SIGNAL LINE 2                 │
│ > SIGNAL LINE 3                 │
└─────────────────────────────────┘
```

World view takes all space except handle (32px) and signal (64px).
Handle bar has `[≡]` toggle button on left, decorative line fill.

### State 2 — TabsOnly

```
┌─────────────────────────────────┐
│                                 │
│         WORLD VIEW              │
│         (reduced by tab bar)    │
│                                 │
├─────────────────────────────────┤  ← handle bar, 32px
│ [≡] ════════════════════════   │
├─────────────────────────────────┤  ← tab bar, 48px
│ ROUTES │ QUEST │ (docked tabs)  │
├─────────────────────────────────┤
│ > SIGNAL LINE 1                 │  ← signal strip, 64px
│ > SIGNAL LINE 2                 │
└─────────────────────────────────┘
```

Tab bar visible. No content area. Tapping a tab transitions to Expanded.

### State 3 — Expanded

```
┌─────────────────────────────────┐
│                                 │
│      WORLD VIEW                 │
│      (~50% of screen height)    │
│                                 │
├─────────────────────────────────┤  ← handle bar, 32px
│ [≡] ════════════════════════   │
├─────────────────────────────────┤  ← tab bar, 48px
│ ROUTES │ QUEST │ RES│PWR│FRG│  │
├─────────────────────────────────┤  ← content area, ~35% screen
│                                 │
│    Active tab content           │
│    (single column, scrollable)  │
│                                 │
├─────────────────────────────────┤
│ > SIGNAL LINE 1                 │  ← signal strip, 64px
│ > SIGNAL LINE 2                 │
└─────────────────────────────────┘
```

---

## 4. Transition Rules

| Trigger | From | To | Notes |
|---------|------|----|-------|
| Tap handle/[≡] | Collapsed | TabsOnly | Opens tab bar |
| Tap handle/[≡] | TabsOnly | Collapsed | Hides tab bar |
| Tap handle/[≡] | Expanded | TabsOnly | Collapses content, keeps tabs |
| Tap any tab | TabsOnly | Expanded | Opens content to that tab |
| Tap active tab | Expanded | TabsOnly | Collapses content |
| Tap different tab | Expanded | Expanded | Switches content only |
| Ship docks | Any | Expanded | Auto-expands to RESERVES |
| Ship undocks | Any | Collapsed | Auto-collapses |

Auto-transitions (dock/undock) fire from the autopilot system, not the UI system.
Add to `autopilot.rs` docking and undocking transitions:
```rust
// On dock:
*drawer_state = DrawerState::Expanded;
*active_tab = ActiveStationTab::Reserves;

// On undock:
*drawer_state = DrawerState::Collapsed;
```

---

## 5. Tab Bar

### 5.1 Tabs When Flying

Only two tabs visible:
- **ROUTES** — navigation map (replaces MAP)
- **QUEST** — quest tracker

### 5.2 Tabs When Docked

Five additional tabs appear:
- **RESERVES** — resources, auto-dock, repair
- **POWER** — power status
- **REFINERY** — magnetite + carbon queues
- **FORGE** — hull forge + AI core queues
- **SHIP PORT** — fleet assembly

Full tab bar when docked (7 tabs):
```
ROUTES | QUEST | RESERVES | POWER | REFINERY | FORGE | SHIP PORT
```

Use short labels that fit — abbreviate if needed:
```
ROUTES | QUEST | RES | PWR | RFNY | FORGE | PORT
```

### 5.3 Tab Sizing

Tab bar height: `layout.tab_bar_height = 48.0`
Each tab width: `screen_width / visible_tab_count`
Font: `layout.font_size_label` (10-11px)
Active tab: bright, accent color border on top
Inactive tab: dim, no border

### 5.4 Scroll if needed

If 7 tabs don't fit comfortably at 720px width, use `egui::ScrollArea::horizontal`
on the tab bar with `show_scrollbar: false`. The player swipes the tab bar to
reveal hidden tabs. Active tab is always scrolled into view.

---

## 6. Handle Bar

Height: 32px, full width.
Contents:
- Left: `[≡]` button, 44px wide, 32px tall (full handle height)
- Center: decorative horizontal line `═══════` in dim color
- Right: nothing (reserved for future status icons)

The entire handle bar is tappable — not just the `[≡]` button. Any tap
on the handle triggers the state transition.

```rust
// Handle tap detection
let handle_response = ui.allocate_rect(handle_rect, egui::Sense::click());
if handle_response.clicked() {
    *drawer_state = match *drawer_state {
        DrawerState::Collapsed => DrawerState::TabsOnly,
        DrawerState::TabsOnly => DrawerState::Collapsed,
        DrawerState::Expanded => DrawerState::TabsOnly,
    };
}
```

---

## 7. Content Area

### 7.1 Sizing

Content area height from `UiLayout`:
```rust
layout.content_area_height = layout.screen_height
    - layout.handle_height        // 32px
    - layout.tab_bar_height       // 48px
    - layout.signal_strip_height  // 64px
    - layout.world_view_min;      // minimum world view ~45% of screen
```

Update `UiLayout` to include:
```rust
pub handle_height: f32,          // 32.0
pub tab_bar_height: f32,         // 48.0
pub content_area_height: f32,    // computed
pub world_view_min: f32,         // screen_height * 0.45
```

### 7.2 Scrolling

Content area is a `egui::ScrollArea::vertical()` with `show_scrollbar: false`.
Scrolls naturally with finger swipe. Content taller than the area scrolls.

### 7.3 Single Column Layout

All content renders in a single column at full content width.
No `ui.horizontal()` layouts for queue cards.
No `ui.columns()`.
One thing at a time, full width, stacked vertically.

---

## 8. Queue Card — Single Column, Full Width

Each queue card is full content width. Two cards in REFINERY tab stack
vertically with a separator between them.

```
┌─────────────────────────────────────┐
│ MAGNETITE -> POWER CELLS            │  font_size_title, bold
│ 10 ore · 20 sec per batch           │  font_size_label, dim
├─────────────────────────────────────┤  separator
│ ████████████████░░░░  14s left      │  progress bar, 16px tall
│ PROCESSING · 3 batches queued       │  status line
│ You can make: 8 more                │  resource feedback, green if >0
├─────────────────────────────────────┤  separator
│ [ +1 ]    [ +10 ]    [ MAX ]        │  3 buttons, equal width, 44px tall
│ [        CLEAR QUEUE              ] │  full width, 44px tall
└─────────────────────────────────────┘
```

```rust
fn render_queue_card(
    ui: &mut egui::Ui,
    layout: &UiLayout,
    // ... existing params
) {
    let card_width = layout.content_width;
    let btn_width = (card_width - 8.0) / 3.0;  // 3 add buttons

    ui.set_width(card_width);

    // Header
    ui.label(egui::RichText::new("MAGNETITE -> POWER CELLS")
        .size(layout.font_size_title)
        .strong());
    ui.label(egui::RichText::new("10 ore · 20 sec per batch")
        .size(layout.font_size_label)
        .color(egui::Color32::from_gray(140)));

    ui.separator();

    // Progress bar
    let fraction = timer_remaining / batch_time;
    ui.add(egui::ProgressBar::new(1.0 - fraction)
        .desired_width(card_width)
        .desired_height(16.0));

    // Status
    ui.label(status_text);
    ui.label(feedback_text);

    ui.separator();

    // Add buttons — three equal width
    ui.horizontal(|ui| {
        let btn_size = egui::vec2(btn_width, layout.button_height);
        if ui.add_sized(btn_size, egui::Button::new("+1")).clicked() {
            // queue 1
        }
        if ui.add_sized(btn_size, egui::Button::new("+10")).clicked() {
            // queue 10
        }
        if ui.add_sized(btn_size, egui::Button::new("MAX")).clicked() {
            // queue max
        }
    });

    // Clear button — full width
    let clear_size = egui::vec2(card_width, layout.button_height);
    if ui.add_sized(clear_size, egui::Button::new("CLEAR QUEUE")).clicked() {
        // clear
    }
}
```

---

## 9. Tab Content Specs

### ROUTES Tab
Current map functionality. Sector markers, tap-to-navigate.
Rename all references from "MAP" to "ROUTES" in code and Signal lines.

### QUEST Tab
Three sections: ACTIVE, COMPLETED, UPCOMING.
Progress bar for Q-003 (repair kits).
No changes to quest data — UI only.

### RESERVES Tab
Resource counts in a clean grid:

```
MAGNETITE      240    CARBON         85
POWER CELLS     12    HULL PLATES     3
SHIP HULLS       1    AI CORES        0
HELIUM           8
```

Auto-dock toggles below resources:
```
[✓] Auto-Unload Cargo
[ ] Auto-Smelt Magnetite
[ ] Auto-Smelt Carbon
```

Repair button (pre-online only):
```
[ REPAIR STATION — 25 CELLS ]
```

### POWER Tab
Station power bar, ship power bar, consumption table.
Read-only. No buttons.

### REFINERY Tab
Two queue cards stacked vertically:
1. Magnetite -> Power Cells
2. Carbon -> Hull Plates

`ui.separator()` with 8px spacing between cards.

### FORGE Tab
Two queue cards stacked vertically:
1. Hull Plates -> Ship Hull
2. Power Cells -> AI Core

### SHIP PORT Tab
Assemble button, fleet status.
Stub — full implementation deferred.

---

## 10. egui Panel Structure

Render order (egui claims space bottom-up):

```rust
pub fn hud_ui_system(
    mut contexts: EguiContexts,
    layout: Res<UiLayout>,
    drawer_state: Res<DrawerState>,
    active_tab: Res<ActiveStationTab>,
    // ... other resources
) {
    let ctx = contexts.ctx_mut();

    // 1. Signal strip — always, claims bottom 64px full width
    egui::TopBottomPanel::bottom("signal_strip")
        .resizable(false)
        .exact_height(layout.signal_strip_height)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            render_signal_strip(ui, &layout, &signal_log, &expanded);
        });

    // 2. Content area — only when Expanded
    if *drawer_state == DrawerState::Expanded {
        egui::TopBottomPanel::bottom("content_area")
            .resizable(false)
            .exact_height(layout.content_area_height)
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .show(ui, |ui| {
                        ui.set_width(layout.content_width);
                        render_active_tab(ui, &layout, &active_tab, /* ... */);
                    });
            });
    }

    // 3. Tab bar — when TabsOnly or Expanded
    if *drawer_state != DrawerState::Collapsed {
        egui::TopBottomPanel::bottom("tab_bar")
            .resizable(false)
            .exact_height(layout.tab_bar_height)
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                render_tab_bar(ui, &layout, &mut active_tab, is_docked);
            });
    }

    // 4. Handle bar — always
    egui::TopBottomPanel::bottom("drawer_handle")
        .resizable(false)
        .exact_height(layout.handle_height)
        .frame(egui::Frame::NONE)
        .show(ctx, |ui| {
            render_handle(ui, &layout, &mut drawer_state);
        });

    // 5. Central panel — world view, fills remainder
    egui::CentralPanel::default()
        .frame(egui::Frame::NONE)
        .show(ctx, |_ui| {
            // world rendered by Bevy, not egui
        });
}
```

No `SidePanel`. No `egui::Window` for tabs.
Everything is `TopBottomPanel` registered bottom-up.

---

## 11. UiLayout Updates

Add new fields to `UiLayout` in `ui_layout_system`:

```rust
pub handle_height: f32,           // 32.0
pub tab_bar_height: f32,          // 48.0
pub content_area_height: f32,     // computed
pub world_view_min_height: f32,   // screen_height * 0.45
pub content_width: f32,           // screen_width (full width, no side panel)
```

`content_width` is now `screen_width` — no side panel to subtract.

Remove from `UiLayout`:
- `left_panel_width`
- `card_width` (was half content — now full content)
- `card_gap`
- `queue_button_width` (recalculate inside render_queue_card)

---

## 12. Remove Old Constants

After implementation, remove from `constants.rs`:
- `UI_LEFT_PANEL_WIDTH`
- `UI_LEFT_PANEL_WIDTH_LANDSCAPE`
- Any remaining hardcoded panel pixel values

---

## 13. File Scope

| File | Change |
|------|--------|
| `src/components.rs` | Add DrawerState resource, update UiLayout fields |
| `src/lib.rs` | Register DrawerState, update UiLayout registration |
| `src/systems/hud.rs` | Complete rewrite of panel structure, handle bar, tab bar |
| `src/systems/station_tabs.rs` | Rewrite render_queue_card for single column, update all tab renders |
| `src/systems/autopilot.rs` | Add DrawerState transitions on dock/undock |
| `src/systems/autonomous.rs` | Add DrawerState transitions on autonomous ship dock |
| `src/constants.rs` | Remove old panel constants |
| `Cargo.toml` | READ-ONLY |

---

## 14. Implementation Sequence

**Do not combine steps. Deploy after each. Verify before next.**

1. Add `DrawerState` and updated `UiLayout` to `components.rs` — verify compile
2. Add handle bar rendering only — no tabs, no content. Deploy, verify handle visible
3. Add tab bar rendering — ROUTES and QUEST only. Deploy, verify tabs appear
4. Wire handle tap to toggle drawer state. Deploy, verify collapse/expand works
5. Add content area for RESERVES tab only. Deploy, verify scrollable, full width
6. Add REFINERY tab with single-column queue cards. Deploy, verify no overflow
7. Add FORGE, POWER, SHIP PORT tabs. Deploy after each
8. Add QUEST tab content. Deploy, verify
9. Add ROUTES tab (rename from MAP). Deploy, verify navigation still works
10. Wire auto-expand on dock, auto-collapse on undock. Deploy, verify
11. Add docked tabs (RES, PWR, RFNY, FORGE, PORT) to tab bar when docked. Deploy
12. Remove old SidePanel code. Deploy, verify nothing missing
13. Remove old constants. Verify compile. Final deploy

---

## 15. Completion Criteria

- [ ] Handle bar visible always, 32px, full width
- [ ] `[≡]` tap cycles: Collapsed → TabsOnly → (tap tab) → Expanded → TabsOnly
- [ ] ROUTES and QUEST tabs visible when flying
- [ ] Station tabs appear when docked
- [ ] Content area full screen width, scrollable
- [ ] Queue cards single column, no overflow
- [ ] Progress bars full width, 16px tall, visible
- [ ] All four queue buttons 44px tall
- [ ] CLEAR QUEUE full width
- [ ] Auto-expands to RESERVES on dock
- [ ] Auto-collapses on undock
- [ ] Signal strip always visible
- [ ] No SidePanel remaining in code
- [ ] No floating egui::Window for tab content
- [ ] All hardcoded pixel values removed
- [ ] Gate: full play session — fly, dock, use refinery, forge, undock, repeat

---

## 16. Branch Policy

This directive executes on `feature/bottom-drawer-ui` branch.
Merge to `dev` only after all completion criteria pass on device.
Merge to `main` only after dev is verified stable.

---

*Voidrift Comprehensive UI Redesign Directive | April 2026 | RFD IT Services Ltd.*  
*Bottom drawer. Three states. Full width. Single column. Always visible signal.*
