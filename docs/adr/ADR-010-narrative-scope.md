# ADR-010: Survival Narrative as Mechanical Justification

**Date:** April 26, 2026  
**Status:** Accepted

## Context

During the April 18-25, 2026 narrative pivot, Voidrift shifted from "space mining sim" to "Event Horizon-inspired narrative." Initial documentation suggested horror elements, but this created confusion between narrative frame and gameplay experience.

The core issue: The game is fundamentally an arcade mining/production loop, but documentation was describing it as a horror experience. This misalignment risked setting wrong player expectations and complicating future development.

## Decision

**Voidrift uses survival sci-fi as narrative justification for mechanics, not as horror genre.**

Key principles:
1. **Narrative Frame, Not Genre:** The black hole setting explains WHY mechanics exist, doesn't change WHAT the game is
2. **Mechanical Justification:** Every core mechanic has a narrative reason for existing
3. **Tone Separation:** Story context is mysterious/thought-provoking, gameplay remains arcade/industrial
4. **No Horror Elements:** No scares, dread, or horror atmosphere. Mystery and discovery, not terror.

## Narrative Frame Summary

- **Premise:** Crashed into black hole, station AI fused with player consciousness to survive
- **Mechanical Justification:** 
  - Mining = need ore to power station/consciousness
  - Drones = extensions of merged consciousness
  - Isolation = trapped at event horizon boundary
  - Factions = other trapped ships communicating via unmanned drones
- **Tone:** Hard sci-fi mystery, not horror
- **Experience:** Arcade mining loop with narrative context

## Consequences

### Positive
- **Clear Player Expectations:** Players understand they're playing an arcade game with story context
- **Mechanical Coherence:** Every system has narrative justification
- **Development Clarity:** Team knows horror elements are out of scope
- **Design Flexibility:** Can add mystery/discovery without horror requirements

### Trade-offs
- **Limited Horror Appeal:** Won't attract horror genre fans
- **Narrative Restraint:** Must resist temptation to add horror elements
- **Marketing Challenge:** Need to communicate "sci-fi mystery" not "space horror"

### Implementation Requirements
- Update all documentation to reflect this scope
- Remove horror language from GDD and design docs
- Emphasize mystery/discovery over dread/terror
- Ensure ECHO AI is helpful partner, not threatening presence

## Alternatives Considered

### Full Horror Implementation
- **Rejected:** Would require complete gameplay redesign
- **Reasoning:** Core mechanics (mining, production) don't support horror gameplay

### Pure Arcade (No Narrative)
- **Rejected:** Narrative justification adds meaning to mechanics
- **Reasoning:** Black hole setting explains constraints and motivations

### Mystery without Sci-Fi Frame
- **Rejected:** Sci-fi frame provides clear mechanical justifications
- **Reasoning:** "Why are we mining?" needs better answer than "because it's a game"

## Status

This ADR locks the narrative scope for Voidrift:
- **Survival sci-fi frame** for mechanical justification
- **No horror elements** in gameplay or tone
- **Mystery/discovery focus** for narrative progression
- **Arcade mining loop** remains core experience

All future development must adhere to this scope. Any deviation requires new ADR.

---

**Related Documents:**
- NARRATIVE_JUSTIFICATION.md - Detailed mechanical justifications
- GDD.md - Updated to remove horror tone
- README.md - Updated to reflect survival sci-fi frame
