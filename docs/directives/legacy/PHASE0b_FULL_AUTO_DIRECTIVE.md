# PHASE 0b: Full Auto Refinery & Forge
**Objective:** Remove manual auto-smelt toggles, make refinery and forge process ore automatically without player input  
**Status:** Ready for implementation  
**Estimated time:** 1-2 hours  
**Risk level:** Low (isolated changes)

---

## Overview

Currently: Checkboxes for "auto smelt" exist but don't work properly. Manual buttons clutter the UI.

After Phase 0b: Refinery and Forge run continuously, automatically processing available ore into ingots and products. No toggles, no buttons. Just ore in, products out.

---

## What Needs to Change

### 1. Remove AutoDockSettings Struct (Not Needed Yet)

**File:** `src/components.rs`

**Find:**
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

**Delete entire struct and impl block.**

(We'll re-add it in Phase 1 with toggles. For now, processing is always-on.)

---

### 2. Remove AutoDockSettings from lib.rs

**File:** `src/lib.rs`

**Find the line:**
```rust
.insert_resource(AutoDockSettings::default())
```

**Delete it.**

---

### 3. Verify Auto-Processing System Exists and Works

**File:** `src/systems/auto_process.rs` (should exist from Phase 0)

**Check that you have three systems:**
1. `auto_refine_system()` — ore → ingots
2. `auto_forge_system()` — ingots → products
3. `auto_build_drones_system()` — products → drones

**These should run EVERY FRAME without any toggles.**

If they exist and are registered in `lib.rs`, move to Step 4.

**If they DON'T exist**, Antigravity needs to create them. Post code here and we'll fix it.

---

### 4. Remove UI Checkboxes for Auto-Smelt

**File:** `src/systems/hud/content.rs`

**Find any UI code that renders checkboxes for:**
- `auto_smelt_iron`
- `auto_smelt_tungsten`
- `auto_smelt_nickel`
- `auto_unload`

**Delete the entire checkbox rendering code.**

Example of what to delete:
```rust
ui.checkbox(&mut settings.auto_smelt_iron, "Auto-Smelt Iron");
ui.checkbox(&mut settings.auto_smelt_tungsten, "Auto-Smelt Tungsten");
ui.checkbox(&mut settings.auto_smelt_nickel, "Auto-Smelt Nickel");
```

---

### 5. Clean Up HUD Content Display

**File:** `src/systems/hud/content.rs`

**In the Refinery/Forge tabs, remove any rendering that shows:**
- Queue progress
- Manual buttons
- Timer displays for individual batches

**Replace with:** Simple text showing current station reserves (ore, ingots, products).

Example of what to show instead:
```
IRON ORE: 125.3
IRON INGOTS: 45.2
HULLS: 3

TUNGSTEN ORE: 67.8
TUNGSTEN INGOTS: 12.1
THRUSTERS: 1

NICKEL ORE: 89.5
NICKEL INGOTS: 22.3
AI CORES: 2
```

No progress bars. No buttons. Just status.

---

## Verification Checklist

After all changes:

```bash
cargo check
# Should pass with 0 errors
```

**On device:**

- [ ] Mine for 30 seconds (ore accumulates)
- [ ] Dock at station (cargo unloads)
- [ ] Wait 5 seconds (ore starts converting to ingots visibly)
- [ ] Watch ingots tick up in real-time
- [ ] Wait another 10 seconds (ingots convert to products)
- [ ] Watch products appear
- [ ] When you have 1 Hull + 1 Thruster + 1 AI Core, drone builds automatically
- [ ] Drone count increases
- [ ] No checkboxes visible in UI
- [ ] No manual buttons anywhere
- [ ] Everything is automatic

---

## Commit

When it's working:

```bash
git add -A
git commit -m "Phase 0b: Full auto refinery & forge, remove manual toggles

- Remove AutoDockSettings (re-add in Phase 1 with toggles)
- Strip manual checkbox UI
- Verify auto_process systems run continuously
- Ore now flows: Mining -> Unload -> Refine -> Forge -> Build automatically
- No player interaction needed, just watch it work"

git tag v0.5.17-full-auto
git push origin dev --tags
```

---

## If Something Breaks

Post the error or describe what's not working.

Common issues:
- **Ingots still not appearing:** auto_refine_system not registered or not running
- **Products not building:** auto_forge_system or auto_build_drones_system broken
- **Compile errors:** Missing imports after deleting AutoDockSettings

Show me the error and we'll fix it quickly.

---

**That's it. Simple, focused, one job: Make it fully automatic.**

Go.
