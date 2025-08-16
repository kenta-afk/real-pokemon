use bevy::prelude::*;

use crate::systems::{animation_systems::camera_follow, camera_systems::setup_camera};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_follow);
    }
}
