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

---

## 3. Drone Depot — Visual Structure

### 3.1 Design

The Drone Depot reuses the existing square station visual. It is static — no rotation. It is anonymous — no hub-and-spoke identity. It looks industrial because it is industrial.

### 3.2 Berths

Drone Depot berths are invisible — implied by the square structure. Autonomous ships dock at the Depot center with a fixed offset per ship index.

| Drone Depot Berth | Offset | Notes |
|-------------------|--------|-------|
| 1 | `(-30, 0)` | First drone |
| 2 | `(30, 0)` | Second drone |
| 3 | `(0, 30)` | Third drone |

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
```

---

## 5. Autopilot Integration

Ships navigate to a **berth position**, not the station center. The berth position is dynamic — it changes as the station rotates. The autopilot system must recalculate the target position each tick when the destination is a berth.

---

## 6. Implementation Phasing

**Phase A — Station Rotation (visual only)**
- Add `rotation` field to Station
- Render hub as circle, 6 arms, berth circles
- Rotation updates each tick

**Phase B — Berth Navigation**
- Berths as entities with dynamic world positions
- Autopilot navigates to berth position

**Phase C — NPC Berth (future)**
- Berth 3 open for visitors
- NPC ships self-assign on approach

**Phase D — Drone Depot (future)**
- Drone Depot as buildable structure
- Autonomous ships reassigned to Depot
