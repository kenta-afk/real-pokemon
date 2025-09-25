use bevy::prelude::*;

use crate::{components::*, setup::background_setup::BackgroundLayer};

pub fn background_system(
    camera_query: Query<&Transform, (With<Camera2d>, Without<BackgroundLayer>)>,
    mut background_query: Query<
        &mut Transform,
        (With<BackgroundLayer>, With<Background>, Without<Camera2d>),
    >,
) {
    // Fixed background system - background follows camera to appear stationary in screen space
    if let Ok(camera_transform) = camera_query.get_single() {
        for mut bg_transform in &mut background_query {
            // Move background with camera to keep it stationary in screen space
            bg_transform.translation.x = camera_transform.translation.x;
            bg_transform.translation.y = camera_transform.translation.y;
            // Keep original z-position for layering
        }
    }
}
