use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::CursorGrabMode,
};

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

#[derive(Component)]
pub struct MouseInput {
    pub dx: f32,
    pub dy: f32,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub wheel: f32,
}

pub fn add_mouse(mut commands: Commands) {
    commands.spawn(MouseInput {
        dx: 0.,
        dy: 0.,
        left_pressed: false,
        right_pressed: false,
        wheel: 0.,
    });
}

pub fn set_mouse(
    mut query: Query<&mut MouseInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    let mut mouse_input = query.single_mut();

    let (dx, dy) = {
        let (mut dx, mut dy) = (0., 0.);

        for event in mouse_motion_events.read() {
            dx += event.delta.x;
            dy += event.delta.y;
        }

        (dx, dy)
    };

    let (left_pressed, right_pressed) = {
        let (mut left_pressed, mut right_pressed) =
            (mouse_input.left_pressed, mouse_input.right_pressed);

        for event in mouse_button_input_events.read() {
            match event.state {
                bevy::input::ButtonState::Pressed => match event.button {
                    MouseButton::Left => left_pressed = true,
                    MouseButton::Right => right_pressed = true,
                    _ => (),
                },
                bevy::input::ButtonState::Released => match event.button {
                    MouseButton::Left => left_pressed = false,
                    MouseButton::Right => right_pressed = false,
                    _ => (),
                },
            }
        }

        (left_pressed, right_pressed)
    };

    let wheel = {
        let mut wheel = 0.;

        for event in mouse_wheel_events.read() {
            wheel += event.y;
        }

        wheel
    };

    mouse_input.dx = dx;
    mouse_input.dy = dy;
    mouse_input.left_pressed = left_pressed;
    mouse_input.right_pressed = right_pressed;
    mouse_input.wheel = wheel;
}

fn print_mouse(query: Query<&MouseInput>) {
    if let Ok(mouse_move) = query.get_single() {
        info!(
            "(dx, dy) = ({}, {}), (left, right) = ({}, {}), wheel = {}",
            mouse_move.dx,
            mouse_move.dy,
            mouse_move.left_pressed,
            mouse_move.right_pressed,
            mouse_move.wheel
        );
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
