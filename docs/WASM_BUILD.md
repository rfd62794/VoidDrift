# WASM Build Guide

VoidDrift builds to WebAssembly for browser deployment via itch.io.

---

## Prerequisites

- Rust 1.85+
- wasm-pack v0.14.0 (`cargo install wasm-pack`)
- Node.js (for wasm-pack dependencies)

---

## Build Process

### 1. Build WASM Binary

```powershell
wasm-pack build --target web --out-dir pkg
```

This compiles the Rust code to WASM and generates JavaScript bindings.

**Output Location:** `C:\Github\VoidDrift\pkg\`

**Artifacts:**
- `voidrift_bg.wasm` (~22.7MB) - The compiled WASM binary
- `voidrift.js` (~149KB) - JavaScript bindings
- `package.json` - NPM package metadata

### 2. Copy Assets

The build script automatically copies assets:

```powershell
Copy-Item -Path "assets\fonts\FiraSans-Bold.ttf" -Destination "pkg\assets\fonts\FiraSans-Bold.ttf" -Force
```

This is required because WASM cannot access files outside its build output directory.

### 3. One-Click Build

Use the provided PowerShell script:

```powershell
.\build_wasm.ps1
```

This runs steps 1 and 2 in sequence.

---

## Entry Point

The WASM entry point is in `src/lib.rs`:

```rust
#[cfg(target_arch = "wasm32")]
pub fn start() {
    // WASM initialization
}
```

This is gated to only compile for the wasm32 architecture.

---

## HTML Entry Point

`pkg/index.html` is hand-maintained. wasm-pack does NOT own this file.

Key points:
- Loads `voidrift.js` and `voidrift_bg.wasm`
- Calls `start()` to initialize the game
- Sets up the canvas for rendering

Do not delete or overwrite this file.

---

## Asset Pipeline

### Font Loading

Fonts live at `assets/fonts/FiraSans-Bold.ttf` in the repository root.

The build script copies them to `pkg/assets/fonts/FiraSans-Bold.ttf` so the WASM build can load them.

### Git Tracking

`pkg/.gitignore` has exceptions:
```
!assets/
!assets/**
```

This ensures assets are tracked in git and can be pushed by Butler to itch.io.

---

## wasm-opt

wasm-opt is DISABLED in `Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = false
```

Do not remove this setting. It was disabled due to build issues.

---

## Local Testing

### Serve the Build

Use Python's built-in HTTP server:

```powershell
python -m http.server 8000 --directory pkg
```

Then open `http://localhost:8000` in a browser.

### Alternative: Use any local server

- VS Code Live Server extension
- Node.js http-server
- Any static file server

---

## Browser Compatibility

Tested on modern browsers:
- Chrome/Edge (Chromium)
- Firefox
- Safari

Requires WebAssembly support.

---

## Troubleshooting

### Build Fails

Ensure wasm-pack is installed:
```powershell
cargo install wasm-pack
```

Check Rust version:
```powershell
rustc --version
```

### Fonts Not Loading

Ensure assets were copied:
```powershell
Test-Path pkg\assets\fonts\FiraSans-Bold.ttf
```

If false, run the build script again.

### wasm-opt Errors

If you see wasm-opt errors, ensure it's disabled in `Cargo.toml` (it should be by default).

---

## See Also

- `build_wasm.ps1` - Build script
- `publish.ps1` - Deployment script
- `docs/directives/VoidDrift_WASM_Polish_Sprint_Directive.md` - WASM polish directive
