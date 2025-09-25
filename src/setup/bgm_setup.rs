use bevy::{audio::Volume, prelude::*};

pub fn setup_bgm(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bgm: Handle<AudioSource> = asset_server.load("output.ogg");
    commands.spawn((
        AudioPlayer(bgm),
        PlaybackSettings::LOOP.with_volume(Volume::Linear(0.3)),
    ));
}
