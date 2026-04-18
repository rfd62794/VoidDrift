# Voidrift - Screenshot Utility
# April 2026

$ADB = "C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools\adb.exe"
$FILENAME = "screenshots/gate1_screenshot.png"

Write-Host "Capturing screen from Moto G 2025..." -ForegroundColor Cyan

# Use System.IO.File.WriteAllBytes to ensure binary integrity on Windows
$bytes = & $ADB exec-out screencap -p
[System.IO.File]::WriteAllBytes($FILENAME, $bytes)

if (Test-Path $FILENAME) {
    $size = (Get-Item $FILENAME).Length
    if ($size -gt 0) {
        Write-Host "Success: Captured $FILENAME ($size bytes)" -ForegroundColor Green
    } else {
        Write-Error "Failed: Screenshot file is empty."
    }
} else {
    Write-Error "Failed: Screenshot file was not created."
}
