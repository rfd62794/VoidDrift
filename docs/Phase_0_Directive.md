# Voidrift — Phase 0 Directive
## Bevy Android Spike

**Document:** Phase 0 Directive v1.0
**Date:** April 2026
**Status:** Approved — Pending Execution
**Gate:** Physical device screenshot + logcat (no exceptions)
**Produced By:** OQ-001 / OQ-002 research findings, April 2026

---

## 1. Objective

Prove that a Bevy 0.15 application builds, installs, and renders on the Moto G 2025 (API 35)
with touch input registering correctly.

No game logic. No ECS components. No systems. No sprites.
A coloured screen and a touch event printed to logcat is the entire deliverable.

Phase 0 is complete when — and only when — physical device evidence is produced per Section 7.

---

## 2. Confirmed Decisions (Pre-Locked)

These decisions were researched and confirmed before this directive was written.
They are not open for re-evaluation during Phase 0 execution.

| Decision | Choice | Rationale |
|---|---|---|
| Bevy version | **0.15** | Best Android community documentation coverage. GameActivity + cargo-ndk pipeline documented against this version. Version lock is explicit — not "latest". |
| Activity type | **GameActivity** | Supported default in Bevy 0.15. NativeActivity is the legacy path and creates technical debt. Moto G 2025 (API 35, API 31+ required) fully supports it. |
| Android project layout | **`android/` subfolder inside VoidDrift repo** | One repo, everything co-located. Matches OperatorGame discipline. |
| Gradle wrapper source | **Official Bevy mobile example — no hand-rolling** | Source: `github.com/bevyengine/bevy` tree `release-0.15.2`, path `examples/mobile/android_example`. Clone or copy this structure exactly. Do not invent a Gradle structure. |
| NDK version | **r29 (29.0.14206865) — already installed** | Confirmed installed at `%LOCALAPPDATA%\Android\Sdk\ndk\29.0.14206865`. Exceeds minimum recommended (r26+). No NDK install step required. |
| Linker | **`aarch64-linux-android35-clang.cmd`** | Confirmed present in NDK r29. API 35 matches device target. |
| STL | **`-lc++_shared` via rustflags** | Carried from OperatorGame `.cargo/config.toml` — proven working on this machine. |
| Page-size flag | **`-Wl,-z,max-page-size=16384`** | Mandatory for API 35 / Android 15+ devices. Already in OperatorGame config — carry forward. |
| Build toolchain | **`cargo-ndk` + `gradlew`** | `cargo-apk` is deprecated. Bevy 0.15 community docs and official examples use `cargo-ndk` for compile, Gradle for APK packaging. |
| Keystore | **New `voidrift.keystore`** | Do NOT reuse OperatorGame keystore. Generate fresh during Phase 0. |
| ABI target | **`arm64-v8a` (`aarch64-linux-android`) only** | Moto G 2025 is ARM64. armv7 not required for Gate 0. |

---

## 3. Scope — Explicit

### In Scope (Phase 0 Only)

- `Cargo.toml` — Bevy 0.15, lib crate type, Android conditional dependencies
- `.cargo/config.toml` — NDK linker and rustflags for `aarch64-linux-android`
- `src/lib.rs` — Bevy app entry point. Minimal: clear-colour system, single touch event log
- `android/` — Gradle wrapper project seeded from official Bevy mobile example
- `build_android.ps1` — PowerShell build script: cargo-ndk compile → gradlew build → ADB install
- `voidrift.keystore` — New keystore generated during this phase
- `docs/SDD_v0.2_corrections.md` — Correction notes for SDD v0.1 (see Section 8)

### Out of Scope (Hard Exclusions)

Any of the following appearing in a Phase 0 PR or commit is a scope violation:

- ECS components of any kind
- Bevy systems beyond a single clear-colour and a single touch-log system
- Sprite loading or image assets
- Any game logic or state machines
- Multiple scenes
- Desktop-only binary target (desktop build is allowed as a sanity check but is not the gate)
- bevy_android as a Cargo.toml entry (it is bundled — no entry required)
- Copying `.cargo/config.toml` from OperatorGame verbatim (the lib name, keystore path, and
  signing metadata differ — use it as a reference only)

---

## 4. Repository Structure After Phase 0

```
VoidDrift/
├── .cargo/
│   └── config.toml            ← New. aarch64 linker + rustflags
├── android/                   ← New. Seeded from Bevy release-0.15.2 mobile example
│   ├── app/
│   │   ├── build.gradle
│   │   ├── src/
│   │   │   └── main/
│   │   │       ├── AndroidManifest.xml
│   │   │       └── jniLibs/   ← cargo-ndk output lands here
│   ├── build.gradle
│   ├── gradle/
│   ├── gradlew
│   ├── gradlew.bat
│   └── settings.gradle
├── assets/                    ← Empty at Phase 0. Created so asset pipeline doesn't error
├── docs/
│   ├── Phase_0_Directive.md   ← This file
│   ├── SDD_v0.2_corrections.md ← New. Produced during Phase 0 (see Section 8)
│   └── Voidrift_SDD_v0_1.docx
├── src/
│   └── lib.rs                 ← Bevy app. Minimal — see Section 5
├── build_android.ps1          ← New. cargo-ndk + gradlew pipeline
├── voidrift.keystore          ← New. Do not commit to git (add to .gitignore)
├── Cargo.toml                 ← New
└── .gitignore                 ← New
```

---

## 5. Code Specifications

### 5.1 `Cargo.toml`

```toml
[package]
name = "voidrift"
version = "0.1.0"
edition = "2021"

[lib]
name = "voidrift"
crate-type = ["cdylib"]

[dependencies]
# Bevy version is pinned at 0.15 — confirmed by OQ-001 research.
# Do not upgrade without a new directive. "latest stable" in SDD v0.1 was inaccurate.
bevy = { version = "0.15", features = ["2d"] }

# serde and rand are deferred to Phase 1+ when game state exists.
# Do not add them here.

[profile.release]
opt-level     = 3
lto           = true
codegen-units = 1
panic         = "abort"
strip         = true
```

> ⚠ No `[package.metadata.android]` block. That block is cargo-apk metadata. Bevy 0.15 uses
> cargo-ndk + Gradle. Parameters like min/target SDK live in `android/app/build.gradle`.

### 5.2 `.cargo/config.toml`

```toml
# Voidrift Android NDK configuration
# NDK r29 (29.0.14206865) confirmed installed — do not change version.
# API 35 linker matches Moto G 2025 target.
# Rustflags carried from OperatorGame — confirmed working on this machine.

[target.aarch64-linux-android]
linker = "C:\\Users\\cheat\\AppData\\Local\\Android\\Sdk\\ndk\\29.0.14206865\\toolchains\\llvm\\prebuilt\\windows-x86_64\\bin\\aarch64-linux-android35-clang.cmd"
rustflags = [
    "-C", "link-arg=-lc++_shared",
    "-C", "link-arg=-Wl,-z,max-page-size=16384"
]
```

> ⚠ The linker path is machine-specific (hardcoded to `cheat` user profile). This is acceptable
> for solo development. If the machine changes, update this path.

### 5.3 `src/lib.rs`

The Phase 0 app must:
- Start a Bevy app
- Set a clear colour (deep space dark — `Color::rgb(0.02, 0.02, 0.08)`)
- Add a single system that reads `Touches` and prints to logcat when any touch is detected
- Use `#[bevy_main]` or the appropriate Android entry point macro for Bevy 0.15

**Exact system behaviour required:**
```
Touch detected at (x, y) — printed to logcat via info!() or println!()
```

No UI. No text on screen. No sprites. A coloured screen that logs touches is the full deliverable.

Refer to the Bevy 0.15 mobile example (`examples/mobile/`) for correct entry point macro usage.
Do not invent an Android entry point — the example defines the correct pattern.

### 5.4 `android/` — Gradle Wrapper

Seed from:
```
https://github.com/bevyengine/bevy/tree/release-0.15.2/examples/mobile/android_example
```

Adapt the following fields only — do not restructure the example layout:

| Field | Value |
|---|---|
| `applicationId` | `com.rfditservices.voidrift` |
| `minSdk` | `31` (GameActivity minimum; Moto G 2025 is API 35) |
| `targetSdk` | `35` |
| `lib_name` in AndroidManifest | `voidrift` (matches `[lib] name` in Cargo.toml) |
| Asset path reference | `../../assets` (relative to android/app/) |

### 5.5 `build_android.ps1`

The build script must perform these steps in order:

```
Step 1: Verify prerequisites
  - ANDROID_SDK_ROOT or ANDROID_HOME is set
  - cargo-ndk is installed (cargo ndk --version)
  - rustup target aarch64-linux-android is installed

Step 2: cargo-ndk compile
  cargo ndk -t arm64-v8a -o android/app/src/main/jniLibs build --release

Step 3: gradlew build
  cd android && ./gradlew build

Step 4: ADB install
  adb install -r android/app/build/outputs/apk/debug/<apkname>.apk

Step 5: ADB logcat tail (filtered)
  adb logcat | Select-String -Pattern "voidrift|bevy|wgpu|RustStdoutStderr|touch"
```

The script must print a clear failure message and exit if any step fails.
It must NOT silently continue past a failed step.

### 5.6 `voidrift.keystore`

Generate with:
```powershell
keytool -genkey -v `
  -keystore voidrift.keystore `
  -alias voidrift `
  -keyalg RSA -keysize 2048 `
  -validity 10000
```

Add `voidrift.keystore` to `.gitignore` immediately after generation.
Do not reuse OperatorGame keystore or alias.

---

## 6. Build Sequence (Reference)

This is the canonical sequence for Gate 0 verification:

```powershell
# From VoidDrift repo root

# 1. Add Rust Android target (once per machine)
rustup target add aarch64-linux-android

# 2. Install cargo-ndk (once per machine)
cargo install cargo-ndk

# 3. Build
.\build_android.ps1

# 4. Confirm device is connected
adb devices

# 5. Watch logcat for touch events
adb logcat | Select-String "voidrift|bevy|wgpu|touch|RustStdoutStderr"
```

---

## 7. Gate 0 — Evidence Requirements

Gate 0 is **not passed** until all three of the following are produced and attached:

| Evidence | What It Must Show |
|---|---|
| **Terminal output** | Full output of `.\build_android.ps1` — including `cargo ndk` compile output and `gradlew build` output. No truncation. |
| **Physical device screenshot** | Moto G 2025 screen showing the dark-coloured Bevy app running. Screenshot taken via `adb exec-out screencap -p > gate0_screenshot.png`. |
| **Logcat output** | `adb logcat` capture showing app launch without crash AND at least one touch event logged. Filter: `voidrift\|bevy\|wgpu\|RustStdoutStderr`. |

Agent summaries are NOT accepted as proof of any of these items.
AI-generated confirmation of success is NOT accepted.
A clean build that has never run on device does NOT pass the gate.

---

## 8. Named Test Behaviours (Phase 0 — Minimum 5 Required)

All five must be verified on physical device before gate is considered met:

| ID | Behaviour | Verification Method |
|---|---|---|
| TB-P0-01 | APK installs on Moto G 2025 without error | `adb install -r` exits 0, logcat shows no install failure |
| TB-P0-02 | App launches and renders without crash | Logcat shows no panic, no SIGSEGV, no `FATAL EXCEPTION` |
| TB-P0-03 | Background colour is visible (not black) | Physical device screenshot shows non-black screen |
| TB-P0-04 | Touch event registers and prints to logcat | Tap the screen; logcat shows touch coordinates within 3 seconds |
| TB-P0-05 | App survives 60 seconds of background/foreground cycle | Minimise app, restore, confirm it does not crash. Logcat clean. |

---

## 9. What the Next Phase Unlocks

Gate 0 passing unlocks **Phase 1: World Scaffold** only. Phase 1 directive will be written after
Gate 0 evidence is reviewed and signed off. Phase 1 scope (ECS components, sprites, scene) is
defined in the SDD but is not active until this gate closes.

Do not begin any Phase 1 work — even scaffolding or placeholder files — until Gate 0 is closed.

---

## 10. SDD v0.2 Corrections (Produced Alongside Phase 0)

> This section defines the corrections to be captured in `docs/SDD_v0.2_corrections.md`.
> The corrections file is a Phase 0 deliverable alongside the code. It does not require
> a new .docx — a markdown diff record is sufficient.

| Location in SDD v0.1 | Correction |
|---|---|
| Section 4.2 — Bevy Version & Dependencies | Remove "Latest stable as of April 2026" annotation from bevy 0.15 row. Replace with: "Pinned at 0.15 — chosen for Android community guide coverage (OQ-001). Current latest is 0.18. Migration to 0.18 deferred to post-slice." |
| Section 4.2 — Dependencies table | Remove `bevy_android` row entirely. Add footnote: "Android support is bundled within the `bevy` crate. No separate `bevy_android` Cargo.toml entry is required. The crate is an internal Bevy workspace component — it is not a user-facing dependency." |
| Section 4.1 — ADR-002 | Add resolved note: "Android-first confirmed. GameActivity (not NativeActivity) selected as activity type. Rationale: Bevy 0.15 default, API 31+ requirement met by Moto G 2025 (API 35), avoids deprecated NativeActivity path." |
| Open Questions — OQ-001 | Update Status from "Open" to "Resolved: Bevy 0.15 pinned." |
| Open Questions — OQ-002 | Update Status from "Open" to "Resolved: NDK r29 in use. Partial reuse from OperatorGame — rustflags and linker path carry over. Build toolchain (cargo-apk → cargo-ndk + Gradle) and activity type (NativeActivity → GameActivity) are new. See Phase 0 Directive." |

---

*Voidrift Phase 0 Directive v1.0 | April 2026 | RFD IT Services Ltd.*
*Produced from OQ-001 / OQ-002 research. All decisions confirmed by author before writing.*
*No file in the VoidDrift repository was modified to produce this document.*
