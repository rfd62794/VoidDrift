# Voidrift - Screenshot Utility
# April 2026

$ADB = "C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools\adb.exe"
$FILENAME = "screenshots/gate1_screenshot.png"

$TEMP_REMOTE = "/sdcard/voidrift_screen.png"
$FILENAME = "screenshots/gate2_screenshot.png"

Write-Host "Capturing screen from Moto G 2025 (Safe Mode)..." -ForegroundColor Cyan

# Use shell screencap + pull to ensure binary integrity on Windows
& $ADB shell screencap -p $TEMP_REMOTE
& $ADB pull $TEMP_REMOTE $FILENAME
& $ADB shell rm $TEMP_REMOTE

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
