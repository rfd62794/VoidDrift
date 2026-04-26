# PHASE 1: Power Deletion + Ore Simplification
**Objective:** Remove power system entirely, simplify ore to Iron/Tungsten/Nickel only  
**Status:** Ready for implementation in Windsurf  
**Estimated time:** 2-3 hours of focused editing + compile checks  
**Risk level:** Medium (pervasive changes, but mechanical)

---

## Overview

After Phase 1, Voidrift will have:
- ✅ No power mechanics (power_cells, power_cost, power_warnings all gone)
- ✅ Three ore types only: Iron, Tungsten, Nickel
- ✅ Clean resource pipeline: Ore → Ingot → Product → Drone
- ✅ Simpler Station initialization
- ✅ Smaller constants.rs
- ✅ Game runs and compiles cleanly

---

## STEP 1: Update components.rs — Delete Power Fields from Ship

**File:** `src/components.rs`  
**Lines to delete:** 34-35

**Current code (lines 27-37):**
```rust
#[derive(Component)]
pub struct Ship {
    pub state: ShipState,
    pub speed: f32,
    pub cargo: f32,
    pub cargo_type: OreType,
    pub cargo_capacity: u32,
    pub power: f32,              // ← DELETE THIS LINE
    pub power_cells: u32,         // ← DELETE THIS LINE
    pub laser_tier: LaserTier,
}
```

**After deletion:**
```rust
#[derive(Component)]
pub struct Ship {
    pub state: ShipState,
    pub speed: f32,
    pub cargo: f32,
    pub cargo_type: OreType,
    pub cargo_capacity: u32,
    pub laser_tier: LaserTier,
}
```

**Action in Windsurf:**
1. Open `src/components.rs`
2. Go to line 34, select entire line `pub power: f32,`
3. Delete
4. Go to line 35 (now line 34), select entire line `pub power_cells: u32,`
5. Delete
6. Save file

**Verify:** No red squiggles on Ship struct

---

## STEP 2: Update components.rs — Delete Power Fields from Station

**File:** `src/components.rs`  
**Lines to delete:** 100-103 (after Step 1, will shift up)

**Current code (lines 92-109):**
```rust
#[derive(Component)]
pub struct Station {
    pub repair_progress: f32,
    pub online: bool,
    pub magnetite_reserves: f32,
    pub carbon_reserves: f32,
    pub hull_plate_reserves: u32,
    pub ship_hulls: u32,
    pub ai_cores: u32,
    pub power_cells: u32,         // ← DELETE THIS LINE
    pub power: f32,                // ← DELETE THIS LINE
    pub maintenance_timer: Timer,  // ← DELETE THIS LINE
    pub last_power_warning_time: f32, // ← DELETE THIS LINE
    pub log: VecDeque<String>,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub dock_state: StationDockState,
    pub resume_timer: f32,
}
```

**After deletion:**
```rust
#[derive(Component)]
pub struct Station {
    pub repair_progress: f32,
    pub online: bool,
    pub magnetite_reserves: f32,
    pub carbon_reserves: f32,
    pub hull_plate_reserves: u32,
    pub ship_hulls: u32,
    pub ai_cores: u32,
    pub log: VecDeque<String>,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub dock_state: StationDockState,
    pub resume_timer: f32,
}
```

**Action in Windsurf:**
1. Open `src/components.rs` (should still be open)
2. Find `pub power_cells: u32,` in Station struct
3. Delete that line
4. Delete `pub power: f32,`
5. Delete `pub maintenance_timer: Timer,`
6. Delete `pub last_power_warning_time: f32,`
7. Save file

**Verify:** Station struct compiles, no red squiggles

---

## STEP 3: Update components.rs — Replace OreType with OreDeposit

**File:** `src/components.rs`  
**Action:** Remove OreType enum, use OreDeposit everywhere

**Current code (lines 12-17):**
```rust
#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub enum OreType {
    #[default]
    Empty,
    Magnetite,
    Carbon,
}
```

**DELETE ENTIRE ENUM** (lines 12-17)

**Current code (lines 32):**
```rust
pub cargo_type: OreType,
```

**REPLACE WITH:**
```rust
pub cargo_type: OreDeposit,
```

**Current code (lines 52-60):**
```rust
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum OreDeposit {
    Magnetite,
    Iron,
    Carbon,
    Tungsten,
    Titanite,
    CrystalCore,
}
```

**REPLACE WITH:**
```rust
#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum OreDeposit {
    Iron,
    Tungsten,
    Nickel,
}
```

**Update ore_name() function (lines 69-78):**

**Current:**
```rust
pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Magnetite  => "MAGNETITE",
        OreDeposit::Iron       => "IRON",
        OreDeposit::Carbon     => "CARBON",
        OreDeposit::Tungsten   => "TUNGSTEN",
        OreDeposit::Titanite   => "TITANITE",
        OreDeposit::CrystalCore => "CRYSTAL",
    }
}
```

**REPLACE WITH:**
```rust
pub fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Iron     => "IRON",
        OreDeposit::Tungsten => "TUNGSTEN",
        OreDeposit::Nickel   => "NICKEL",
    }
}
```

**Update ore_laser_required() function (lines 80-89):**

**Current:**
```rust
pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Magnetite  => LaserTier::Basic,
        OreDeposit::Iron       => LaserTier::Basic,
        OreDeposit::Carbon     => LaserTier::Basic,
        OreDeposit::Tungsten   => LaserTier::Tungsten,
        OreDeposit::Titanite   => LaserTier::Tungsten,
        OreDeposit::CrystalCore => LaserTier::Composite,
    }
}
```

**REPLACE WITH:**
```rust
pub fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Iron     => LaserTier::Basic,
        OreDeposit::Tungsten => LaserTier::Tungsten,
        OreDeposit::Nickel   => LaserTier::Basic,
    }
}
```

**Action in Windsurf:**
1. Delete OreType enum (lines 12-17)
2. In Ship struct, change `cargo_type: OreType` → `cargo_type: OreDeposit`
3. Replace OreDeposit enum (lines 52-60) with 3-ore version
4. Update ore_name() function to match new enum
5. Update ore_laser_required() function to match new enum
6. Save file

**Verify:** Run `cargo check 2>&1 | grep -i error` — should show errors about unused OreType in other files (expected, we'll fix next)

---

## STEP 4: Update constants.rs — Delete Power Constants

**File:** `src/constants.rs`  
**Lines to delete:** 106-127 (all power-related constants)

**Current code (lines 106-127):**
```rust
// [PHASE 8] POWER COSTS & TIMING
pub const POWER_COST_CYCLE_TOTAL: u32 = 4;
pub const POWER_COST_REFINERY: u32 = 1;
pub const POWER_COST_HULL_FORGE: u32 = 2;
pub const POWER_WARNING_INTERVAL: f32 = 30.0;

// [PHASE 10] PROCESSING QUEUE TIMES (Seconds per batch)
pub const REFINERY_MAGNETITE_TIME: f32 = 20.0;
pub const REFINERY_CARBON_TIME: f32    = 30.0;
pub const FORGE_HULL_TIME: f32         = 45.0;
pub const FORGE_CORE_TIME: f32         = 60.0;

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
pub const MAP_PAN_SPEED: f32 = 1.5;
```

**KEEP ONLY:**
```rust
// [PHASE 10] PROCESSING QUEUE TIMES (Seconds per batch)
pub const REFINERY_IRON_TIME: f32      = 20.0;
pub const REFINERY_TUNGSTEN_TIME: f32  = 25.0;
pub const REFINERY_NICKEL_TIME: f32    = 15.0;
pub const FORGE_HULL_TIME: f32         = 45.0;
pub const FORGE_CORE_TIME: f32         = 60.0;

pub const MAP_PAN_SPEED: f32 = 1.5;
```

**Delete POWER_COST_CYCLE_TOTAL, POWER_COST_*, POWER_CELL_*, SHIP_POWER_*, STATION_POWER_*, EMERGENCY_REFINE_COST**

**Update lines 20-26 (Production chain costs):**

**Current:**
```rust
// [PHASE 9] PRODUCTION CHAIN COSTS
pub const SHIP_HULL_COST_PLATES: u32   = 3;
pub const HULL_PLATE_COST_CARBON: u32  = 5;
pub const AI_CORE_COST_CELLS: u32      = 55;
pub const POWER_COST_SHIP_FORGE: u32   = 3;
pub const POWER_COST_AI_FABRICATE: u32 = 5;

// Adding missing AI_CORE_COST from audit directive
pub const AI_CORE_COST: u32 = 55;
```

**REPLACE WITH:**
```rust
// [PHASE 9] PRODUCTION CHAIN COSTS
pub const SHIP_HULL_COST_PLATES: u32   = 3;
pub const HULL_PLATE_COST_IRON: u32    = 2;
pub const HULL_PLATE_COST_TUNGSTEN: u32 = 1;
pub const AI_CORE_COST_NICKEL: u32     = 1;
```

**Update sector definitions (lines 31-36):**

**Current:**
```rust
pub const SECTOR_1_POS: Vec2     = Vec2::new(320.0, 140.0);   // Magnetite — basic
pub const SECTOR_2_POS: Vec2     = Vec2::new(-220.0, 340.0);  // Iron — basic
pub const SECTOR_3_POS: Vec2     = Vec2::new(380.0, -280.0);  // Carbon — basic
pub const SECTOR_4_POS: Vec2     = Vec2::new(-520.0, -380.0); // Tungsten — Tungsten Laser gated
pub const SECTOR_5_POS: Vec2     = Vec2::new(680.0, 320.0);   // Titanite — Tungsten Laser gated
pub const SECTOR_6_POS: Vec2     = Vec2::new(-650.0, 520.0);  // Crystal Core — Composite Laser gated
```

**REPLACE WITH:**
```rust
pub const SECTOR_1_POS: Vec2     = Vec2::new(320.0, 140.0);   // Iron
pub const SECTOR_2_POS: Vec2     = Vec2::new(-220.0, 340.0);  // Tungsten
pub const SECTOR_3_POS: Vec2     = Vec2::new(380.0, -280.0);  // Nickel
```

**Delete SECTOR_4_POS, SECTOR_5_POS, SECTOR_6_POS**

**Update asteroid constants (lines 44-56):**

**Current:**
```rust
pub const ASTEROID_RADIUS_MAGNETITE: f32 = 26.0;
pub const ASTEROID_RADIUS_IRON: f32      = 20.0;
pub const ASTEROID_RADIUS_CARBON: f32    = 30.0;
pub const ASTEROID_RADIUS_TUNGSTEN: f32  = 22.0;
pub const ASTEROID_RADIUS_TITANITE: f32  = 28.0;
pub const ASTEROID_RADIUS_CRYSTAL: f32   = 18.0;

pub const COLOR_MAGNETITE: Color  = Color::srgb(0.55, 0.75, 1.0);   // Blue-white
pub const COLOR_IRON: Color       = Color::srgb(0.75, 0.38, 0.15);  // Rust orange
pub const COLOR_CARBON: Color     = Color::srgb(0.28, 0.28, 0.28);  // Dark grey
pub const COLOR_TUNGSTEN: Color   = Color::srgb(0.72, 0.68, 0.35);  // Yellow-grey
pub const COLOR_TITANITE: Color   = Color::srgb(0.72, 0.78, 0.82);  // Silver-blue
pub const COLOR_CRYSTAL: Color    = Color::srgb(0.55, 1.0, 0.88);   // Cyan-green
```

**REPLACE WITH:**
```rust
pub const ASTEROID_RADIUS_IRON: f32      = 20.0;
pub const ASTEROID_RADIUS_TUNGSTEN: f32  = 22.0;
pub const ASTEROID_RADIUS_NICKEL: f32    = 24.0;

pub const COLOR_IRON: Color       = Color::srgb(0.75, 0.38, 0.15);  // Rust orange
pub const COLOR_TUNGSTEN: Color   = Color::srgb(0.72, 0.68, 0.35);  // Yellow-grey
pub const COLOR_NICKEL: Color     = Color::srgb(0.75, 0.75, 0.75);  // Silver
```

**Action in Windsurf:**
1. Delete lines 106-127 (all POWER_* and SHIP_POWER_* constants)
2. Update production chain costs (lines 20-26)
3. Delete SECTOR_4_POS, SECTOR_5_POS, SECTOR_6_POS (keep only 3)
4. Update asteroid radius/color constants
5. Save file

**Verify:** `cargo check` should now show type errors about magnetite_reserves, carbon_reserves (we'll fix next)

---

## STEP 5: Update components.rs — Rename Station Reserves

**File:** `src/components.rs`  
**Lines to update:** 95-96 in Station struct

**Current:**
```rust
pub magnetite_reserves: f32,
pub carbon_reserves: f32,
```

**REPLACE WITH:**
```rust
pub iron_reserves: f32,
pub tungsten_reserves: f32,
pub nickel_reserves: f32,
```

**Action in Windsurf:**
1. Open `src/components.rs`
2. Find Station struct, lines with `magnetite_reserves` and `carbon_reserves`
3. Replace magnetite_reserves → iron_reserves
4. Replace carbon_reserves → tungsten_reserves
5. Add new line: `pub nickel_reserves: f32,` (after tungsten_reserves)
6. Save file

**Verify:** `cargo check` should show errors in setup.rs (expected, next step)

---

## STEP 6: Update setup.rs — Remove Power Initialization

**File:** `src/systems/setup.rs`  
**Lines to delete:** 158-159 (Ship power init)

**Current code (lines 155-165):**
```rust
pub fn spawn_player_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerShip,
        Ship {
            state: ShipState::Idle,
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: OreType::Empty,
            cargo_capacity: CARGO_CAPACITY,
            power: SHIP_POWER_MAX,           // ← DELETE THIS LINE
            power_cells: 0,                  // ← DELETE THIS LINE
            laser_tier: LaserTier::Basic,
        },
```

**AFTER DELETION:**
```rust
pub fn spawn_player_ship(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        PlayerShip,
        Ship {
            state: ShipState::Idle,
            speed: SHIP_SPEED,
            cargo: 0.0,
            cargo_type: OreDeposit::Iron,  // Also update OreType → OreDeposit
            cargo_capacity: CARGO_CAPACITY,
            laser_tier: LaserTier::Basic,
        },
```

**Also change line:** `cargo_type: OreType::Empty,` → `cargo_type: OreDeposit::Iron,`

**Find lines 240-250 (Station initialization):**

**Current:**
```rust
Station {
    repair_progress: 0.0,
    online: true,
    magnetite_reserves: 50.0,
    carbon_reserves: 25.0,
    hull_plate_reserves: 0,
    ship_hulls: 0,
    ai_cores: 0,
    power_cells: 5,
    power: STATION_POWER_MAX,
    maintenance_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
    last_power_warning_time: -100.0,
    log: VecDeque::new(),
    rotation: 0.0,
    rotation_speed: STATION_ROTATION_SPEED,
    dock_state: StationDockState::Rotating,
    resume_timer: 0.0,
}
```

**REPLACE WITH:**
```rust
Station {
    repair_progress: 0.0,
    online: true,
    iron_reserves: 50.0,
    tungsten_reserves: 25.0,
    nickel_reserves: 10.0,
    hull_plate_reserves: 0,
    ship_hulls: 0,
    ai_cores: 0,
    log: VecDeque::new(),
    rotation: 0.0,
    rotation_speed: STATION_ROTATION_SPEED,
    dock_state: StationDockState::Rotating,
    resume_timer: 0.0,
}
```

**Action in Windsurf:**
1. Find spawn_player_ship function
2. Delete `power: SHIP_POWER_MAX,` line
3. Delete `power_cells: 0,` line
4. Change `cargo_type: OreType::Empty,` → `cargo_type: OreDeposit::Iron,`
5. Find Station initialization block
6. Replace all power fields (power_cells, power, maintenance_timer, last_power_warning_time)
7. Replace magnetite_reserves → iron_reserves (50.0)
8. Replace carbon_reserves → tungsten_reserves (25.0)
9. Add nickel_reserves: 10.0,
10. Save file

**Verify:** `cargo check` — should now show errors about ore mesh generation (expected, next step)

---

## STEP 7: Update setup.rs — Update Ore Mesh Generation

**File:** `src/systems/setup.rs`  
**Lines to update:** Mesh generation functions and spawn_asteroid_field

**Find spawn_asteroid_field function (around line 380-450), look for this code:**

**Current:**
```rust
let ore_deposits = vec![
    OreDeposit::Magnetite,
    OreDeposit::Iron,
    OreDeposit::Carbon,
    OreDeposit::Tungsten,
    OreDeposit::Titanite,
    OreDeposit::CrystalCore,
];
```

**REPLACE WITH:**
```rust
let ore_deposits = vec![
    OreDeposit::Iron,
    OreDeposit::Tungsten,
    OreDeposit::Nickel,
];
```

**Find generate_ore_mesh function (around line 550):**

**Current code:**
```rust
pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64) -> Mesh {
    match ore {
        OreDeposit::Magnetite   => generate_magnetite_mesh(seed),
        OreDeposit::Iron        => generate_iron_mesh(seed),
        OreDeposit::Carbon      => generate_carbon_mesh(seed),
        OreDeposit::Tungsten    => generate_tungsten_mesh(seed),
        OreDeposit::Titanite    => generate_titanite_mesh(seed),
        OreDeposit::CrystalCore => generate_crystal_mesh(seed),
    }
}
```

**REPLACE WITH:**
```rust
pub fn generate_ore_mesh(ore: &OreDeposit, seed: u64) -> Mesh {
    match ore {
        OreDeposit::Iron     => generate_iron_mesh(seed),
        OreDeposit::Tungsten => generate_tungsten_mesh(seed),
        OreDeposit::Nickel   => generate_nickel_mesh(seed),
    }
}
```

**Now delete these entire functions (around lines 559-660):**
- `generate_magnetite_mesh()`
- `generate_carbon_mesh()`
- `generate_titanite_mesh()`
- `generate_crystal_mesh()`

**Add new function for Nickel (around line 640, after generate_tungsten_mesh):**

```rust
pub fn generate_nickel_mesh(seed: u64) -> Mesh {
    let mut rng = rand::thread_rng();
    rng.seed_from_u64(seed);
    
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    
    // Generate a simple spherical asteroid
    let rings = 6;
    let segments = 8;
    
    for i in 0..rings {
        let phi = std::f32::consts::PI * i as f32 / rings as f32;
        for j in 0..segments {
            let theta = 2.0 * std::f32::consts::PI * j as f32 / segments as f32;
            let x = phi.sin() * theta.cos();
            let y = phi.cos();
            let z = phi.sin() * theta.sin();
            vertices.push([x, y, z]);
        }
    }
    
    for i in 0..rings - 1 {
        for j in 0..segments {
            let a = i * segments + j;
            let b = i * segments + (j + 1) % segments;
            let c = (i + 1) * segments + j;
            let d = (i + 1) * segments + (j + 1) % segments;
            
            indices.push(a as u32);
            indices.push(c as u32);
            indices.push(b as u32);
            indices.push(b as u32);
            indices.push(c as u32);
            indices.push(d as u32);
        }
    }
    
    Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        bevy::render::render_resource::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_index_buffer(indices)
}
```

**Action in Windsurf:**
1. Find spawn_asteroid_field function
2. Update ore_deposits vec to only include Iron, Tungsten, Nickel
3. Find generate_ore_mesh function
4. Update match arms to only handle Iron, Tungsten, Nickel
5. Delete generate_magnetite_mesh, generate_carbon_mesh, generate_titanite_mesh, generate_crystal_mesh functions
6. Add generate_nickel_mesh function (paste above code)
7. Save file

**Verify:** `cargo check 2>&1 | grep -i "error\|unresolved"` — should now show errors in hud.rs and station_tabs.rs (expected)

---

## STEP 8: Update hud.rs — Remove Power Tab

**File:** `src/systems/hud.rs`  
**Action:** Delete Power tab rendering and associated logic

**Find the tab registration (around line 191), looks like:**
```rust
(ActiveStationTab::Power, "POWER"),
```

**Delete that entire line**

**Find the Power tab match arm (around line 280-289):**

**Current:**
```rust
ActiveStationTab::Power => {
    ui.heading("POWER");
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(format!("STATION POWER: {:.1}/{:.0}", station.power, STATION_POWER_MAX));
        ui.add(egui::ProgressBar::new(station.power / STATION_POWER_MAX).desired_width(120.0));
    });
    ui.horizontal(|ui| {
        ui.label(format!("SHIP POWER: {:.1}/{:.0}", ship.power, SHIP_POWER_MAX));
        ui.add(egui::ProgressBar::new(ship.power / SHIP_POWER_MAX).desired_width(120.0));
    });
}
```

**DELETE ENTIRE MATCH ARM**

**Find Cargo tab in hud.rs (around line 250-270), update resource display:**

**Current (approximately):**
```rust
ui.label("MAGNETITE:"); ui.label(egui::RichText::new(format!("{:.1}", station.magnetite_reserves)).color(egui::Color32::WHITE)); ui.end_row();
ui.label("CARBON:"); ui.label(egui::RichText::new(format!("{:.1}", station.carbon_reserves)).color(egui::Color32::WHITE)); ui.end_row();
ui.label("POWER CELLS:"); ui.label(egui::RichText::new(format!("{}", station.power_cells)).color(egui::Color32::GREEN)); ui.end_row();
```

**REPLACE WITH:**
```rust
ui.label("IRON:"); ui.label(egui::RichText::new(format!("{:.1}", station.iron_reserves)).color(egui::Color32::WHITE)); ui.end_row();
ui.label("TUNGSTEN:"); ui.label(egui::RichText::new(format!("{:.1}", station.tungsten_reserves)).color(egui::Color32::WHITE)); ui.end_row();
ui.label("NICKEL:"); ui.label(egui::RichText::new(format!("{:.1}", station.nickel_reserves)).color(egui::Color32::WHITE)); ui.end_row();
```

**Find repair button (around line 275):**

**Current:**
```rust
if ui.button(format!("REPAIR STATION [{} CELLS]", REPAIR_COST)).clicked() && station.power_cells >= REPAIR_COST {
    station.power_cells -= REPAIR_COST; station.repair_progress = 1.0; station.online = true;
}
```

**REPLACE WITH (for now, defer repair cost):**
```rust
if ui.button("REPAIR STATION").clicked() && !station.online {
    station.repair_progress = 1.0; station.online = true;
}
```

**Find processing queue rendering (around line 291-301), update ore types:**

**Current (approximately):**
```rust
render_queue_card(ui, &mut station, &mut queues.magnetite_refinery, ProcessingOperation::MagnetiteRefinery, REFINERY_RATIO as f32, POWER_COST_REFINERY as f32, REFINERY_MAGNETITE_TIME);
...
render_queue_card(ui, &mut station, &mut queues.carbon_refinery, ProcessingOperation::CarbonRefinery, HULL_PLATE_COST_CARBON as f32, POWER_COST_HULL_FORGE as f32, REFINERY_CARBON_TIME);
```

**Update to:**
```rust
render_queue_card(ui, &mut station, &mut queues.iron_refinery, ProcessingOperation::IronRefinery, HULL_PLATE_COST_IRON as f32, REFINERY_IRON_TIME);
...
render_queue_card(ui, &mut station, &mut queues.tungsten_refinery, ProcessingOperation::TungstenRefinery, HULL_PLATE_COST_TUNGSTEN as f32, REFINERY_TUNGSTEN_TIME);
```

(We'll deal with render_queue_card signature next in station_tabs.rs)

**Find "TOP UP SHIP" button (around line 318-319):**

**Current:**
```rust
if ui.button("TOP UP SHIP [3 CELLS]").clicked() && station.power_cells >= 3 && ship.power_cells < 5 {
    station.power_cells -= 3; ship.power_cells = (ship.power_cells + 3).min(5);
}
```

**DELETE ENTIRE BUTTON**

**Action in Windsurf:**
1. Find and delete the Power tab registration line
2. Find and delete the entire Power match arm
3. Update Cargo grid display: magnetite → iron, carbon → tungsten, add nickel, remove power_cells
4. Update repair button (remove power_cells cost requirement)
5. Update processing queue rendering (update ore types and function calls)
6. Delete TOP UP SHIP button entirely
7. Save file

**Verify:** `cargo check` — should now show errors in station_tabs.rs about render_queue_card signature

---

## STEP 9: Update station_tabs.rs — Update render_queue_card Calls

**File:** `src/systems/station_tabs.rs`  
**Action:** Update function signature and calls

**Find render_queue_card function signature (around line 6-14):**

**Current:**
```rust
pub fn render_queue_card(
    ui: &mut egui::Ui,
    station: &mut Station,
    queue: &mut ProcessingJob,
    operation: ProcessingOperation,
    input_cost: f32,
    power_cost: f32,
    time: f32,
) {
```

**REPLACE WITH:**
```rust
pub fn render_queue_card(
    ui: &mut egui::Ui,
    station: &mut Station,
    queue: &mut ProcessingJob,
    operation: ProcessingOperation,
    input_cost: f32,
    time: f32,
) {
```

(Remove power_cost parameter)

**Update function body to remove power_cost logic** — find all references to power_cost in the function and delete them

**Action in Windsurf:**
1. Open `src/systems/station_tabs.rs`
2. Find render_queue_card function
3. Remove `power_cost: f32,` parameter
4. Delete any lines inside the function that reference `power_cost`
5. Save file

**Verify:** `cargo check` — should now resolve render_queue_card errors in hud.rs

---

## STEP 10: Update components.rs — Simplify AutoDockSettings

**File:** `src/components.rs`  
**Find AutoDockSettings (around line 331-342):**

**Current:**
```rust
#[derive(Resource, Clone)]
pub struct AutoDockSettings {
    pub auto_unload: bool,
    pub auto_smelt_magnetite: bool,  // default: false
    pub auto_smelt_carbon: bool,     // default: false
}

impl Default for AutoDockSettings {
    fn default() -> Self {
        Self {
            auto_unload: true,
            auto_smelt_magnetite: false,
            auto_smelt_carbon: false,
        }
    }
}
```

**REPLACE WITH:**
```rust
#[derive(Resource, Clone)]
pub struct AutoDockSettings {
    pub auto_unload: bool,
    pub auto_smelt_iron: bool,
    pub auto_smelt_tungsten: bool,
    pub auto_smelt_nickel: bool,
}

impl Default for AutoDockSettings {
    fn default() -> Self {
        Self {
            auto_unload: true,
            auto_smelt_iron: false,
            auto_smelt_tungsten: false,
            auto_smelt_nickel: false,
        }
    }
}
```

**Action in Windsurf:**
1. Open `src/components.rs`
2. Find AutoDockSettings struct
3. Replace auto_smelt_magnetite → auto_smelt_iron
4. Replace auto_smelt_carbon → auto_smelt_tungsten
5. Add auto_smelt_nickel
6. Update Default impl to match
7. Save file

**Verify:** `cargo check`

---

## STEP 11: Delete economy.rs System Registration

**File:** `src/lib.rs`  
**Action:** Remove all economy system registrations

**Find lines 83-87 (Update schedule with economy systems):**

**Current:**
```rust
systems::economy::station_status_system,
systems::economy::station_maintenance_system,
systems::economy::ship_self_preservation_system,
systems::economy::processing_queue_system,
systems::economy::auto_dock_system,
```

**DELETE ALL FIVE LINES**

**Action in Windsurf:**
1. Open `src/lib.rs`
2. Find the Update system registration block
3. Delete the 5 lines that reference systems::economy::*
4. Save file

**Verify:** `cargo check` — should show error about economy module still being imported/defined

---

## STEP 12: Delete economy.rs Module

**File:** `src/systems/mod.rs`  
**Action:** Remove economy module declaration

**Current (around line 8):**
```rust
pub mod economy;
```

**DELETE THIS LINE**

**Action in Windsurf:**
1. Open `src/systems/mod.rs`
2. Find and delete `pub mod economy;`
3. Save file

**Then delete the actual file:**
1. In Windsurf file explorer, right-click `src/systems/economy.rs`
2. Delete file
3. Confirm deletion

**Verify:** `cargo check` — should now compile with no errors

---

## FINAL VERIFICATION CHECKLIST

After all 12 steps, run these checks:

```bash
cargo check 2>&1 | head -20
# Should show: "Finished `dev` profile [unoptimized + debuginfo] target(s) in X.XXs"
# If errors appear, scroll up to find first error and fix it

cargo build --release 2>&1 | tail -10
# Should compile without errors

.\build_android.ps1  # (if on Windows/have Android SDK)
# Should build APK successfully
```

**On device, verify:**
- [ ] Game starts (no black screen)
- [ ] Opening sequence runs (Adrift → Powered → Complete)
- [ ] Can fly to asteroids
- [ ] Mining collects ore (Iron, Tungsten, Nickel only)
- [ ] Ore displays in Cargo tab with correct names
- [ ] No Power tab in drawer
- [ ] No power bars anywhere
- [ ] Can dock at station
- [ ] Drawer opens/closes (no power mechanics interfering)
- [ ] Refinery UI shows (without power costs)
- [ ] Save/load still works (CONTINUE button functions)

---

## ROLLBACK PLAN (If Something Breaks)

If the code won't compile after a step, the fastest rollback:

```bash
git status
git diff src/  # See what changed
git checkout -- src/  # Revert all changes
```

Then go back to the last step that compiled and try again more carefully.

If you're stuck on a specific error:
1. Copy the error message
2. Show it here with the line of code
3. We'll diagnose and fix it

---

## You're Ready

You now have enough detail to execute Phase 1 in Windsurf.

**After Phase 1 is working and tested on device, let me know and we'll do Phase 2 (world/ directory refactoring).**

Good luck!
