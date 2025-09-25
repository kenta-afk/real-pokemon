use bevy::prelude::*;

#[derive(Component)]
pub struct Animation {
    pub start_index: usize,
    pub end_index: usize,
    pub current_index: usize,
    pub timer: Timer,
}

impl Animation {
    pub fn new(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
            current_index: start_index,
            timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        }
    }
}
