# VoidDrift — Desktop Release Build
# Produces an optimized standalone EXE in releases/
# Use .\run.ps1 for development iteration (faster recompile via dynamic linking).

$ErrorActionPreference = "Stop"
$releaseDir = "$PSScriptRoot\releases"

Write-Host ""
Write-Host "============================================================"
Write-Host "  VoidDrift - Desktop Release Build"
Write-Host "============================================================"
Write-Host ""

Write-Host "[1/3] Building release binary..."
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Host "  ERROR: cargo build --release failed."; exit 1 }
Write-Host "  Build complete."
Write-Host ""

Write-Host "[2/3] Copying artifact..."
if (-not (Test-Path $releaseDir)) { New-Item -ItemType Directory -Path $releaseDir | Out-Null }
$src = "$PSScriptRoot\target\release\voidrift.exe"
$dst = "$releaseDir\voidrift-windows.exe"
Copy-Item $src $dst -Force
Write-Host "  Output: $dst"
Write-Host ""

Write-Host "[3/3] Artifact info..."
$size = [math]::Round((Get-Item $dst).Length / 1MB, 1)
Write-Host "  voidrift-windows.exe : $size MB"
Write-Host ""
Write-Host "  Release build complete."
Write-Host ""
