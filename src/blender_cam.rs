use bevy::{
    input::{common_conditions::input_pressed, mouse::MouseMotion},
    prelude::*,
};

/// Spawns and controls a camera similar to Blender.
/// Has late ordering and a separate window.
///
/// # Controls
///
/// - Mouse wheel scroll is zoom in/out
/// - Mouse wheel hold is yaw on the horizontal axis and pitch on the vertical axis
/// - Mouse wheel hold + shift is panning in the camera's local XY plane
pub struct BlenderCamPlugin;

#[derive(Debug, Component)]
struct BlenderCam;

impl Plugin for BlenderCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_blender_cam).add_systems(
            Update,
            (
                // show_grid, // TODO: Next Bevy version
                wheel_scroll_zoom,
                wheel_hold_yaw_pitch.run_if(
                    input_pressed(MouseButton::Middle)
                        .and_then(not(input_pressed(KeyCode::ShiftLeft))),
                ),
                wheel_hold_pan.run_if(
                    input_pressed(MouseButton::Middle).and_then(input_pressed(KeyCode::ShiftLeft)),
                ),
            ),
        );
    }
}

// fn show_grid(mut gizmos: Gizmos, time: Res<Time>) {
//     gizmos.grid(
//         Vec3::ZERO,
//         Quat::from_rotation_x(PI / 2.),
//         UVec2::splat(20),
//         Vec2::new(2., 2.),
//         // Light gray
//         // LinearRgba::gray(0.65),
//     );
// }

fn wheel_scroll_zoom() {}

// Yaw (local y rotation) on horizontal movement
// Pitch (local x rotation) on vertical movement
fn wheel_hold_yaw_pitch(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut transform: Query<&mut Transform, With<BlenderCam>>,
) {
    // Delta: Window top-left is origin, we generally get +- unit values when we move our mouse
    for MouseMotion { delta } in mouse_motion_events.read() {
        let strength = 0.0025;

        let mut blender_cam_transform = transform.single_mut();

        let (mut yaw, mut pitch, roll) =
            blender_cam_transform.rotation.to_euler(EulerRot::default());

        yaw += delta.x * strength;
        pitch += delta.y * strength;

        blender_cam_transform.rotation = Quat::from_euler(EulerRot::default(), yaw, pitch, roll);
    }
}

fn wheel_hold_pan(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut transform: Query<&mut Transform, With<BlenderCam>>,
) {
    // Delta: Window top-left is origin, we generally get +- unit values when we move our mouse
    for MouseMotion { delta } in mouse_motion_events.read() {
        let strength = 0.01;

        let (local_x, local_y) = { (transform.single().local_x(), transform.single().local_y()) };

        let mut blender_cam_transform = transform.single_mut();

        blender_cam_transform.translation +=
            (local_x * (-delta.x) + local_y * delta.y) * Vec3::splat(strength);
    }
}

fn spawn_blender_cam(mut commands: Commands) {
    let win_id = commands
        .spawn(Window {
            title: "Blender Cam".to_owned(),
            ..default()
        })
        .id();

    let _cam_id = commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
                camera: Camera {
                    order: 1000000, // aka late
                    target: bevy::render::camera::RenderTarget::Window(
                        bevy::window::WindowRef::Entity(win_id),
                    ),
                    ..default()
                },
                ..default()
            },
            BlenderCam,
        ))
        .id();
}
