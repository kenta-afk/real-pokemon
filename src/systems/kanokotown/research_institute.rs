use bevy::prelude::*;

use crate::entities::obstacle::Obstacle;

/// アララギ研究所を配置するシステム
pub fn setup_research_institute(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko_town/araragi's_research_institute.png"),
            custom_size: Some(Vec2::new(600.0, 600.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-250.0, 280.0, -9.0),
            ..default()
        },
        Obstacle {
            size: Vec2::new(300.0, 130.0),
        },
    ));
}
