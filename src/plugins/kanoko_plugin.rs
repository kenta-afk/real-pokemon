use bevy::prelude::*;

use crate::systems::kanoko_systems::setup_kanoko;

pub struct KanokoPlugin;

impl Plugin for KanokoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_kanoko);
    }
}
