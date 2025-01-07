use bevy::prelude::*;

mod camera;
mod constants;
mod maths;
mod menu;
mod planet;
mod player;

use camera::CameraPlugin;
use planet::PlanetPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((PlanetPlugin, CameraPlugin))
        .run();
}
