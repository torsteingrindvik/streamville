pub mod blender_cam;
pub mod world_axes_gizmo;

pub mod bevy_example_animated_fox;

pub mod render_util;

pub mod prelude {
    pub use super::{
        bevy_example_animated_fox::BevyExampleAnimatedFoxPlugin,
        default_plugin_extensions::DefaultPluginExtensions, world_axes_gizmo::WorldAxesGizmoPlugin,
    };
}

pub mod default_plugin_extensions;
