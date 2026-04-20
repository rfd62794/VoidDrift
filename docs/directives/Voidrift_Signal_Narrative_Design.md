# Voidrift — Signal: Narrative Design Document
**Status:** Locked — Design Session 2026-04-19  
**Type:** Narrative & UI Voice Design  
**Author:** RFD IT Services Ltd.

---

## 1. What the Signal Is

The Signal is Voidrift's only narrative voice. It is not a UI element — it is a character. It exists before the station wakes up, before the player has any context, before anything is explained. It is the north star the player follows from the first frame to the last.

The Signal does not explain. It observes, reports, and directs. It speaks in the terse functional register of a system that has been running longer than the player has been conscious.

As the game progresses the Signal's source expands — the station AI, a passing trader, a distant beacon, an unknown transmission. But it always speaks in one voice. Always at the bottom of the screen. Always on.

---

## 2. Signal Voice Rules

These rules are permanent. Every Signal line written — now and in future content — must pass all of them.

- **Present tense only.** Never past, never future.
- **No subject pronoun.** Not "I detect" or "You should." Just the fact.
- **One clause per line.** Never compound sentences.
- **No punctuation flourishes.** Period at end only. No ellipsis, no exclamation, no question mark.
- **Reads like telemetry that happens to be meaningful.**
- **Maximum 60 characters per line.**

**Examples of correct voice:**
```
> SIGNAL RECEIVED.
> SOURCE IDENTIFIED. BEARING 047.
> MOVING TO INVESTIGATE.
> STRUCTURE IDENTIFIED. POWER OFFLINE.
> REPAIRS POSSIBLE. MATERIALS REQUIRED.
> MAGNETITE RESERVES LOW. MINING RUN RECOMMENDED.
> AUTONOMOUS UNIT HOLDING. POWER INSUFFICIENT.
> CARBON SIGNATURE DETECTED. SECTOR 7.
> UNKNOWN VESSEL. BEARING 312.
```

**Examples of incorrect voice:**
```
> I have detected a signal at bearing 047.       ← subject pronoun, past tense
> You should mine more Magnetite!                ← second person, exclamation
> Hmm... power seems to be running low...        ← ellipsis, hedging
> The station AI has reported that reserves...   ← third person, compound
```

---

## 3. Opening Sequence — Locked

This is the exact sequence of Signal lines that play on game start. Timing and trigger conditions are defined in §4.

```
Frame 1 — ship spawns adrift, no UI visible except Signal strip:
> SIGNAL RECEIVED.

Frame 2 — 2 second pause:
> SOURCE IDENTIFIED. BEARING 047.

Frame 3 — autopilot engages automatically, ship begins moving:
> MOVING TO INVESTIGATE.

Frame 4 — station enters visual range:
> STRUCTURE DETECTED. DERELICT CLASS.

Frame 5 — ship docks automatically:
> DOCKING COMPLETE.

Frame 6 — station status scan runs:
> POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.

Frame 7 — RESERVES tab unlocks, UI appears:
> REPAIRS POSSIBLE. MATERIALS REQUIRED.
```

The player has not touched the screen. They have watched their situation explained entirely through the Signal. When the UI appears they already know what to do.

---

## 4. Trigger Conditions — Full Signal Event Table

Every Signal line has a trigger. No line fires without a condition. No condition fires a line twice.

| ID | Trigger | Signal Line |
|----|---------|-------------|
| S-001 | Game start | `> SIGNAL RECEIVED.` |
| S-002 | 2 seconds after S-001 | `> SOURCE IDENTIFIED. BEARING 047.` |
| S-003 | Autopilot engages toward station | `> MOVING TO INVESTIGATE.` |
| S-004 | Station enters camera range | `> STRUCTURE DETECTED. DERELICT CLASS.` |
| S-005 | Ship docks at station | `> DOCKING COMPLETE.` |
| S-006 | 1 second after dock | `> POWER OFFLINE. STRUCTURAL INTEGRITY: 73%.` |
| S-007 | RESERVES tab unlocks | `> REPAIRS POSSIBLE. MATERIALS REQUIRED.` |
| S-008 | Player mines first Magnetite | `> MAGNETITE ACQUIRED. REFINERY READY.` |
| S-009 | First Power Cells produced | `> POWER CELLS PRODUCED. REPAIR THRESHOLD: 25.` |
| S-010 | Power Cells reach 25 | `> REPAIR THRESHOLD MET. INITIATE WHEN READY.` |
| S-011 | Station repaired | `> POWER RESTORED. STATION ONLINE.` |
| S-012 | 2 seconds after S-011 | `> AI CORE FABRICATION NOW AVAILABLE.` |
| S-013 | First AI Core fabricated | `> AI CORE NOMINAL. SECTOR 7 SCAN INITIATED.` |
| S-014 | 3 seconds after S-013 | `> CARBON SIGNATURE DETECTED. BEARING 047. DESIGNATION: SECTOR 7.` |
| S-015 | First Hull Plate produced | `> HULL PLATE FABRICATED. FORGE AVAILABLE.` |
| S-016 | First Ship Hull forged | `> SHIP HULL COMPLETE. ASSEMBLY POSSIBLE.` |
| S-017 | First Autonomous Ship assembled | `> AUTONOMOUS UNIT LAUNCHED. SECTOR 1 ASSIGNED.` |
| S-018 | Second Autonomous Ship assembled | `> AUTONOMOUS UNIT LAUNCHED. SECTOR 7 ASSIGNED.` |
| S-019 | Power reserves drop below 5 | `> POWER RESERVES CRITICAL. MINING RUN REQUIRED.` |
| S-020 | Autonomous ship holding | `> AUTONOMOUS UNIT HOLDING. POWER INSUFFICIENT.` |
| S-021 | Autonomous ship resumes | `> AUTONOMOUS UNIT DISPATCHED.` |
| S-022 | Unknown vessel detected (future) | `> UNKNOWN VESSEL. BEARING 312. HAILING OPEN.` |
| S-023 | Trader docks (future) | `> TRADER DOCKED. EXCHANGE AVAILABLE.` |

---

## 5. Signal Strip Implementation Spec

### 5.1 Display
- Always visible — full stop
- Position: bottom of screen, full width
- Height: sufficient for 2 lines of text at EGUI_SCALE 3.0
- Background: near-black, slight transparency — `Color::rgba(0.05, 0.05, 0.05, 0.85)`
- Text color: dim green — `#00CC66` — terminal aesthetic
- Font: FiraSans-Bold (existing project font)
- Prefix: `>` on every line

### 5.2 Behavior
- **When docked:** shows last 3 signal lines, scrolling upward as new lines arrive
- **When flying:** shows last 2 signal lines, ambient only
- **New line arrival:** previous line shifts up, new line appears at bottom
- **No interaction:** Signal strip is read-only, never tappable
- **No duplicate lines:** each trigger ID fires exactly once per session

### 5.3 Data Structure
```rust
#[derive(Resource)]
struct SignalLog {
    entries: VecDeque<String>,  // max 10 stored
    fired: HashSet<u32>,        // trigger IDs already fired — prevents duplicates
}
```

### 5.4 Signal System
```rust
fn signal_system(
    mut signal: ResMut<SignalLog>,
    // queries for trigger conditions
) {
    // Check each trigger condition
    // If condition met and ID not in fired set:
    //   Push line to signal.entries
    //   Insert ID into signal.fired
    //   Trim entries to max 10
}
```

---

## 6. Opening Sequence — Automatic Autopilot

The opening sequence requires the ship to move automatically toward the station before the player touches the screen. This is a one-time startup behavior:

```rust
#[derive(Resource)]
struct OpeningSequence {
    phase: OpeningPhase,
    timer: f32,
}

enum OpeningPhase {
    Adrift,           // waiting — show S-001
    SignalIdentified, // 2s timer — show S-002
    AutoPiloting,     // ship moving to station — show S-003
    InRange,          // station visible — show S-004
    Docked,           // auto-docked — show S-005, S-006, S-007
    Complete,         // player has control — normal game
}
```

After `OpeningPhase::Complete` the opening sequence resource is no longer needed. Player has full control from that point.

---

## 7. Future Signal Sources

The Signal architecture supports multiple sources speaking in the same voice. Source is tracked internally but not displayed — the player always sees one unified Signal strip.

| Source | When Active | Examples |
|--------|-------------|---------|
| Ship Systems | Always | Power warnings, arrival confirmations |
| Station AI | After station online | Resource alerts, discovery reports |
| Trader | When trader in range | Exchange available, departure imminent |
| Beacon | Future sectors | Distress signal, ore density report |
| Unknown | Deep game | Encrypted transmission, unidentified origin |

The Signal is the game's spine. It grows with the game without changing its voice.

---

## 8. Content Locked in This Document

The following are locked and cannot be changed without a design session:

- Signal voice rules (§2)
- Opening sequence lines and order (§3)
- Trigger condition table S-001 through S-021 (§4)
- Signal strip visual spec (§5.1)
- Signal strip behavior rules (§5.2)
- `SignalLog` data structure (§5.3)
- `OpeningSequence` state machine (§6)

---

*Voidrift Signal Narrative Design Document | April 2026 | RFD IT Services Ltd.*  
*The Signal is always on. The Signal is always true. The Signal does not explain — it reports.*
