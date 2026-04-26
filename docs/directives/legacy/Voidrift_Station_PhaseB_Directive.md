# Voidrift — Directive: World Expansion, Asteroid Identity & Pinch Zoom
**Status:** Approved — Ready for Execution  
**Type:** World Design + Visual + Input  
**Date:** April 2026  
**Depends On:** Processing Queue Directive COMPLETE ✅

---

## 1. Objective

Four things in one directive:

1. **World expansion** — all six asteroid fields placed, world scale increased, opening sequence adjusted
2. **Asteroid shape families** — distinct geometry per ore type
3. **Ore labels** — world-space text label below each asteroid, child of asteroid entity
4. **Pinch/pull zoom** — multi-touch zoom on space view

All fields visible from the start. Laser tier gates extraction, not visibility.

---

## 2. World Layout — All Fields

### 2.1 New Sector Positions

Replace all existing sector position constants in `constants.rs`:

```rust
pub const STATION_POS: Vec2      = Vec2::new(0.0, 0.0);
pub const SECTOR_1_POS: Vec2     = Vec2::new(320.0, 140.0);   // Magnetite — basic, further than before
pub const SECTOR_2_POS: Vec2     = Vec2::new(-220.0, 340.0);  // Iron — basic
pub const SECTOR_3_POS: Vec2     = Vec2::new(380.0, -280.0);  // Carbon — basic
pub const SECTOR_4_POS: Vec2     = Vec2::new(-520.0, -380.0); // Tungsten — Tungsten Laser gated
pub const SECTOR_5_POS: Vec2     = Vec2::new(680.0, 320.0);   // Titanite — Tungsten Laser gated
pub const SECTOR_6_POS: Vec2     = Vec2::new(-650.0, 520.0);  // Crystal Core — Composite Laser gated
```

### 2.2 Field Definitions

| Sector | Position | Ore | Laser Gate | Visible | Notes |
|--------|----------|-----|-----------|---------|-------|
| Sector 1 | `(320, 140)` | Magnetite | Basic | Always | Starting field — further than before |
| Sector 2 | `(-220, 340)` | Iron | Basic | Always | Basic field |
| Sector 3 | `(380, -280)` | Carbon | Basic | Always | Basic field |
| Sector 4 | `(-520, -380)` | Tungsten | Tungsten Laser | Always | Visible but inaccessible early |
| Sector 5 | `(680, 320)` | Titanite | Tungsten Laser | Always | Distant — feels like frontier |
| Sector 6 | `(-650, 520)` | Crystal Core | Composite Laser | Always | Very distant — endgame |

All six fields spawn at game start. All are visible in space view and on the map. Laser gating is enforced by the mining system — if the player's laser tier is insufficient, mining does not begin and Signal reports:
```
> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.
```

### 2.3 Opening Sequence Adjustment

The opening sequence spawns the player ship at `(-1000, -800)` and autopilots to the station at `(0, 0)`. With the world expanded, Sector 1 at `(320, 140)` is still within reasonable range for first mining run. No opening sequence changes required beyond verifying the approach distance still feels cinematic at the new scale.

---

## 3. Asteroid Shape Families

Replace the current single `generate_asteroid_mesh` function with ore-type-specific generation functions.

### 3.1 Shape Family Specifications

Each function takes a `seed: u64` for per-asteroid variation within the family.

**Magnetite — Crystalline**
```rust
fn generate_magnetite_mesh(seed: u64) -> Mesh {
    // 10-12 vertices
    // High radius variation: base ± 40%
    // Some vertices pushed out sharply (spikes)
    // Elongated overall shape — not circular
    // Base radius: 26.0
}
```

**Iron — Jagged**
```rust
fn generate_iron_mesh(seed: u64) -> Mesh {
    // 8-10 vertices
    // Medium radius variation: base ± 25%
    // No spikes — compact and rough
    // Slightly irregular but not extreme
    // Base radius: 20.0
}
```

**Carbon — Smooth Round**
```rust
fn generate_carbon_mesh(seed: u64) -> Mesh {
    // 12-14 vertices
    // Low radius variation: base ± 10%
    // Nearly circular — gentle lumps only
    // Largest of the basic ores
    // Base radius: 30.0
}
```

**Tungsten — Dense Blocky**
```rust
fn generate_tungsten_mesh(seed: u64) -> Mesh {
    // 6-8 vertices
    // Very low variation: base ± 8%
    // Nearly rectangular/hexagonal
    // Heavy and regular — industrial looking
    // Base radius: 22.0
}
```

**Titanite — Layered**
```rust
fn generate_titanite_mesh(seed: u64) -> Mesh {
    // 10 vertices arranged in stepped profile
    // Flat top and bottom
    // Wide in middle, narrower at poles
    // Striated appearance
    // Base radius: 28.0
}
```

**Crystal Core — Faceted**
```rust
fn generate_crystal_mesh(seed: u64) -> Mesh {
    // 6 vertices — hexagonal base
    // Very low variation: base ± 5%
    // Near-perfect geometric form
    // Gem-like, high symmetry
    // Base radius: 18.0 (rare — smaller but valuable)
}
```

### 3.2 Asteroid Size Constants

Add to `constants.rs`:

```rust
pub const ASTEROID_RADIUS_MAGNETITE: f32 = 26.0;
pub const ASTEROID_RADIUS_IRON: f32      = 20.0;
pub const ASTEROID_RADIUS_CARBON: f32    = 30.0;
pub const ASTEROID_RADIUS_TUNGSTEN: f32  = 22.0;
pub const ASTEROID_RADIUS_TITANITE: f32  = 28.0;
pub const ASTEROID_RADIUS_CRYSTAL: f32   = 18.0;
```

### 3.3 Ore Colors

Update or confirm in `constants.rs`:

```rust
pub const COLOR_MAGNETITE: Color  = Color::srgb(0.55, 0.75, 1.0);   // Blue-white
pub const COLOR_IRON: Color       = Color::srgb(0.75, 0.38, 0.15);  // Rust orange
pub const COLOR_CARBON: Color     = Color::srgb(0.28, 0.28, 0.28);  // Dark grey
pub const COLOR_TUNGSTEN: Color   = Color::srgb(0.72, 0.68, 0.35);  // Yellow-grey
pub const COLOR_TITANITE: Color   = Color::srgb(0.72, 0.78, 0.82);  // Silver-blue
pub const COLOR_CRYSTAL: Color    = Color::srgb(0.55, 1.0, 0.88);   // Cyan-green

// Depleted state — same for all
pub const COLOR_DEPLETED: Color   = Color::srgb(0.18, 0.18, 0.18);  // Very dark grey
```

---

## 4. Ore Type Component

Add to `components.rs`:

```rust
#[derive(Component, Clone, PartialEq)]
pub enum OreDeposit {
    Magnetite,
    Iron,
    Carbon,
    Tungsten,
    Titanite,
    CrystalCore,
}

pub const fn ore_name(ore: &OreDeposit) -> &'static str {
    match ore {
        OreDeposit::Magnetite  => "MAGNETITE",
        OreDeposit::Iron       => "IRON",
        OreDeposit::Carbon     => "CARBON",
        OreDeposit::Tungsten   => "TUNGSTEN",
        OreDeposit::Titanite   => "TITANITE",
        OreDeposit::CrystalCore => "CRYSTAL",
    }
}

pub const fn ore_laser_required(ore: &OreDeposit) -> LaserTier {
    match ore {
        OreDeposit::Magnetite  => LaserTier::Basic,
        OreDeposit::Iron       => LaserTier::Basic,
        OreDeposit::Carbon     => LaserTier::Basic,
        OreDeposit::Tungsten   => LaserTier::Tungsten,
        OreDeposit::Titanite   => LaserTier::Tungsten,
        OreDeposit::CrystalCore => LaserTier::Composite,
    }
}

#[derive(PartialEq)]
pub enum LaserTier {
    Basic,
    Tungsten,
    Composite,
}
```

Add `LaserTier::Basic` to the `Ship` component as the player's current laser tier. Default: `Basic`. Upgrade path deferred to future directive.

---

## 5. Ore Labels

### 5.1 Label Entity

Each asteroid field spawns a `Text2d` label as a direct child of the asteroid root entity.

**Position:** Below the asteroid center by `asteroid_radius + 12.0` units.

```rust
// In setup_world, when spawning asteroid field:
commands.entity(asteroid_entity).with_children(|parent| {
    // ... existing map icon and map label children ...
    
    // Ore type label — world space, always visible in space view
    parent.spawn((
        Text2d::new(ore_name(&ore_type)),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 10.0,
            ..default()
        },
        TextColor(Color::srgba(0.8, 0.8, 0.8, 0.6)),  // dim white, partial opacity
        Transform::from_xyz(0.0, -(asteroid_radius + 12.0), Z_HUD),
        // NOT a MapElement — visible in space view, not on map
    ));
});
```

### 5.2 Label Visibility

The ore label is NOT a `MapElement` — it does not toggle with the map. It is always visible in space view.

In map view (camera zoomed out), the label will be tiny but readable as an identifier — this is acceptable and desirable.

### 5.3 Laser-Gated Fields Visual

For Tungsten, Titanite, and Crystal Core fields (laser-gated), apply additional visual treatment:

- Asteroid color slightly desaturated — 70% of normal color saturation
- Label shows laser requirement below ore name:

```
TUNGSTEN
[LASER UPGRADE]
```

Two `Text2d` entities as children — ore name above, requirement below. Requirement label hidden once player has the required laser tier.

This is deferred complexity — implement ore name label first, add laser requirement label in a follow-up if needed.

---

## 6. Pinch/Pull Zoom

### 6.1 Input System

Add to `map.rs` or new section in `map.rs`:

```rust
pub fn pinch_zoom_system(
    touches: Res<Touches>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut last_pinch_distance: Local<Option<f32>>,
) {
    let touch_positions: Vec<Vec2> = touches.iter()
        .map(|t| t.position())
        .collect();
    
    if touch_positions.len() == 2 {
        let current_distance = touch_positions[0].distance(touch_positions[1]);
        
        if let Some(last_distance) = *last_pinch_distance {
            let delta = current_distance - last_distance;
            
            if let Ok(mut projection) = camera_query.get_single_mut() {
                projection.scale = (projection.scale - delta * ZOOM_SPEED)
                    .clamp(ZOOM_MIN, ZOOM_MAX);
            }
        }
        
        *last_pinch_distance = Some(current_distance);
    } else {
        *last_pinch_distance = None;
    }
}
```

### 6.2 Zoom Constants

Add to `constants.rs`:

```rust
pub const ZOOM_MIN: f32   = 0.3;   // most zoomed in
pub const ZOOM_MAX: f32   = 8.0;   // most zoomed out — sees full solar system
pub const ZOOM_SPEED: f32 = 0.005; // scale change per pixel of pinch delta
```

### 6.3 Zoom Interaction with Map View

The current map toggle switches to a fixed `MAP_STRATEGIC_SCALE`. With pinch zoom, the player can now zoom out naturally rather than toggling.

Keep the MAP button behavior unchanged — it's a shortcut to the strategic overview. Pinch zoom works in both Space View and Map View as a continuous control.

### 6.4 System Registration

Add to `lib.rs`:
```rust
.add_systems(Update, systems::map::pinch_zoom_system)
```

No ordering constraint needed — reads touch input, writes only to camera projection.

---

## 7. Mining System Update — Laser Gate

Update `mining_system` to check `OreDeposit` laser requirement against player's `LaserTier`:

```rust
// In mining_system, before beginning extraction:
if ore_laser_required(&field.ore_deposit) > ship.laser_tier {
    // Cannot mine — wrong laser
    // Fire signal once (not every tick):
    signal_log.push("> INSUFFICIENT LASER RATING. UPGRADE REQUIRED.");
    ship.state = ShipState::Idle;
    return;
}
```

Player's laser tier is `LaserTier::Basic` for now — all basic fields are accessible, advanced fields are not.

---

## 8. Setup Changes

### 8.1 Spawn All Six Fields

Update `setup_world` to spawn all six asteroid fields:

```rust
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_1_POS, OreDeposit::Magnetite, seed_from_pos(SECTOR_1_POS));
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_2_POS, OreDeposit::Iron, seed_from_pos(SECTOR_2_POS));
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_3_POS, OreDeposit::Carbon, seed_from_pos(SECTOR_3_POS));
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_4_POS, OreDeposit::Tungsten, seed_from_pos(SECTOR_4_POS));
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_5_POS, OreDeposit::Titanite, seed_from_pos(SECTOR_5_POS));
spawn_asteroid_field(&mut commands, &mut meshes, &mut materials, &asset_server,
    SECTOR_6_POS, OreDeposit::CrystalCore, seed_from_pos(SECTOR_6_POS));
```

### 8.2 Spawn Helper Function

Refactor asteroid spawning into a reusable function:

```rust
fn spawn_asteroid_field(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    asset_server: &AssetServer,
    position: Vec2,
    ore: OreDeposit,
    seed: u64,
) {
    let mesh = generate_ore_mesh(&ore, seed);
    let color = ore_color(&ore);
    let radius = ore_radius(&ore);
    let name = ore_name(&ore);
    
    // Spawn asteroid with appropriate mesh, color, label
}
```

### 8.3 Map Markers for All Fields

All six fields get map markers. Gated fields (4, 5, 6) use desaturated color on the map marker — same color family but dimmer. Label shows ore name.

---

## 9. File Scope

| File | Change |
|------|--------|
| `src/constants.rs` | New sector positions, asteroid radii, ore colors, zoom constants, processing times |
| `src/components.rs` | Add OreDeposit enum, LaserTier enum, ore_name/ore_laser_required helpers |
| `src/systems/setup.rs` | Spawn all 6 fields, shape family generators, spawn helper function, ore labels |
| `src/systems/map.rs` | Add pinch_zoom_system, update map markers for all 6 fields |
| `src/systems/mining.rs` | Add laser tier gate check |
| `src/systems/narrative.rs` | Sector 7 discovery trigger now uses Sector 3 (Carbon) position — update if needed |
| `src/lib.rs` | Register pinch_zoom_system |
| `Cargo.toml` | READ-ONLY |

---

## 10. Implementation Sequence

1. Add constants and `OreDeposit`/`LaserTier` components — verify compile
2. Create shape family generator functions — verify in desktop build visually
3. Spawn all 6 asteroid fields with correct shapes and colors — deploy, verify on device
4. Add ore labels as children — deploy, verify readable on device
5. Add laser gate to mining system — deploy, verify inaccessible fields cannot be mined
6. Add pinch zoom — deploy, verify smooth zoom gesture on device
7. Update map markers for all 6 fields — deploy, verify map shows all sectors

---

## 11. Completion Criteria

- [ ] All 6 asteroid fields visible in space view at game start
- [ ] Each ore type has visually distinct shape
- [ ] Each ore type has correct color
- [ ] Ore name label visible below each asteroid
- [ ] Laser-gated fields (4, 5, 6) cannot be mined — Signal reports reason
- [ ] Pinch to zoom in/out works smoothly on device
- [ ] Zoom clamped to ZOOM_MIN and ZOOM_MAX
- [ ] Map markers for all 6 fields on strategic view
- [ ] Opening sequence still completes correctly at new world scale
- [ ] No B0001 crashes — all new queries follow Universal Disjointness pattern

**Gate screenshot:** Space view showing at least 3 distinct asteroid field shapes simultaneously, with ore labels visible, station rotating in background.

---

## 12. Future Notes (Not In Scope)

- Individual smaller asteroids within field boundary circle — deferred
- Particle/shimmer effects per ore type — deferred  
- Laser upgrade system — deferred (LaserTier::Basic hardcoded for now)
- Sector discovery animation — deferred

---

*Voidrift World Expansion Directive | April 2026 | RFD IT Services Ltd.*  
*The world exists before the player can reach it. That's what makes it worth exploring.*
