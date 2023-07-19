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

fn apply_gravity(mut query: Query<(Entity, &mut Transform, &mut Velocity)>) {
    for (_entity, mut transform, mut velocity) in query.iter_mut() {
        transform.translation += velocity.velocity;
        // gravitational constant near surface of earth
        velocity.velocity.y -= 9.764 * 1e-4;
    }
}
