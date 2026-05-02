# VoidDrift — WASM Polish Sprint
**Directive Version:** 1.0  
**Date:** May 1, 2026  
**Branch:** `dev`  
**Prerequisite:** WASM port renders in browser, mouse click input working

---

## AGENT CONTRACT

Three focused fixes before tagging the WASM port complete. No new features. No behavior changes on Android.

**You are NOT allowed to:**
- Change any game logic, balancing, or constants
- Modify save system
- Touch any file not listed in the File Touch Map
- Break Android build — verify both targets after every change

**You ARE responsible for:**
- Replacing hardcoded `pixels_per_point` with dynamic device scale
- Adding scroll wheel zoom and click-drag pan for WASM/desktop
- Verifying both Android and WASM builds clean
- Tagging `v2.8.0-wasm-port` on completion

**Definition of Done:**
- `pixels_per_point` derived dynamically from `window.scale_factor()` on all platforms
- Scroll wheel zooms in/out in browser
- Click-drag pans the map in browser
- Touch zoom and touch pan unchanged on Android
- `cargo build` Android clean
- `cargo build --target wasm32-unknown-unknown` clean
- Verified on Moto G and in browser
- Tagged `v2.8.0-wasm-port`

---

## Fix 1 — Dynamic egui Scale

### Problem
`pixels_per_point` is hardcoded to `3.0` in the egui setup, tuned specifically for the Moto G 2025 (1080px wide, XXHDPI). Any other device — tablet, foldable, WASM canvas — gets the wrong scale. A tablet at `3.0` renders UI too small. WASM canvas at `3.0` may be too large or too small depending on browser DPI.

### Fix
Replace hardcoded value with a system that reads `window.scale_factor()` at startup and derives the egui scale dynamically.

**Find current hardcoded value** — confirm file and line before changing. Likely in `lib.rs` or the egui plugin setup. Report location before modifying.

**Replace with:**
```rust
fn configure_egui_scale(
    windows: Query<&Window>,
    mut egui_settings: ResMut<bevy_egui::EguiSettings>,
) {
    if let Ok(window) = windows.get_single() {
        let device_scale = window.scale_factor() as f32;
        // Clamp to reasonable range across all targets
        // Android Moto G: scale_factor ~3.0 → egui_scale ~3.0 (1.0 * 1.0 clamp)
        // Tablet: scale_factor ~2.0 → egui_scale ~2.0
        // WASM: scale_factor ~1.0–2.0 → egui_scale ~2.0 minimum
        let egui_scale = (device_scale).clamp(2.0, 4.0);
        egui_settings.scale_factor = egui_scale;
    }
}
```

Register as a startup system in `lib.rs` — runs once on app start before first render.

**Note:** If `window.scale_factor()` returns `1.0` on WASM (browser default DPI), the clamp floor of `2.0` ensures the UI is still readable. If it returns the browser's device pixel ratio (e.g. `2.0` on a Retina display), the scale applies correctly. Test both cases.

**Verify:**
- Moto G: UI renders identically to current — scale_factor should be ~3.0 producing same result
- Browser: UI readable, touch targets appropriately sized for mouse interaction
- No layout regressions on either platform

---

## Fix 2 — WASM Zoom + Pan Map Interaction

### Problem
VoidDrift has pinch-zoom and touch-drag pan on Android. In the browser, the player has no way to zoom or pan the map — mouse has no pinch gesture.

### Fix
Add scroll wheel zoom and click-drag pan for WASM/desktop. Keep existing touch systems unchanged on Android.

Both inputs read on all platforms — Bevy provides `MouseWheel` and `MouseButton` everywhere. No cfg guards needed.

### Scroll Wheel Zoom

**Find existing zoom system** — confirm file and line. Likely `pinch_zoom_system` in `src/systems/ui/` or `src/systems/visuals/`. Report before modifying.

**Add to existing zoom system params:**
```rust
mut scroll_events: EventReader<MouseWheel>,
```

**Add scroll wheel handling:**
```rust
for event in scroll_events.read() {
    let zoom_delta = match event.unit {
        MouseScrollUnit::Line => event.y * 0.1,
        MouseScrollUnit::Pixel => event.y * 0.001,
    };
    // Apply same zoom logic as pinch — adjust OrthographicProjection.scale
    // Clamp to same min/max zoom bounds as pinch zoom
    proj.scale = (proj.scale - zoom_delta).clamp(MIN_ZOOM, MAX_ZOOM);
}
```

Confirm `MIN_ZOOM` and `MAX_ZOOM` constant names before implementing.

### Click-Drag Pan

**Find existing pan system** — confirm file and line. Likely `map_pan_system` in `src/systems/ui/`. Report before modifying.

**Add to existing pan system params:**
```rust
mouse_button: Res<ButtonInput<MouseButton>>,
mut cursor_moved: EventReader<CursorMoved>,
mut last_cursor_pos: Local<Option<Vec2>>,
```

**Add click-drag handling:**
```rust
if mouse_button.pressed(MouseButton::Left) {
    for event in cursor_moved.read() {
        if let Some(last) = *last_cursor_pos {
            let delta = event.position - last;
            // Apply same pan logic as touch drag
            // Invert delta direction to match touch drag feel
            pan_state.cumulative_offset -= delta;
        }
        *last_cursor_pos = Some(event.position);
    }
} else {
    *last_cursor_pos = None;
    cursor_moved.clear();
}
```

**Important:** Click-drag pan must not fire when the player clicks an asteroid or bottle. The existing asteroid and bottle input systems consume the click on their entities. If the click hits empty space it should pan. Verify this doesn't conflict with asteroid/bottle dispatch — test clicking asteroid vs clicking empty space.

**Verify:**
- Scroll wheel zooms in and out smoothly in browser
- Click-drag on empty space pans the map
- Click on asteroid dispatches drone (not pan)
- Click on bottle dispatches drone (not pan)
- Touch pinch zoom unchanged on Android
- Touch drag pan unchanged on Android

---

## Verification Checklist

### Fix 1 — Dynamic Scale
- [ ] Moto G: UI appearance unchanged from current
- [ ] Browser: UI readable, labels not clipped, tabs fit
- [ ] No egui scale jump on first frame

### Fix 2 — WASM Input
- [ ] Scroll wheel zooms in browser
- [ ] Scroll wheel zoom clamped — doesn't go infinite or inverted
- [ ] Click-drag pans in browser
- [ ] Clicking asteroid dispatches drone correctly
- [ ] Clicking bottle dispatches drone correctly
- [ ] Android touch zoom unchanged
- [ ] Android touch pan unchanged

### Final
- [ ] `cargo build` Android — zero errors
- [ ] `cargo build --target wasm32-unknown-unknown` — zero errors
- [ ] Full play session in browser — mine, collect bottle, fulfill request
- [ ] Save persists across browser page refresh
- [ ] Tag `v2.8.0-wasm-port` on dev
- [ ] Merge to main

---

## File Touch Map

**Modified:**
- `src/lib.rs` — register `configure_egui_scale` startup system
- `src/systems/visuals/viewport.rs` OR wherever pinch zoom lives — add scroll wheel zoom
- `src/systems/ui/` wherever map pan lives — add click-drag pan
- Wherever `pixels_per_point` is currently hardcoded — replace with dynamic system

**Created:**
- None expected — changes fold into existing systems

---

## Out of Scope

- Any gameplay changes
- Phase 4 LOGS tab or FORGE rename
- Narrative drops
- Save format changes
- UI layout changes beyond scale fix
- Any new systems beyond what is specified
