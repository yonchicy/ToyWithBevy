use bevy::prelude::*;

use crate::constants;

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PlayerStats {
    pub move_speed: f32,
}

impl Default for PlayerStats {
    fn default() -> Self {
        Self {
            move_speed: constants::PLAYER_DEFAULT_SPEED,
        }
    }
}
