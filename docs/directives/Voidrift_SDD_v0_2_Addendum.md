# Voidrift — SDD v0.2 Addendum: Post-Slice Economy & Narrative
**Date:** April 2026  
**Supersedes:** SDD v0.1 §9 Post-Slice Roadmap (informational only)  
**Status:** Locked — Design session 2026-04-18

---

## 1. What Changed

SDD v0.1 defined the MVP slice only. This addendum locks the first post-slice design layer: the two-resource economy, the station AI narrative voice, and the autonomous ship assembly system. These decisions are final and do not require further design sessions before Phase 7 begins.

---

## 2. Resource Naming — Locked

| Resource | Type | Source | Notes |
|----------|------|---------|-------|
| Magnetite | Raw ore | Sector 1 (existing asteroid field) | Magnetic, conductive — energy and AI systems |
| Carbon | Raw ore | Sector 7 (new field — not yet implemented) | Structural bulk material — hull-grade yield |
| Power Cells | Refined product | Magnetite × REFINERY_RATIO (10) | Existing — name unchanged |
| Hull Plate | Refined product | Carbon × HULL_REFINERY_RATIO (TBD Phase 7) | New — feeds ship assembly |
| AI Core | Built module | Power Cells × AI_CORE_COST (50) | Existing — name unchanged |
| Autonomous Ship | Assembled unit | 1 Hull Plate + 1 AI Core | New — first fleet unit |

> ⚠️ Retired terms — must not appear in code, UI, or logcat:
> - "Ore" → use Magnetite or Carbon specifically
> - "Drone" → use Autonomous Ship
> - "Raw ore" in UI → use the named resource

---

## 3. Why These Names

**Magnetite** — a real magnetic iron oxide mineral. Genuinely conductive, used in energy contexts. Sounds like something a station AI would report as a specific ore signature. Pairs cleanly with Carbon without either word overpowering the other.

**Carbon** — carbon composite, carbon fiber, structural. Real material. Industrial and grounded. High-density carbon signature is exactly what a station AI would detect as hull-grade.

The naming logic is internally consistent: Magnetite is the energy ore, Carbon is the structural ore. Every refinery decision flows from that distinction.

---

## 4. Two-Resource Economy — Locked

The economy ceiling is two of everything. Hard constraint, not a guideline.

| Layer | Item A | Item B | Ceiling |
|-------|--------|--------|---------|
| Raw ore | Magnetite | Carbon | 2 — no third ore type |
| Refined product | Power Cells | Hull Plate | 2 — no third product |
| Built module | AI Core | TBD (Phase 8) | 2 — second module deferred |
| Autonomous ships | Ship 1 | Ship 2 (deferred) | 2 per design session |

**Named deferral — do not scope until two-resource economy is proven on device:**
- Third ore type
- Crew system
- Trading economy
- Sector gate network beyond Sector 7
- Second autonomous ship

---

## 5. Station AI — Narrative Voice

The AI Core gives the station awareness. The station reports observations in the docking UI as a terse, functional log. No warmth, no exposition. Present tense. System log register.

**Voice rules:**
- One line per observation
- Present tense only
- No punctuation flourishes
- Reads like telemetry, not dialogue

**Example log lines:**
```
[STATION AI] AI Core nominal. Awaiting directive.
[STATION AI] Carbon signature detected. Bearing 047. Hull-grade yield confirmed. Designation: Sector 7.
[STATION AI] Hull synthesis possible. Second AI Core required for autonomous operation.
[STATION AI] Hull Plate fabricated. Assembly ready on AI Core confirmation.
[STATION AI] Ship assembly complete. Autonomous unit launched.
[STATION AI] Magnetite reserves: 240. Power Cells: 12.
```

**Implementation:** Scrolling text area in the egui docking panel. New entries append to bottom. Maximum 10 visible lines — older entries scroll out. The log IS the tutorial. No separate help system.

**Trigger conditions:**

| Trigger | Log Entry |
|---------|-----------|
| AI Core built | `[STATION AI] AI Core nominal. Awaiting directive.` |
| First dock after AI Core built | `[STATION AI] Carbon signature detected. Bearing 047. Hull-grade yield confirmed. Designation: Sector 7.` |
| Carbon in reserves > 0 | `[STATION AI] Hull synthesis possible. Second AI Core required for autonomous operation.` |
| Hull Plate produced | `[STATION AI] Hull Plate fabricated. Assembly ready on AI Core confirmation.` |
| Ship assembled | `[STATION AI] Ship assembly complete. Autonomous unit launched.` |

---

## 6. Autonomous Ship Assembly — Locked

**Assembly recipe:**
- 1 Hull Plate (in station reserves)
- 1 AI Core (built — costs 50 Power Cells)
- Trigger: BUILD SHIP button in docking UI — appears only when both components present

**On assembly:**
- Hull Plate consumed from reserves
- AI Core marker removed from station
- Autonomous Ship entity spawned at station position
- Ship begins Outbound → Mining (Sector 1, Magnetite) → Returning → Unloading loop
- Station AI logs: `[STATION AI] Ship assembly complete. Autonomous unit launched.`

**Visual identity:**
- Autonomous Ship: Orange
- Player Ship: Cyan
- Cargo bar above each — same implementation pattern

**Autonomous Ship behavior:**
- Mines Sector 1 (Magnetite) by default — fixed route
- Unloads into station ore reserves automatically
- Player cannot redirect in Phase 7 — redirection deferred post-Phase 8

---

## 7. Sector 7 — New Field

**Ore:** Carbon only. No Magnetite at Sector 7.

**Position:** `(350.0, 250.0)` — suggested. Far enough to feel like a discovery, reachable on the existing map scale. Confirm on device in Phase 7.

**Discovery:** Not visible on map until Station AI detects it. Detection triggers after first dock with AI Core built. The player does not know Sector 7 exists until the station tells them.

**Player mining:** Player ship mines Carbon at Sector 7 manually. The Autonomous Ship does not mine Sector 7 — it stays on Magnetite at Sector 1. Carbon acquisition is a player decision, at least until a second autonomous ship exists.

---

## 8. Phase 7 Scope — What Gets Built Next

Phase 7 implements the two-resource economy end-to-end:

1. Rename existing ore to Magnetite throughout code and UI
2. Add Sector 7 to map — hidden until Station AI detection trigger fires
3. Station AI log panel in docking UI — scrolling egui text area
4. Carbon mining at Sector 7 — player ship only
5. Carbon refinery chain — Carbon → Hull Plate at new ratio (TBD)
6. Ship assembly UI — BUILD SHIP button when Hull Plate + AI Core present
7. Autonomous Ship spawn on assembly — replaces Phase 6 drone spawn pattern

**Gate:** Player assembles first Autonomous Ship from components they produced. Station AI log confirms assembly. Orange ship visible mining Sector 1 while player mines Sector 7. Both on device simultaneously.

---

## 9. Constants To Lock in Phase 7

| Constant | Value | Status |
|----------|-------|--------|
| `HULL_REFINERY_RATIO` | TBD | Decide before Phase 7 directive |
| `SECTOR_7_POSITION` | `(350.0, 250.0)` | Confirm on device |
| `STATION_AI_LOG_MAX_LINES` | 10 | Locked |

> ⚠️ `HULL_REFINERY_RATIO` must be decided before the Phase 7 directive is written. Suggested: 5 Carbon → 1 Hull Plate. Carbon runs are longer-range trips — fewer units per run justifies a lower ratio than Magnetite.

---

*Voidrift SDD v0.2 Addendum | April 2026 | RFD IT Services Ltd.*  
*Living document — update on every design decision that affects the economy or narrative.*
