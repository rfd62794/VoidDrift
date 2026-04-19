# Voidrift — Station Phase B Directive: Berth Navigation & Docking Sequence
**Status:** Approved — Ready for Execution  
**Type:** Gameplay + Visual  
**Date:** April 2026  
**Depends On:** Station Phase A VERIFIED ✅ | Station Architecture Design LOCKED ✅

---

## 1. Objective

Ships navigate to specific berths, not the station center. The station cooperates with the docking sequence — it slows as a ship approaches, pauses as the ship arrives, and resumes rotation carrying the docked ship on the berth arm.

This is the moment the station feels alive and aware.

---

## 2. Scope

**In scope:**
- Berth entities with dynamic world positions
- Player ship navigates to Berth 1 position
- Autonomous ships navigate to Berth 2 position
- Station slowdown on ship approach
- Station pause on ship arrival
- Station resume after docking with ship attached to berth
- Ship visual attaches to berth — rotates with station while docked
- Signal lines for docking sequence
- Berth occupancy visual — berth circle color reflects occupancy

**Explicitly out of scope:**
- Berth 3 NPC logic (future)
- Dynamic berth assignment beyond Berth 1 (player) and Berth 2 (drone)
- Drone Depot construction
- Additional berth unlocking
- Multi-device egui scaling (separate directive)

---

## 3. New Constants

Add to `constants.rs`:

```rust
pub const STATION_DOCK_SLOWDOWN_DISTANCE: f32 = 200.0; // Distance at which station begins slowing
pub const STATION_SLOWDOWN_RATE: f32 = STATION_ROTATION_SPEED * 3.0; // Deceleration rate per second
pub const STATION_RESUME_DELAY: f32 = 1.5;  // Seconds after dock before rotation resumes
pub const STATION_RESUME_RATE: f32 = STATION_ROTATION_SPEED * 2.0; // Acceleration rate on resume
pub const BERTH_1_ARM_INDEX: u8 = 0;  // Player berth — arm 0
pub const BERTH_2_ARM_INDEX: u8 = 1;  // Drone/NPC berth — arm 1
pub const BERTH_3_ARM_INDEX: u8 = 2;  // Open berth — arm 2
```

---

## 4. Station Component Extension

Extend `Station` in `components.rs`:

```rust
pub struct Station {
    // ... existing fields ...
    pub rotation: f32,
    pub rotation_speed: f32,        // NEW — current speed, varies during dock sequence
    pub dock_state: StationDockState, // NEW
    pub resume_timer: f32,          // NEW — countdown before rotation resumes
}

#[derive(PartialEq, Default)]
pub enum StationDockState {
    #[default]
    Rotating,      // Normal rotation at STATION_ROTATION_SPEED
    Slowing,       // Incoming ship detected — decelerating
    Paused,        // Ship arrived — fully stopped
    Resuming,      // Ship docked — accelerating back to full speed
}
```

Initialize:
- `rotation_speed = STATION_ROTATION_SPEED`
- `dock_state = StationDockState::Rotating`
- `resume_timer = 0.0`

---

## 5. Berth Entities

### 5.1 Berth Component

Add to `components.rs`:

```rust
#[derive(Component)]
pub struct Berth {
    pub arm_index: u8,
    pub occupied_by: Option<Entity>,
    pub berth_type: BerthType,
}

#[derive(PartialEq)]
pub enum BerthType {
    Player,   // Berth 1 — always player
    Drone,    // Berth 2 — autonomous ship
    Open,     // Berth 3 — NPC/visitor
}
```

### 5.2 Berth World Position Calculation

Berth position is dynamic — recalculated every tick from current station rotation:

```rust
pub fn berth_world_pos(
    station_pos: Vec2,
    station_rotation: f32,
    arm_index: u8,
) -> Vec2 {
    let arm_angle = station_rotation + (arm_index as f32 * std::f32::consts::TAU / 6.0);
    station_pos + Vec2::new(
        arm_angle.cos() * STATION_ARM_LENGTH,
        arm_angle.sin() * STATION_ARM_LENGTH,
    )
}
```

Add this as a public function in `systems/setup.rs` or a new `systems/station.rs` file.

### 5.3 Berth Spawn

Spawn 3 Berth entities in `setup_world`. These are logical entities — not visual. The visual berth circles are already children of the station visual container.

```rust
// Berth 1 — Player
commands.spawn((
    Berth {
        arm_index: BERTH_1_ARM_INDEX,
        occupied_by: None,
        berth_type: BerthType::Player,
    },
    Name::new("Berth1"),
));

// Berth 2 — Drone
commands.spawn((
    Berth {
        arm_index: BERTH_2_ARM_INDEX,
        occupied_by: None,
        berth_type: BerthType::Drone,
    },
    Name::new("Berth2"),
));

// Berth 3 — Open
commands.spawn((
    Berth {
        arm_index: BERTH_3_ARM_INDEX,
        occupied_by: None,
        berth_type: BerthType::Open,
    },
    Name::new("Berth3"),
));
```

---

## 6. Autopilot Changes

### 6.1 Player Ship

When player taps station on map, assign `AutopilotTarget` to Berth 1 entity (not station entity).

In `handle_input` — when station marker tapped:
```rust
// Find Berth 1 entity
// Set AutopilotTarget { destination: berth_world_pos(...), target_entity: Some(berth1_entity) }
```

The berth destination must be recalculated every tick since the station is rotating. In `autopilot_system`, when `target_entity` has a `Berth` component:

```rust
// Recalculate destination each tick
if let Ok(berth) = berth_query.get(target.target_entity.unwrap()) {
    if let Ok((station, station_transform)) = station_query.get_single() {
        target.destination = berth_world_pos(
            station_transform.translation.truncate(),
            station.rotation,
            berth.arm_index,
        );
    }
}
```

### 6.2 Autonomous Ships

Same pattern — autonomous ships target Berth 2 entity. `autonomous_ship_system` recalculates berth position each tick on return.

### 6.3 Arrival Threshold

Berth arrival uses `ARRIVAL_THRESHOLD = 8.0` — same as current station arrival. No change.

---

## 7. Docking Sequence

### 7.1 Station Slowdown

In `station_rotation_system` — check distance from any approaching ship:

```rust
fn station_rotation_system(
    time: Res<Time>,
    mut station_query: Query<(&mut Station, &Transform)>,
    ship_query: Query<(&Ship, &Transform), Without<Station>>,
    autonomous_query: Query<(&AutonomousShip, &Transform), Without<Station>>,
) {
    for (mut station, station_transform) in &mut station_query {
        let station_pos = station_transform.translation.truncate();
        
        // Check if any ship is approaching
        let ship_approaching = ship_query.iter().any(|(ship, ship_transform)| {
            ship.state == ShipState::Navigating &&
            ship_transform.translation.truncate().distance(station_pos) < STATION_DOCK_SLOWDOWN_DISTANCE
        });
        
        let drone_approaching = autonomous_query.iter().any(|(drone, drone_transform)| {
            drone.state == AutonomousShipState::Returning &&
            drone_transform.translation.truncate().distance(station_pos) < STATION_DOCK_SLOWDOWN_DISTANCE
        });
        
        let incoming = ship_approaching || drone_approaching;
        
        match station.dock_state {
            StationDockState::Rotating => {
                if incoming {
                    station.dock_state = StationDockState::Slowing;
                }
                station.rotation += station.rotation_speed * time.delta_secs();
            }
            StationDockState::Slowing => {
                if !incoming {
                    station.dock_state = StationDockState::Rotating;
                    station.rotation_speed = STATION_ROTATION_SPEED;
                } else {
                    station.rotation_speed = (station.rotation_speed - STATION_SLOWDOWN_RATE * time.delta_secs())
                        .max(0.0);
                    if station.rotation_speed == 0.0 {
                        station.dock_state = StationDockState::Paused;
                    }
                }
                station.rotation += station.rotation_speed * time.delta_secs();
            }
            StationDockState::Paused => {
                // No rotation — waiting for dock
                // Transition to Resuming is triggered externally when ship docks
            }
            StationDockState::Resuming => {
                station.rotation_speed = (station.rotation_speed + STATION_RESUME_RATE * time.delta_secs())
                    .min(STATION_ROTATION_SPEED);
                station.rotation += station.rotation_speed * time.delta_secs();
                if station.rotation_speed >= STATION_ROTATION_SPEED {
                    station.dock_state = StationDockState::Rotating;
                }
            }
        }
    }
}
```

### 7.2 Dock Trigger

When ship arrives at berth (autopilot arrival condition met):
- Set `ShipState::Docked`
- Set `berth.occupied_by = Some(ship_entity)`
- Set `station.dock_state = StationDockState::Resuming`
- Start resume timer: `station.resume_timer = STATION_RESUME_DELAY`
- Fire Signal S-030: `> DOCKING COMPLETE. ROTATION RESUMING.`

### 7.3 Ship Attaches to Berth

When docked, the ship visual should rotate with the station. Two approaches:

**Option A — Reparent ship to station visual container.** On dock, make the ship entity a child of `StationVisualsContainer`. On undock, reparent back to world. This is clean but reparenting in Bevy requires `Commands` and takes one frame.

**Option B — Match rotation each tick.** While `ShipState::Docked`, set ship Transform to match the berth world position each tick. Simple, no reparenting.

**Use Option B for Phase B.** Less elegant but reliable. Reparenting approach can be revisited in Phase C.

```rust
// In a new docked_ship_system or inside autopilot_system
// While ship.state == Docked:
//   ship_transform.translation = berth_world_pos(...).extend(Z_SHIP);
```

---

## 8. Berth Occupancy Visual

Update berth circle colors based on occupancy. This requires access to the berth visual entities (children of StationVisualsContainer).

Add marker components to berth visual circles at spawn:

```rust
#[derive(Component)]
pub struct BerthVisual(pub u8); // arm_index
```

In `station_visual_system` — update berth circle colors:

```rust
for (berth_visual, material_handle) in &berth_visual_query {
    if let Some(material) = materials.get_mut(&material_handle.0) {
        material.color = match berth_occupancy[berth_visual.0 as usize] {
            BerthOccupancy::Empty => Color::srgb(0.4, 0.4, 0.4),     // grey
            BerthOccupancy::Player => Color::srgb(0.0, 0.67, 1.0),   // cyan
            BerthOccupancy::Drone => Color::srgb(1.0, 0.53, 0.0),    // orange
            BerthOccupancy::Open => Color::srgb(0.4, 0.4, 0.4),      // grey — waiting
        };
    }
}
```

---

## 9. Signal Lines

Add to `narrative.rs`:

| ID | Trigger | Line |
|----|---------|------|
| S-028 | Ship within DOCK_SLOWDOWN_DISTANCE of station | `> INCOMING VESSEL DETECTED. DOCKING SEQUENCE INITIATED.` |
| S-029 | Station fully stopped (dock_state == Paused) | `> ROTATION SUSPENDED. BERTH ALIGNED.` |
| S-030 | Ship docks, rotation resuming | `> DOCKING COMPLETE. ROTATION RESUMING.` |
| S-031 | Ship undocks | `> VESSEL DEPARTED. BERTH CLEAR.` |

S-028 and S-029 may refire per docking event — not one-time triggers. Use the same cooldown pattern as S-019/S-020.

---

## 10. File Scope

| File | Change |
|------|--------|
| `src/constants.rs` | Add berth and docking sequence constants |
| `src/components.rs` | Add Berth, BerthType, StationDockState, BerthVisual components |
| `src/systems/setup.rs` | Spawn 3 Berth entities, add BerthVisual marker to berth circles |
| `src/systems/autopilot.rs` | Recalculate berth destination each tick, trigger dock sequence |
| `src/systems/autonomous.rs` | Target Berth 2 on return, recalculate each tick |
| `src/systems/visuals.rs` | Extend station_rotation_system with dock state machine |
| `src/systems/ui.rs` | Update station tap target to use Berth 1 entity |
| `src/systems/map.rs` | Update handle_input to assign Berth 1 as target |
| `src/systems/narrative.rs` | Add S-028 through S-031 |
| `Cargo.toml` | READ-ONLY |

---

## 11. Pre-Implementation Research

Before writing any code:

1. How is the station currently referenced in `handle_input` when the player taps the station marker — by entity, by component query, or by stored position? This determines how to switch from station target to berth target.
2. Does the current `autopilot_system` update `AutopilotTarget.destination` in place, or does it read it as fixed? This determines whether dynamic destination recalculation requires a system change or just works.
3. What is the current approach in `autonomous_ship_system` for the return-to-station destination? Is it a hardcoded `STATION_POS` or an entity reference?

Report findings before implementation.

---

## 12. Completion Criteria

- [ ] Player ship navigates to Berth 1 position (end of arm 0)
- [ ] Autonomous ship navigates to Berth 2 position (end of arm 1)
- [ ] Station slows as ship approaches within 200 units
- [ ] Station pauses fully when ship arrives at berth
- [ ] Signal S-028 fires on approach detection
- [ ] Signal S-029 fires when station fully stopped
- [ ] Station resumes rotation after dock, carrying ship on berth
- [ ] Ship visual tracks berth position while docked — rotates with station
- [ ] Signal S-030 fires on dock completion
- [ ] Berth circle colors reflect occupancy
- [ ] Gate screenshot: ship visibly docked on rotating berth arm

---

*Voidrift Station Phase B Directive | April 2026 | RFD IT Services Ltd.*  
*The station slows. The station stops. The station waits for you.*
