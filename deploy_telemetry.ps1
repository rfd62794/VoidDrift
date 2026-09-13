<#
.SYNOPSIS
VoidDrift — deploy rfd-telemetry to the server (local replacement for the
former GitHub Actions workflow deploy-telemetry.yml).

.DESCRIPTION
Pipeline:
  1. Copy rfd-telemetry/main.py to /var/www/rfd-telemetry/main.py (scp)
  2. Restart the rfd-telemetry service (ssh -t + sudo; you type the sudo
     password in this terminal — it is never stored)
  3. Check https://rfditservices.com/api/telemetry/health

Reads SSH_HOST, SSH_USER, SSH_PORT (default 22) and SSH_KEY (optional private
key path) from environment variables first, then from .deploy.env in the repo
root (gitignored — copy .deploy.env.example).

.EXAMPLE
  .\deploy_telemetry.ps1 -DryRun     # Print the commands without running them
  .\deploy_telemetry.ps1             # Deploy
#>

param (
    [switch]$DryRun,
    [switch]$SkipHealthCheck
)

$ErrorActionPreference = "Stop"

$RepoRoot   = $PSScriptRoot
$EnvFile    = Join-Path $RepoRoot ".deploy.env"
$Source     = Join-Path $RepoRoot "rfd-telemetry\main.py"
$RemotePath = "/var/www/rfd-telemetry/main.py"
$Service    = "rfd-telemetry"
$HealthUrl  = "https://rfditservices.com/api/telemetry/health"

function Get-Setting([string]$Name, [string]$Default = "") {
    $value = [Environment]::GetEnvironmentVariable($Name)
    if (-not $value -and (Test-Path $EnvFile)) {
        $line = Get-Content $EnvFile | Where-Object { $_ -match "^\s*$Name\s*=" } | Select-Object -First 1
        if ($line -and ($line -match "^\s*$Name\s*=\s*(.*)$")) { $value = $Matches[1].Trim() }
    }
    if ($value) { return $value }
    return $Default
}

Write-Host ""
Write-Host "============================================================" -ForegroundColor Cyan
Write-Host "  VoidDrift - Deploy rfd-telemetry" -ForegroundColor Cyan
Write-Host "============================================================" -ForegroundColor Cyan
if ($DryRun) { Write-Host "  DRY RUN - nothing will be copied or restarted." -ForegroundColor Yellow }

# ---------------------------------------------------------------------------
# Step 0: Settings
# ---------------------------------------------------------------------------
$sshHost = Get-Setting "SSH_HOST"
$sshUser = Get-Setting "SSH_USER"
$sshPort = Get-Setting "SSH_PORT" "22"
$sshKey  = Get-Setting "SSH_KEY"

$missing = @()
if (-not $sshHost) { $missing += "SSH_HOST" }
if (-not $sshUser) { $missing += "SSH_USER" }
if ($missing) {
    Write-Host ""
    Write-Host "  ERROR: missing $($missing -join ', ')." -ForegroundColor Red
    Write-Host "  Fix: copy .deploy.env.example to .deploy.env and fill it in (it is gitignored)." -ForegroundColor Yellow
    exit 1
}
if (-not (Test-Path $Source)) {
    Write-Host "  ERROR: $Source not found." -ForegroundColor Red
    exit 1
}

$keyArgs = @()
if ($sshKey) {
    $sshKey = $sshKey -replace '^~', $HOME
    if (-not (Test-Path $sshKey)) {
        Write-Host "  ERROR: SSH_KEY file not found: $sshKey" -ForegroundColor Red
        exit 1
    }
    $keyArgs = @("-i", $sshKey)
}

$target  = "$sshUser@$sshHost"
$scpArgs = @("-P", $sshPort) + $keyArgs + @($Source, "${target}:$RemotePath")
$sshArgs = @("-t", "-p", $sshPort) + $keyArgs + @($target, "sudo systemctl restart $Service")

Write-Host "  Target : $target (port $sshPort)" -ForegroundColor DarkGray
Write-Host "  Source : $Source -> $RemotePath" -ForegroundColor DarkGray

if ($DryRun) {
    Write-Host ""
    Write-Host "  Would execute:" -ForegroundColor Yellow
    Write-Host "    scp $($scpArgs -join ' ')" -ForegroundColor Yellow
    Write-Host "    ssh $($sshArgs -join ' ')" -ForegroundColor Yellow
    if (-not $SkipHealthCheck) { Write-Host "    GET $HealthUrl (after 10s)" -ForegroundColor Yellow }
    exit 0
}

# ---------------------------------------------------------------------------
# Step 1: Copy
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[1/3] Copying main.py..." -ForegroundColor Cyan
& scp @scpArgs
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: scp failed (exit $LASTEXITCODE)." -ForegroundColor Red
    exit $LASTEXITCODE
}

# ---------------------------------------------------------------------------
# Step 2: Restart (sudo prompts for the password in this terminal)
# ---------------------------------------------------------------------------
Write-Host ""
Write-Host "[2/3] Restarting $Service (enter the sudo password if prompted)..." -ForegroundColor Cyan
& ssh @sshArgs
if ($LASTEXITCODE -ne 0) {
    Write-Host "  ERROR: restart failed (exit $LASTEXITCODE)." -ForegroundColor Red
    exit $LASTEXITCODE
}

# ---------------------------------------------------------------------------
# Step 3: Health check
# ---------------------------------------------------------------------------
if ($SkipHealthCheck) {
    Write-Host ""
    Write-Host "  Deployed (health check skipped)." -ForegroundColor Green
    exit 0
}

Write-Host ""
Write-Host "[3/3] Checking health..." -ForegroundColor Cyan
Start-Sleep -Seconds 10
try {
    $response = Invoke-WebRequest -UseBasicParsing -Uri $HealthUrl -TimeoutSec 15
    if ($response.StatusCode -ne 200) { throw "HTTP $($response.StatusCode)" }
} catch {
    Write-Host "  ERROR: health check failed: $_" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "  Deployed and healthy: $HealthUrl" -ForegroundColor Green
exit 0
