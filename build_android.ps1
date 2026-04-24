<#
.SYNOPSIS
Voidrift Phase 0 - Android build script.
Compiles the Rust library via cargo-ndk, packages via Gradle, and installs to device.

.DESCRIPTION
Pipeline (per Phase 0 Directive Section 5.5):
  Step 1: Verify prerequisites
  Step 2: cargo-ndk compile -> android/app/src/main/jniLibs/arm64-v8a/
  Step 3: gradlew build -> APK
  Step 4: ADB install
  Step 5: Launch app and tail logcat

.EXAMPLE
  .\build_android.ps1              # Full build + install + logcat
  .\build_android.ps1 -BuildOnly  # Compile + package, no install
  .\build_android.ps1 -LogcatOnly # Just start logcat filter (APK already installed)

.NOTES
  NDK: r29 (29.0.14206865) - confirmed installed.
  Target: aarch64-linux-android, API 35, arm64-v8a ABI.
  Build tool: cargo-ndk (NOT cargo-apk - cargo-apk is deprecated).
#>

param (
    [switch]$BuildOnly,
    [switch]$LogcatOnly
)

$ErrorActionPreference = "Stop"

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
$RepoRoot   = $PSScriptRoot
$AndroidDir = Join-Path $RepoRoot "android"
$JniLibsDir = Join-Path $AndroidDir "app\src\main\jniLibs"
$LibName    = "voidrift"
$AbiTarget  = "arm64-v8a"

# Wireless ADB: set your phone's IP here (or leave empty to skip)
# One-time setup: plug in USB, run `adb tcpip 5555`, then unplug.
# Set a static IP on the phone so this never changes:
#   Settings → WiFi → long-press network → Modify → Static
$PhoneIP = "10.0.0.14"  # Moto G 2025 - set static IP to keep this stable

# Resolve ADB from Android SDK platform-tools - do not rely on system PATH.
# Same SDK root used by the NDK linker config.
$SdkRoot = $env:ANDROID_SDK_ROOT
if (-not $SdkRoot) { $SdkRoot = $env:ANDROID_HOME }
if (-not $SdkRoot) { $SdkRoot = "$env:LOCALAPPDATA\Android\Sdk" }
$AdbExe = Join-Path $SdkRoot "platform-tools\adb.exe"
if (-not (Test-Path $AdbExe)) {
    $AdbExe = "adb"
}

# ---------------------------------------------------------------------------
# Device connection: prefer wired USB, fall back to wireless
# ---------------------------------------------------------------------------
if (-not $LogcatOnly) {
    $wiredDevice = & $AdbExe devices 2>&1 | Select-String "^\w+\s+device$"
    if ($wiredDevice) {
        Write-Host "  ADB: wired device found." -ForegroundColor Green
    } elseif ($PhoneIP) {
        Write-Host "  ADB: no wired device, trying wireless ${PhoneIP}:5555..." -ForegroundColor Yellow
        $connectOut = & $AdbExe connect "${PhoneIP}:5555" 2>&1
        if ($connectOut -match "connected") {
            Write-Host "  ADB: wireless connected to ${PhoneIP}:5555" -ForegroundColor Green
        } else {
            Write-Host "  ADB: wireless failed - $connectOut" -ForegroundColor Red
            Write-Host "  Plug in USB or run: adb tcpip 5555 while wired." -ForegroundColor DarkGray
        }
    }
}

# Logcat filter pattern stored as variable - prevents PowerShell from
# misreading the regex | characters as pipeline operators at parse time.
$LogcatPattern = "voidrift|bevy|wgpu|RustStdoutStderr|AndroidRuntime|FATAL"

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
Write-Host "  Voidrift Phase 0 - Android Build" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "[1/5] Verifying prerequisites..." -ForegroundColor Cyan

# Android SDK
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

# Rust target
$targetInstalled = & rustup target list --installed 2>&1 | Select-String "aarch64-linux-android"
if (-not $targetInstalled) {
    Write-Host "  ERROR: aarch64-linux-android target not installed." -ForegroundColor Red
    Write-Host "  Run: rustup target add aarch64-linux-android" -ForegroundColor Yellow
    exit 1
}
Write-Host "  Rust target aarch64-linux-android: installed" -ForegroundColor DarkGray

# ADB
if (-not $BuildOnly) {
    $adbCheck = & $AdbExe version 2>&1
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: ADB not found at: $AdbExe" -ForegroundColor Red
        Write-Host "  Ensure Android Studio is installed with platform-tools." -ForegroundColor Yellow
        exit 1
    }
    Write-Host "  ADB: $($adbCheck | Select-Object -First 1)" -ForegroundColor DarkGray
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

$SoPath = Join-Path $JniLibsDir "$AbiTarget\lib$LibName.so"
if (-not (Test-Path $SoPath)) {
    Write-Host "  ERROR: lib$LibName.so not found at $SoPath" -ForegroundColor Red
    Write-Host "  Check cargo-ndk output above." -ForegroundColor Yellow
    exit 1
}
Write-Host "  lib$LibName.so produced: OK" -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 3: Gradle build
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
    Write-Host "  APK: $ApkPath" -ForegroundColor Cyan
    exit 0
}

# ---------------------------------------------------------------------------
# Step 4: ADB install
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[4/5] Installing APK on device..." -ForegroundColor Cyan

$devices = & $AdbExe devices
Write-Host "  Connected devices:" -ForegroundColor DarkGray
$devices | ForEach-Object { Write-Host "    $_" -ForegroundColor DarkGray }

# Pick the first available device serial so multi-device setups don't error
$targetSerial = & $AdbExe devices 2>&1 |
    Select-String "^\S+\s+device$" |
    Select-Object -First 1 |
    ForEach-Object { ($_ -split "\s+")[0] }

if (-not $targetSerial) {
    Write-Host "  ERROR: No ADB device found." -ForegroundColor Red
    exit 1
}
Write-Host "  Target device: $targetSerial" -ForegroundColor DarkGray

& $AdbExe -s $targetSerial install -r $ApkPath
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: ADB install failed (exit code $LASTEXITCODE)." -ForegroundColor Red
    Write-Host "  Is the device connected with USB debugging enabled?" -ForegroundColor Yellow
    exit $LASTEXITCODE
}
Write-Host "  Install: OK" -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 5: Launch + logcat
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[5/5] Launching app and tailing logcat..." -ForegroundColor Cyan
Write-Host "  Press Ctrl+C to stop." -ForegroundColor DarkGray
Write-Host ""

& $AdbExe -s $targetSerial shell am start -n "com.rfditservices.voidrift/.MainActivity"

Write-Host ""
Write-Host "  === LOGCAT (filtered) ===" -ForegroundColor Cyan
Write-Host "  Filter: $LogcatPattern" -ForegroundColor DarkGray
Write-Host ""

& $AdbExe -s $targetSerial logcat -c
& $AdbExe -s $targetSerial logcat | Select-String -Pattern $LogcatPattern
