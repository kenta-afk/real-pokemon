use bevy::prelude::*;

use crate::components::*;

pub fn camera_follow_system(
    player_query: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    if let (Ok(player_transform), Ok(mut camera_transform)) =
        (player_query.single(), camera_query.single_mut())
    {
        // Follow the player with some offset
        camera_transform.translation.x = player_transform.translation.x + 200.0;
        camera_transform.translation.y = player_transform.translation.y;
    }
}
