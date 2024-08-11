use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorGrabMode,
};

#[derive(Component)]
struct MouseMove {
    dx: f32,
    dy: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, add_mouse)
        .add_systems(Update, set_mouse)
        .add_systems(Update, print_mouse)
        //.add_systems(Update, grab_mouse)
        //.add_systems(Update, print_mouse_events_system)
        .run();
}

fn add_mouse(mut commands: Commands) {
    commands.spawn(MouseMove { dx: 0., dy: 0. });
}

fn set_mouse(mut query: Query<&mut MouseMove>, mut mouse_motion_events: EventReader<MouseMotion>) {
    let (dx, dy) = {
        let (mut dx, mut dy) = (0., 0.);

        for event in mouse_motion_events.read() {
            dx += event.delta.x;
            dy += event.delta.y;
        }

        (dx, dy)
    };
    if let Ok(mut mouse_move) = query.get_single_mut() {
        mouse_move.dx = dx;
        mouse_move.dy = dy;
    }
}

fn print_mouse(query: Query<&MouseMove>) {
    if let Ok(mouse_move) = query.get_single() {
        info!("{}, {}", mouse_move.dx, mouse_move.dy);
    }
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
    }

    for event in mouse_motion_events.read() {
        info!("{:?}", event);
    }

    for event in cursor_moved_events.read() {
        info!("{:?}", event);
    }

    for event in mouse_wheel_events.read() {
        info!("{:?}", event);
    }
}
