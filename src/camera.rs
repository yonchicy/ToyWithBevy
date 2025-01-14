use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::VolumetricFog,
    prelude::*,
};

use crate::constants;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow_player);
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct GameCamera;

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Game Camera"),
            GameCamera,
            Transform::from_xyz(
                0.0,
                7.,
                constants::PLANET_RADIUS + constants::CAMERA_DISTANCE,
            )
            .looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
            Camera3d { ..default() },
            Camera {
                hdr: true,
                ..default()
            },
        ))
        .insert(Tonemapping::TonyMcMapface)
        .insert(Bloom::NATURAL)
        .insert(VolumetricFog {
            // This value is explicitly set to 0 since we have no environment map light
            ambient_intensity: 0.0,
            ..default()
        });
}
fn camera_follow_player(
    mut query: Query<&mut Transform, With<GameCamera>>,
    player_query: Query<(&Transform, &Player), Without<GameCamera>>,
) {
    for mut camera_transform in query.iter_mut() {
        for (player_transform, player) in player_query.iter() {
            *camera_transform = Transform::from_translation(
                player_transform.translation.normalize()
                    * (constants::PLANET_RADIUS + constants::CAMERA_DISTANCE),
            )
            .looking_at(player_transform.translation, player.up);
        }
    }
}
