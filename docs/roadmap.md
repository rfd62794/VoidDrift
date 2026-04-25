# VoidDrift Arcade Loop Roadmap

## Phase 0: Foundation (Current)
**Goal:** Make the ore flow feel good and ensure all crafting is fully automated.
**Steps:**
- Tune ore/ingot/product conversion rates (make it satisfying to watch).
- Verify all refinery/forge/drone-building is invisible (no manual buttons).
- Test on device until feedback loop feels right.
*(No new features. Just fix what we have.)*

## Phase 1: Production Control
**Goal:** Toggle refinery and forge products on/off individually.
**UI:** 
- Refinery tab shows vertical scrolling cards (Iron Ingot: 🟢 ON, Tungsten Ingot: 🔴 OFF, etc.)
- Same for Forge (Hull: 🟢 ON, Thruster: 🔴 OFF, etc.)
**System:** Station only processes enabled products.

## Phase 2: Upgrade Tab + Turn-In System
**Goal:** New "Upgrades" tab where you turn in resources/products for upgrades.
**UI:**
- List of available upgrades (Mining Speed +10%, Cargo +25%, Refinery Efficiency +15%, etc.)
- Shows cost (requires X Iron, Y Tungsten, Z Nickel)
- Turn-In button
- Once claimed, upgrade active (shows "✓ Mining Speed +10%")
**System:**
- Resources deducted on turn-in
- Upgrade applied to gameplay (drones mine faster, ships carry more, etc.)

## Phase 3: Composite Products (Late-Game)
**Goal:** Higher-tier products for exponential progression.
**Example:**
- Drone: Hull + Thruster + AI Core (current)
- Advanced Drone: Drone + Power Core + Armor Plating (late-game)
- Station Module: Hull + Tungsten Block + Crystal (builds station upgrades)
*(Deferred for now — focus on upgrading the core loop first)*
