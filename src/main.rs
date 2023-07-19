use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use fireworks::{FireworkClass, FireworkColor, FireworkScheduled, FireworksPlugin};
use input::InputPlugin;
use path_mesh::MeshPlugin;
use physics::{PhysicsPlugin, Velocity};
use rng::RngPlugin;

mod fireworks;
mod input;
mod path_mesh;
mod physics;
mod rng;

fn main() {
    App::new().add_plugins(ScenePlugin).run();
}

struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_systems(Startup, scene_setup)
            .add_plugins((DefaultPlugins,))
            .add_plugins((
                FireworksPlugin,
                MeshPlugin,
                InputPlugin,
                RngPlugin,
                PhysicsPlugin,
            ));
    }
}

fn scene_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(25.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(FireworkScheduled::new(
        0.2,
        0.1,
        FireworkColor::Red,
        FireworkClass::Sphere,
        Velocity::new(0.02, 0.11, 0.04),
        Vec3::new(0.0, 0.0, 0.0),
    ));
    commands.spawn(FireworkScheduled::new(
        2.2,
        0.1,
        FireworkColor::White,
        FireworkClass::Sphere,
        Velocity::new(0.02, 0.11, 0.04),
        Vec3::new(0.0, 0.0, 0.0),
    ));
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.2, 0.2).into()),
        transform: Transform::from_xyz(8.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 1.0, 0.2).into()),
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.2, 0.2, 1.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 8.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(30.0, 10.0, 20.0),
            // .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // enable bloom
    ));
}
