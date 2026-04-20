# Voidrift — Directive A: Structural Stabilization
**Status:** Approved — Ready for Execution  
**Type:** Structural Fix — No gameplay changes  
**Date:** April 2026  
**Depends On:** Structural Analysis COMPLETE ✅  
**Blocks:** Station Phase B (cannot proceed until this passes)

---

## 1. Objective

Stop the B0001 crashes. Get the build running on device. Two changes only:

1. Apply minimal disjoint filters to all Critical and High risk systems
2. Extract beam logic from `autonomous_ship_system` into a dedicated `autonomous_beam_system`

No gameplay changes. No component decomposition. No new features. Stabilize first.

---

## 2. Scope

**In scope:**
- `Without<>` filter additions to resolve all identified Transform conflicts
- Extract `autonomous_beam_system` from `autonomous_ship_system`
- Register new system in `lib.rs`

**Explicitly out of scope:**
- Component decomposition (Directive B)
- StationLog extraction
- StationRotation extraction
- Cargo component unification
- CameraTarget refactor
- Any gameplay logic changes
- Any new features

---

## 3. Transform Conflict Fixes

Apply these filters to each system. Do not add any other changes.

### 3.1 autonomous_ship_system (CRITICAL — currently crashing)

Current conflicting queries: `ship_query` ↔ `station_query` ↔ `beam_query`

**Fix — add Without filters to each query:**

```rust
// ship_query
Query<
    (&mut AutonomousShip, &mut Transform, &mut AutonomousAssignment, Option<&Children>),
    (Without<Station>, Without<MiningBeam>)
>

// station_query  
Query<
    (&mut Station, &Transform),
    (Without<AutonomousShip>, Without<MiningBeam>)
>

// beam_query — REMOVE from this system entirely
// Beam logic moves to autonomous_beam_system (see §4)
```

After beam query is removed from `autonomous_ship_system`, the only remaining conflict is `ship_query` ↔ `station_query`. The Without filters on those two resolve it.

### 3.2 mining_system (HIGH)

Conflicting queries: `beam_query` ↔ `ship_query` ↔ `field_query`

```rust
// ship_query
Query<
    (&mut Ship, &Transform),
    (Without<MiningBeam>, Without<AsteroidField>)
>

// beam_query
Query<
    (&mut Transform, &mut Visibility),
    (With<MiningBeam>, Without<Ship>, Without<AsteroidField>)
>

// field_query
Query<
    &Transform,
    (With<AsteroidField>, Without<Ship>, Without<MiningBeam>)
>
```

### 3.3 camera_follow_system (HIGH)

Conflicting queries: `cam_query` ↔ `ship_query`

```rust
// cam_query
Query<
    &mut Transform,
    (With<MainCamera>, Without<Ship>)
>

// ship_query
Query<
    &Transform,
    (With<Ship>, Without<MainCamera>)
>
```

### 3.4 starfield_scroll_system (HIGH)

Conflicting queries: `star_query` ↔ `cam_query`

```rust
// star_query
Query<
    (&mut Transform, &StarLayer),
    Without<MainCamera>
>

// cam_query — read only, no mutation needed here
Query<
    &Transform,
    (With<MainCamera>, Without<StarLayer>)
>
```

### 3.5 autopilot_system (HIGH)

Conflicting queries: `ship_query` ↔ `station_query`

```rust
// ship_query
Query<
    (&mut Ship, &mut Transform, &mut AutopilotTarget),
    Without<Station>
>

// station_query
Query<
    (&Station, &Transform),
    Without<Ship>
>
```

---

## 4. autonomous_beam_system (New System)

Extract all MiningBeam logic from `autonomous_ship_system` into a new dedicated system.

### 4.1 Responsibility

`autonomous_beam_system` owns:
- Reading autonomous ship position and state
- Reading asteroid field position
- Updating MiningBeam Transform and Visibility

It does NOT own:
- Ship movement
- Cargo accumulation
- Station interaction
- Docking logic

### 4.2 Implementation

Add to `autonomous.rs`:

```rust
pub fn autonomous_beam_system(
    ship_query: Query<
        (&AutonomousShip, &Transform),
        Without<MiningBeam>
    >,
    field_query: Query<
        &Transform,
        (With<AsteroidField>, Without<AutonomousShip>)
    >,
    mut beam_query: Query<
        (&mut Transform, &mut Visibility),
        (With<MiningBeam>, Without<AutonomousShip>, Without<AsteroidField>)
    >,
) {
    // For each autonomous ship in Mining state:
    //   Find its assigned field position
    //   Update beam Transform (position, rotation, scale) to connect ship to field
    //   Set Visibility::Visible
    // For all other states:
    //   Set beam Visibility::Hidden
}
```

### 4.3 System Ordering

`autonomous_beam_system` must run AFTER `autonomous_ship_system` in the schedule. The beam reads ship state that the main system writes.

In `lib.rs`:
```rust
.add_systems(Update, (
    // ... existing systems ...
    systems::autonomous::autonomous_ship_system,
    systems::autonomous::autonomous_beam_system,  // runs after
))
```

Use `.after(systems::autonomous::autonomous_ship_system)` if explicit ordering is needed.

---

## 5. Verification Before Proceeding

After applying all fixes, verify this sequence before declaring done:

1. `cargo check` — zero errors
2. `cargo build` (debug, not release) — zero errors  
3. `.\build_android.ps1` — build succeeds
4. App launches on Moto G 2025 — no crash, logcat confirms no B0001
5. App runs for 30 seconds — no crash

**Do not proceed to Phase B until all 5 pass.**

---

## 6. File Scope

| File | Change |
|------|--------|
| `src/systems/autonomous.rs` | Remove beam query from `autonomous_ship_system`, add `autonomous_beam_system` |
| `src/systems/mining.rs` | Add Without filters to all Transform queries |
| `src/systems/visuals.rs` | Add Without filters to camera and starfield systems |
| `src/systems/autopilot.rs` | Add Without filters to ship and station queries |
| `src/lib.rs` | Register `autonomous_beam_system`, ensure correct ordering |
| `Cargo.toml` | READ-ONLY |

**All other files are read-only.**

---

## 7. Completion Criteria

- [ ] App launches without B0001 crash
- [ ] No new B0001 errors introduced
- [ ] `autonomous_beam_system` runs independently and beam still displays correctly
- [ ] Mining beam visible on player ship while mining
- [ ] Mining beam visible on autonomous ship while mining
- [ ] Station docking still works
- [ ] 30 seconds of runtime on device with no crash

**This directive is complete when the app runs stably on device. Nothing else.**

---

## 8. Note to Agent

Do not fix anything beyond what is listed in §3 and §4. If you notice other issues during implementation — note them in a comment but do not fix them. This directive has exactly one job: stop the crashes. Everything else is Directive B.

If a filter you add causes a compile error because an entity genuinely can have both components simultaneously, report it immediately rather than guessing a workaround.

---

*Voidrift Directive A — Structural Stabilization | April 2026 | RFD IT Services Ltd.*  
*Stop the crashes. Nothing else.*
