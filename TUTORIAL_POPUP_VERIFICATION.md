# Tutorial Popup Fix Verification — v3.0.4

## Changes
- **Fix 1**: Removed duplicate `.frame()` call that overwrote cyan border styling
- **Fix 2**: Moved popup Window rendering after CentralPanel with `.order(egui::Order::Foreground)` to ensure input priority

## Verification Checklist

### Desktop (.\run.ps1)
- [ ] Launch game, reach opening sequence completion
- [ ] T-101 popup appears with cyan border and dark background
- [ ] Popup text: "Mining protocols active. Tap the highlighted asteroid to dispatch a drone."
- [ ] Click "Understood" button — popup dismisses immediately
- [ ] Tutorial advances to T-102 (highlight moves to Nickel asteroid)
- [ ] Walk through all six tutorial steps (T-101 to T-106)
- [ ] All buttons dismiss popups correctly
- [ ] No visual artifacts or overlapping UI

### Mobile Web (WASM via itch.io or local)
- [ ] Build: `.\build_wasm.ps1`
- [ ] Serve pkg/ locally or deploy to itch.io
- [ ] T-101 popup renders with correct styling on mobile viewport
- [ ] Touch "Understood" button — popup dismisses
- [ ] All six tutorial steps complete on touch input
- [ ] No input lag or double-tap issues

### Code Quality
- [ ] `cargo check` — zero new errors
- [ ] Pre-existing warnings only (DeviceType ambiguity — unrelated)
- [ ] Git tag: `v3.0.4-tutorial-popup-fix` created

## Expected Behavior After Fix
1. Popup has **cyan border** (2px stroke) and **dark fill** (rgb 5,5,10)
2. Button click registers immediately (no world interaction)
3. Popup dismisses and tutorial state advances
4. Next popup fires on next trigger condition
5. All six steps (T-101 to T-106) complete without issues

## Deployment Gate
✅ Code compiles cleanly
⏳ Desktop verification pending
⏳ Mobile web verification pending
⏳ itch.io deployment pending

---
**Tag**: v3.0.4-tutorial-popup-fix  
**Files Modified**: src/systems/ui/hud/mod.rs (2 changes, 24 lines net)  
**Ship Blocker**: YES — new players cannot progress past T-101 without this fix
