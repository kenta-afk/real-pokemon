use bevy::prelude::*;
mod components;
mod resources;
mod setup;
mod systems;

use setup::*;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::srgb(1.0, 0.6, 0.3))) // Sunset orange background
        .add_systems(
            Startup,
            (
                setup_camera,
                setup_player,
                setup_background,
                setup_bgm,
                setup_ui,
            ),
        )
        .add_systems(
            Update,
            (
                click_runner_system,
                manual_movement_system,
                animate_player,
                camera_follow_system,
                background_system,
                update_ui_display,
                speed_boost_system,
            ),
        )
        .run();
}
