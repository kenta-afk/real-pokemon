use bevy::prelude::*;
use pokemon::plugins::animation_plugin::AnimationPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) 
        .add_plugins(AnimationPlugin)
        .run();
}
