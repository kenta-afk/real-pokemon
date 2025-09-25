use bevy::prelude::*;

use crate::components::*;

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create multiple parallax background layers with assets 0.png to 5.png
    // Each layer will have infinite scrolling with different speeds for depth effect
    let background_configs = [
        ("0.png", -500.0, 0.1), // Farthest background (slowest)
        ("1.png", -400.0, 0.2), // Far background
        ("2.png", -300.0, 0.4), // Mid-far background
        ("3.png", -200.0, 0.6), // Mid background
        ("4.png", -100.0, 0.8), // Near background
        ("5.png", -50.0, 1.0),  // Nearest background (fastest)
    ];

    for (image_path, z_pos, speed_multiplier) in background_configs {
        // Create single fixed background for each layer that covers the entire screen
        commands.spawn((
            Sprite {
                image: asset_server.load(image_path),
                custom_size: Some(Vec2::new(1920.0, 1080.0)), // Larger size to prevent any gaps
                ..default()
            },
            Transform::from_translation(Vec3::new(
                0.0, // Center position
                0.0, z_pos,
            )),
            Background { scroll_speed: 0.0 }, // Set scroll speed to 0 for fixed background
            BackgroundLayer,
        ));
    }
}

#[derive(Component)]
pub struct BackgroundLayer;
