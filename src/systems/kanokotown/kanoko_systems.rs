use bevy::prelude::*;

use super::{fence, grass, house, kanoko_background, research_institute, sea, trees};
use crate::entities::area::{Area, Player};

pub fn setup_kanoko(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Area::KanokoTown);

    commands.spawn((
        Player,
        Transform::from_xyz(100.0, 100.0, 0.0),
        GlobalTransform::default(),
    ));

    sea::setup_sea(&mut commands, &asset_server);
    grass::setup_grass(&mut commands, &asset_server);
    kanoko_background::setup_kanoko_background(&mut commands, &asset_server);
    fence::setup_fence(&mut commands, &asset_server);
    research_institute::setup_research_institute(&mut commands, &asset_server);
    house::setup_houses(&mut commands, &asset_server);
    trees::setup_trees(&mut commands, &asset_server);
}
