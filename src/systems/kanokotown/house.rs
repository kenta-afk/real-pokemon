use bevy::prelude::*;

use crate::entities::obstacle::Obstacle;

/// カノコタウンの家々を配置するシステム
pub fn setup_houses(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let house_texture = asset_server.load("kanoko_town/house.png");
    let house_size = Vec2::new(300.0, 300.0);
    let house_obstacle = Vec2::new(140.0, 140.0);
    let house_positions = [
        Vec3::new(40.0, 70.0, 0.0),
        Vec3::new(-150.0, -180.0, 0.0),
        Vec3::new(200.0, -180.0, 0.0),
    ];

    for pos in house_positions.iter() {
        commands.spawn((
            Sprite {
                image: house_texture.clone(),
                custom_size: Some(house_size),
                ..default()
            },
            Transform {
                translation: Vec3::new(pos.x, pos.y, -9.0),
                ..default()
            },
            Obstacle {
                size: house_obstacle,
            },
        ));
    }
}
