// Voidrift — Constants

pub const SHIP_SPEED: f32 = 180.0;
pub const ARRIVAL_THRESHOLD: f32 = 8.0;
pub const ARRIVAL_THRESHOLD_MINING: f32 = 40.0;
pub const MAP_OVERVIEW_SCALE: f32 = 1.5;

// [PHASE 4] EGUI CONFIG 
pub const EGUI_SCALE: f32 = 3.0;

pub const CARGO_CAPACITY: u32 = 100;
pub const MINING_RATE: f32 = 20.0;

pub const REFINERY_RATIO: u32 = 10;
pub const HULL_REFINERY_RATIO: u32 = 5;
pub const REPAIR_COST: u32 = 25;

// [PHASE 9] PRODUCTION CHAIN COSTS
pub const SHIP_HULL_COST_PLATES: u32 = 3;
pub const AI_CORE_COST_CELLS: u32 = 50;
pub const POWER_COST_SHIP_FORGE: u32 = 3;
pub const POWER_COST_AI_FABRICATE: u32 = 5;

// Adding missing AI_CORE_COST from audit directive
pub const AI_CORE_COST: u32 = 50; 

use bevy::math::Vec2;
use bevy::prelude::Color;
pub const STATION_POS: Vec2 = Vec2::new(0.0, 0.0);
pub const SECTOR_1_POS: Vec2 = Vec2::new(180.0, 80.0);
pub const SECTOR_7_POS: Vec2 = Vec2::new(-280.0, -180.0);
pub const SECTOR_3_POS: Vec2 = Vec2::new(-500.0, 400.0);

pub const MAP_STRATEGIC_SCALE: f32 = 4.0;

// ── Z-LAYER SYSTEM ───────────────────────────────────────────────────────────
pub const Z_STARS_FAR: f32     = -100.0;
pub const Z_STARS_NEAR: f32    = -50.0;
pub const Z_CONNECTORS: f32    = -5.0;
pub const Z_ENVIRONMENT: f32   = 0.5;  // Asteroids, station
pub const Z_MAP_MARKERS: f32   = 0.6;  // Opaque border/marker overlay
pub const Z_SHIP: f32          = 1.0;
pub const Z_BEAM: f32          = 0.8;
pub const Z_CARGO_BAR: f32     = 1.1;
pub const Z_HUD: f32           = 2.0;

// NARRATIVE TIMING
pub const SIGNAL_PAUSE_S2: f32          = 2.0; // Between S-001 and S-002
pub const SIGNAL_PAUSE_DOCK_REPORT: f32 = 1.0; // Between S-005 and S-006
pub const SIGNAL_PAUSE_COMPLETE: f32    = 1.5; // Between S-006 and S-007/UI Unlock

// STATION VISUALS (Phase A)
pub const STATION_HUB_RADIUS: f32     = 40.0;
pub const STATION_ARM_LENGTH: f32     = 120.0;
pub const STATION_ARM_THICKNESS: f32  = 6.0;
pub const STATION_BERTH_RADIUS: f32   = 22.0;
pub const STATION_STUB_LENGTH: f32    = 60.0;
pub const STATION_STUB_ALPHA: f32     = 0.3;
pub const STATION_ROTATION_SPEED: f32 = std::f32::consts::TAU / 90.0;
pub const STATION_BERTHS_INITIAL: u8  = 3;
// ─────────────────────────────────────────────────────────────────────────────

// [STEP 6] MAP COLORS
pub const COLOR_MAP_STATION: Color    = Color::srgb(1.0, 1.0, 0.0);       // Yellow
pub const COLOR_MAP_S1: Color         = Color::srgb(0.0, 1.0, 1.0);       // Cyan (Magnetite)
pub const COLOR_MAP_S7: Color         = Color::srgb(0.3, 0.8, 0.3);       // Green (Carbon)
pub const COLOR_MAP_S3: Color         = Color::srgb(0.2, 0.2, 0.2);       // Dark Grey
pub const COLOR_MAP_LINE: Color       = Color::srgba(1.0, 1.0, 1.0, 0.2); // Dim White
pub const COLOR_MAP_HIGHLIGHT: Color  = Color::srgb(1.0, 1.0, 1.0);       // White

pub const LOG_MAX_LINES: usize = 10;

// [PHASE 8] POWER COSTS & TIMING
pub const POWER_COST_CYCLE_TOTAL: u32 = 4;
pub const POWER_COST_REFINERY: u32 = 1;
pub const POWER_COST_HULL_FORGE: u32 = 2;
pub const POWER_WARNING_INTERVAL: f32 = 30.0;

// [PHASE 8b] POWER vs POWER CELLS
pub const POWER_CELL_RESTORE_VALUE: f32 = 3.0;
pub const SHIP_POWER_MAX: f32 = 10.0;
pub const SHIP_POWER_FLOOR: f32 = 3.0;
pub const SHIP_POWER_COST_TRANSIT: f32 = 1.0;
pub const SHIP_POWER_COST_MINING: f32 = 2.0;
pub const STATION_POWER_MAX: f32 = 50.0;
pub const STATION_POWER_FLOOR: f32 = 10.0;
pub const STATION_POWER_RESTORE_VALUE: f32 = 5.0;
pub const EMERGENCY_REFINE_COST: f32 = 10.0;
