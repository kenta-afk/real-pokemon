use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Runner {
    pub speed: f32,
    pub distance: f32,
    pub score: u64,
    pub distance_multiplier: f32,
    pub auto_speed_boost: f32,
}

impl Default for Runner {
    fn default() -> Self {
        Self {
            speed: 200.0,
            distance: 0.0,
            score: 0,
            distance_multiplier: 1.0,
            auto_speed_boost: 1.0,
        }
    }
}
