use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use fireworks::FireworksPlugin;
use input::InputPlugin;
use path_mesh::MeshPlugin;
use physics::PhysicsPlugin;
use rng::RngPlugin;
use spawner::SpawnerPlugin;

mod fireworks;
mod input;
mod path_mesh;
mod physics;
mod rng;
mod spawner;

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
                SpawnerPlugin,
            ));
    }
}

fn scene_setup(mut commands: Commands) {
    // camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            transform: Transform::from_xyz(0.0, 170.0, 0.0)
                .looking_at(Vec3::new(2_674.0, 157.0, 2_769.0), Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // enable bloom
    ));
}
