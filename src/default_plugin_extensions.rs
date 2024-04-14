use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    window::{Cursor, WindowResolution},
};

pub trait DefaultPluginExtensions {
    /// A decoration-less transparent window always on top at a given position
    fn with_transparent_window_at(self, pos: IVec2) -> PluginGroupBuilder
    where
        Self: Sized,
    {
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                transparent: true,
                decorations: false,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                // TODO: Would be better expressed in a normalized size
                position: WindowPosition::At(pos),
                ..default()
            }),
            ..default()
        })
    }

    /// Sets window plugin to be:
    /// - Hit test false (but doesn't work on X11)
    /// - Transparent
    /// - No decorations
    /// - Always on top
    /// - Borderless fullscreen
    /// - No scale factor (helps make physical resolution match mouse position coords)
    fn with_transparent_fullscreen_window(self) -> PluginGroupBuilder
    where
        Self: Sized,
    {
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                cursor: Cursor {
                    // BUG: Doesn't work on X11, seems to be a bug in winit
                    hit_test: false,
                    ..default()
                },
                transparent: true,
                decorations: false,
                window_level: bevy::window::WindowLevel::AlwaysOnTop,
                mode: bevy::window::WindowMode::BorderlessFullscreen,
                resolution: WindowResolution::default().with_scale_factor_override(1.0),
                ..default()
            }),
            ..default()
        })
    }
}

impl DefaultPluginExtensions for DefaultPlugins {}
