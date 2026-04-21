use bevy::prelude::*;
use bevy::ui::Interaction;
use crate::components::*;

#[derive(Component)]
pub struct SignalStripRoot;

#[derive(Component)]
pub struct SignalEntryContainer;

#[derive(Component)]
pub struct SignalEntry;

pub fn setup_signal_strip(mut commands: Commands, mut signal_log: ResMut<SignalLog>) {
    // Add test signals to verify text rendering
    signal_log.entries.push_back("> SIGNAL SYSTEM ONLINE".to_string());
    signal_log.entries.push_back("> Bevy UI rendering active".to_string());
    signal_log.entries.push_back("> Terminal green text visible".to_string());
    
    // FINAL SIGNAL STRIP SETUP - Fixed width issue
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(0.0),
            left: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Percent(100.0), // CRITICAL FIX: Explicit width makes strip visible
            height: Val::Px(60.0), // Revert to proper collapsed height
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.78)), // Match egui black with 200 alpha
        ZIndex(1000), // High z-index to render over egui
        Interaction::None, // Enable click interaction
        SignalStripRoot,
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                overflow: Overflow::clip_y(),
                ..default()
            },
            SignalEntryContainer,
        ))
        .with_children(|parent| {
            // Container for signal entries - no initial test text
        });
    });
}

pub fn signal_strip_interaction(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<SignalStripRoot>)>,
    mut expanded: ResMut<SignalStripExpanded>,
) {
    for interaction in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            expanded.0 = !expanded.0;
        }
    }
}

pub fn signal_strip_system(
    signal_log: Res<SignalLog>,
    mut expanded: ResMut<SignalStripExpanded>,
    mut strip_query: Query<&mut Node, With<SignalStripRoot>>,
    container_query: Query<Entity, With<SignalEntryContainer>>,
    entry_query: Query<Entity, With<SignalEntry>>,
    mut commands: Commands,
) {
    // Update strip height based on expanded state
    if let Ok(mut node) = strip_query.get_single_mut() {
        node.height = if expanded.0 { Val::Px(180.0) } else { Val::Px(60.0) };
    } else {
        return;
    }

    // Only update text if signal log has entries and we don't already have text entities
    let entry_count = entry_query.iter().count();
    let signal_count = signal_log.entries.len();
    
    if signal_count > 0 && entry_count == 0 {
        // Create initial text entities
        let display_count = if expanded.0 { 20 } else { 3 };
        let entries: Vec<&String> = signal_log.entries.iter().rev().take(display_count).collect();

        println!("[Bevy UI] Creating {} text entities from {} signal entries", entries.len(), signal_count);

        // Spawn new entries
        if let Ok(container_entity) = container_query.get_single() {
            commands.entity(container_entity).with_children(|parent| {
                for line in entries.iter().rev() {
                    parent.spawn((
                        Text::new((*line).clone()),
                        TextFont {
                            font_size: 14.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.0, 0.8, 0.4)), // explicit terminal green
                        SignalEntry,
                    ));
                }
            });
        }
    } else if signal_count == 0 && entry_count > 0 {
        // Clear text if signal log is empty
        println!("[Bevy UI] Clearing {} text entities - signal log empty", entry_count);
        for entry_entity in &entry_query {
            commands.entity(entry_entity).despawn_recursive();
        }
    }
    // Otherwise, leave existing text entities alone to prevent respawning every frame
}
