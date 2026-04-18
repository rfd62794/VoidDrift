<#
.SYNOPSIS
Voidrift Phase 0 — Android build script.
Compiles the Rust library via cargo-ndk, packages via Gradle, and installs to device.

.DESCRIPTION
Pipeline (per Phase 0 Directive Section 5.5):
  Step 1: Verify prerequisites
  Step 2: cargo-ndk compile → android/app/src/main/jniLibs/arm64-v8a/
  Step 3: gradlew build → APK
  Step 4: ADB install
  Step 5: ADB logcat tail (filtered for voidrift output)

.EXAMPLE
  .\build_android.ps1              # Full build + install + logcat
  .\build_android.ps1 -BuildOnly  # Compile + package, no install
  .\build_android.ps1 -LogcatOnly # Just start logcat filter (device must already have APK)

.NOTES
  NDK: r29 (29.0.14206865) — confirmed installed.
  Target: aarch64-linux-android, API 35, arm64-v8a ABI.
  Build tool: cargo-ndk (NOT cargo-apk — cargo-apk is deprecated).
#>

param (
    [switch]$BuildOnly,
    [switch]$LogcatOnly
)

$ErrorActionPreference = "Stop"

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
$RepoRoot    = $PSScriptRoot
$AndroidDir  = Join-Path $RepoRoot "android"
$JniLibsDir  = Join-Path $AndroidDir "app\src\main\jniLibs"
$GradlewPath = Join-Path $AndroidDir "gradlew.bat"
$LibName     = "voidrift"
$AbiTarget   = "arm64-v8a"

# Resolve ADB path from Android SDK — do not rely on system PATH.
# SDK location is the same machine-specific path used for the NDK linker.
$SdkRootEarly = $env:ANDROID_SDK_ROOT
if (-not $SdkRootEarly) { $SdkRootEarly = $env:ANDROID_HOME }
if (-not $SdkRootEarly) { $SdkRootEarly = "$env:LOCALAPPDATA\Android\Sdk" }
$AdbExe = Join-Path $SdkRootEarly "platform-tools\adb.exe"
if (-not (Test-Path $AdbExe)) {
    # Fall back to adb on PATH if platform-tools not found at expected location
    $AdbExe = "adb"
}

# Logcat filter pattern — stored in variable to prevent PowerShell misreading
# the regex | alternation characters as pipeline operators at parse time.
$LogcatPattern = 'voidrift|bevy|wgpu|RustStdoutStderr|AndroidRuntime|FATAL'

# ---------------------------------------------------------------------------
# Logcat-only shortcut
# ---------------------------------------------------------------------------
if ($LogcatOnly) {
    Write-Host "[Voidrift] Starting logcat filter..." -ForegroundColor Cyan
    Write-Host "[Voidrift] Filter: $LogcatPattern" -ForegroundColor DarkGray
    & $AdbExe logcat | Select-String -Pattern $LogcatPattern
    exit 0
}

# ---------------------------------------------------------------------------
# Step 1: Prerequisites
# ---------------------------------------------------------------------------
Write-Host "" 
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  Voidrift Phase 0 — Android Build" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "[1/5] Verifying prerequisites..." -ForegroundColor Cyan

# ANDROID_SDK_ROOT
$SdkRoot = $env:ANDROID_SDK_ROOT
if (-not $SdkRoot) { $SdkRoot = $env:ANDROID_HOME }
if (-not $SdkRoot) { $SdkRoot = "$env:LOCALAPPDATA\Android\Sdk" }
if (-not (Test-Path $SdkRoot)) {
    Write-Host "  ERROR: Android SDK not found. Set ANDROID_SDK_ROOT or install Android Studio." -ForegroundColor Red
    exit 1
}
Write-Host "  Android SDK: $SdkRoot" -ForegroundColor DarkGray

# cargo-ndk
try {
    $cndkVersion = & cargo ndk --version 2>&1
    Write-Host "  cargo-ndk: $cndkVersion" -ForegroundColor DarkGray
} catch {
    Write-Host "  ERROR: cargo-ndk not found. Run: cargo install cargo-ndk" -ForegroundColor Red
    exit 1
}

# rustup target
$targets = & rustup target list --installed 2>&1
if ($targets -notmatch "aarch64-linux-android") {
    Write-Host "  ERROR: Rust target aarch64-linux-android not installed." -ForegroundColor Red
    Write-Host "  Run: rustup target add aarch64-linux-android" -ForegroundColor Yellow
    exit 1
}
Write-Host "  Rust target aarch64-linux-android: installed" -ForegroundColor DarkGray

# ADB (only needed if not BuildOnly)
if (-not $BuildOnly) {
    $adbTest = & $AdbExe version 2>&1
    if ($LASTEXITCODE -ne 0 -or $adbTest -match "not recognized") {
        Write-Host "  ERROR: ADB not found at $AdbExe" -ForegroundColor Red
        Write-Host "  Ensure Android Studio is installed with platform-tools." -ForegroundColor Yellow
        exit 1
    }
    Write-Host "  ADB: $($adbTest | Select-Object -First 1)" -ForegroundColor DarkGray
}

Write-Host "  Prerequisites OK." -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 2: cargo-ndk compile
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/5] Compiling Rust library via cargo-ndk..." -ForegroundColor Cyan
Write-Host "  Target ABI: $AbiTarget" -ForegroundColor DarkGray
Write-Host "  Output:     $JniLibsDir" -ForegroundColor DarkGray
Write-Host ""

# Ensure jniLibs directory exists
New-Item -ItemType Directory -Path "$JniLibsDir\$AbiTarget" -Force | Out-Null

Push-Location $RepoRoot
try {
    & cargo ndk -t $AbiTarget -o $JniLibsDir build --release --lib
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: cargo-ndk build failed (exit code $LASTEXITCODE)." -ForegroundColor Red
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

# Verify .so was produced
$SoPath = Join-Path $JniLibsDir "$AbiTarget\lib$LibName.so"
if (-not (Test-Path $SoPath)) {
    Write-Host "  ERROR: Expected lib$LibName.so not found at $SoPath" -ForegroundColor Red
    Write-Host "  Check cargo-ndk output above for compilation errors." -ForegroundColor Yellow
    exit 1
}
Write-Host "  lib$LibName.so produced: OK" -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 3: gradlew build
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/5] Building APK via Gradle..." -ForegroundColor Cyan

Push-Location $AndroidDir
try {
    & .\gradlew.bat assembleDebug
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: Gradle build failed (exit code $LASTEXITCODE)." -ForegroundColor Red
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

# Find APK
$ApkPath = Get-ChildItem -Path "$AndroidDir\app\build\outputs\apk" -Filter "*.apk" -Recurse |
    Sort-Object LastWriteTime -Descending | Select-Object -First 1 -ExpandProperty FullName

if (-not $ApkPath) {
    Write-Host "  ERROR: No APK found under $AndroidDir\app\build\outputs\apk\" -ForegroundColor Red
    exit 1
}
Write-Host "  APK built: $ApkPath" -ForegroundColor Green

if ($BuildOnly) {
    Write-Host ""
    Write-Host "  Build-only mode. Skipping install." -ForegroundColor Yellow
    Write-Host "  APK location: $ApkPath" -ForegroundColor Cyan
    exit 0
}

# ---------------------------------------------------------------------------
# Step 4: ADB install
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[4/5] Installing APK on device..." -ForegroundColor Cyan

$devices = & $AdbExe devices 2>&1
Write-Host "  Connected devices:" -ForegroundColor DarkGray
$devices | ForEach-Object { Write-Host "    $_" -ForegroundColor DarkGray }

& $AdbExe install -r $ApkPath
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: ADB install failed (exit code $LASTEXITCODE)." -ForegroundColor Red
    Write-Host "  Is the device connected with USB debugging enabled?" -ForegroundColor Yellow
    exit $LASTEXITCODE
}
Write-Host "  Install: OK" -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 5: Launch app + logcat
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[5/5] Launching app and tailing logcat..." -ForegroundColor Cyan
Write-Host "  Press Ctrl+C to stop logcat." -ForegroundColor DarkGray
Write-Host ""

# Launch the app
& $AdbExe shell am start -n "com.rfditservices.voidrift/.MainActivity" 2>&1

Write-Host ""
Write-Host "  === LOGCAT (filtered) ===" -ForegroundColor Cyan
Write-Host "  Watching for: $LogcatPattern" -ForegroundColor DarkGray
Write-Host ""

# Clear logcat first, then tail
& $AdbExe logcat -c
& $AdbExe logcat | Select-String -Pattern $LogcatPattern
