use std::f32::consts::PI;

use bevy::prelude::*;

pub(crate) struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

const LINES: &[[[f32; 3]; 2]] = &[
    // // left side path
    // [[0.25_f32, 158.0, -0.25], [1.25, 157.0, 0.75]],
    // [[1.25, 157.0, 0.75], [2.25, 156.0, 1.75]],
    // [[2.25, 156.0, 1.75], [3.25, 155.0, 2.75]],
    // // right side path
    // [[-0.25, 158.0, 0.25], [0.75, 157.0, 1.25]],
    // [[0.75, 157.0, 1.25], [1.75, 156.0, 2.25]],
    // [[1.75, 156.0, 2.25], [2.75, 155.0, 3.25]],
];

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // white glowing
    let material_handle = materials.add(StandardMaterial {
        emissive: Color::rgba(1.2, 1.2, 1.2, 0.2).into(),
        ..Default::default()
    });

    let mesh_handle = meshes.add(Mesh::from(shape::Cylinder {
        radius: 0.02,
        height: 1.0,
        ..Default::default()
    }));

    for line in LINES {
        commands.spawn(transform_mesh(
            mesh_handle.clone(),
            material_handle.clone(),
            Vec3::from_array(line[0]),
            Vec3::from_array(line[1]),
        ));
    }
}

// point from p1 to p2
fn transform_mesh(
    // mut commands: Commands,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    p1: Vec3,
    p2: Vec3,
) -> PbrBundle {
    PbrBundle {
        mesh,
        material,
        // we want y (e.g. up) to point towards, p2 - p1
        transform: Transform::from_translation(p1)
            * Transform::default().looking_to(p2 - p1, Vec3::ZERO)
            * Transform::from_rotation(Quat::from_rotation_x(-PI / 2.0))
            * Transform::from_scale(Vec3::new(1.0, (p2 - p1).length(), 1.0))
            * Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    }
}
