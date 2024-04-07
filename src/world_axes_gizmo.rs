use bevy::{input::common_conditions::input_pressed, prelude::*};

pub struct WorldAxesGizmoPlugin;

impl Plugin for WorldAxesGizmoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            show_axes
                .run_if(input_pressed(KeyCode::ControlLeft).and_then(input_pressed(KeyCode::KeyG))),
        );
    }
}

fn show_axes(mut gizmos: Gizmos) {
    // TODO: Bevy 0.14 - Switch to axes
    gizmos.arrow(Vec3::splat(0.0), Vec3::X, Color::RED);
    gizmos.arrow(Vec3::splat(0.0), Vec3::Y, Color::GREEN);
    gizmos.arrow(Vec3::splat(0.0), Vec3::Z, Color::BLUE);
}
