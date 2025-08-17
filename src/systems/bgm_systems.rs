use bevy::{
    audio::{AudioPlayer, PlaybackSettings, Volume},
    prelude::*,
};

use crate::entities::{
    area::{Area, Player},
    bgm::{BgmHandles, BgmTag, BgmType},
};

/// BGMを管理する汎用システム
pub fn bgm_control_system(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    bgm_handles: Res<BgmHandles>,
    mut area: ResMut<Area>,
    audio_players: Query<Entity, (With<AudioPlayer>, With<BgmTag>)>,
) {
    let player_pos = player_query.iter().next().map(|t| t.translation.truncate());
    let current_area = determine_area_from_position(player_pos);

    if current_area != *area {
        // エリアが変わったので現在のBGMを停止
        stop_all_bgm(&mut commands, &audio_players);

        // 新しいエリアのBGMを再生
        play_bgm_for_area(&mut commands, &bgm_handles, &current_area);

        *area = current_area;
    }
}

/// BGMリソースを初期化する
pub fn setup_bgm_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let kanoko_bgm = asset_server.load("kanoko_town/output.ogg");
    // 将来的に他のBGMも追加
    // let masara_bgm = asset_server.load("masara_town/bgm.ogg");

    commands.insert_resource(BgmHandles {
        kanoko_town: kanoko_bgm,
        // masara_town: masara_bgm,
    });
}

/// 初期BGMを再生する（プレイヤーの位置を確認してから）
pub fn start_initial_bgm(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    bgm_handles: Res<BgmHandles>,
    mut area: ResMut<Area>,
) {
    // プレイヤーの現在位置を取得
    if let Some(player_transform) = player_query.iter().next() {
        let player_pos = player_transform.translation.truncate();
        let current_area = Area::from_position(player_pos);

        // 現在のエリアに応じてBGMを再生
        play_bgm_for_area(&mut commands, &bgm_handles, &current_area);

        // エリア状態を更新
        *area = current_area;
    }
}

/// プレイヤーの位置からエリアを判定する
fn determine_area_from_position(pos: Option<Vec2>) -> Area {
    match pos {
        Some(pos) => Area::from_position(pos),
        None => Area::Other,
    }
}

/// エリアに応じてBGMを再生する
fn play_bgm_for_area(commands: &mut Commands, bgm_handles: &BgmHandles, area: &Area) {
    match area {
        Area::KanokoTown => {
            spawn_bgm(
                commands,
                bgm_handles.kanoko_town.clone(),
                BgmType::KanokoTown,
            );
        }
        Area::Other => {
            // Other エリアではBGMなし（または将来的にデフォルトBGM）
        }
    }
}

/// BGMエンティティを生成する汎用関数
fn spawn_bgm(
    commands: &mut Commands,
    audio_handle: Handle<bevy::audio::AudioSource>,
    bgm_type: BgmType,
) {
    commands.spawn((
        AudioPlayer(audio_handle),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: Volume::Linear(1.0),
            ..default()
        },
        BgmTag::new(bgm_type),
    ));
}

/// 全てのBGMを停止する汎用関数
fn stop_all_bgm(
    commands: &mut Commands,
    audio_players: &Query<Entity, (With<AudioPlayer>, With<BgmTag>)>,
) {
    for entity in audio_players.iter() {
        commands.entity(entity).despawn();
    }
}
