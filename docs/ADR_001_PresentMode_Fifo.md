# ADR 001: Mandatory PresentMode::Fifo for Mali GPUs

## Context
During Phase 3 (Mining Systems) development on the Moto G 2025 (Android 15, Mali GPU), we encountered severe screen flickering and buffer starvation (`Can't acquire next buffer`). Bevy's default `AutoVsync` present mode failed to negotiate correctly with the Mali driver's buffer queue, leading to frame drops and rendering stalls.

## Decision
All Android builds targetting Mali-based hardware MUST explicitly set the window's `present_mode` to `bevy::window::PresentMode::Fifo`.

## Consequences
- **Positive**: Resolves the hardware-specific flicker and stabilizes frame pacing at a steady 60 FPS.
- **Positive**: Eliminates `BLASTBufferQueue` errors in logcat.
- **Negative**: May introduce slightly higher input latency compared to immediate modes, but is required for visual integrity on this device class.
