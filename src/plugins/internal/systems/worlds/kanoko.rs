use bevy::prelude::*;

use crate::plugins::internal::systems::components::obstacle::Obstacle;

/// System for setting up the world environment (trees, buildings, etc.)
pub fn setup_kanoko(mut commands: Commands, asset_server: Res<AssetServer>) {
    // ヘルパー関数: 汎用的なスプライト生成
    fn spawn_sprite(
        commands: &mut Commands,
        image: Handle<Image>,
        size: Option<Vec2>,
        translation: Vec3,
        z: f32,
    ) {
        commands.spawn((
            Sprite {
                image,
                custom_size: size,
                ..default()
            },
            Transform {
                translation: Vec3::new(translation.x, translation.y, z),
                ..default()
            },
        ));
    }

    // sea, grass の配置座標リスト
    let sea_texture = asset_server.load("utils/sea.png");
    let sea_size = Vec2::new(1200.0, 800.0);
    let sea_positions = [
        Vec3::new(-1200.0, -800.0, 0.0),
        Vec3::new(0.0, -800.0, 0.0),
        Vec3::new(1200.0, -800.0, 0.0),
    ];
    for pos in sea_positions.iter() {
        spawn_sprite(
            &mut commands,
            sea_texture.clone(),
            Some(sea_size),
            *pos,
            -12.0,
        );
    }

    let grass_texture = asset_server.load("utils/grass.png");
    let grass_size = Vec2::new(1200.0, 800.0);
    let grass_positions = [Vec3::new(-1200.0, 0.0, 0.0), Vec3::new(1200.0, 0.0, 0.0)];
    for pos in grass_positions.iter() {
        spawn_sprite(
            &mut commands,
            grass_texture.clone(),
            Some(grass_size),
            *pos,
            -11.0,
        );
    }

    // メイン背景・フェンスもspawn_spriteで
    spawn_sprite(
        &mut commands,
        asset_server.load("kanoko_town/kanoko.png"),
        Some(Vec2::new(1200.0, 800.0)),
        Vec3::ZERO,
        -10.0,
    );
    spawn_sprite(
        &mut commands,
        asset_server.load("kanoko_town/fence.png"),
        Some(Vec2::new(1050.0, 600.0)),
        Vec3::new(0.0, -382.0, 0.0),
        -9.0,
    );

    // house配置もループでまとめる
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
                translation: Vec3::new(pos.x, pos.y, -8.0),
                ..default()
            },
            Obstacle {
                size: house_obstacle,
            },
        ));
    }

    // tree配置もまとめる
    let tree_texture = asset_server.load("utils/tree.png");
    let mut tree_positions = Vec::new();
    // Left grass area trees
    for x in (-1800..=-600).step_by(50) {
        let mut z_depth = -1.0;
        for y in (-350..=400).step_by(50) {
            tree_positions.push(Vec3::new(x as f32, y as f32, z_depth));
            z_depth -= 0.1;
        }
    }
    // Right grass area trees
    for x in (600..=1800).step_by(50) {
        let mut z_depth = -1.0;
        for y in (-350..=400).step_by(50) {
            tree_positions.push(Vec3::new(x as f32, y as f32, z_depth));
            z_depth -= 0.1;
        }
    }
    // Left/Right boundary trees
    let mut z_depth = -1.0;
    for y in (-350..=400).step_by(50) {
        tree_positions.push(Vec3::new(-550.0, y as f32, z_depth));
        z_depth -= 0.1;
    }
    for y in (-350..=400).step_by(50) {
        tree_positions.push(Vec3::new(550.0, y as f32, z_depth));
        z_depth -= 0.1;
    }
    // tree spawnもループで
    for position in tree_positions {
        commands.spawn((
            Sprite {
                image: tree_texture.clone(),
                ..default()
            },
            Transform {
                translation: position,
                scale: Vec3::splat(4.0),
                ..default()
            },
            Obstacle {
                size: Vec2::new(20.0, 20.0),
            },
        ));
    }
}
