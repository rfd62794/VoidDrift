# ADR-007: Update Schedule System Partitioning

**Date:** April 2026  
**Status:** Accepted

## Context
Rust's type system limits tuple size to 20 elements. Bevy's 
`.add_systems(Update, (sys1, sys2, ...))` uses tuples internally.
As Voidrift grew beyond 20 systems, the compiler rejected the single 
registration tuple with error E0277.

## Decision
Partition the Update schedule into two named groups:
- **Group 1**: Gameplay & Logistics systems (movement, mining, economy, autopilot, autonomous).
- **Group 2**: Station, Narrative & UI systems (visuals, narrative, tutorial, quest, ui, map).

Each group is registered as a separate `.add_systems(Update, (...))` call in `lib.rs`.

## Consequences
- New systems must be assigned to a group on registration.
- Groups must be monitored for size — stop at 18 to leave headroom for chaining.
- System ordering between groups relies on Bevy's default scheduling; if cross-group ordering is needed, use `.after()` or `.before()` explicitly.
- Chaining should only be used within a partition unless the entire partition is chained.

## Alternatives Considered
- **SystemSet labeling**: More complex to manage, rejected for the simplicity of partition groups.
- **Single flat tuple**: Rejected, hits compiler limit and makes `lib.rs` harder to read.
