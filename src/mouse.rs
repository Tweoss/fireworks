use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

pub(crate) struct MousePlugin;

impl Plugin for MousePlugin {
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
}
