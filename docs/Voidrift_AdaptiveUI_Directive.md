# Voidrift — Directive: Adaptive UI System
**Status:** Approved — Ready for Execution  
**Type:** UI Architecture  
**Date:** April 2026  
**Branch:** dev  
**Depends On:** Phase A Refactor COMPLETE ✅ | egui Alignment Fix COMPLETE ✅

---

## 1. Objective

Replace all hardcoded pixel values in the UI with a single adaptive layout
system. Every measurement derives from actual screen dimensions at runtime.
The same code produces correct layouts on the Moto G 2025 (720×1604 portrait)
and the onn TBSPG100110027 (1200×2000 landscape) without any device-specific
branches.

---

## 2. Scope

**In scope:**
- `UiLayout` resource — computed each frame from actual window dimensions
- `ui_layout_system` — updates `UiLayout` every frame
- `render_queue_card` — replace all hardcoded values with `UiLayout` values
- All four call sites of `render_queue_card`
- Left panel width — derive from `UiLayout`
- Signal strip height — derive from `UiLayout`
- Context panel height — derive from `UiLayout`
- Button heights — derive from `UiLayout`

**Explicitly out of scope:**
- Any gameplay logic changes
- Any Signal or quest content changes
- Economy redesign (separate directive)
- New features of any kind

---

## 3. UiLayout Resource

### 3.1 Definition

Add to `components.rs`:

```rust
#[derive(Resource, Default)]
pub struct UiLayout {
    // Screen dimensions (logical pixels, post-scale-factor)
    pub screen_width: f32,
    pub screen_height: f32,
    pub is_landscape: bool,

    // Panel dimensions
    pub left_panel_width: f32,
    pub signal_strip_height: f32,
    pub context_panel_height: f32,

    // Content dimensions (derived)
    pub content_width: f32,      // screen_width - left_panel_width
    pub card_width: f32,         // (content_width - card_gap) / 2
    pub card_gap: f32,           // space between two side-by-side cards

    // Touch targets
    pub button_height: f32,
    pub tab_button_height: f32,

    // Typography
    pub font_size_body: f32,
    pub font_size_label: f32,
    pub font_size_title: f32,

    // Queue card internals
    pub queue_button_width: f32, // (card_width - gaps) / 4 for +1/+10/MAX/CLEAR
}
```

### 3.2 Update System

New system: `ui_layout_system` in `systems/hud.rs`.

```rust
pub fn ui_layout_system(
    windows: Query<&Window>,
    mut layout: ResMut<UiLayout>,
) {
    let Ok(window) = windows.get_single() else { return };

    let scale = window.scale_factor() as f32;
    let w = window.physical_width() as f32 / scale;
    let h = window.physical_height() as f32 / scale;
    let landscape = w > h;

    let left_panel = if landscape { w * 0.20 } else { w * 0.30 };
    let signal_height = if landscape { 56.0 } else { 64.0 };
    let context_height = if landscape { h * 0.42 } else { h * 0.38 };
    let content_w = w - left_panel;
    let card_gap = 12.0;
    let card_w = (content_w - card_gap) / 2.0;
    let btn_gap = 4.0;
    let queue_btn_w = (card_w - btn_gap * 3.0) / 4.0;

    *layout = UiLayout {
        screen_width: w,
        screen_height: h,
        is_landscape: landscape,
        left_panel_width: left_panel,
        signal_strip_height: signal_height,
        context_panel_height: context_height,
        content_width: content_w,
        card_width: card_w,
        card_gap,
        button_height: 44.0,
        tab_button_height: 44.0,
        font_size_body: if landscape { 13.0 } else { 12.0 },
        font_size_label: if landscape { 11.0 } else { 10.0 },
        font_size_title: if landscape { 15.0 } else { 14.0 },
        queue_button_width: queue_btn_w,
    };
}
```

### 3.3 Registration

Add to `components.rs` init and `lib.rs`:

```rust
// lib.rs
.insert_resource(UiLayout::default())
.add_systems(PreUpdate, systems::hud::ui_layout_system)
```

Run in `PreUpdate` — must compute before any UI system reads it in `Update`.

---

## 4. render_queue_card Signature Change

### 4.1 New Signature

```rust
pub fn render_queue_card(
    ui: &mut egui::Ui,
    layout: &UiLayout,           // NEW — second parameter
    station: &mut Station,
    queue: &mut Option<ProcessingJob>,
    op: ProcessingOperation,
    resource_cost: f32,
    pwr_cost: f32,
    batch_time: f32,
)
```

### 4.2 Internal Changes

Replace every hardcoded value inside the function:

| Was | Becomes |
|-----|---------|
| `ui.set_width(180.0)` | `ui.set_width(layout.card_width)` |
| `.desired_width(160.0)` | `.desired_width(layout.card_width - 8.0)` |
| `egui::vec2(40.0, 32.0)` | `egui::vec2(layout.queue_button_width, layout.button_height)` |
| `.min_size(egui::vec2(160.0, 30.0))` | `.min_size(egui::vec2(layout.card_width - 8.0, layout.button_height))` |

### 4.3 Button Row

The four buttons `+1`, `+10`, `MAX`, `CLEAR` share the card width equally:

```rust
ui.horizontal(|ui| {
    let btn_size = egui::vec2(layout.queue_button_width, layout.button_height);
    for (label, action) in [
        ("+1", BatchAction::Add1),
        ("+10", BatchAction::Add10),
        ("MAX", BatchAction::AddMax),
        ("CLEAR", BatchAction::Clear),
    ] {
        if ui.add_sized(btn_size, egui::Button::new(label)).clicked() {
            // handle action
        }
    }
});
```

### 4.4 Card Arrangement in Tabs

The two cards in Smelter and Forge are arranged side by side:

```rust
ui.horizontal(|ui| {
    render_queue_card(ui, layout, station, &mut queues.magnetite, ...);
    ui.add_space(layout.card_gap);
    render_queue_card(ui, layout, station, &mut queues.carbon, ...);
});
```

No hardcoded `add_space(16.0)` — use `layout.card_gap`.

---

## 5. Panel Width Changes

### 5.1 Left Panel

```rust
egui::SidePanel::left("left_panel")
    .exact_width(layout.left_panel_width)  // was UI_LEFT_PANEL_WIDTH constant
    .show(ctx, |ui| { ... });
```

### 5.2 Signal Strip

```rust
egui::TopBottomPanel::bottom("signal_strip")
    .exact_height(layout.signal_strip_height)  // was hardcoded
    .show(ctx, |ui| { ... });
```

### 5.3 Context Panel

```rust
egui::TopBottomPanel::bottom("tab_detail_panel")
    .exact_height(layout.context_panel_height)  // was hardcoded
    .show(ctx, |ui| { ... });
```

---

## 6. Tab Button Heights

All tab buttons in the left panel use `layout.tab_button_height`:

```rust
if ui.add_sized(
    [layout.left_panel_width - 8.0, layout.tab_button_height],
    egui::Button::new("RESERVES")
).clicked() { ... }
```

The `-8.0` is internal padding. Keep as a local constant `BUTTON_PADDING: f32 = 8.0`.

---

## 7. Remove Old Constants

After `UiLayout` is implemented and all references updated, remove from
`constants.rs`:

- `UI_LEFT_PANEL_WIDTH` (replaced by `layout.left_panel_width`)
- `UI_LEFT_PANEL_WIDTH_LANDSCAPE` (replaced by `layout.left_panel_width`)
- `CONTEXT_PANEL_HEIGHT` if it exists (replaced by `layout.context_panel_height`)

Do not remove `SIGNAL_STRIP_HEIGHT` if other systems reference it — check
before removing.

---

## 8. UiLayout Access Pattern

Every UI system that needs layout values takes `Res<UiLayout>` as a parameter:

```rust
pub fn hud_ui_system(
    mut contexts: EguiContexts,
    layout: Res<UiLayout>,          // NEW
    // ... existing params
) {
    let ctx = contexts.ctx_mut();
    let layout = layout.into_inner(); // or just pass &layout
    // All panel sizes come from layout
}
```

Pass `&layout` down into `render_queue_card` and any other render helpers
that need sizing information.

---

## 9. File Scope

| File | Change |
|------|--------|
| `src/components.rs` | Add `UiLayout` resource struct |
| `src/lib.rs` | Register `UiLayout`, register `ui_layout_system` in PreUpdate |
| `src/systems/hud.rs` | Add `ui_layout_system`, update panel sizes to use `layout.*` |
| `src/systems/station_tabs.rs` | Update `render_queue_card` signature and internals, update call sites |
| `src/constants.rs` | Remove replaced constants after migration |
| `Cargo.toml` | READ-ONLY |

---

## 10. Implementation Sequence

1. Add `UiLayout` to `components.rs` — verify compile
2. Add `ui_layout_system` to `hud.rs`, register in `lib.rs` PreUpdate — verify compile
3. Update `render_queue_card` signature and all four call sites — verify compile
4. Replace hardcoded values inside `render_queue_card` — deploy, verify cards fit correctly on Moto G
5. Update left panel, signal strip, context panel sizes to use `layout.*` — deploy, verify
6. Update tab button heights — deploy, verify
7. Remove old constants — verify compile
8. Final deploy — verify portrait on Moto G, landscape behavior on onn tablet if available

---

## 11. Completion Criteria

- [ ] `UiLayout` resource computes correct values on each device
- [ ] Queue cards fully contained within context panel — no overflow right
- [ ] Two cards side by side fill available width with no overflow
- [ ] All four queue buttons fit within card width equally
- [ ] Left panel correct width in portrait and landscape
- [ ] Signal strip correct height in portrait and landscape
- [ ] Context panel correct height in portrait and landscape
- [ ] Tab buttons minimum 44px height
- [ ] No hardcoded pixel values remaining in `hud.rs` or `station_tabs.rs`
- [ ] Removed old width constants from `constants.rs`
- [ ] Gate: deploy to Moto G, verify all panels aligned, no overflow anywhere

---

## 12. Future Extension

`UiLayout` is the single place to tune the entire UI for any new device.
Adding the onn tablet as a test device means verifying `UiLayout` produces
correct values for 1200×2000 landscape — no other changes needed.

When the Economy Redesign adds new department tabs, those tabs read from
`UiLayout` from the start. No hardcoded values ever enter new code.

---

*Voidrift Adaptive UI Directive | April 2026 | RFD IT Services Ltd.*  
*One resource. Every measurement derived. Any screen, any orientation.*
