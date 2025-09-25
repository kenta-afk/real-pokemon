use bevy::prelude::*;

use crate::components::*;

pub fn update_ui_display(
    runner_query: Query<&Runner, With<Player>>,
    mut score_text_query: Query<&mut Text, (With<ScoreText>, Without<DistanceText>)>,
    mut distance_text_query: Query<&mut Text, (With<DistanceText>, Without<ScoreText>)>,
) {
    if let Ok(runner) = runner_query.single() {
        // Update Score Text
        if let Ok(mut text) = score_text_query.single_mut() {
            *text = Text::new(format!("Score: {}", runner.score));
        }

        // Update Distance Text
        if let Ok(mut text) = distance_text_query.single_mut() {
            *text = Text::new(format!("Distance: {:.1}m", runner.distance / 100.0));
        }
    }
}
