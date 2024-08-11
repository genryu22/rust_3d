use bevy::prelude::*;

mod input;

use input::add_mouse;
use input::set_mouse;
use input::MouseInput;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, add_mouse)
        .add_systems(Update, set_mouse)
        .add_systems(Update, rotate_camera)
        .add_systems(Update, move_camera)
        .add_systems(Update, pan_camera)
        .add_systems(Update, draw_cursor)
        .run();
}

fn rotate_camera(
    mouse_query: Query<&MouseInput>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mouse_input) = mouse_query.get_single() {
        if mouse_input.left_pressed {
            let mut camera_transform = camera_query.single_mut();
            camera_transform.rotate_local_x(mouse_input.dy * 0.001);
            camera_transform.rotate_y(mouse_input.dx * 0.001);
        }
    }
}

fn move_camera(
    mouse_query: Query<&MouseInput>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mouse_input) = mouse_query.get_single() {
        if mouse_input.right_pressed {
            let mut camera_transform = camera_query.single_mut();
            let right = mouse_input.dx * camera_transform.left() * 0.01;
            let up = mouse_input.dy
                * (camera_transform.forward().as_vec3()
                    - camera_transform.forward().dot(Dir3::Y.as_vec3()) * Dir3::Y)
                * 0.01;
            camera_transform.translation += right + up;
        }
    }
}

fn pan_camera(
    mouse_query: Query<&MouseInput>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    if let Ok(mouse_input) = mouse_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        let forward = camera_transform.forward() * mouse_input.wheel * 1.;
        camera_transform.translation += forward;
    }
}

fn draw_cursor(
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ground_query: Query<&GlobalTransform, With<Ground>>,
    windows: Query<&Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = camera_query.single();
    let ground = ground_query.single();

    let Some(cursor_position) = windows.single().cursor_position() else {
        return;
    };

    // Calculate a ray pointing from the camera into the world based on the cursor's position.
    let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
        return;
    };

    // Calculate if and where the ray is hitting the ground plane.
    let Some(distance) =
        ray.intersect_plane(ground.translation(), InfinitePlane3d::new(ground.up()))
    else {
        return;
    };
    let point = ray.get_point(distance);

    // Draw a circle just above the ground plane at that position.
    gizmos.circle(point + ground.up() * 0.01, ground.up(), 0.2, Color::WHITE);
}

#[derive(Component)]
struct Ground;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));

    // light
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::ONE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 5.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
