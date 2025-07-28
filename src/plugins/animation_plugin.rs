use bevy::prelude::*;

use crate::plugins::internal::systems::animation_systems::{
    camera_follow, change_direction, execute_animations, move_character,
};
use crate::plugins::internal::systems::camera_systems::setup_camera;
use crate::plugins::internal::systems::character_systems::setup_characters;
use crate::plugins::internal::systems::worlds::kanoko::setup_kanoko;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_kanoko, setup_characters))
            .add_systems(
                Update,
                (
                    execute_animations,
                    change_direction,
                    move_character,
                    camera_follow,
                ),
            );
    }
}
