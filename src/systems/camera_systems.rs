use bevy::prelude::*;

use crate::entities::animation::Character;

/// System for setting up the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.5)), // Zoom in by scaling camera
    ));
}

pub fn camera_follow(
    q_chara: Query<&Transform, (With<Character>, Without<Camera2d>)>,
    mut q_cam: Query<&mut Transform, (With<Camera2d>, Without<Character>)>,
) {
    if let Some(chara_tf) = q_chara.iter().next() {
        for mut cam_tf in &mut q_cam {
            let target = chara_tf.translation;
            let factor = 0.1;

            cam_tf.translation.x += (target.x - cam_tf.translation.x) * factor;
            cam_tf.translation.y += (target.y - cam_tf.translation.y) * factor;
            cam_tf.translation.z = 0.0;
        }
    }
}
