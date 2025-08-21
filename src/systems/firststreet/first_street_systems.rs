use bevy::prelude::*;

use crate::{
    entities::area::Area,
    systems::firststreet::first_street_background::setup_first_street_background,
};

pub fn setup_first_street(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Area::FirstStreet);
    setup_first_street_background(&mut commands, &asset_server);
}
