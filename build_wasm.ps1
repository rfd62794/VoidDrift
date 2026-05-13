<#
.SYNOPSIS
VoidDrift — WASM build script.
Compiles the Rust library to WebAssembly via wasm-pack.

.DESCRIPTION
Pipeline:
  Step 1: Verify wasm-pack is installed
  Step 2: wasm-pack build --target web --out-dir pkg
  Step 3: Verify output artifacts and report sizes

.EXAMPLE
  .\build_wasm.ps1
#>

$ErrorActionPreference = "Stop"

$RepoRoot = $PSScriptRoot
$PkgDir   = Join-Path $RepoRoot "pkg"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  VoidDrift - WASM Build" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host ""

# ---------------------------------------------------------------------------
# Step 1: Verify wasm-pack
# ---------------------------------------------------------------------------
Write-Host "[1/3] Checking wasm-pack..." -ForegroundColor Cyan

try {
    $wasmPackVersion = & wasm-pack --version 2>&1
    if ($LASTEXITCODE -ne 0) { throw }
    Write-Host "  wasm-pack: $wasmPackVersion" -ForegroundColor DarkGray
} catch {
    Write-Host "  ERROR: wasm-pack not found." -ForegroundColor Red
    Write-Host "  Install: cargo install wasm-pack" -ForegroundColor Yellow
    Write-Host "  Or:      https://rustwasm.github.io/wasm-pack/installer/" -ForegroundColor Yellow
    exit 1
}

# ---------------------------------------------------------------------------
# Step 2: Build
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/3] Building WASM..." -ForegroundColor Cyan
Write-Host "  Target : web" -ForegroundColor DarkGray
Write-Host "  Out dir: $PkgDir" -ForegroundColor DarkGray
Write-Host ""

# Preserve index.html — it is hand-maintained and not owned by wasm-pack.
# wasm-pack --target web does not generate index.html, but guard anyway.
$indexPath   = Join-Path $PkgDir "index.html"
$indexBackup = $null
if (Test-Path $indexPath) {
    $indexBackup = Get-Content $indexPath -Raw
}

Push-Location $RepoRoot
try {
    & wasm-pack build --target web --out-dir pkg
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: wasm-pack build failed (exit code $LASTEXITCODE)." -ForegroundColor Red
        exit $LASTEXITCODE
    }
} finally {
    Pop-Location
}

# Copy game assets into WASM web root so Bevy's asset server can fetch them via HTTP.
# Bevy WASM resolves asset_server.load("fonts/x.ttf") as GET assets/fonts/x.ttf
# relative to the page root. Without this copy, font loads return 404 silently.
Write-Host "  Copying assets to pkg/assets/..." -ForegroundColor DarkGray
robocopy (Join-Path $RepoRoot "assets") (Join-Path $PkgDir "assets") /E /NFL /NDL /NJH /NJS
Write-Host "  Assets copied." -ForegroundColor DarkGray

# Restore index.html if wasm-pack unexpectedly modified it
if ($null -ne $indexBackup) {
    $currentIndex = Get-Content $indexPath -Raw -ErrorAction SilentlyContinue
    if ($currentIndex -ne $indexBackup) {
        Write-Host "  WARNING: wasm-pack modified index.html - restoring hand-maintained version." -ForegroundColor Yellow
        Set-Content -Path $indexPath -Value $indexBackup -NoNewline
        Write-Host "  index.html restored." -ForegroundColor Green
    }
}

# Restore custom canvas CSS for fullscreen support.
# Strip ALL existing canvas CSS rules (handles base block, fullscreen variants,
# and any duplicates accumulated from prior buggy builds), then inject the
# canonical block once before </style>. Idempotent across rebuilds.
$customCSS = @"
        canvas {
            width: 100%; height: 100%;
            max-width: 720px; max-height: 100vh;
            aspect-ratio: 9 / 16; display: block;
            margin: 0 auto; touch-action: none;
        }
        canvas:fullscreen, canvas:-webkit-full-screen, canvas:-moz-full-screen {
            width: 100vw; height: 100vh;
            max-width: 100vw; max-height: 100vh;
            aspect-ratio: auto; margin: 0;
        }
"@
$indexContent = Get-Content $indexPath -Raw
# Strip every CSS rule whose selector starts with `canvas` (multiline-safe).
# Anchored to line start with optional indent to avoid matching the <canvas> HTML tag.
$indexContent = $indexContent -replace '(?m)^[ \t]*canvas[^{]*\{[^}]*\}\r?\n?', ''
# Inject canonical CSS once, just before </style>.
$indexContent = $indexContent -replace '</style>', ($customCSS.Trim() + "`r`n    </style>")
Set-Content -Path $indexPath -Value $indexContent -NoNewline
Write-Host "  Custom canvas CSS restored." -ForegroundColor Green

# ---------------------------------------------------------------------------
# Step 3: Verify artifacts
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/3] Verifying artifacts..." -ForegroundColor Cyan

$required = @("voidrift_bg.wasm", "voidrift.js")
$allOk    = $true

foreach ($artifact in $required) {
    $path = Join-Path $PkgDir $artifact
    if (Test-Path $path) {
        $sizeKb = [math]::Round((Get-Item $path).Length / 1KB, 1)
        Write-Host "  $artifact : ${sizeKb} KB" -ForegroundColor Green
    } else {
        Write-Host "  MISSING : $artifact" -ForegroundColor Red
        $allOk = $false
    }
}

if (-not $allOk) {
    Write-Host "  ERROR: One or more required artifacts are missing." -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "  Build complete. Output: $PkgDir" -ForegroundColor Green
Write-Host ""
