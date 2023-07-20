use bevy::prelude::*;
use rand::{rngs::SmallRng, RngCore, SeedableRng};

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

impl RngCore for SeededRng {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand::Error> {
        self.rng.try_fill_bytes(dest)
    }
}
