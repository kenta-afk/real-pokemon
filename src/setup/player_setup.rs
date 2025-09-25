use bevy::prelude::*;

use crate::components::*;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("gabe-idle-run.png");
    let layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(24),
        7,
        4,
        None,
        None,
    ));

    commands.spawn((
        Sprite {
            image: texture,
            texture_atlas: Some(TextureAtlas {
                layout: layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)).with_scale(Vec3::splat(3.0)),
        Runner::default(),
        Animation::new(0, 6), // Running animation frames 0-6
        Player,
    ));
}
