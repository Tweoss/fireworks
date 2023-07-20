use bevy::prelude::*;
use rand_distr::{Distribution, Normal, UnitSphere};

use crate::{physics::Velocity, rng::SeededRng};

pub(crate) struct FireworksPlugin;

impl Plugin for FireworksPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ShellTimer(Timer::from_seconds(0.8, TimerMode::Repeating)))
            .add_systems(Update, (launch, explode, update_streamer, update_trail));
    }
}

#[derive(Component)]
pub(crate) struct FireworkScheduled {
    timer: Timer,
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
            timer: Timer::from_seconds(delay, TimerMode::Once),
            velocity,
            position,
            shell: FireworkShell {
                timer: Timer::from_seconds(FireworkClass::LIFETIME, TimerMode::Once),
                streamer_speed,
                color,
                class,
            },
        }
    }
}

#[derive(Component, Clone)]
struct FireworkShell {
    timer: Timer,
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
        StandardMaterial {
            emissive: self.get_color(),
            ..default()
        }
    }
    fn get_color(&self) -> Color {
        use FireworkColor::*;
        match self {
            Blue => Color::rgb(0.3, 0.3, 3.0),
            Green => Color::rgb(0.3, 3.0, 0.3),
            Red => Color::rgb(3.0, 0.3, 0.3),
            White => Color::rgb(3.0, 3.0, 3.0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub(crate) enum FireworkClass {
    Sphere,
    Flyer,
}

impl FireworkClass {
    const LIFETIME: f32 = 2.0;
    const STREAMER_COUNT: usize = 100;
    const STREAMER_LIFETIME: f32 = 4.0;

    fn get_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
        meshes.add(
            (shape::Icosphere {
                radius: 4.0,
                subdivisions: 5,
            })
            .try_into()
            .unwrap(),
        )
    }

    fn get_trail_interval(&self) -> f32 {
        use FireworkClass::*;
        match self {
            Flyer => 10.0,
            Sphere => 0.3,
        }
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
        // normal around 1.0 with std dev 0.1
        let direction_dist = UnitSphere;
        let speed_dist = Normal::new(1.0, 0.1).unwrap();
        let life_dist = Normal::new(1.0, 0.1).unwrap();
        (0..Self::STREAMER_COUNT)
            .map(|_| {
                (
                    PbrBundle {
                        mesh: Self::get_mesh(&mut meshes),
                        material: materials.add(color.get_material()),
                        transform: Transform::from_translation(base_position.clone()),
                        ..Default::default()
                    },
                    FireworkStreamer {
                        color,
                        class: self.clone(),
                        timer: Timer::from_seconds(
                            Self::STREAMER_LIFETIME * life_dist.sample(rng.as_mut()),
                            TimerMode::Once,
                        ),
                        trail_timer: Timer::from_seconds(
                            self.get_trail_interval(),
                            TimerMode::Repeating,
                        ),
                    },
                    base_velocity.clone().add(
                        Into::<Vec3>::into(direction_dist.sample(rng.as_mut()))
                            * start_speed
                            * speed_dist.sample(rng.as_mut()),
                    ),
                )
            })
            .collect()
    }
}

#[derive(Component)]
struct FireworkStreamer {
    timer: Timer,
    trail_timer: Timer,
    color: FireworkColor,
    class: FireworkClass,
}

#[derive(Resource)]
struct ShellTimer(Timer);

fn launch(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut FireworkScheduled)>,
) {
    for (entity, mut schedule) in query.iter_mut() {
        schedule.timer.tick(time.delta());
        if schedule.timer.just_finished() {
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
                material: materials.add(Color::rgb(0.8, 0.4, 0.4).into()),
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
    mut query: Query<(Entity, &mut FireworkShell, &Transform, &Velocity)>,
) {
    for (entity, mut shell, transform, velocity) in query.iter_mut() {
        shell.timer.tick(time.delta());
        if shell.timer.just_finished() {
            let streamers = shell.class.get_explosion(
                shell.color,
                velocity.clone(),
                transform.translation,
                shell.streamer_speed,
                &mut meshes,
                &mut materials,
                &mut rng,
            );
            commands.spawn_batch(streamers);
            commands.entity(entity).despawn();
        }
    }
}

// a trail that is unaffected by gravity
#[derive(Component)]
struct FireworkTrail {
    timer: Timer,
    emissive: Color,
}

fn update_streamer(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(
        Entity,
        &mut FireworkStreamer,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut rng: ResMut<SeededRng>,
) {
    let offset_dist = UnitSphere;
    for (entity, mut streamer, mut transform, material) in query.iter_mut() {
        streamer.timer.tick(time.delta());
        streamer.trail_timer.tick(time.delta());
        if streamer.timer.just_finished() {
            commands.entity(entity).despawn();
        }

        // flashing that is periodic with remaining time, but flashes faster at end
        if let Some(m) = materials.get_mut(&material) {
            m.emissive = streamer.color.get_color()
                * (0.5 * (f32::sin(1.0 / streamer.timer.remaining_secs()) - 0.5) + 1.0);
        }

        if streamer.class == FireworkClass::Flyer {
            // random walk
            let offset: Vec3 = offset_dist.sample(rng.as_mut()).into();
            transform.translation += offset * 4.0;
        }

        if streamer.trail_timer.just_finished() && streamer.timer.percent_left() < 0.5 {
            // add a trail
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(
                        shape::Icosphere {
                            radius: 3.0,
                            subdivisions: 5,
                        }
                        .try_into()
                        .unwrap(),
                    ),
                    material: materials.add(StandardMaterial {
                        emissive: streamer.color.get_color()
                            * (0.8 + 0.2 * streamer.timer.percent()),
                        ..Default::default()
                    }),
                    transform: Transform::from_translation(transform.translation.clone()),
                    ..Default::default()
                },
                FireworkTrail {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                    emissive: streamer.color.get_color() * (0.8 + 0.2 * streamer.timer.percent()),
                },
            ));
        }
    }
}

fn update_trail(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &mut FireworkTrail, &Handle<StandardMaterial>)>,
) {
    for (entity, mut trail, material) in query.iter_mut() {
        trail.timer.tick(time.delta());
        if trail.timer.just_finished() {
            commands.entity(entity).despawn();
        }

        // linearly decreasing to 0 as time goes on
        if let Some(m) = materials.get_mut(&material) {
            m.emissive = trail.emissive * trail.timer.percent_left();
        }
    }
}
