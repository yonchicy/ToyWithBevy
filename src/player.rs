use bevy::{color::palettes::css, prelude::*};

use crate::{camera::GameCamera, constants, maths::get_angle_for_arc_length};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerResources>()
            .add_systems(Startup, spawn_initial_player)
            .add_systems(Update, (handle_input, setup_new_players));
    }
}

#[derive(Resource, Default, Debug, Reflect)]
#[reflect(Resource, Default, Debug)]
pub struct PlayerResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<StandardMaterial>>,
}

impl PlayerResources {
    pub fn get_or_create_material(
        &mut self,
        materials: &mut Assets<StandardMaterial>,
    ) -> Handle<StandardMaterial> {
        if let Some(ref material) = self.material {
            material.clone()
        } else {
            let material = materials.add(StandardMaterial {
                base_color: css::BLUE.into(),
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
            let mesh = meshes.add(Cuboid::from_length(constants::ENEMY_SIZE));
            self.mesh = Some(mesh.clone());
            mesh
        }
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component, Debug)]
pub struct Player {
    pub up: Vec3,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub player: Player,
    pub transform: Transform,
    pub visible: Visibility,
}

impl PlayerBundle {
    pub fn new() -> Self {
        let up = Vec3::Y;
        Self {
            name: Name::new("Player"),
            player: Player { up },
            transform: Transform::from_translation(Vec3::new(0., 0., constants::PLANET_RADIUS)),
            visible: Visibility::Visible,
        }
    }
}

fn spawn_initial_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::new());
}

fn setup_new_players(
    mut commands: Commands,
    query: Query<(Entity, &Transform), Added<Player>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut player_resources: ResMut<PlayerResources>,
) {
    for (entity, transform) in query.iter() {
        commands.entity(entity).insert(
            (
                Mesh3d(player_resources.get_or_create_mesh(&mut meshes)),
                MeshMaterial3d(player_resources.get_or_create_material(&mut materials)),
            ), //(MaterialMeshBundle {
               //mesh: player_resources.get_or_create_mesh(&mut meshes),
               //material: player_resources.get_or_create_material(&mut materials),
               //transform: *transform,
               //..default()
               //}
               //,)
        );
    }
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Player)>,
    camera_query: Query<&Transform, (With<GameCamera>, Without<Player>)>,
) {
    for (mut transform, mut player) in query.iter_mut() {
        let camera_transform = camera_query.single();

        let angle = get_angle_for_arc_length(stats.move_speed, constants::PLANET_RADIUS)
            * time.delta_seconds();

        // Y movement
        let y = if keys.pressed(KeyCode::ArrowUp) {
            Some(-1.)
        } else if keys.pressed(KeyCode::ArrowDown) {
            Some(1.)
        } else {
            None
        };
        if let Some(y) = y {
            // compute horizontal axis (cross product)
            let camera_up = player.up.normalize();
            let towards_camera = camera_transform.translation.normalize();
            let camera_right = camera_up.cross(towards_camera).normalize();

            // rotate around horizontal axis
            let rot = Quat::from_axis_angle(camera_right, angle * y);
            transform.rotate_around(Vec3::ZERO, rot);

            // compute new up
            player.up = towards_camera.cross(camera_right);
        }

        // X movement
        let x = if keys.pressed(KeyCode::ArrowLeft) {
            Some(-1.)
        } else if keys.pressed(KeyCode::ArrowRight) {
            Some(1.)
        } else {
            None
        };
        if let Some(x) = x {
            // rotate around vertical axis
            let rot = Quat::from_axis_angle(player.up.normalize(), angle * x);
            transform.rotate_around(Vec3::ZERO, rot);
        }
    }
}
