// Voidrift — Phase 0 Android Spike
// ============================================================================
// Scope: Phase 0 ONLY.
// Deliverable: Coloured screen + touch event logged to logcat.
// No ECS components. No game systems. No sprites. No assets loaded.
//
// Gate 0 behaviours this file must satisfy:
//   TB-P0-02: App launches without crash
//   TB-P0-03: Background colour is visible (not black)
//   TB-P0-04: Touch event registers and prints to logcat
// ============================================================================

use bevy::prelude::*;

// bevy_main is the correct Android entry point macro for Bevy 0.15.
// It sets up the Android runtime bridge (GameActivity via android-activity).
// Do NOT replace this with fn main() — that will not work on Android.
#[bevy_main]
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // Fullscreen on Android — no title bar, no decorations.
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    MonitorSelection::Primary,
                ),
                title: "Voidrift".to_string(),
                ..default()
            }),
            ..default()
        }))
        // Deep-space dark background — confirms TB-P0-03 (not a black screen).
        // RGB approx: #050512
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.07)))
        .add_systems(Update, log_touches)
        .run();
}

/// Reads all active touches each frame and prints their position to logcat.
/// On device: visible in `adb logcat | grep voidrift` or RustStdoutStderr.
/// Satisfies TB-P0-04: touch event registers and prints within 3 seconds.
fn log_touches(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "[Voidrift Gate0] Touch detected — id: {}, position: ({:.1}, {:.1})",
            touch.id(),
            touch.position().x,
            touch.position().y,
        );
    }
}
