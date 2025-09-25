use bevy::prelude::*;

use crate::components::*;

pub fn click_runner_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Runner), With<Player>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let delta = time.delta_secs();

        for (mut transform, mut runner) in &mut query {
            // Calculate effective speed with boosts
            let effective_speed = runner.speed * runner.auto_speed_boost;

            // Move the player to the right when clicked
            let movement_distance = effective_speed * 0.1; // Fixed movement per click
            transform.translation.x += movement_distance;

            // Update distance and score with multipliers
            let distance_gained = movement_distance * runner.distance_multiplier;
            runner.distance += distance_gained;

            // Score increases based on distance with better scaling
            runner.score = (runner.distance * 10.0) as u64;

            // Small boost for clicking
            runner.auto_speed_boost += 0.01;
        }
    }
}

pub fn manual_movement_system(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Runner), With<Player>>,
) {
    if let Ok((mut transform, mut runner)) = query.single_mut() {
        let movement_speed = 300.0;
        let delta = time.delta_secs();

        if input.pressed(KeyCode::KeyJ) {
            let movement_distance = movement_speed * delta;
            transform.translation.x += movement_distance;

            // Update distance and score
            let distance_gained = movement_distance * runner.distance_multiplier;
            runner.distance += distance_gained;
            runner.score = (runner.distance * 10.0) as u64;
        }
    }
}

pub fn speed_boost_system(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Runner, With<Player>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for mut runner in &mut query {
            // Temporary speed boost when spacebar is pressed
            runner.distance_multiplier = 2.0;
            runner.auto_speed_boost += 0.1; // Permanent small boost
        }
    }

    if input.just_released(KeyCode::Space) {
        for mut runner in &mut query {
            // Reset multiplier when spacebar is released
            runner.distance_multiplier = 1.0;
        }
    }
}
