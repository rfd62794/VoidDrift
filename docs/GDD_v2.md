# VoidDrift — Game Design Document v2.0

**Date:** May 2026
**Status:** Active — supersedes `GDD_v1_ARCHIVED.md`
**Source:** Locked design from project knowledge.
Full content migration pending dedicated session.

---

## Ring 2 Design — Locked May 13 2026

### Drone Roster (locked)

**Mining Drone (global upgrade tiers):**
- **Mk I:** Surface extractor, Ring 1 operations.
- **Mk II — Breaker:** Cracks medium asteroid cores, extracts H3 Gas into Canisters. Global upgrade — all Mining drones convert simultaneously when researched.
- **Mk III — Grappler:** Long-range retrieval, Ring 3. Reserved. Not yet implemented.

**Scout (unique, one per ring):**
Automates that ring's mining operations, frees player focus.
- Scout Mk I unlocks Ring 1 automation.
- Scout Mk II unlocks Ring 2 automation (later gate).

**Hauler (unique):**
Tugs medium asteroids from Ring 2 into Ring 1 range.
Gate 1 Ring 2 unlock — awarded as first Ring 2 quest reward.
Does not require Canisters to operate.

### Resource Changes (locked)
- **Aluminum:** Ring 2 exclusive. Removed from Ring 1 entirely.
- **Ring 1 ores:** Iron, Tungsten, Nickel only.
- **Medium asteroids:** One metal type exterior (Iron / Tungsten / Nickel / Aluminum variants) + H3 Gas core.
- **H3 Gas:** Extracted from medium asteroid cores by Mining Mk II Breaker into Canisters.

### Canister System (locked)
- Canisters crafted from Aluminum in Forge.
- Two states: **Empty** (crafted) / **Filled** (H3 Gas loaded).
- Hauler tugs asteroid in. Breaker extracts H3 Gas → fills Canisters.

### Crystal Recipe (locked)

```
Iron + Tungsten + Nickel + Filled Aluminum Canister = Crystal
```

### Ring 2 Unlock Gate Sequence (locked)
- **Gate 1:** Hauler (quest reward) — enables asteroid tug mechanic
- **Gate 2:** Canisters (Forge recipe, requires X Aluminum)
- **Gate 3:** Mining Mk II Breaker (requires X Canisters, global upgrade)
- **Gate 4:** Scout Mk I (after first Crystal produced, Ring 1 automation)
- **Gate 5:** Scout Mk II (Ring 3 readiness gate, later)

### Upgrade Model (locked)
Global per-tier. When Mk II is researched, all existing Mining drones convert to Breaker simultaneously.
Per-drone specialization deferred post-launch.

### Named Deferrals
- **Mining Mk III Grappler:** Ring 3 slot reserved, not implemented.
- **Per-drone specialization:** post-launch if player feedback requests it.
- **Scout Mk III:** Ring 3 automation, future.
