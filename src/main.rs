use bevy::prelude::*;

mod camera;
mod constants;
mod foilage;
mod maths;
mod menu;
mod planet;
mod player;
mod player_stat;
mod ring;

use camera::CameraPlugin;
use foilage::FoilagePlugin;
use planet::PlanetPlugin;
use player::PlayerPlugin;
use ring::RingPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PlanetPlugin,
            RingPlugin,
            PlayerPlugin,
            CameraPlugin,
            FoilagePlugin,
        ))
        .run();
}
