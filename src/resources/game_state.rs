use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub game_time: f32,
    pub high_score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game_time: 0.0,
            high_score: 0,
        }
    }
}
