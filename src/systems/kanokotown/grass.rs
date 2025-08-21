use bevy::prelude::*;

pub fn setup_grass(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let grass_texture = asset_server.load("utils/grass.png");
    let grass_size = Vec2::new(1200.0, 800.0);
    let grass_positions = [Vec3::new(-1200.0, 0.0, 0.0), Vec3::new(1200.0, 0.0, 0.0)];

    for pos in grass_positions.iter() {
        commands.spawn((
            Sprite {
                image: grass_texture.clone(),
                custom_size: Some(grass_size),
                ..default()
            },
            Transform {
                translation: Vec3::new(pos.x, pos.y, -11.0),
                ..default()
            },
        ));
    }
}
