use bevy::prelude::*;

/// Component to mark entities as solid obstacles (trees, buildings, etc.)
#[derive(Component)]
pub struct Obstacle {
    pub size: Vec2,
}

/// System for setting up the camera
pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_scale(Vec3::splat(0.5)) // Zoom in by scaling camera
    ));
}

/// System for setting up the world environment (trees, buildings, etc.)
pub fn setup_kanoko(mut commands: Commands, asset_server: Res<AssetServer>) {
    // First, create the sea background (deepest layer)
    let sea_texture = asset_server.load("sea.png");
    let sea_size = Vec2::new(1200.0, 800.0); // Same size as kanoko and grass

    // Place sea tiles only at the bottom (under kanoko and grass areas)
    // Bottom row of sea tiles
    commands.spawn((
        Sprite {
            image: sea_texture.clone(),
            custom_size: Some(sea_size),
            ..default()
        },
        Transform {
            translation: Vec3::new(-1200.0, -800.0, -12.0), // Bottom left (under left grass)
            ..default()
        }
    ));

    commands.spawn((
        Sprite {
            image: sea_texture.clone(),
            custom_size: Some(sea_size),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -800.0, -12.0), // Bottom center (under kanoko)
            ..default()
        }
    ));

    commands.spawn((
        Sprite {
            image: sea_texture.clone(),
            custom_size: Some(sea_size),
            ..default()
        },
        Transform {
            translation: Vec3::new(1200.0, -800.0, -12.0), // Bottom right (under right grass)
            ..default()
        }
    ));

    // Create grass tiles around the main kanoko area to prevent black borders
    let grass_texture = asset_server.load("grass.png");
    let grass_size = Vec2::new(1200.0, 800.0); // Same size as kanoko

    // Place grass tiles around the main area
    // Left grass
    commands.spawn((
        Sprite {
            image: grass_texture.clone(),
            custom_size: Some(grass_size),
            ..default()
        },
        Transform {
            translation: Vec3::new(-1200.0, 0.0, -11.0), // Left of kanoko
            ..default()
        }
    ));

    // Right grass
    commands.spawn((
        Sprite {
            image: grass_texture.clone(),
            custom_size: Some(grass_size),
            ..default()
        },
        Transform {
            translation: Vec3::new(1200.0, 0.0, -11.0), // Right of kanoko
            ..default()
        }
    ));

    // Now create the main background for Kanoko Town
    commands.spawn((
        Sprite {
            image: asset_server.load("kanoko.png"),
            custom_size: Some(Vec2::new(1200.0, 800.0)), // Adjust size as needed
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0), // Far background
            ..default()
        }
    ));

    // Add fence at the bottom of Kanoko Town
    commands.spawn((
        Sprite {
            image: asset_server.load("fence.png"),
            custom_size: Some(Vec2::new(1050.0, 600.0)), // Narrower width to avoid trees, taller height
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, -382.0, -9.0), // Bottom of kanoko area, in front of background
            ..default()
        }
    ));

    let house_texture = asset_server.load("house.png");
    // Add house at the left-upper area of Kanoko Town (like in Pokemon BW)
    commands.spawn((
        Sprite {
            image: house_texture.clone(),
            custom_size: Some(Vec2::new(300.0, 300.0)), // Appropriate house size
            ..default()
        },
        Transform {
            translation: Vec3::new(40.0, 70.0, -8.0), // Left-upper area of kanoko, in front of background
            ..default()
        },
        Obstacle {
            size: Vec2::new(140.0, 140.0), // Even smaller house collision box for tighter movement
        }
    ));

    commands. spawn((
        Sprite {
            image: house_texture.clone(),
            custom_size: Some(Vec2::new(300.0, 300.0)), // Same size as the first house
            ..default()
        },
        Transform {
            translation: Vec3::new(-150.0, -180.0, -8.0), // Another house in the left-upper area
            ..default()
        },
        Obstacle {
            size: Vec2::new(140.0, 140.0), // Smaller collision box for tighter movement
        }
    ));

    commands.spawn((
        Sprite {
            image: house_texture.clone(),
            custom_size: Some(Vec2::new(300.0, 300.0)), // Same size as the first house
            ..default()
        },
        Transform {
            translation: Vec3::new(200.0, -180.0, -8.0), // Another house in the right-upper area
            ..default()
        },
        Obstacle {
            size: Vec2::new(140.0, 140.0), // Smaller collision box for tighter movement
        }
    ));

    // Load the tree texture
    let tree_texture = asset_server.load("tree.png");

    // Spawn trees to create boundaries and also place them on grass areas
    let mut tree_positions = Vec::new();

    // Trees on grass areas (fill the grass areas with trees)
    // Left grass area trees
    for x in (-1800..=-600).step_by(50) {
        let mut z_depth = -1.0;
        for y in (-350..=400).step_by(50) {
            tree_positions.push(Vec3::new(x as f32, y as f32, z_depth));
            z_depth -= 0.1; // Each subsequent tree goes further back
        }
    }

    // Right grass area trees
    for x in (600..=1800).step_by(50) {
        let mut z_depth = -1.0;
        for y in (-350..=400).step_by(50) {
            tree_positions.push(Vec3::new(x as f32, y as f32, z_depth));
            z_depth -= 0.1; // Each subsequent tree goes further back
        }
    }

    // Original boundary trees around the main kanoko area
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
    for y in (-350..=400).step_by(50) {
        tree_positions.push(Vec3::new(-550.0, y as f32, z_depth));
        z_depth -= 0.1; // Each subsequent tree goes further back
    }

    // Right boundary - vertical line of trees (overlapping)
    for y in (-350..=400).step_by(50) {
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
            Obstacle {
                size: Vec2::new(20.0, 20.0), // Smaller tree collision box
            }
        ));
    }
}
