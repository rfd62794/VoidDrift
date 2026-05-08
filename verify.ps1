# verify.ps1 — Run cargo check and cargo test, capture output
# Usage: .\verify.ps1

$ErrorActionPreference = "Continue"
$outputFile = "context/last_error.txt"

New-Item -ItemType Directory -Force -Path "context" | Out-Null

Write-Host "Running cargo check..." -ForegroundColor Cyan
$checkOutput = cargo check 2>&1
$checkExit = $LASTEXITCODE

Write-Host "Running cargo test..." -ForegroundColor Cyan
$testOutput = cargo test 2>&1
$testExit = $LASTEXITCODE

$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

if ($checkExit -eq 0 -and $testExit -eq 0) {
    $result = "Sprint 8 verification pass. No errors found. [$timestamp]"
    Write-Host $result -ForegroundColor Green
} else {
    $result = @"
VERIFICATION FAILED [$timestamp]

--- cargo check ---
$checkOutput

--- cargo test ---
$testOutput
"@
    Write-Host "Errors found. See context/last_error.txt" -ForegroundColor Red
}

$result | Out-File -FilePath $outputFile -Encoding utf8
Write-Host "Output written to $outputFile"
