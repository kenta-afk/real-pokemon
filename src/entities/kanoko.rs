use bevy::{audio::AudioSource, prelude::*};
/// System for setting up the world environment (trees, buildings, etc.)
pub const KANOKO_RECT: (Vec2, Vec2) = (Vec2::new(0.0, 0.0), Vec2::new(256.0, 256.0));

#[derive(Resource, Default, PartialEq, Eq)]
pub enum Area {
    #[default]
    Other,
    KanokoTown,
}

#[derive(Resource)]
pub struct KanokoBgmHandle(pub Handle<AudioSource>);

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct KanokoBgmTag;
