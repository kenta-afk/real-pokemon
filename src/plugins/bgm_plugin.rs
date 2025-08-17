use bevy::prelude::*;

use crate::systems::bgm_systems::{bgm_control_system, setup_bgm_resources, start_initial_bgm};

pub struct BgmPlugin;

impl Plugin for BgmPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_bgm_resources)
            .add_systems(PostStartup, start_initial_bgm)
            .add_systems(Update, bgm_control_system);
    }
}
