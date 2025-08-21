use bevy::prelude::*;

pub fn setup_first_street_background(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("first_street/first_street.png"),
            custom_size: Some(Vec2::new(1200.0, 800.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 800.0, -10.0),
            ..default()
        },
    ));
}
