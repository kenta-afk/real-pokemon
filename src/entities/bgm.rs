use bevy::{audio::AudioSource, prelude::*};

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
pub struct BgmTag(BgmType);

impl BgmTag {
    pub fn new(bgm_type: BgmType) -> Self {
        Self(bgm_type)
    }
}
