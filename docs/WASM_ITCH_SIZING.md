# WASM Itch.io Sizing Reference

## Confirmed Runtime Values (May 2026)

### Canvas and Embed
- itch.io embed: 1280x640 (landscape, manually set)
- Canvas CSS: max-width 720px, max-height 100vh, aspect-ratio 9/16
- Fullscreen CSS: canvas:fullscreen removes constraints

### Content Area (cargo bay / drawer content)
- `ui.available_height()` on itch.io WASM = **162.1875px** (confirmed via console log)
- This is egui logical pixels, not physical pixels
- EGUI_SCALE = 3.0 applies to world coordinates, not egui panels
- content_height in mod.rs set to 250.0 → actual available = ~162px after header/margins

### Panel Heights (egui logical px)
- signal_height: 64.0 (collapsed), 64.0 (expanded — do not multiply)
- handle_height: 32.0
- primary_tab_height: 48.0
- secondary_tab_height: 48.0
- content_height: 250.0

### Row Layout (cargo bay chain)
- available_height = ui.available_height() - 10.0 ≈ 152px
- row_height = available_height / 4.0 ≈ 38px
- 4 rows fit using 0.2/1.2/2.2/3.2 multipliers from content_top_y
- symbol_size clamped 24.0-36.0px

### Key Learnings
- wasm-pack regenerates pkg/index.html on every build — canvas CSS must be reinjected by build_wasm.ps1
- fit_canvas_to_parent: true makes canvas fit iframe, not browser window
- fullscreenchange event does NOT fire inside cross-origin iframe
- window resize event DOES fire when iframe is fullscreened — but canvas CSS :fullscreen pseudo-class is more reliable
- Desktop local runs use larger available_height than itch.io WASM — always verify on itch.io
