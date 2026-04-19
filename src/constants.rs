// Voidrift — Constants

pub const SHIP_SPEED: f32 = 180.0;
pub const ARRIVAL_THRESHOLD: f32 = 8.0;
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
pub const STATION_POS: Vec2 = Vec2::new(-150.0, -200.0);
pub const SECTOR_1_POS: Vec2 = Vec2::new(150.0, 100.0);
pub const SECTOR_7_POS: Vec2 = Vec2::new(350.0, 250.0);
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
