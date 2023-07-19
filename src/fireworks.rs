use bevy::prelude::*;

use crate::{physics::Velocity, rng::SeededRng};

pub(crate) struct FireworksPlugin;

impl Plugin for FireworksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShellTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
            .add_systems(Update, launch)
            .add_systems(Update, explode);
    }
}

#[derive(Component)]
pub(crate) struct FireworkScheduled {
    delay: f32,
    shell: FireworkShell,
    velocity: Velocity,
    position: Vec3,
}

impl FireworkScheduled {
    pub fn new(
        delay: f32,
        streamer_speed: f32,
        color: FireworkColor,
        class: FireworkClass,
        velocity: Velocity,
        position: Vec3,
    ) -> Self {
        Self {
            delay,
            velocity,
            position,
            shell: FireworkShell {
                streamer_speed,
                color,
                class,
            },
        }
    }
}

#[derive(Component, Clone)]
struct FireworkShell {
    streamer_speed: f32,
    color: FireworkColor,
    class: FireworkClass,
}

#[derive(Clone, Copy)]
pub(crate) enum FireworkColor {
    Blue,
    Green,
    Red,
    White,
}

impl FireworkColor {
    fn get_material(&self) -> StandardMaterial {
        use FireworkColor::*;
        StandardMaterial {
            emissive: match self {
                Blue => Color::rgb(0.2, 0.2, 2.0),
                Green => Color::rgb(0.2, 2.0, 0.2),
                Red => Color::rgb(2.0, 0.2, 0.2),
                White => Color::rgb(2.0, 2.0, 2.0),
            },
            ..default()
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) enum FireworkClass {
    Streamer,
    Sphere,
}

impl FireworkClass {
    fn get_lifetime(&self) -> f32 {
        use FireworkClass::*;
        match self {
            Streamer => 2.0,
            Sphere => 1.0,
        }
    }

    fn get_streamer_count(&self) -> usize {
        use FireworkClass::*;
        match self {
            Streamer => 100,
            Sphere => 100,
        }
    }

    fn get_streamer_lifetime(&self) -> f32 {
        use FireworkClass::*;
        match self {
            Streamer => 4.0,
            Sphere => 4.0,
        }
    }

    fn get_mesh(&self, meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        use FireworkClass::*;

        let shape = match self {
            Streamer => shape::Icosphere {
                radius: 0.5,
                subdivisions: 5,
            },
            Sphere => shape::Icosphere {
                radius: 0.5,
                subdivisions: 5,
            },
        };
        meshes.add(shape.try_into().unwrap())
    }

    fn get_explosion(
        &self,
        color: FireworkColor,
        base_velocity: Velocity,
        base_position: Vec3,
        start_speed: f32,
        mut meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
        rng: &mut ResMut<SeededRng>,
    ) -> Vec<(PbrBundle, FireworkStreamer, Velocity)> {
        // TODO: create random remaining_life
        (0..self.get_streamer_count())
            .map(|_| {
                (
                    PbrBundle {
                        mesh: self.get_mesh(&mut meshes),
                        material: materials.add(color.get_material()),
                        transform: Transform::from_translation(base_position.clone()),
                        ..Default::default()
                    },
                    FireworkStreamer {
                        color,
                        class: self.clone(),
                        remaining_life: self.get_streamer_lifetime(),
                    },
                    base_velocity.clone().add(rng.sample_sphere() * start_speed),
                )
            })
            .collect()
    }
}

#[derive(Component)]
struct FireworkStreamer {
    color: FireworkColor,
    class: FireworkClass,
    remaining_life: f32,
}

#[derive(Resource)]
struct ShellTimer(Timer);

fn launch(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    query: Query<(Entity, &mut FireworkScheduled)>,
) {
    for (entity, schedule) in query.iter() {
        if time.elapsed_seconds() > schedule.delay {
            commands.entity(entity).despawn();
            let pbr = PbrBundle {
                mesh: meshes.add(
                    shape::Icosphere {
                        radius: 0.5,
                        subdivisions: 5,
                    }
                    .try_into()
                    .unwrap(),
                ),
                material: materials.add(Color::rgb(0.3, 0.1, 0.1).into()),
                transform: Transform::from_translation(schedule.position.clone()),
                ..Default::default()
            };
            commands.spawn((pbr, schedule.shell.clone(), schedule.velocity.clone()));
        }
    }
}

fn explode(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rng: ResMut<SeededRng>,
    query: Query<(Entity, &mut FireworkShell, &Transform, &Velocity)>,
) {
    for (entity, shell, transform, velocity) in query.iter() {
        if time.elapsed_seconds() > shell.class.get_lifetime() {
            commands.entity(entity).despawn();
            commands.spawn_batch(shell.class.get_explosion(
                shell.color,
                velocity.clone(),
                transform.translation,
                shell.streamer_speed,
                &mut meshes,
                &mut materials,
                &mut rng,
            ));
        }
    }
}

// fn addition(
//     mut commands: Commands,
//     time: Res<Time>,
//     mut timer: ResMut<ShellTimer>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     if timer.0.tick(time.delta()).just_finished() {
//         let material_emissive1 = materials.add(StandardMaterial {
//             emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
//             ..default()
//         });
//         let mesh = meshes.add(
//             shape::Icosphere {
//                 radius: 0.5,
//                 subdivisions: 5,
//             }
//             .try_into()
//             .unwrap(),
//         );
//         commands.spawn((
//             PbrBundle {
//                 mesh,
//                 material: material_emissive1,
//                 transform: Transform::from_xyz(1.0, 0.2, 0.0),
//                 ..default()
//             },
//             FireworkShell {
//                 velocity: Vec3::new(
//                     f32::sin(time.elapsed_seconds()) * 0.2,
//                     0.11,
//                     f32::sin(time.elapsed_seconds()) * 0.04,
//                 ),
//                 color: FireworkColor::Blue,
//                 streamer_speed: 1.0,
//                 class: FireworkClass::Streamer,
//             },
//         ));
//     }
// }
