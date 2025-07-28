use bevy::prelude::*;

/// Component to mark entities as solid obstacles (trees, buildings, etc.)
#[derive(Component)]
pub struct Obstacle {
    pub size: Vec2,
}
