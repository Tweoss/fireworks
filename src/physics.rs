use bevy::prelude::*;

pub(crate) struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}

#[derive(Component, Clone)]
pub(crate) struct Velocity {
    velocity: Vec3,
}

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            velocity: Vec3::new(x, y, z),
        }
    }
    pub fn add(self, velocity: Vec3) -> Self {
        Self {
            velocity: self.velocity + velocity,
        }
    }
}

fn apply_gravity(time: Res<Time>, mut query: Query<(Entity, &mut Transform, &mut Velocity)>) {
    for (_entity, mut transform, mut velocity) in query.iter_mut() {
        transform.translation += velocity.velocity * 5.0 * time.delta_seconds();
        // gravitational constant near surface of earth
        // unfortunately, some constant is incorrect => increasing gravity
        velocity.velocity.y -= 9.806 * 5.0 * time.delta_seconds();
        // drag is proportional to velocity
        velocity.velocity *= 0.99;
    }
}
