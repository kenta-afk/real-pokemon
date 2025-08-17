use bevy::prelude::*;

/// エリアの座標範囲定数
pub const KANOKO_RECT: (Vec2, Vec2) = (Vec2::new(0.0, 0.0), Vec2::new(256.0, 256.0));
// 将来追加される他のエリアの座標
// pub const MASARA_RECT: (Vec2, Vec2) = (Vec2::new(-300.0, -300.0), Vec2::new(-50.0, -50.0));

#[derive(Resource, Default, PartialEq, Eq)]
pub enum Area {
    #[default]
    Other,
    KanokoTown,
}

impl Area {
    /// プレイヤーの位置からエリアを判定する汎用メソッド
    pub fn from_position(pos: Vec2) -> Self {
        // 各エリアの座標範囲をチェック
        if Self::is_in_rect(pos, KANOKO_RECT) {
            return Area::KanokoTown;
        }

        // 将来的に他のエリアを追加
        // if Self::is_in_rect(pos, MASARA_RECT) {
        //     return Area::MasaraTown;
        // }

        Area::Other
    }

    /// 座標が指定した矩形範囲内にあるかチェックする汎用メソッド
    fn is_in_rect(pos: Vec2, rect: (Vec2, Vec2)) -> bool {
        let (min, max) = rect;
        pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
    }
}

#[derive(Component)]
pub struct Player;
