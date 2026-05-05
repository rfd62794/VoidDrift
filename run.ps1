# VoidDrift — Desktop Development Runner
# Reads assets/balance.toml at runtime — edit TOML and re-run without rebuilding Rust.
# Dynamic linking shortens incremental compile from ~40s to ~5s.

cargo run --features bevy/dynamic_linking
