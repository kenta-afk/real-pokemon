use bevy::prelude::*;

use super::components::animation::{AnimationConfig, Character, Direction};
use super::components::obstacle::Obstacle;

type ObstacleQuery<'w, 's> =
    Query<'w, 's, (&'static Transform, &'static Obstacle), (Without<Character>, With<Obstacle>)>;

fn check_collision(pos1: Vec3, size1: Vec2, pos2: Vec3, size2: Vec2) -> bool {
    let (h1, h2) = (size1 / 2.0, size2 / 2.0);
    pos1.x - h1.x < pos2.x + h2.x
        && pos1.x + h1.x > pos2.x - h2.x
        && pos1.y - h1.y < pos2.y + h2.y
        && pos1.y + h1.y > pos2.y - h2.y
}

pub fn change_direction(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Character, &mut AnimationConfig, &mut Sprite)>,
) {
    for (mut chara, mut anim_cfg, mut sprite) in &mut query {
        let (dir, cfg) = match (
            input.pressed(KeyCode::ArrowRight),
            input.pressed(KeyCode::ArrowLeft),
            input.pressed(KeyCode::ArrowUp),
            input.pressed(KeyCode::ArrowDown),
        ) {
            (true, _, _, _) => (Direction::Right, chara.move_right_config.clone()),
            (_, true, _, _) => (Direction::Left, chara.move_left_config.clone()),
            (_, _, true, _) => (Direction::Forward, chara.move_forward_config.clone()),
            (_, _, _, true) => (Direction::Backward, chara.move_backward_config.clone()),
            _ => {
                if chara.is_moving {
                    chara.is_moving = false;
                    let idle = match chara.current_direction {
                        Direction::Right => AnimationConfig::new(0, 0, 1),
                        Direction::Left => AnimationConfig::new(7, 7, 1),
                        Direction::Backward => AnimationConfig::new(14, 14, 1),
                        Direction::Forward => AnimationConfig::new(21, 21, 1),
                        Direction::Idle => chara.idle_config.clone(),
                    };
                    (Direction::Idle, idle)
                } else {
                    continue;
                }
            }
        };

        if chara.current_direction != dir {
            chara.current_direction = dir;
            chara.is_moving = dir != Direction::Idle;
            *anim_cfg = cfg;
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = anim_cfg.first_sprite_index;
            }
        }
    }
}

pub fn move_character(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut q_char: Query<&mut Transform, With<Character>>,
    obstacles: ObstacleQuery,
) {
    let delta = time.delta_secs();
    let speed = 150.0;
    let size = Vec2::splat(20.0);

    for mut transform in &mut q_char {
        let mut new_pos = transform.translation;

        if input.pressed(KeyCode::ArrowRight) {
            new_pos.x += speed * delta;
        }
        if input.pressed(KeyCode::ArrowLeft) {
            new_pos.x -= speed * delta;
        }
        if input.pressed(KeyCode::ArrowUp) {
            new_pos.y += speed * delta;
        }
        if input.pressed(KeyCode::ArrowDown) {
            new_pos.y -= speed * delta;
        }

        let collision = obstacles
            .iter()
            .any(|(obs_tf, obs)| check_collision(new_pos, size, obs_tf.translation, obs.size));

        if !collision {
            transform.translation = new_pos;
        }

        transform.translation.x = transform.translation.x.clamp(-500.0, 500.0);
        transform.translation.y = transform.translation.y.clamp(-280.0, 350.0);
    }
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

pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite, &Character)>,
) {
    for (mut cfg, mut sprite, chara) in &mut query {
        cfg.frame_timer.tick(time.delta());

        if !cfg.frame_timer.just_finished() {
            continue;
        }

        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.index = match (atlas.index == cfg.last_sprite_index, chara.is_moving) {
                (true, true) => cfg.first_sprite_index,
                (true, false) => atlas.index,
                (false, _) => atlas.index + 1,
            };
            cfg.frame_timer = AnimationConfig::timer_from_fps(cfg.fps);
        }
    }
}
