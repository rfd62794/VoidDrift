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
    "--window-size=720,1280",
    "--window-position=100,0",
    "--app=http://localhost:8080/index.html"
)

Write-Host "Server running at http://localhost:8080"
Write-Host "Browser open at 720x1280 portrait"

# Launch shot guide in a small Chrome window
Start-Sleep -Seconds 1
Start-Process $chrome -ArgumentList @(
    "--new-window",
    "--window-size=400,520",
    "--window-position=840,0",
    "--app=file:///C:/Github/VoidDrift/scripts/shot_guide.html"
)

# Launch OBS (standard install path)
$obs = "C:\Program Files\obs-studio\bin\64bit\obs64.exe"
if (Test-Path $obs) {
    Start-Process $obs
    Write-Host "OBS launched - import scene from scripts/obs_scene.json if needed"
    Write-Host "Set output path to: C:/Github/VoidDrift/raw_demo.mp4"
    Write-Host "Hit Record in OBS when ready"
} else {
    Write-Host "OBS not found at standard path - launch manually"
}

Write-Host ""
Write-Host "=== RECORDING SETUP COMPLETE ==="
Write-Host "1. Game open at 720x1280 (left)"
Write-Host "2. Shot guide open at 400x520 (right)"
Write-Host "3. OBS open - configure window capture to Chrome game window"
Write-Host "4. Hit Record in OBS"
Write-Host "5. Follow the shot guide"
Write-Host "6. Run trim_demo.ps1 when done"
