use bevy::prelude::*;

use super::components::animation::{AnimationConfig, Character, Direction};

// This system changes the character's direction and animation when arrow keys are pressed
pub fn change_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Character, &mut AnimationConfig, &mut Sprite)>
) {
    for (mut character, mut animation_config, mut sprite) in &mut query {
        let mut new_direction = None;
        let mut new_config = None;

        // Check if any movement key is pressed
        if input.pressed(KeyCode::ArrowRight) {
            new_direction = Some(Direction::Right);
            new_config = Some(character.move_right_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowLeft) {
            new_direction = Some(Direction::Left);
            new_config = Some(character.move_left_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowUp) {
            new_direction = Some(Direction::Forward);
            new_config = Some(character.move_forward_config.clone());
            character.is_moving = true;
        } else if input.pressed(KeyCode::ArrowDown) {
            new_direction = Some(Direction::Backward);
            new_config = Some(character.move_backward_config.clone());
            character.is_moving = true;
        } else {
            // No keys pressed - go to idle
            if character.is_moving {
                new_direction = Some(Direction::Idle);
                // Set idle frame based on the last direction
                let idle_config = match character.current_direction {
                    Direction::Right => AnimationConfig::new(0, 0, 1),
                    Direction::Left => AnimationConfig::new(7, 7, 1),
                    Direction::Backward => AnimationConfig::new(14, 14, 1),
                    Direction::Forward => AnimationConfig::new(21, 21, 1),
                    Direction::Idle => character.idle_config.clone()
                };
                new_config = Some(idle_config);
                character.is_moving = false;
            }
        }

        // Update direction and animation config if changed
        if let (Some(direction), Some(config)) = (new_direction, new_config)
            && character.current_direction != direction
        {
            character.current_direction = direction;
            *animation_config = config;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = animation_config.first_sprite_index;
            }
        }
    }
}

// This system handles character movement
pub fn move_character(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Character>>
) {
    for mut transform in &mut query {
        let movement_speed = 200.0; // pixels per second
        let delta_time = time.delta().as_secs_f32();

        if input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += movement_speed * delta_time;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= movement_speed * delta_time;
        }
        if input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement_speed * delta_time;
        }
        if input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement_speed * delta_time;
        }

        // Optional: Add boundaries to keep character on screen
        transform.translation.x = transform.translation.x.clamp(-500.0, 500.0);
        transform.translation.y = transform.translation.y.clamp(-280.0, 350.0); // Adjusted for fence collision
    }
}

// This system makes the camera follow the character
pub fn camera_follow(
    character_query: Query<&Transform, (With<Character>, Without<Camera2d>)>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Character>)>
) {
    // Get the first character (main player)
    if let Some(character_transform) = character_query.iter().next() {
        for mut camera_transform in camera_query.iter_mut() {
            // Smoothly follow the character
            let target_position = character_transform.translation;
            let lerp_factor = 0.1; // Adjust this for smoother/faster following

            camera_transform.translation.x = camera_transform.translation.x
                + (target_position.x - camera_transform.translation.x) * lerp_factor;
            camera_transform.translation.y = camera_transform.translation.y
                + (target_position.y - camera_transform.translation.y) * lerp_factor;

            // Keep camera's Z position unchanged for proper layering
            camera_transform.translation.z = 0.0;
        }
    }
}

// This system loops through all the sprites in the `TextureAtlas`, from  `first_sprite_index` to
// `last_sprite_index` (both defined in `AnimationConfig`).
pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &Character)>
) {
    for (mut config, mut sprite, character) in &mut query {
        // We track how long the current sprite has been displayed for
        config.frame_timer.tick(time.delta());

        // If it has been displayed for the user-defined amount of time (fps)...
        if config.frame_timer.just_finished()
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            if atlas.index == config.last_sprite_index {
                // If we're moving, loop the animation, otherwise stay on first frame
                if character.is_moving {
                    atlas.index = config.first_sprite_index;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            } else {
                // ...and it is NOT the last frame, then we move to the next frame...
                atlas.index += 1;
                // ...and reset the frame timer to start counting all over again
                config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
            }
        }
    }
}

pub fn setup_characters(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // Load the sprite sheet using the `AssetServer`
    let texture = asset_server.load("gabe-idle-run.png");

    // The sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 7, 4, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    // The first (left-hand) sprite runs at 10 FPS
    let move_right_sprite = AnimationConfig::new(0, 6, 10);
    let move_left_sprite = AnimationConfig::new(7, 13, 10);
    let move_backward_sprite = AnimationConfig::new(14, 16, 10);
    let move_forward_sprite = AnimationConfig::new(21, 25, 10);
    let idle_sprite = AnimationConfig::new(0, 0, 1);

    // Create a single character that can move in both directions
    let character = Character {
        move_right_config: move_right_sprite,
        move_left_config: move_left_sprite,
        move_backward_config: move_backward_sprite,
        move_forward_config: move_forward_sprite,
        idle_config: idle_sprite.clone(),
        current_direction: Direction::Idle,
        is_moving: false
    };

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: idle_sprite.first_sprite_index
            }),
            ..default()
        },
        Transform::from_scale(Vec3::splat(3.0)).with_translation(Vec3::new(0.0, 0.0, 0.0)),
        idle_sprite,
        character
    ));
}
