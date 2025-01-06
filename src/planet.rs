use crate::constants;
use bevy::{color::palettes::css, pbr::CascadeShadowConfigBuilder, prelude::*};
use std::f32::consts::PI;
pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlanetResources>()
            .add_systems(Startup, spawn_initial_planet);
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PlanetResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl PlanetResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::SANDY_BROWN.into(),
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
            let mesh = meshes.add(Sphere::new(constants::PLANET_RADIUS).mesh().ico(5).unwrap());
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Planet;

fn spawn_initial_planet(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut planet_resources: ResMut<PlanetResources>,
) {
    let mesh = planet_resources.get_or_create_mesh(&mut meshes);
    let material = planet_resources.get_or_create_material(&mut materials);

    // spawn the planet
    commands.spawn((Planet, Mesh3d(mesh), MeshMaterial3d(material)));

    // directional 'sun' light

    commands.spawn((
        Name::new("Light (Sun)"),
        DirectionalLight {
            illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        //CascadeShadowConfigBuilder {
        //    maximum_distance: constants::PLANET_RADIUS * 2.0 + constants::CAMERA_DISTANCE * 2.,
        //    first_cascade_far_bound: 1.0,
        //    ..default()
        //}.into(),
    ));

    //commands.spawn((
    //    Name::new("Light (Sun)"),
    //    DirectionalLight {
    //        illuminance: light_consts::lux::AMBIENT_DAYLIGHT,
    //        shadows_enabled: true,
    //        ..default()
    //    },
    //    Transform {
    //        translation: Vec3::new(0.0, 2.0, 0.0),
    //        rotation: Quat::from_rotation_x(-PI / 4.),
    //        ..default()
    //    },
    //    // The default cascade config is designed to handle large scenes.
    //    // As this example has a much smaller world, we can tighten the shadow
    //    // bounds for better visual quality.
    //    CascadeShadowConfigBuilder {
    //        first_cascade_far_bound: 1.0,
    //        maximum_distance: constants::PLANET_RADIUS * 2.0 + constants::CAMERA_DISTANCE * 2.,
    //        ..default()
    //    }
    //    .into(),
    //    ..default(),
    //));
}
