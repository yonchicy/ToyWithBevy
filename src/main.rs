use bevy::prelude::*;

mod camera;
mod constants;
mod foilage;
mod maths;
mod menu;
mod planet;
mod player;
mod player_stat;

use camera::CameraPlugin;
use foilage::FoilagePlugin;
use planet::PlanetPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlanetPlugin, PlayerPlugin, CameraPlugin, FoilagePlugin))
        .run();
}
