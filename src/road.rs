use std::{fs::File, io::Read};

use bevy::{color::palettes::css, prelude::*};
use rand::{distributions::Uniform, Rng};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::constants;

pub struct RoadPlugin;

impl Plugin for RoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoadResources>()
            .add_systems(Startup, spawn_roads);
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Foilage;

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct RoadResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl RoadResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::PURPLE.into(),
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
            let base_shape = Line3d::new();
            let mesh = meshes.add(::from_size(Vec3::new(1., 1., constants::FOILAGE_HEIGHT)));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

fn spawn_roads(//mut commands: Commands,
    //mut meshes: ResMut<Assets<Mesh>>,
    //mut materials: ResMut<Assets<StandardMaterial>>,
    //mut foilage_resources: ResMut<RoadResources>,
) {
    //let start = Vec3::new(0., 0., constants::PLANET_RADIUS);
    //let mesh = foilage_resources.get_or_create_mesh(&mut meshes);
    //let material = foilage_resources.get_or_create_material(&mut materials);
    //
    //let between = Uniform::from(0..=100);
    //let mut rng = rand::thread_rng();
    //
    //for _ in 0..constants::FOILAGE_COUNT {
    //    let mut pos = Vec3::new(
    //        (rng.sample(between) - 50) as f32 / 100.,
    //        (rng.sample(between) - 50) as f32 / 100.,
    //        (rng.sample(between) - 50) as f32 / 100.,
    //    );
    //    pos = pos.normalize() * constants::PLANET_RADIUS;
    //
    //    commands.spawn((
    //        Name::new("Foilage"),
    //        Foilage,
    //        Mesh3d(mesh.clone()),
    //        MeshMaterial3d(material.clone()),
    //        Transform::from_translation(pos)
    //            .with_rotation(Quat::from_rotation_arc(start.normalize(), pos.normalize())),
    //    ));
    //}
    //
    let nodes = read_path_data().expect("error reading path data");
}

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub lat: f64,
    pub lng: f64,
}

impl Node {
    pub fn new(lat: f64, lng: f64) -> Self {
        Node { lat, lng }
    }
}
fn read_path_data() -> Option<Vec<Node>> {
    let path = "./asserts/paths.json";
    let mut f = File::open(path).expect("file not found");
    let mut buf = String::new();
    f.read_to_string(&mut buf).expect("error reading path data");
    let path_data: Value = serde_json::from_str(&buf).expect("error parsing json");
    let mut path_nodes: Vec<Node> = Vec::new();
    if let Some(nodes) = path_data[0]["path"].as_array() {
        for node in nodes {
            let lat = node[0].as_f64()?;
            let lng = node[1].as_f64()?;
            path_nodes.push(Node::new(lat, lng));
        }
    }
    Some(path_nodes)
}
