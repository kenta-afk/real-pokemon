use bevy::prelude::*;

use crate::components::*;

pub fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Animation, &mut Sprite), With<Player>>,
) {
    for (mut animation, mut sprite) in &mut query {
        animation.timer.tick(time.delta());

        if animation.timer.just_finished() {
            animation.current_index += 1;
            if animation.current_index > animation.end_index {
                animation.current_index = animation.start_index;
            }

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation.current_index;
            }
        }
    }
}
