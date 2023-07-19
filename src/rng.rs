use bevy::prelude::*;
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, UnitSphere};

const RANDOM_CONSTANT: u64 = 12302398701;

pub(crate) struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SeededRng {
            rng: SmallRng::seed_from_u64(RANDOM_CONSTANT),
        });
    }
}

#[derive(Resource)]
pub(crate) struct SeededRng {
    rng: SmallRng,
}

impl SeededRng {
    pub fn sample_sphere(&mut self) -> Vec3 {
        UnitSphere.sample(&mut self.rng).into()
    }
}
