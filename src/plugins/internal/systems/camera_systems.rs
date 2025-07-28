use bevy::prelude::*;

/// System for setting up the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.5)), // Zoom in by scaling camera
    ));
}
