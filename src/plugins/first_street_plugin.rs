use bevy::prelude::*;

use crate::systems::firststreet::first_street_systems::setup_first_street;
pub struct FirstStreetPlugin;

impl Plugin for FirstStreetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_first_street);
    }
}
