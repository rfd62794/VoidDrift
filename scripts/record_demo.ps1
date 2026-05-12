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
Write-Host "OBS: capture the Chrome window"
Write-Host "FFmpeg trim when done: see scripts/trim_demo.ps1"
