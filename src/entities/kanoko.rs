use bevy::{audio::AudioSource, prelude::*};
/// System for setting up the world environment (trees, buildings, etc.)
pub const KANOKO_RECT: (Vec2, Vec2) = (Vec2::new(0.0, 0.0), Vec2::new(256.0, 256.0));

#[derive(Resource, Default, PartialEq, Eq)]
pub enum Area {
    #[default]
    Other,
    KanokoTown,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BgmType {
    KanokoTown,
    // 将来追加予定: MasaraTown, Route1, Battle, etc.
}

#[derive(Resource)]
pub struct BgmHandles {
    pub kanoko_town: Handle<AudioSource>,
    // 将来追加予定: pub masara_town: Handle<AudioSource>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct BgmTag(BgmType);

impl BgmTag {
    pub fn new(bgm_type: BgmType) -> Self {
        Self(bgm_type)
    }
}
