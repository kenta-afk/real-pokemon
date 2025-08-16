use bevy::prelude::*;

mod entities;
mod plugins;
mod systems;

use plugins::{
    camera_plugin::CameraPlugin, character_plugin::CharacterPlugin, kanoko_plugin::KanokoPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(CharacterPlugin)
        .add_plugins(KanokoPlugin)
        .run();
}
