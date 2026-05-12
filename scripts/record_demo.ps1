# Detect screen height — scale game window if monitor is short
Add-Type -AssemblyName System.Windows.Forms
$screen = [System.Windows.Forms.Screen]::PrimaryScreen
$screenH = $screen.Bounds.Height

if ($screenH -ge 1280) {
    $gameW = 720; $gameH = 1280
} else {
    # 75% scale — fits 1080p, OBS scales back up to 720x1280
    $gameW = 540; $gameH = 960
}

Write-Host "Monitor: $($screen.Bounds.Width)x$screenH"
Write-Host "Game window: ${gameW}x${gameH}"

# Start WASM server
Start-Process powershell -ArgumentList `
    "-NoExit", "-Command", `
    "cd '$PSScriptRoot\..\pkg'; python -m http.server 8080"

# Wait for server to start
Start-Sleep -Seconds 2

# Launch Chrome at exact portrait dimensions (matches itch embed)
$chrome = "C:\Program Files\Google\Chrome\Application\chrome.exe"
Start-Process $chrome -ArgumentList @(
    "--new-window",
    "--window-size=$gameW,$gameH",
    "--window-position=0,0",
    "--app=http://localhost:8080/index.html"
)

Write-Host "Server running at http://localhost:8080"
Write-Host "Browser open at ${gameW}x${gameH} portrait"

# Launch shot guide in a small Chrome window
Start-Sleep -Seconds 1
Start-Process $chrome -ArgumentList @(
    "--new-window",
    "--window-size=400,640",
    "--window-position=$gameW,0",
    "--app=file:///C:/Github/VoidDrift/scripts/shot_guide.html"
)

# Launch OBS (try standard paths)
$obs = @(
    "C:\Program Files\obs-studio\bin\64bit\obs64.exe",
    "C:\Program Files (x86)\obs-studio\bin\64bit\obs64.exe"
) | Where-Object { Test-Path $_ } | Select-Object -First 1

if ($obs) {
    Start-Process $obs
    Start-Sleep -Seconds 3
    Write-Host "OBS launched from: $obs"
} else {
    Write-Host "WARNING: OBS not found - launch manually before recording"
}

Write-Host ""
Write-Host "=== READY TO RECORD ==="
Write-Host "GAME:      Left  (0,0)         - OBS captures this window only"
Write-Host "SHOT GUIDE: Right ($gameW,0)   - visible to you, NOT in recording"
Write-Host ""
Write-Host "In OBS:"
Write-Host "  1. File > Scene Collection > Import > scripts/obs_scene.json"
Write-Host "  2. Confirm source shows Chrome game window"
Write-Host "  3. Set output: Settings > Output > Recording Path"
Write-Host "     Path: C:\Github\VoidDrift\raw_demo.mp4"
Write-Host "  4. Hit Start Recording"
Write-Host "  5. Follow the shot guide"
