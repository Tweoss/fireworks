use std::time::Duration;

use bevy::prelude::*;

pub(crate) struct FireworksPlugin;

impl Plugin for FireworksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShellTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
            .add_systems(Startup, setup)
            .add_systems(Update, (movement, addition));
    }
}

#[derive(Component)]
struct FireworkShell {
    velocity: Vec3,
}

#[derive(Resource)]
struct ShellTimer(Timer);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    commands.spawn((
        PbrBundle {
            mesh: mesh.clone(),
            material: material_emissive1,
            transform: Transform::from_xyz(1.0, 0.2, 0.0),
            ..default()
        },
        FireworkShell {
            velocity: Vec3::new(0.02, 0.11, 0.04),
        },
    ));
}

fn movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut FireworkShell)>,
) {
    for (entity, mut transform, mut shell) in query.iter_mut() {
        transform.translation += shell.velocity;
        // gravitational constant near surface of earth
        shell.velocity.y -= 9.764 * 1e-4;
        // delete after some time
        if time.elapsed() > Duration::from_secs(9) {
            commands.entity(entity).despawn();
        }
    }
}

fn addition(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<ShellTimer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let material_emissive1 = materials.add(StandardMaterial {
            emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
            ..default()
        });
        let mesh = meshes.add(
            shape::Icosphere {
                radius: 0.5,
                subdivisions: 5,
            }
            .try_into()
            .unwrap(),
        );
        commands.spawn((
            PbrBundle {
                mesh,
                material: material_emissive1,
                transform: Transform::from_xyz(1.0, 0.2, 0.0),
                ..default()
            },
            FireworkShell {
                velocity: Vec3::new(
                    f32::sin(time.elapsed_seconds()) * 0.2,
                    0.11,
                    f32::sin(time.elapsed_seconds()) * 0.04,
                ),
            },
        ));
    }
}
