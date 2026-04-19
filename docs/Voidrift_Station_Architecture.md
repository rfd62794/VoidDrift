# Voidrift — Station Architecture Design Document
**Status:** Locked — Design Session 2026-04-19  
**Type:** Visual & Gameplay System Design  
**Author:** RFD IT Services Ltd.

---

## 1. Two Station Types

Voidrift has two distinct station types. They share docking logic but have different visual identities, purposes, and gameplay roles.

| Station Type | Visual | Rotates | Departments | Berths For |
|-------------|--------|---------|-------------|-----------|
| Main Station | Hub + 6 spokes | Yes — slow | All player departments | Player + NPCs/Traders/Wanderers |
| Drone Depot | Square (existing design) | No — static | None | Autonomous ships only |

**The Main Station** is the social and operational hub. The player interacts with it for all crafting, refining, and NPC encounters. It has physical berths for ships to dock at specific positions.

**The Drone Depot** is industrial infrastructure. Autonomous ships dock here exclusively. No player departments, no NPC interaction. The existing square station visual is reused — it is functional and anonymous by design.

The player builds the Drone Depot when autonomous ship capacity needs to grow without sacrificing NPC berths at the Main Station. That is the natural forcing function that makes the Drone Depot feel necessary rather than arbitrary.

---

## 2. Main Station — Visual Structure

### 2.1 Layout

Six arms radiate from the hub at 60-degree intervals. Three are active from the start. Three are stubs — visible but dim, implying future expansion.

```
      [Berth 3 — Open]
            |
[stub] —— [HUB] —— [Berth 2 — Drone/NPC]
            |
      [Berth 1 — Player]
            |
         [stub]
            |
         [stub]
```

**Active arms:** Full brightness, thick line, berth circle at end.
**Stub arms:** Dim, half length, no berth circle — expansion implied.

The entire structure rotates slowly as a unit. Arm positions are calculated dynamically from the hub center + current rotation angle.

### 2.2 Constants

| Constant | Value | Notes |
|----------|-------|-------|
| `STATION_HUB_RADIUS` | 40.0 | Central circle radius |
| `STATION_ARM_LENGTH` | 120.0 | Hub center to berth center |
| `STATION_ARM_THICKNESS` | 6.0 | Spoke line width |
| `STATION_BERTH_RADIUS` | 22.0 | Docking circle radius |
| `STATION_STUB_LENGTH` | 60.0 | Inactive arm length |
| `STATION_STUB_ALPHA` | 0.3 | Inactive arm opacity |
| `STATION_BERTHS_INITIAL` | 3 | Player + Drone/NPC + Open |
| `STATION_ROTATION_SPEED` | `TAU / 90.0` | One full rotation per 90 seconds |

### 2.3 Colors

| Element | State | Color | Notes |
|---------|-------|-------|-------|
| Hub | Online | `#FFD700` | Powered — warm yellow |
| Hub | Offline/Derelict | `#554400` | Dark amber — no power |
| Arms | Active | `#AAAAAA` | Structural grey |
| Arms | Stub | `#333333` | Dim — implied expansion |
| Berth | Empty | `#666666` | Open, awaiting ship |
| Berth | Player | `#00AAFF` | Cyan — player ship home |
| Berth | Drone | `#FF8800` | Orange — autonomous ship |
| Berth | NPC/Trader | `#FFFFFF` | White — visitor |

### 2.4 Rotation

The station rotates as a rigid body. All arms and berths move together. Hub stays at `STATION_POS = (0, 0)`.

```rust
// Every tick
station.rotation += STATION_ROTATION_SPEED * delta;

// Berth world position — calculated dynamically
fn berth_world_pos(station_pos: Vec2, rotation: f32, arm_index: u8) -> Vec2 {
    let arm_angle = rotation + (arm_index as f32 * TAU / 6.0);
    station_pos + Vec2::new(
        arm_angle.cos() * STATION_ARM_LENGTH,
        arm_angle.sin() * STATION_ARM_LENGTH,
    )
}
```

### 2.5 Berth Assignment

| Berth | Index | Reserved For |
|-------|-------|-------------|
| 1 | 0 | Player ship — always |
| 2 | 1 | First autonomous ship OR NPC |
| 3 | 2 | Open — NPC/Trader/Wanderer priority |
| 4-6 | 3-5 | Unlockable — future expansion |

Berth 3 is held open by design. It is civic infrastructure — the station's invitation to visitors. When a trader or wanderer arrives, they dock at the first available open berth. If no berth is open, they pass by.

The player cannot manually assign NPCs to berths. Visitors self-assign to the nearest open berth on approach.

---

## 3. Drone Depot — Visual Structure

### 3.1 Design

The Drone Depot reuses the existing square station visual. It is static — no rotation. It is anonymous — no hub-and-spoke identity. It looks industrial because it is industrial.

The Drone Depot is spawned as a `StationType::DroneDepot` entity. It uses the same docking arrival logic as the Main Station but different berth rules.

### 3.2 Berths

Drone Depot berths are invisible — implied by the square structure. Autonomous ships dock at the Depot center with a fixed offset per ship index. No visual berth circles. The ship just arrives and stops.

| Drone Depot Berth | Offset | Notes |
|-------------------|--------|-------|
| 1 | `(-30, 0)` | First drone |
| 2 | `(30, 0)` | Second drone |
| 3 | `(0, 30)` | Third drone |
| N | calculated | Expand as needed |

### 3.3 Unlock Condition

The Drone Depot is a buildable structure. It does not exist at game start. The player constructs it when Main Station berths are needed for NPC traffic.

Build cost: TBD design session — likely involves Ship Hulls and Power Cells at significant quantity. It is a major infrastructure investment.

---

## 4. Station Component Refactor

### 4.1 StationType Enum

```rust
#[derive(Component, PartialEq)]
pub enum StationType {
    MainStation,
    DroneDepot,
}
```

### 4.2 Berth Component

```rust
#[derive(Component)]
pub struct Berth {
    pub station_entity: Entity,
    pub arm_index: u8,
    pub occupied_by: Option<Entity>,
    pub berth_type: BerthType,
}

#[derive(PartialEq)]
pub enum BerthType {
    Player,
    Drone,
    Open,
}
```

### 4.3 Station Component Extension

Add to existing `Station` component:

```rust
// Add to Station struct
pub station_type: StationType,
pub rotation: f32,           // current rotation angle (MainStation only)
pub berths: Vec<Entity>,     // berth entities
```

---

## 5. Autopilot Integration

Ships navigate to a **berth position**, not the station center. The berth position is dynamic — it changes as the station rotates.

The autopilot system must recalculate the target position each tick when the destination is a berth:

```rust
// In autopilot_system — when target is a Berth
if let Ok(berth) = berth_query.get(target_entity) {
    if let Ok(station) = station_query.get(berth.station_entity) {
        let target_pos = berth_world_pos(
            station_transform.translation.truncate(),
            station.rotation,
            berth.arm_index,
        );
        // Navigate toward dynamic target_pos
    }
}
```

Arrival threshold for berths: `ARRIVAL_THRESHOLD = 8.0` (existing) — same as station docking.

---

## 6. Map View Updates

The Main Station marker on the map must reflect its visual identity:

- Hub circle at station position
- 6 spoke lines (3 bright, 3 dim) radiating outward — scaled down for map view
- Berth circles at spoke ends — colored by occupancy

The Drone Depot marker on the map:
- Small square — same as current station marker
- Distinct color from Main Station — perhaps `#884400` industrial orange-brown

---

## 7. Signal Integration

New Signal lines for station events:

| ID | Trigger | Line |
|----|---------|------|
| S-030 | Berth 3 occupied by visitor | `> VESSEL DOCKED. BERTH 3 OCCUPIED.` |
| S-031 | Visitor departs | `> VESSEL DEPARTED. BERTH 3 OPEN.` |
| S-032 | Drone Depot built | `> DRONE DEPOT ONLINE. AUTONOMOUS CAPACITY EXPANDED.` |
| S-033 | All berths occupied | `> ALL BERTHS OCCUPIED. VISITORS UNABLE TO DOCK.` |
| S-034 | New berth unlocked | `> BERTH {N} ONLINE. DOCKING CAPACITY EXPANDED.` |

---

## 8. Implementation Phasing

This is a significant visual and architectural change. It should be implemented in phases:

**Phase A — Station Rotation (visual only)**
- Add `rotation` field to Station
- Render hub as circle, 6 arms, berth circles
- Rotation updates each tick
- No berth gameplay logic yet

**Phase B — Berth Navigation**
- Berths as entities with dynamic world positions
- Autopilot navigates to berth position
- Player ship always targets Berth 1
- Autonomous ships target Berth 2

**Phase C — NPC Berth (future)**
- Berth 3 open for visitors
- NPC ships self-assign on approach
- Signal triggers S-030, S-031, S-033

**Phase D — Drone Depot (future)**
- Drone Depot as buildable structure
- Autonomous ships reassigned to Depot
- Main Station berths freed for NPCs

---

## 9. Locked Decisions

| Decision | Value |
|----------|-------|
| Main Station berths initial | 3 |
| Drone Depot visual | Existing square — reused |
| Rotation speed | TAU / 90.0 |
| Arm length | 120.0 |
| Hub radius | 40.0 |
| Berth radius | 22.0 |
| Berth 3 policy | Always open for visitors |
| NPC berth assignment | Self-assign, no player control |

---

*Voidrift Station Architecture Design Document | April 2026 | RFD IT Services Ltd.*  
*The Main Station rotates. The Drone Depot waits. Both serve the operation.*
