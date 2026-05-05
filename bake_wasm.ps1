# bake_wasm.ps1
# Use when only assets/balance.toml or assets/visual.toml changed.
# Touches config loaders to force include_str! rebake, then builds WASM.
# Everything else uses the incremental cache — much faster than a full rebuild.

Write-Host "Touching config loaders to force include_str! rebake..."
(Get-Item src/config/balance.rs).LastWriteTime = Get-Date
(Get-Item src/config/visual.rs).LastWriteTime = Get-Date
(Get-Item src/config/content.rs).LastWriteTime = Get-Date

Write-Host "Building WASM with baked config..."
.\build_wasm.ps1
