# bake_android.ps1
# Use when only assets/balance.toml or assets/visual.toml changed.
# Touches config loaders to force include_str! rebake, then builds Android APK.
# Everything else uses the incremental cache — much faster than a full rebuild.

Write-Host "Touching config loaders to force include_str! rebake..."
(Get-Item src/config/balance.rs).LastWriteTime = Get-Date
(Get-Item src/config/visual.rs).LastWriteTime = Get-Date

Write-Host "Building Android with baked config..."
.\build_android.ps1
