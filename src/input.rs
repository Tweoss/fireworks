use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (grab, motion));
    }
}

fn grab(mut windows: Query<&mut Window>, mouse: Res<Input<MouseButton>>, key: Res<Input<KeyCode>>) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Confined;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn motion(
    mut motion_events: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>,
    windows: Query<&mut Window>,
    _key: Res<Input<KeyCode>>,
) {
    // make sure cursor is grabbed
    let window = windows.single();
    if window.cursor.grab_mode != CursorGrabMode::Confined || window.cursor.visible == true {
        return;
    }

    let mut camera = query.single_mut();

    for m in motion_events.iter() {
        camera.rotate_y(-m.delta.x * 1e-3);
        camera.rotate_local_x(-m.delta.y * 1e-3);
    }

    // disable movement :)
    // let mut direction = Vec3::ZERO;
    // if key.pressed(KeyCode::W) {
    //     let mut forward = camera.forward();
    //     forward.y = 0.0;
    //     direction += forward;
    // }
    // if key.pressed(KeyCode::A) {
    //     direction += camera.left();
    // }
    // if key.pressed(KeyCode::S) {
    //     let mut back = camera.back();
    //     back.y = 0.0;
    //     direction += back;
    // }
    // if key.pressed(KeyCode::D) {
    //     direction += camera.right();
    // }
    // if key.pressed(KeyCode::Space) {
    //     direction += Vec3::Y;
    // }
    // if key.pressed(KeyCode::ShiftLeft) {
    //     direction -= Vec3::Y;
    // }

    // camera.translation += direction.normalize_or_zero() * 0.4;
}
