use bevy::prelude::*;

pub(crate) struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

// fn setup(mut commands: Commands) {
//     commands.spawn(
//         PbrBundle {
//             mesh: meshes.a
//         }

//     )
// }
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    // // cube
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Cube { size: 10.0 })),
    //         material: materials.add(Color::NONE.into()),
    //         transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //         ..default()
    //     },
    //     // This enables wireframe drawing on this entity
    //     Wireframe,
    // ));
    // // light
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
}
