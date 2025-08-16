use bevy::prelude::*;

use crate::entities::animation::{AnimationConfig, Character, Direction};

pub fn setup_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let tex = asset_server.load("gabe-idle-run.png");
    let layout = atlases.add(TextureAtlasLayout::from_grid(
        UVec2::splat(24),
        7,
        4,
        None,
        None,
    ));

    let right = AnimationConfig::new(0, 6, 10);
    let left = AnimationConfig::new(7, 13, 10);
    let back = AnimationConfig::new(14, 16, 10);
    let front = AnimationConfig::new(21, 25, 10);
    let idle = AnimationConfig::new(0, 0, 1);

    let chara = Character {
        move_right_config: right.clone(),
        move_left_config: left.clone(),
        move_backward_config: back.clone(),
        move_forward_config: front.clone(),
        idle_config: idle.clone(),
        current_direction: Direction::Idle,
        is_moving: false,
    };

    commands.spawn((
        Sprite {
            image: tex,
            texture_atlas: Some(TextureAtlas {
                layout,
                index: idle.first_sprite_index,
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(2.0)).with_translation(Vec3::new(0.0, -100.0, 0.0)),
        idle,
        chara,
    ));
}
