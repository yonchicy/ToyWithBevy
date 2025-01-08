use std::f32::consts::PI;

use crate::constants;
use bevy::{color::palettes::css, prelude::*};

pub struct RingPlugin;

impl Plugin for RingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RingResources>()
            .add_systems(Startup, spawn_initial_ring);
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct RingResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl RingResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::DARK_RED.into(),
                ..default()
            });
            self.material = Some(material.clone());
            material
        }
    }

    pub fn get_or_create_mesh(&mut self, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        if let Some(ref mesh) = self.mesh {
            mesh.clone()
        } else {
            // 使用 Extrusion 创建环形网格
            let base_shape = Circle::new(constants::PLANET_RADIUS + 0.1);
            let extrusion = Extrusion {
                base_shape,
                half_depth: constants::RING_HALF_DEPTH,
            };
            let mesh = meshes.add(extrusion.mesh().resolution(64));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Ring;

fn spawn_initial_ring(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ring_resources: ResMut<RingResources>,
) {
    let mesh = ring_resources.get_or_create_mesh(&mut meshes);
    let material = ring_resources.get_or_create_material(&mut materials);

    // spawn the ring
    commands.spawn((Ring, Mesh3d(mesh.clone()), MeshMaterial3d(material.clone())));
    commands.spawn((
        Ring,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_xyz(0., 0., 0.).with_rotation(Quat::from_rotation_y(PI / 2.)),
    ));
}
