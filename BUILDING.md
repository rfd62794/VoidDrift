# Building — Voidrift

Voidrift uses a modern Android pipeline involving `cargo-ndk` for Rust compilation and Gradle for APK packaging.

## Prerequisites

- **Rust**: 1.95.0+ (requires `aarch64-linux-android` target: `rustup target add aarch64-linux-android`)
- **Android SDK**: API 35 installed.
- **NDK**: r26+ (r29 proven).
- **cargo-ndk**: `cargo install cargo-ndk`
- **Gradle**: 8.6+ (auto-initialized by the `android/` wrapper).
- **ADB**: With USB debugging enabled on target device.

## NDK Configuration

`.cargo/config.toml` must point to your local NDK clang command.

```toml
[target.aarch64-linux-android]
linker = "C:\\Users\\<USER>\\AppData\\Local\\Android\\Sdk\\ndk\\<VERSION>\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\aarch64-linux-android35-clang.cmd"
rustflags = [
    "-C", "link-arg=-lc++_shared",
    "-C", "link-arg=-Wl,-z,max-page-size=16384"
]
```

### Rationale:
- `-lc++_shared`: Required for Bevy's C++ dependencies on Android.
- `max-page-size=16384`: Mandatory for Android 15+ (API 35) physical devices due to memory alignment changes.

## Build & Deploy Pipeline

Run the automated PowerShell script from the root:
```powershell
.\build_android.ps1
```

### What the script does:
1. **Verification**: Checks for SDK, NDK, and cargo-ndk.
2. **Compile**: Runs `cargo ndk build` targeting `aarch64-linux-android`.
3. **Packaging**: Triggers `gradlew build` in the `android/` directory.
4. **Install**: Uses `adb install -r` to deploy to the connected device.
5. **Logcat**: Automatically tails logs filtered for game events.

## Verification Tools

### `capture_gate_evidence.ps1`
Captures binary-correct PNG screenshots from the device via ADB. Use this for gate certification to avoid image artifacts from standard screen-grabbing tools.

## Common Failures

- **Linker path not found**: Occurs if the path in `.cargo/config.toml` doesn't match your local NDK installation.
- **Can't acquire next buffer**: Re-verify `PresentMode::Fifo` in `src/lib.rs`.
- **ADB device not authorized**: Check your phone screen for the permission prompt.
- **Invisible UI**: Ensure `bevy_egui` is used for all screenspace elements.

## WASM Build (Web / itch.io)

### Prerequisites

- **wasm-pack**: `cargo install wasm-pack`
- **wasm32-unknown-unknown target**: `rustup target add wasm32-unknown-unknown`

### Build Command

Run from the repo root:
```powershell
wasm-pack build --target web --out-dir pkg
```

Output lands in `pkg/`. wasm-pack writes `voidrift.js`, `voidrift_bg.wasm`, and binding files there.  
`pkg/index.html` is **hand-maintained** — wasm-pack does not own it. Do not let wasm-pack overwrite it.

### Notes

- There is no `build_wasm.ps1`. Run the command above directly.
- `[package.metadata.wasm-pack.profile.release]` in `Cargo.toml` disables wasm-opt (`wasm-opt = false`). Do not remove this.
- WASM entry point is the `start()` function in `src/lib.rs` (gated `#[cfg(target_arch = "wasm32")]`). It is separate from the Android/desktop `main()`.

---

## Deploying to itch.io (Butler)

Butler binary lives at `C:\Butler\butler.exe`. It must be on the system PATH.  
The deployment pipeline lives in a separate private repo: `C:\Github\RFD_IT_Publishing`.

### Deploy Command

```powershell
cd C:\Github\RFD_IT_Publishing
python publisher.py deploy voidrift --target itchio
```

This runs: `butler push C:\Github\VoidDrift\pkg rdug627/voidrift:html5`

### Dry Run (verify without pushing)

```powershell
python publisher.py deploy voidrift --target itchio --dry-run
```

### First-Time Auth

```powershell
butler login
```

Credentials are stored locally by Butler. No `.env` key needed for itch.io.

---

## Desktop Build (Development Only)

You can run the game locally for logic verification:
```powershell
cargo run
```
*Note: Some Mali-specific rendering issues (like ADR-003) will not manifest on Desktop.*
