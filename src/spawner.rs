use bevy::prelude::*;

use crate::{
    fireworks::{FireworkClass, FireworkColor, FireworkScheduled},
    physics::Velocity,
};

pub(crate) struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    // == START ==
    commands.spawn(FireworkScheduled::new(
        0.0,
        75.0,
        FireworkColor::Red,
        FireworkClass::Sphere,
        Velocity::new(0.02, 186.0, 0.04),
        Vec3::new(2_674.0, 67.0, 2_769.0),
    ));
    commands.spawn(FireworkScheduled::new(
        2.2,
        100.0,
        FireworkColor::White,
        FireworkClass::Sphere,
        Velocity::new(0.02, 263.0, 0.04),
        Vec3::new(2_674.0, 67.0, 2_769.0),
    ));
    commands.spawn(FireworkScheduled::new(
        4.0,
        80.0,
        FireworkColor::Green,
        FireworkClass::Flyer,
        Velocity::new(40.02, 187.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // == MIDDLE ==
    commands.spawn(FireworkScheduled::new(
        6.0,
        100.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(0.02, 187.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    commands.spawn(FireworkScheduled::new(
        7.0,
        300.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(0.02, 263.0, 60.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // two together: red
    commands.spawn(FireworkScheduled::new(
        8.0,
        250.0,
        FireworkColor::Red,
        FireworkClass::Sphere,
        Velocity::new(30.02, 187.0, 10.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // two together: blue
    commands.spawn(FireworkScheduled::new(
        8.0,
        200.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(30.02, 187.0, 10.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    commands.spawn(FireworkScheduled::new(
        12.0,
        100.0,
        FireworkColor::Green,
        FireworkClass::Flyer,
        Velocity::new(0.02, 207.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // == END ==
    // left
    commands.spawn(FireworkScheduled::new(
        17.0,
        250.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(10.02, 187.0, -20.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // right
    commands.spawn(FireworkScheduled::new(
        17.5,
        300.0,
        FireworkColor::Red,
        FireworkClass::Sphere,
        Velocity::new(-30.02, 187.0, 30.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // two large far left: blue
    commands.spawn(FireworkScheduled::new(
        17.8,
        350.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(60.02, 207.0, -40.04),
        Vec3::new(2_774.0, 67.0, 1_769.0),
    ));
    // two large far left: red
    commands.spawn(FireworkScheduled::new(
        17.9,
        330.0,
        FireworkColor::Red,
        FireworkClass::Sphere,
        Velocity::new(60.02, 207.0, -40.04),
        Vec3::new(2_774.0, 67.0, 1_769.0),
    ));
    // two large far right: green
    commands.spawn(FireworkScheduled::new(
        17.95,
        350.0,
        FireworkColor::Green,
        FireworkClass::Flyer,
        Velocity::new(-40.02, 207.0, 60.04),
        Vec3::new(1_774.0, 67.0, 3_269.0),
    ));
    // two large far right: blue
    commands.spawn(FireworkScheduled::new(
        17.95,
        320.0,
        FireworkColor::Green,
        FireworkClass::Sphere,
        Velocity::new(-60.02, 207.0, 60.04),
        Vec3::new(1_474.0, 67.0, 3_769.0),
    ));
    // two middle: top
    commands.spawn(FireworkScheduled::new(
        18.0,
        250.0,
        FireworkColor::Green,
        FireworkClass::Flyer,
        Velocity::new(0.02, 250.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // two middle: bottom
    commands.spawn(FireworkScheduled::new(
        18.6,
        250.0,
        FireworkColor::Green,
        FireworkClass::Flyer,
        Velocity::new(0.02, 207.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
    // large
    commands.spawn(FireworkScheduled::new(
        18.8,
        400.0,
        FireworkColor::Blue,
        FireworkClass::Sphere,
        Velocity::new(0.02, 207.0, 0.04),
        Vec3::new(2_774.0, 67.0, 2_769.0),
    ));
}
