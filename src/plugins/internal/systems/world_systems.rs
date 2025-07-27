use bevy::prelude::*;

/// System for setting up the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

/// System for setting up the world environment (trees, buildings, etc.)
pub fn setup_kanoko(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // First, create a background for Kanoko Town
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko.png"),
            custom_size: Some(Vec2::new(1200.0, 800.0)), // Adjust size as needed
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0), // Far background
            ..default()
        },
    ));

    // Load the tree texture
    let tree_texture = asset_server.load("tree.png");
    
    // Spawn trees to create boundaries like Nuvema Town (カノコタウン)
    let mut tree_positions = Vec::new();
    
    // Top boundary - horizontal line of trees (overlapping)
    // for x in (-500..=500).step_by(40) {
    //     tree_positions.push(Vec3::new(x as f32, 250.0, -1.0));
    // }
    
    // Bottom boundary - horizontal line of trees (overlapping)
    // for x in (-500..=500).step_by(40) {
    //     tree_positions.push(Vec3::new(x as f32, -250.0, -1.0));
    // }
    
    // Left boundary - vertical line of trees (overlapping)
    let mut z_depth = -1.0;
    for y in (-300..=300).step_by(50) {
        tree_positions.push(Vec3::new(-550.0, y as f32, z_depth));
        z_depth -= 0.1; // Each subsequent tree goes further back
    }
    
    // Right boundary - vertical line of trees (overlapping)
    for y in (-300..=300).step_by(50) {
        tree_positions.push(Vec3::new(550.0, y as f32, z_depth));
        z_depth -= 0.1; // Each subsequent tree goes further back
    }

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
        ));
    }
}
