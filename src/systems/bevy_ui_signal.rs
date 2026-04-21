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
    
    // STANDALONE TEXT TEST - Separate from signal strip hierarchy
    commands.spawn((
        Text::new("> TEST"),
        TextFont {
            font_size: 80.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.0, 0.0)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(10.0),
            ..default()
        },
        ZIndex(9999),
    ));
    
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
            // Create initial placeholder text entities with proper color
            for _i in 0..3 {
                parent.spawn((
                    Text::new(String::new()),
                    TextFont {
                        font_size: 14.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.0, 0.8, 0.4)), // explicit terminal green
                    SignalEntry,
                ));
            }
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
    expanded: ResMut<SignalStripExpanded>,
    mut strip_query: Query<&mut Node, With<SignalStripRoot>>,
    _container_query: Query<Entity, With<SignalEntryContainer>>,
    mut entry_query: Query<&mut Text, With<SignalEntry>>,
    _commands: Commands,
) {
    eprintln!("SIGNAL_STRIP_SYSTEM_RUNNING");

    // Update strip height based on expanded state
    if let Ok(mut node) = strip_query.get_single_mut() {
        node.height = if expanded.0 { Val::Px(180.0) } else { Val::Px(60.0) };
        println!("[Bevy UI] Strip height updated to: {:?}", node.height);
    } else {
        println!("[Bevy UI] ERROR: No signal strip root found!");
        return;
    }

    // Get current signal entries to display
    let display_count = if expanded.0 { 20 } else { 3 };
    let entries: Vec<String> = signal_log.entries
        .iter()
        .rev()
        .take(display_count)
        .cloned()
        .collect();

    // Update existing text entities with current content
    for (i, mut text) in entry_query.iter_mut().enumerate() {
        if let Some(line) = entries.get(i) {
            **text = line.clone();
        } else {
            **text = String::new();
        }
    }
}
