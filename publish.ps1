<#
.SYNOPSIS
VoidDrift — Publish to itch.io via Butler.

.DESCRIPTION
Pushes the pkg/ directory to itch.io using the Butler CLI.

Reads ITCHIO_TARGET from (in order of priority):
  1. $env:ITCHIO_TARGET environment variable
  2. .publish.env file in the repo root (gitignored, never committed)

.EXAMPLE
  .\publish.ps1              # Push pkg/ to itch.io
  .\publish.ps1 -DryRun      # Print Butler command without uploading
  .\publish.ps1 -Build       # Run build_wasm.ps1 first, then publish
  .\publish.ps1 -Build -DryRun

.NOTES
  Butler install : https://itch.io/docs/butler/
  Butler auth    : Run `butler login` once. Credentials stored locally — no key in .env.
  Config         : Copy .publish.env.example to .publish.env and set ITCHIO_TARGET.
                   .publish.env is gitignored and must never be committed.
#>

param (
    [switch]$DryRun,
    [switch]$Build
)

$ErrorActionPreference = "Stop"

$RepoRoot = $PSScriptRoot
$PkgDir   = Join-Path $RepoRoot "pkg"
$EnvFile  = Join-Path $RepoRoot ".publish.env"

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  VoidDrift - Publish to itch.io" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
if ($DryRun) {
    Write-Host "  DRY RUN - no files will be uploaded." -ForegroundColor Yellow
}
Write-Host ""

# ---------------------------------------------------------------------------
# Optional: build WASM first
# ---------------------------------------------------------------------------
if ($Build) {
    Write-Host "[pre] Running WASM build..." -ForegroundColor Cyan
    & (Join-Path $RepoRoot "build_wasm.ps1")
    if ($LASTEXITCODE -ne 0) {
        Write-Host "  ERROR: WASM build failed. Aborting publish." -ForegroundColor Red
        exit $LASTEXITCODE
    }
    Write-Host ""
}

# ---------------------------------------------------------------------------
# Step 1: Load ITCHIO_TARGET
# ---------------------------------------------------------------------------
Write-Host "[1/3] Loading publish target..." -ForegroundColor Cyan

$itchioTarget = $env:ITCHIO_TARGET

if (-not $itchioTarget -and (Test-Path $EnvFile)) {
    $line = Get-Content $EnvFile | Where-Object { $_ -match "^\s*ITCHIO_TARGET\s*=" } | Select-Object -First 1
    if ($line -and ($line -match "^\s*ITCHIO_TARGET\s*=\s*(.+)")) {
        $itchioTarget = $Matches[1].Trim()
    }
}

if (-not $itchioTarget) {
    Write-Host "  ERROR: ITCHIO_TARGET is not set." -ForegroundColor Red
    Write-Host ""
    Write-Host "  Fix: copy .publish.env.example to .publish.env and fill in your target." -ForegroundColor Yellow
    Write-Host "  Example value: ITCHIO_TARGET=yourname/yourgame:html5" -ForegroundColor DarkGray
    Write-Host ""
    Write-Host "  .publish.env is gitignored and stays local to your machine." -ForegroundColor DarkGray
    exit 1
}

Write-Host "  Target : $itchioTarget" -ForegroundColor DarkGray
Write-Host "  Source : $PkgDir" -ForegroundColor DarkGray

# ---------------------------------------------------------------------------
# Step 2: Locate Butler
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/3] Locating Butler..." -ForegroundColor Cyan

$butlerExe = $null

# Check PATH first
try {
    $versionOut = & butler --version 2>&1
    if ($LASTEXITCODE -eq 0) {
        $butlerExe = "butler"
        Write-Host "  Butler (PATH): $versionOut" -ForegroundColor DarkGray
    }
} catch {}

# Fall back to known local install location
if (-not $butlerExe) {
    $localPath = "C:\Butler\butler.exe"
    if (Test-Path $localPath) {
        $butlerExe = $localPath
        $versionOut = & $localPath --version 2>&1
        Write-Host "  Butler (local): $versionOut" -ForegroundColor DarkGray
    }
}

if (-not $butlerExe) {
    Write-Host "  ERROR: Butler not found in PATH or C:\Butler\butler.exe." -ForegroundColor Red
    Write-Host "  Install : https://itch.io/docs/butler/" -ForegroundColor Yellow
    Write-Host "  Auth    : butler login" -ForegroundColor Yellow
    exit 1
}

# ---------------------------------------------------------------------------
# Step 3: Push
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[3/3] Pushing to itch.io..." -ForegroundColor Cyan
Write-Host ""

if ($DryRun) {
    Write-Host "  Would execute: $butlerExe push `"$PkgDir`" $itchioTarget" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  Dry run complete. No files uploaded." -ForegroundColor Yellow
    Write-Host ""
    exit 0
}

& $butlerExe push $PkgDir $itchioTarget
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: Butler push failed (exit code $LASTEXITCODE)." -ForegroundColor Red
    Write-Host "  If auth error: run `butler login` and try again." -ForegroundColor Yellow
    exit $LASTEXITCODE
}

Write-Host ""
Write-Host "  Published successfully to itch.io: $itchioTarget" -ForegroundColor Green
Write-Host ""
