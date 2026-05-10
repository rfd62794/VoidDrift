// Voidrift — Constants

// ─────────────────────────────────────────────────────────────────────────────
// NOTE: Most constants have been moved to balance.toml and visual.toml config files.
// This file now only contains constants that are not yet configurable or are used
// in contexts where config injection is not practical.
// ─────────────────────────────────────────────────────────────────────────────

use bevy::math::Vec2;
use bevy::prelude::Color;
pub const STATION_POS: Vec2      = Vec2::new(0.0, 0.0);
pub const SECTOR_1_POS: Vec2     = Vec2::new(320.0, 140.0);   // Iron
pub const SECTOR_2_POS: Vec2     = Vec2::new(-220.0, 340.0);  // Tungsten
pub const SECTOR_3_POS: Vec2     = Vec2::new(380.0, -280.0);  // Nickel
