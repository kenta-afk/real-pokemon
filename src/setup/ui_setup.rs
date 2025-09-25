use bevy::prelude::*;

use crate::components::*;

pub fn setup_ui(mut commands: Commands) {
    // UI Root
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|parent| {
            // Score Display
            parent
                .spawn(Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Score: 0"),
                        TextFont {
                            font_size: 32.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScoreText,
                    ));
                });

            // Distance Display
            parent
                .spawn(Node {
                    margin: UiRect::bottom(Val::Px(10.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Distance: 0.0m"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        DistanceText,
                    ));
                });

            // Instructions
            parent
                .spawn(Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Keep running to earn score!\nPress J to move forward manually!"),
                        TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
        });
}
