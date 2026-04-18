# SDD v0.2 Corrections
## Voidrift Software Design Document — Errata Record

**Source Document:** SDD v0.1 (`docs/Voidrift_SDD_v0_1.docx`)
**Correction Date:** April 2026
**Produced By:** Phase 0 Directive — OQ-001 / OQ-002 research findings
**Format:** This markdown record is the formal correction log. A revised .docx is not required.

---

## Correction 1 — Section 4.2: Bevy Version Annotation

**Location:** Section 4.2 "Bevy Version & Dependencies", `bevy 0.15` row, Notes column.

| Field | SDD v0.1 Text | Corrected Text |
|---|---|---|
| Notes | "Latest stable as of April 2026" | "Pinned at 0.15 — chosen for Android community guide coverage (see OQ-001). Current latest is 0.18 (released Jan 13, 2026). Migration to 0.18 deferred to post-slice." |

**Rationale:** Bevy 0.18 was released January 13, 2026. 0.15 is not "latest stable" as of April 2026. The version pin is intentional and pragmatic — 0.15 has the most complete Android/GameActivity build documentation. The SDD annotation implied recency rather than deliberate pinning, which would cause confusion if someone upgrades without a new directive.

---

## Correction 2 — Section 4.2: bevy_android Removed from Dependencies Table

**Location:** Section 4.2 "Bevy Version & Dependencies" table.

**Remove:** The `bevy_android` row entirely.

**Add footnote to section 4.2:**
> *Android support is built into the `bevy` crate as the internal `bevy_android` workspace crate. No separate `bevy_android` entry is required in `Cargo.toml`. Adding it separately would be incorrect. This was confirmed by OQ-001 research (April 2026).*

**Rationale:** `bevy_android` is not a user-facing crate on crates.io. It is an internal module within the Bevy workspace and is pulled in automatically when targeting `aarch64-linux-android`. Listing it as a dependency implies a `Cargo.toml` entry that does not exist and would cause confusion during Phase 0 setup.

---

## Correction 3 — Section 4.1: ADR-002 Note Added

**Location:** Section 4.1 Architecture Decision Records, ADR-002 row.

| Field | SDD v0.1 | Addition |
|---|---|---|
| ADR-002 Rationale | "Porting later costs more than building mobile-first from day one." | Add: "Resolved: GameActivity (not NativeActivity) selected as activity type. Bevy 0.15 default. Minimum API 31 required; Moto G 2025 is API 35 — fully supported. NativeActivity is the deprecated path and was used in OperatorGame; Voidrift does not carry this forward." |

**Rationale:** The SDD stated Android-first without specifying the activity model. The GameActivity vs NativeActivity decision is a meaningful one (different Gradle setup, different feature flags, different API minimums) and should be recorded as resolved.

---

## Correction 4 — Section 8: OQ-001 Status Updated

**Location:** Section 8 "Open Questions", OQ-001 row.

| Field | SDD v0.1 | Corrected |
|---|---|---|
| Status | Open | **Resolved** |
| Resolution | — | "Bevy 0.15 pinned. Most complete Android community documentation. cargo-ndk + GameActivity pipeline documented against this version. See OQ-001/OQ-002 Research Findings, April 2026." |

---

## Correction 5 — Section 8: OQ-002 Status Updated

**Location:** Section 8 "Open Questions", OQ-002 row.

| Field | SDD v0.1 | Corrected |
|---|---|---|
| Status | Open | **Resolved** |
| Resolution | — | "NDK r29 (29.0.14206865) in use — confirmed installed on build machine. Partial reuse from OperatorGame: NDK version, rustflags (-lc++_shared, max-page-size=16384), and API 35 linker path carry forward. Build toolchain does NOT carry forward: cargo-apk (OperatorGame) → cargo-ndk + Gradle (Voidrift). Activity type does NOT carry forward: NativeActivity (OperatorGame) → GameActivity (Voidrift). See Phase 0 Directive and OQ-001/OQ-002 Research Findings." |

---

*Voidrift SDD Corrections v0.2 | April 2026 | RFD IT Services Ltd.*
*Produced as Phase 0 deliverable alongside scaffold code.*
