use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::config::ContentConfig;

const AMBIENT_MIN_SECS: f32 = 90.0;
const AMBIENT_MAX_SECS: f32 = 120.0;
const SIGNAL_LOG_MAX: usize = 10;

fn push_echo(log: &mut SignalLog, text: &str) {
    log.entries.push_back(format!("ECHO: {}", text));
    while log.entries.len() > SIGNAL_LOG_MAX {
        log.entries.pop_front();
    }
}

/// Fires one-shot and event-pool Echo lines in response to game events.
pub fn content_event_system(
    mut opening_events: EventReader<OpeningCompleteEvent>,
    mut cargo_events: EventReader<ShipDockedWithCargo>,
    mut bottle_events: EventReader<ShipDockedWithBottle>,
    mut dispatch_events: EventReader<DroneDispatched>,
    mut fulfill_events: EventReader<FulfillRequestEvent>,
    queue: Res<ShipQueue>,
    content: Res<ContentConfig>,
    mut state: ResMut<ContentState>,
    mut signal_log: ResMut<SignalLog>,
) {
    let mut rng = rand::thread_rng();

    // ── OpeningCompleteEvent → opening_complete one-shot ─────────────────────
    for _ev in opening_events.read() {
        fire_one_shot("opening_complete", &content, &mut state, &mut signal_log);
    }

    // ── ShipDockedWithCargo → drone_returned pool + first_drone_returned one-shot ─
    let mut any_cargo = false;
    for _ev in cargo_events.read() {
        any_cargo = true;
    }
    if any_cargo {
        // Event pool: drone_returned
        if let Some(pool) = content.event_pools.iter().find(|p| p.trigger == "drone_returned") {
            if rng.gen::<f32>() < pool.chance && !pool.lines.is_empty() {
                let idx = rng.gen_range(0..pool.lines.len());
                push_echo(&mut signal_log, &pool.lines[idx]);
            }
        }
        // One-shot: first_drone_returned
        fire_one_shot("first_drone_returned", &content, &mut state, &mut signal_log);
    }

    // ── DroneDispatched → drone_dispatched pool ───────────────────────────────
    let mut any_dispatch = false;
    for _ev in dispatch_events.read() {
        any_dispatch = true;
    }
    if any_dispatch {
        if let Some(pool) = content.event_pools.iter().find(|p| p.trigger == "drone_dispatched") {
            if rng.gen::<f32>() < pool.chance && !pool.lines.is_empty() {
                let idx = rng.gen_range(0..pool.lines.len());
                push_echo(&mut signal_log, &pool.lines[idx]);
            }
        }
    }

    // ── ShipDockedWithBottle → first_bottle_collected one-shot ───────────────
    let mut any_bottle = false;
    for _ev in bottle_events.read() {
        any_bottle = true;
    }
    if any_bottle {
        state.observed_triggers.insert("first_bottle_collected".to_string());
        fire_one_shot("first_bottle_collected", &content, &mut state, &mut signal_log);
    }

    // ── FulfillRequestEvent → first_quest_fulfilled one-shot ─────────────────
    let mut any_fulfill = false;
    for _ev in fulfill_events.read() {
        any_fulfill = true;
    }
    if any_fulfill {
        state.observed_triggers.insert("first_quest_fulfilled".to_string());
        fire_one_shot("first_quest_fulfilled", &content, &mut state, &mut signal_log);
    }

    // ── drone_count_5: queue filled to capacity ───────────────────────────────
    if queue.available_count >= 5 {
        fire_one_shot("drone_count_5", &content, &mut state, &mut signal_log);
    }
}

/// Fires a random eligible ambient Echo line on a timer.
pub fn content_ambient_system(
    time: Res<Time>,
    content: Res<ContentConfig>,
    mut state: ResMut<ContentState>,
    mut signal_log: ResMut<SignalLog>,
    opening: Res<OpeningSequence>,
) {
    if opening.phase != OpeningPhase::Complete {
        return;
    }

    // Initialise timer on first frame
    if state.ambient_timer <= 0.0 {
        let mut rng = rand::thread_rng();
        state.ambient_timer = rng.gen_range(AMBIENT_MIN_SECS..=AMBIENT_MAX_SECS);
        return;
    }

    state.ambient_timer -= time.delta_secs();
    if state.ambient_timer > 0.0 {
        return;
    }

    // Collect eligible lines (weight-expanded pool)
    let mut pool: Vec<&str> = Vec::new();
    for line in &content.ambient {
        let eligible = match &line.eligible_after {
            Some(trigger) => state.observed_triggers.contains(trigger.as_str()),
            None => true,
        };
        if eligible {
            for _ in 0..line.weight {
                pool.push(&line.text);
            }
        }
    }

    if !pool.is_empty() {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..pool.len());
        push_echo(&mut signal_log, pool[idx]);
    }

    // Reset timer
    let mut rng = rand::thread_rng();
    state.ambient_timer = rng.gen_range(AMBIENT_MIN_SECS..=AMBIENT_MAX_SECS);
}

/// Fires a one-shot by trigger name if not already fired.
fn fire_one_shot(trigger: &str, content: &ContentConfig, state: &mut ContentState, log: &mut SignalLog) {
    if state.fired_one_shots.contains(trigger) {
        return;
    }
    if let Some(entry) = content.one_shots.iter().find(|s| s.trigger == trigger) {
        push_echo(log, &entry.text);
        state.fired_one_shots.insert(trigger.to_string());
    }
}
