use bevy::prelude::*;

use crate::entities::obstacle::Obstacle;

pub fn setup_fence(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko_town/fence.png"),
            custom_size: Some(Vec2::new(1150.0, 600.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -382.0, -9.0),
            ..default()
        },
        Obstacle {
            size: Vec2::new(1150.0, 100.0),
        },
    ));
}
