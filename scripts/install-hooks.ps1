# Point git at the tracked hooks in .githooks (run once per clone).
# The pre-push hook runs scripts/check.ps1 before every push.
$RepoRoot = Split-Path $PSScriptRoot -Parent
git -C $RepoRoot config core.hooksPath .githooks
if ($LASTEXITCODE -eq 0) {
    Write-Host "Hooks installed: core.hooksPath = .githooks (pre-push runs scripts/check.ps1)" -ForegroundColor Green
}
exit $LASTEXITCODE
