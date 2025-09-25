use bevy::prelude::*;

#[derive(Component)]
pub struct Background {
    pub scroll_speed: f32,
}

impl Default for Background {
    fn default() -> Self {
        Self {
            scroll_speed: 100.0,
        }
    }
}
