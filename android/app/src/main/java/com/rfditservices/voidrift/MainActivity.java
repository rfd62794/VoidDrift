package com.rfditservices.voidrift;

import android.view.View;

import com.google.androidgamesdk.GameActivity;

/**
 * Voidrift — Phase 0 MainActivity
 *
 * Seeded from: github.com/bevyengine/bevy tree release-0.15.2
 * Path: examples/mobile/android_example/app/src/main/java/org/bevyengine/example/MainActivity.java
 *
 * Adaptations:
 *   - package: org.bevyengine.example → com.rfditservices.voidrift
 *   - System.loadLibrary: "bevy_mobile_example" → "voidrift"
 *     (must match [lib] name = "voidrift" in Cargo.toml)
 *   - Everything else is verbatim from the official Bevy example.
 *
 * This class extends GameActivity (Bevy 0.15 default).
 * It loads the compiled Rust .so and hides the system UI for full-screen.
 */
public class MainActivity extends GameActivity {
    static {
        // Load the compiled Rust shared library.
        // Name must match Cargo.toml [lib] name = "voidrift"
        // and AndroidManifest android:value="voidrift".
        System.loadLibrary("voidrift");
    }

    @Override
    public void onWindowFocusChanged(boolean hasFocus) {
        super.onWindowFocusChanged(hasFocus);

        if (hasFocus) {
            hideSystemUi();
        }
    }

    private void hideSystemUi() {
        View decorView = getWindow().getDecorView();
        decorView.setSystemUiVisibility(
                View.SYSTEM_UI_FLAG_IMMERSIVE_STICKY
                        | View.SYSTEM_UI_FLAG_LAYOUT_STABLE
                        | View.SYSTEM_UI_FLAG_LAYOUT_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_LAYOUT_FULLSCREEN
                        | View.SYSTEM_UI_FLAG_HIDE_NAVIGATION
                        | View.SYSTEM_UI_FLAG_FULLSCREEN
        );
    }
}
