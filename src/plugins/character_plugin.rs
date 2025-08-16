use bevy::prelude::*;

use crate::systems::{
    animation_systems::{change_direction, execute_animations, move_character},
    character_systems::setup_characters,
};
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_characters).add_systems(
            Update,
            (execute_animations, change_direction, move_character),
        );
    }
}
