# ADR-001: Mandatory PresentMode::Fifo for Mali GPUs
**Date:** April 2026  
**Status:** Accepted  

## Context
During Phase 3 (Mining Systems) development on the Moto G 2025 (Android 15, Mali GPU), the application experienced severe screen flickering and dropped frames. Logcat analysis revealed persistent `BufferQueueProducer: Can't acquire next buffer` errors, indicating buffer starvation in the display pipeline.

## Decision
All Android builds targeting Mali-based hardware MUST explicitly set the window's `present_mode` to `bevy::window::PresentMode::Fifo`.

## Rationale
Bevy's default `AutoVsync` (Mailbox) present mode fails to negotiate correctly with the Mali driver's buffer queue on Android 15, leading to starvation when the GPU cannot keep up with the swapchain requests. `Fifo` synchronizes with the display's vertical blanking interval (Vsync), which stabilizes frame pacing and eliminates the starvation logs.

## Consequences
- **Positive**: Resolves hardware-specific flickering and stabilizes frame pacing at a steady 60 FPS.
- **Positive**: Eliminates `BLASTBufferQueue` errors in logcat.
- **Constraint**: May introduce slightly higher input latency compared to immediate modes, but is required for visual integrity on this device class.
