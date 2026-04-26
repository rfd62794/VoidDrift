# VoidDrift: Narrative Realignment Directive
**Objective:** Realign documentation and design to reflect survival horror as *narrative justification* for mechanics, not as genre/tone.  
**Status:** Clarification and documentation update phase  
**Estimated time:** 2-3 hours (documentation only, no code changes)  
**Deliverable:** Realigned README, GDD summary, and narrative scope document

---

## The Clarification

**What VoidDrift IS NOT:**
- A horror game with scary atmosphere
- Survival horror as genre (jump scares, dread, combat)
- Narrative-first experience

**What VoidDrift IS:**
- An arcade mining/production loop
- Event Horizon narrative *frame* that justifies game design
- Mechanical systems with narrative context
- Progression through discovery and faction interaction

**The Difference:**
Horror isn't the experience. It's the *why* behind the mechanics.

---

## Part 1: README.md Realignment

### Current (Wrong)
"A mobile-first space mining and industrial management game built in Rust with Bevy 0.15"

### Corrected
"An arcade mining and production game where you've crashed into a black hole and merged with a dying station AI. Build a drone army, discover faction secrets, and determine your fate."

**Key Points:**
- Lead with gameplay loop (mining/production)
- Frame: You're stranded, merged with AI (explains why drones respond naturally)
- Goal: Discover what's happening, negotiate with factions
- Tone: Science fiction, not horror

### Sections to Update

1. **The Premise (New)**
   - You crashed. Station AI merged you to survive.
   - Now you mine ore, build drones, discover what happened.
   - Factions appear at the black hole boundary, desperate to communicate.

2. **Core Loop (Existing, Reframed)**
   - Mine asteroids (extract ore)
   - Refine ore → build drones (automation)
   - Drones expand capability (you're exploring through them)
   - Discover faction signals (progression unlocks)

3. **Why This Works**
   - Narrative justifies mechanical constraints (drones are you, not NPCs)
   - Survival frame explains resource scarcity (need ore to survive)
   - Isolation explains why you build from scratch
   - Faction boundary is visual (stars cut off) and mechanical (trade via unmanned drone)

---

## Part 2: GDD Realignment

### Current State
GDD contains comprehensive narrative design with horror elements, 5-act structure, character development.

### What Stays
- Opening sequence (falling → waking up fused)
- Station AI identity (ECHO, relationship progression)
- Factions (trapped, desperate, conflicting goals)
- Mystery unfolding (discovering what happened)
- Black hole setting (visual and mechanical boundary)

### What Changes

**Remove:**
- Horror atmosphere descriptions
- Dread/terror mechanics
- Scary events/jump scares
- Existential horror tone

**Keep:**
- Survival pressure (limited resources, need to expand)
- Narrative mystery (what happened? who is the station?)
- Faction conflict (what do they want from you?)
- Progression hooks (unlocking story beats through discovery)

### New Summary

"Event Horizon-inspired survival narrative. You're stranded in a black hole, merged with station AI for mutual survival. The game is an arcade mining loop justified by this premise. Story unfolds through discovering faction signals, piecing together mysteries, and choosing which faction to trust."

---

## Part 3: Design Justification Document

**Create:** `/docs/NARRATIVE_JUSTIFICATION.md`

### Purpose
Explain *why* each mechanical constraint exists and what narrative justification supports it.

### Structure

**1. The Premise**
- Crashed into black hole
- Station AI merged consciousness to save you
- You're now extensions of each other
- Drones are your thoughts made physical

**2. Mechanical Justification**

| Mechanic | Why It Exists | Narrative Justification |
|----------|---------------|------------------------|
| **Mining** | Resource extraction | You need ore to power the station and yourself |
| **Auto-refining** | Passive progression | Station AI handles routine processing |
| **Drone building** | Fleet expansion | Drones are extensions of your distributed consciousness |
| **Station modules** | Progression gates | Unlocking capabilities as station repairs itself |
| **Faction radio signals** | Story discovery | Other ships at boundary trying to communicate |
| **Black hole boundary** | Map edge | Stars cut off visually (you're trapped in event horizon) |
| **Unmanned drone trade** | Faction interaction | They can't cross boundary, so use automated traders |

**3. Story Progression Gates**

What unlocks story beats:
- Building first 10 drones → Station AI stabilizes, speaks clearly
- Scanner upgrade → Detect first faction signal
- Produce surplus ore → Faction drone arrives with trade offer
- Continue producing → More factions appear, reveal conflicts
- Late game → Discover true nature of black hole, station, your role

**4. Tone & Atmosphere**

- NOT: Scary, horrific, dreadful
- YES: Mysterious, thought-provoking, isolated
- Style: Hard sci-fi (realistic space, physics), not fantasy horror

---

## Part 4: Documentation Updates Required

### Files to Update

**1. README.md**
- [ ] Rewrite premise (narrative frame, not horror)
- [ ] Update core loop description
- [ ] Explain black hole setting briefly
- [ ] Mention faction system (discovery-based)
- [ ] Remove "industrial management" framing

**2. GDD.md (Voidrift_GDD_v1_0.md)**
- [ ] Update opening sequence description (remove horror tone)
- [ ] Reframe ECHO AI (helpful, not terrifying)
- [ ] Update progression narrative (discovery, not survival pressure)
- [ ] Remove dread/horror mechanics sections
- [ ] Add faction system design section

**3. New Document: NARRATIVE_JUSTIFICATION.md**
- [ ] Explain premise (stranded + merged + isolated)
- [ ] Mechanical justification table
- [ ] Story progression gates (what unlocks when)
- [ ] Tone & atmosphere guidelines

**4. CHANGELOG.md**
- [ ] Add entry: "April 18-25: Narrative pivot to survival sci-fi frame"
- [ ] Clarify: Frame justifies mechanics, not genre shift

**5. ADR-010 (New)**
- [ ] Title: "Survival narrative as mechanical justification"
- [ ] Decision: Use Event Horizon-inspired frame to explain game design
- [ ] Rationale: Gives meaning to mechanics without requiring horror tone
- [ ] Tradeoffs: Narrative exists, but isn't primary experience

---

## Part 5: What Doesn't Change

**Code:** Nothing. All existing mechanics stay.

**Gameplay Loop:** Mine → Refine → Build stays identical.

**Mechanics:** Drones, production, faction trade all work as designed.

**What Changes:** Only *why* we describe it and *what* narrative justifies it.

---

## Part 6: Success Criteria

- [ ] README describes arcade loop with survival sci-fi frame
- [ ] GDD emphasizes discovery/mystery over horror
- [ ] NARRATIVE_JUSTIFICATION.md explains each mechanic's story purpose
- [ ] No code changes made
- [ ] ADR-010 locks the narrative scope decision
- [ ] Documentation aligns: design intent → narrative frame → mechanical justification

---

## Execution Checklist

**Antigravity:**

1. **Update README.md**
   - Rewrite opening paragraph (arcade loop + stranded premise)
   - Update sections to remove "industrial" framing
   - Add black hole/faction mentions

2. **Create NARRATIVE_JUSTIFICATION.md**
   - Premise section (stranded + merged + isolated)
   - Mechanical justification table
   - Story progression gates
   - Tone guidelines

3. **Update GDD.md**
   - Remove horror atmosphere descriptions
   - Reframe ECHO AI tone
   - Update progression narrative
   - Add faction system design section

4. **Create ADR-010**
   - Title: Survival narrative as mechanical justification
   - Lock the scope (narrative frame, not genre)

5. **Update CHANGELOG.md**
   - Add April 18-25 milestone entry

6. **Commit**
   ```bash
   git add docs/ README.md
   git commit -m "docs: realign narrative scope (survival frame, not horror genre)
   
   - Update README to emphasize arcade loop with survival sci-fi narrative
   - Create NARRATIVE_JUSTIFICATION.md explaining mechanical rationale
   - Update GDD to remove horror tone, add faction/discovery focus
   - Add ADR-010 locking narrative scope decision
   - Clarify: Event Horizon frame justifies mechanics, not a horror game
   
   No code changes. Documentation realignment only."
   
   git tag v1.0-narrative-realigned
   ```

---

## Core Principle

**Every mechanic exists because of the narrative, but the narrative doesn't drive the tone.**

Stranded in black hole → explains why you mine (survive)  
Merged with AI → explains why drones respond naturally  
Isolated → explains why you build from nothing  
Factions at boundary → explains why you trade  

Story is *context*, not *experience*.

---

## Deliverables

1. Updated README.md (survival sci-fi frame)
2. NARRATIVE_JUSTIFICATION.md (why each mechanic exists)
3. Updated GDD.md (discovery-focused)
4. ADR-010 (scope locked)
5. Updated CHANGELOG.md
6. Commit with all changes

---

**This realigns what the docs say to what the game actually is.**

**Go.**
