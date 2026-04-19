# Machine-specific environment setup for Voidrift
$env:ADB_PATH = "C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools\adb.exe"
# Add to path for current session
$env:PATH += ";C:\Users\cheat\AppData\Local\Android\Sdk\platform-tools"
Write-Host "Voidrift Environment Ready: ADB found at $env:ADB_PATH"
