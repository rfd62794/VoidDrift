<#
.SYNOPSIS
VoidDrift local checks (replaces GitHub Actions CI checks).

.DESCRIPTION
Runs cargo check and cargo test, exiting non-zero at the first failure.
Called by .githooks/pre-push. Skip the hook once with: git push --no-verify
(verify.ps1 is unchanged: it records output to context/last_error.txt but
always exits 0, so it can't gate a push.)

.EXAMPLE
  .\scripts\check.ps1
#>

$ErrorActionPreference = "Continue"
$RepoRoot = Split-Path $PSScriptRoot -Parent
Set-Location $RepoRoot

function Invoke-Step([string]$Name, [scriptblock]$Command) {
    Write-Host ""
    Write-Host "== $Name ==" -ForegroundColor Cyan
    & $Command
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAILED: $Name (exit $LASTEXITCODE)" -ForegroundColor Red
        exit $LASTEXITCODE
    }
}

$started = Get-Date
Invoke-Step "cargo check" { cargo check }
Invoke-Step "cargo test" { cargo test }

$elapsed = [int]((Get-Date) - $started).TotalSeconds
Write-Host ""
Write-Host "All checks passed in ${elapsed}s." -ForegroundColor Green
exit 0
