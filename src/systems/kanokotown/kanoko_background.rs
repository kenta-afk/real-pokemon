use bevy::prelude::*;

/// カノコタウンのメイン背景を配置するシステム
pub fn setup_kanoko_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko_town/kanoko.png"),
            custom_size: Some(Vec2::new(1200.0, 800.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            ..default()
        },
    ));
}
