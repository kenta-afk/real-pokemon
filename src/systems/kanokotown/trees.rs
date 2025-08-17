use bevy::prelude::*;

use crate::entities::obstacle::Obstacle;

/// カノコタウンの木を配置するシステム
pub fn setup_trees(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let tree_texture = asset_server.load("utils/tree.png");
    let tree_positions = generate_tree_positions();

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
                size: Vec2::new(80.0, 30.0),
            },
        ));
    }
}

/// ツリーの配置座標を生成する関数
fn generate_tree_positions() -> Vec<Vec3> {
    let mut positions = Vec::new();
    let mut z_depth = -1.0;

    // 配置エリアの定義
    let tree_areas = [
        // 左側草エリア
        (-1800..=-600, -350..=400),
        // 右側草エリア
        (600..=1800, -350..=400),
    ];

    // 各エリアにツリーを配置
    for (x_range, y_range) in tree_areas {
        for x in x_range.step_by(50) {
            for y in y_range.clone().step_by(50) {
                positions.push(Vec3::new(x as f32, y as f32, z_depth));
                z_depth -= 0.001; // より細かい深度制御
            }
        }
    }

    positions
}
