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
pub const SHIP_HULL_COST_PLATES: u32   = 3;
pub const HULL_PLATE_COST_IRON: u32    = 2;  // Only iron needed
pub const THRUSTER_COST_TUNGSTEN: u32  = 1;  // 1 Tungsten ingot per thruster
pub const AI_CORE_COST_NICKEL: u32     = 1; 

use bevy::math::Vec2;
use bevy::prelude::Color;
pub const STATION_POS: Vec2      = Vec2::new(0.0, 0.0);
pub const SECTOR_1_POS: Vec2     = Vec2::new(320.0, 140.0);   // Iron
pub const SECTOR_2_POS: Vec2     = Vec2::new(-220.0, 340.0);  // Tungsten
pub const SECTOR_3_POS: Vec2     = Vec2::new(380.0, -280.0);  // Nickel

pub const MAP_STRATEGIC_SCALE: f32 = 8.0;
pub const ZOOM_MIN: f32   = 0.3;   // most zoomed in
pub const ZOOM_MAX: f32   = 8.0;   // most zoomed out
pub const ZOOM_SPEED: f32 = 0.005;

// ── ASTEROID SCALE & COLOR ───────────────────────────────────────────────────
pub const ASTEROID_RADIUS_IRON: f32      = 20.0;
pub const ASTEROID_RADIUS_TUNGSTEN: f32  = 22.0;
pub const ASTEROID_RADIUS_NICKEL: f32    = 24.0;

pub const COLOR_IRON: Color       = Color::srgb(0.75, 0.38, 0.15);  // Rust orange
pub const COLOR_TUNGSTEN: Color   = Color::srgb(0.72, 0.68, 0.35);  // Yellow-grey
pub const COLOR_NICKEL: Color     = Color::srgb(0.75, 0.75, 0.75);  // Silver
pub const COLOR_DEPLETED: Color   = Color::srgb(0.18, 0.18, 0.18);  // Very dark grey

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
pub const STATION_BERTH_INITIAL: u8  = 3; // Fixed typo

// STATION DOCKING (Phase B)
pub const STATION_DOCK_SLOWDOWN_DISTANCE: f32 = 200.0; // Distance at which station begins slowing
pub const STATION_SLOWDOWN_RATE: f32 = STATION_ROTATION_SPEED * 3.0; // Deceleration rate per second
pub const STATION_RESUME_DELAY: f32 = 1.5;  // Seconds after dock before rotation resumes
pub const STATION_RESUME_RATE: f32 = STATION_ROTATION_SPEED * 2.0; // Acceleration rate on resume
pub const BERTH_1_ARM_INDEX: u8 = 0;  // Player berth — arm 0
pub const BERTH_2_ARM_INDEX: u8 = 1;  // Drone/NPC berth — arm 1
pub const BERTH_3_ARM_INDEX: u8 = 2;  // Open berth — arm 2

// ─────────────────────────────────────────────────────────────────────────────

// [STEP 6] MAP COLORS
pub const COLOR_MAP_STATION: Color    = Color::srgb(1.0, 1.0, 0.0);       // Yellow
pub const COLOR_MAP_S1: Color         = Color::srgb(0.0, 1.0, 1.0);       // Cyan (Magnetite)
pub const COLOR_MAP_S7: Color         = Color::srgb(0.3, 0.8, 0.3);       // Green (Carbon)
pub const COLOR_MAP_S3: Color         = Color::srgb(0.2, 0.2, 0.2);       // Dark Grey
pub const COLOR_MAP_LINE: Color       = Color::srgba(1.0, 1.0, 1.0, 0.2); // Dim White
pub const COLOR_MAP_HIGHLIGHT: Color  = Color::srgb(1.0, 1.0, 1.0);       // White

pub const LOG_MAX_LINES: usize = 10;

// [PHASE 10] PROCESSING QUEUE TIMES (Seconds per batch)
pub const REFINERY_IRON_TIME: f32      = 2.0;
pub const REFINERY_TUNGSTEN_TIME: f32  = 2.5;
pub const REFINERY_NICKEL_TIME: f32    = 1.5;
pub const FORGE_HULL_TIME: f32         = 5.0;
pub const FORGE_THRUSTER_TIME: f32     = 5.0;  // Same as Hull
pub const FORGE_CORE_TIME: f32         = 6.0;

pub const MAP_PAN_SPEED: f32 = 1.5;
