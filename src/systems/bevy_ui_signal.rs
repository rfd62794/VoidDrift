use bevy::prelude::*;
use crate::components::*;

#[derive(Component)]
pub struct SignalStripRoot;

#[derive(Component)]
pub struct SignalEntryContainer;

#[derive(Component)]
pub struct SignalEntry;

pub fn setup_signal_strip(mut commands: Commands) {
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
        ));
    });
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

    // Update signal entries
    let display_count = if expanded.0 { 20 } else { 3 };
    let entries: Vec<&String> = signal_log.entries.iter().rev().take(display_count).collect();

    // Clear existing entries
    for entry_entity in &entry_query {
        commands.entity(entry_entity).despawn_recursive();
    }

    // Spawn new entries
    if let Ok(container_entity) = container_query.get_single() {
        commands.entity(container_entity).with_children(|parent| {
            for line in entries.iter().rev() {
                parent.spawn((
                    Text::new((*line).clone()),
                    TextFont {
                        font_size: 11.0,
                        ..default()
                    },
                    TextColor::from(Color::srgb(0.0, 1.0, 0.5)),
                    Node {
                        margin: UiRect::vertical(Val::Px(2.0)),
                        ..default()
                    },
                    SignalEntry,
                ));
            }
        });
    }
}
