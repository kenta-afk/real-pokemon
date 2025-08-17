use bevy::prelude::*;

/// 海のテクスチャとサイズを配置するシステム
pub fn setup_sea(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let sea_texture = asset_server.load("utils/sea.png");
    let sea_size = Vec2::new(1200.0, 800.0);
    let sea_positions = [
        Vec3::new(-1200.0, -800.0, 0.0),
        Vec3::new(0.0, -800.0, 0.0),
        Vec3::new(1200.0, -800.0, 0.0),
    ];

    for pos in sea_positions.iter() {
        commands.spawn((
            Sprite {
                image: sea_texture.clone(),
                custom_size: Some(sea_size),
                ..default()
            },
            Transform {
                translation: Vec3::new(pos.x, pos.y, -11.0),
                ..default()
            },
        ));
    }
}
