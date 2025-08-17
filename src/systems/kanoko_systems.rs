use bevy::{
    audio::{AudioPlayer, PlaybackSettings, Volume},
    prelude::*,
};

use crate::entities::{
    area::{Area, KANOKO_RECT, Player},
    bgm::{BgmHandles, BgmTag, BgmType},
    obstacle::Obstacle,
};

pub fn setup_kanoko(mut commands: Commands, asset_server: Res<AssetServer>) {
    // BGMリソース初期化
    let kanoko_bgm = asset_server.load("kanoko_town/output.ogg");
    commands.insert_resource(BgmHandles {
        kanoko_town: kanoko_bgm,
    });
    commands.insert_resource(Area::Other);
    // プレイヤーの初期位置（仮）
    commands.spawn((
        Player,
        Transform::from_xyz(100.0, 100.0, 0.0),
        GlobalTransform::default(),
    ));

    // sea, grass の配置座標リスト
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

    // メイン背景・フェンスも直接spawn
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
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko_town/fence.png"),
            custom_size: Some(Vec2::new(1050.0, 600.0)),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -382.0, -9.0),
            ..default()
        },
    ));
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
                translation: Vec3::new(pos.x, pos.y, -9.0),
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

pub fn kanoko_bgm_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    bgm_handles: Res<BgmHandles>,
    mut area: ResMut<Area>,
    audio_players: Query<Entity, (With<AudioPlayer>, With<BgmTag>)>,
) {
    let player_pos = player_query.iter().next().map(|t| t.translation.truncate());
    let in_kanoko = player_pos.is_some_and(is_in_kanoko);
    match (in_kanoko, &*area) {
        (true, Area::Other) => {
            // カノコタウンに入ったのでBGM再生
            commands.spawn((
                AudioPlayer(bgm_handles.kanoko_town.clone()),
                PlaybackSettings {
                    mode: bevy::audio::PlaybackMode::Loop,
                    volume: Volume::Linear(1.0),
                    ..default()
                },
                BgmTag::new(BgmType::KanokoTown),
            ));
            *area = Area::KanokoTown;
        }
        (false, Area::KanokoTown) => {
            // カノコタウンから出たのでBGM停止
            // 特定のBGMタイプのみ停止することも可能
            for e in audio_players.iter() {
                commands.entity(e).despawn();
            }
            *area = Area::Other;
        }
        _ => {}
    }
}

fn is_in_kanoko(pos: Vec2) -> bool {
    let (min, max) = KANOKO_RECT;
    pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
}
