# ADR-015: Canvas CSS approach for WASM iframe fullscreen
**Date:** May 2026
**Status:** Accepted

## Context
The game runs on itch.io via WASM in an iframe. itch.io embeds are constrained to a fixed size (1280x640 landscape), but the game needs fullscreen capability for the full mobile experience.

Initial attempts to handle fullscreen:
- **JavaScript fullscreen API**: `requestFullscreen()` doesn't fire inside cross-origin iframes (blocked by browser security)
- **Resize events**: Window resize events fire when the iframe is fullscreened, but timing is inconsistent
- **CSS media queries**: Can't detect iframe fullscreen state from within the iframe
- **Canvas scaling**: Scaling the canvas directly causes pixelation and aspect ratio issues

The problem: The game runs in a sandboxed iframe and cannot directly control the browser's fullscreen state. itch.io controls the iframe's display mode.

## Decision
Use CSS-based canvas sizing with the `:fullscreen` pseudo-class to handle fullscreen transitions:
- **Canvas CSS**: `max-width: 720px, max-height: 100vh, aspect-ratio: 9/16` for normal embed
- **Fullscreen CSS**: `canvas:fullscreen { max-width: 100vw, max-height: 100vh, aspect-ratio: auto }` for fullscreen
- **Injection**: Build script (`build_wasm.ps1`) injects canvas CSS into `pkg/index.html` after wasm-pack regenerates it
- **Detection**: Listen for window resize events as a proxy for fullscreen state changes
- **Fallback**: `:fullscreen` pseudo-class is more reliable than resize events for actual detection

## Rationale
This approach provides:
- **Cross-origin safe**: Works within iframe sandbox without JavaScript fullscreen API
- **CSS-native**: Leverages browser's built-in fullscreen pseudo-class detection
- **Responsive**: Canvas scales correctly between embed and fullscreen modes
- **Aspect-ratio aware**: Maintains 9:16 portrait ratio in embed, fills screen in fullscreen
- **Simple**: No complex JavaScript fullscreen handling, just CSS and resize listener
- **Reliable**: `:fullscreen` pseudo-class fires consistently when iframe enters fullscreen

## Consequences
- **Positive**: Works in cross-origin iframe environment where JavaScript fullscreen API is blocked
- **Positive**: CSS-based approach is performant and native to browser rendering
- **Positive**: Maintains correct aspect ratios in both embed and fullscreen modes
- **Constraint**: Requires canvas CSS injection in build pipeline (wasm-pack regenerates pkg/index.html)
- **Constraint**: Resize events are a proxy for fullscreen detection — timing may be inconsistent on some browsers
- **Constraint**: itch.io embed size is fixed (1280x640) — fullscreen is the only way to get full mobile experience
- **Maintenance**: Build script must maintain CSS injection logic
