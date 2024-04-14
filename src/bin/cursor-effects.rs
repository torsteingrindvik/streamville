use std::ops::DerefMut;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::close_on_esc,
};
use streamville::prelude::*;

const DEBUG: bool = false;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.with_transparent_fullscreen_window())
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins((
            WorldAxesGizmoPlugin,
            Material2dPlugin::<MouseMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                close_on_esc,
                spawn_on_move,
                despawn,
                debug_2d_positions.run_if(|| DEBUG),
            ),
        )
        .run();
}

fn despawn(mut commands: Commands, time: Res<Time>, to_despawn: Query<(Entity, &SpawnedAt)>) {
    let now = time.elapsed_seconds();
    for (entity, spawned) in &to_despawn {
        if now - spawned.elapsed > 2.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Debug, Component)]
struct MainCamara;

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamara));
}

#[derive(Debug, Resource, Component, Clone, Copy)]
struct SpawnedAt {
    /// Position spawned at
    position: Vec2,

    /// App time elapsed at time of spawning
    elapsed: f32,
}

fn debug_2d_positions(
    window: Query<&Window>,
    mut pos: Local<(f32, f32)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MouseMaterial>>,
) {
    let window = window.single();

    let max_x = window.width() / 2.;
    let max_y = window.height() / 2.;

    let increment = 20.0;
    let (x, y) = pos.deref_mut();
    let translation = Vec3::new(*x - increment / 2., *y - increment / 2., 0.0);

    if (*x + increment) > max_x {
        *x = 0.0;
        *y += increment;
    } else {
        *x += increment;
    }

    if *y > max_y {
        *y = 0.0;
    }

    // info!("Spawning at: {translation:?}");
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default()
            .with_scale(Vec3::splat(increment))
            .with_translation(translation),
        material: materials.add(MouseMaterial {
            alpha: 0.1,
            depth_bias: 0.0,
        }),
        ..default()
    });
}

fn spawn_on_move(
    window: Query<&Window>,
    time: Res<Time>,
    last_spawn: Option<ResMut<SpawnedAt>>,
    mut depth_bias: Local<f32>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MouseMaterial>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    // These might not be necessary
    const DEBOUNCE: f32 = 0.0; // seconds
    const MIN_LENGTH: f32 = 0.0; // pixels

    if cursor_moved_events.is_empty() {
        return;
    }

    if let Some(e) = cursor_moved_events.read().last() {
        let elapsed = time.elapsed_seconds();
        let position = e.position;

        let new_spawn = match last_spawn {
            None => {
                let new = SpawnedAt { position, elapsed };
                commands.insert_resource(new);
                new
            }
            Some(mut last) => {
                let time_since_last = elapsed - last.elapsed;
                let length_from_last = (-position + last.position).length();

                debug!("Since last: {time_since_last:.2}, length from last: {length_from_last:.2}");

                if time_since_last < DEBOUNCE || length_from_last < MIN_LENGTH {
                    return;
                }

                *last = SpawnedAt { position, elapsed };
                *last
            }
        };

        let size = 128.0;

        let window_height = window.single().physical_height() as f32;
        let window_width = window.single().physical_width() as f32;

        let translation = Vec3::new(
            new_spawn.position.x - (window_width / 2.),
            -new_spawn.position.y + (window_height / 2.),
            0.0,
        );
        debug!("Spawning at: {translation:?}");

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(Rectangle::default()).into(),
                transform: Transform::default()
                    .with_scale(Vec3::splat(size))
                    .with_translation(translation),
                material: materials.add(MouseMaterial {
                    alpha: 0.25,
                    depth_bias: *depth_bias,
                }),
                ..default()
            },
            new_spawn,
        ));

        *depth_bias += 0.01;
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct MouseMaterial {
    #[uniform(0)]
    alpha: f32,

    // Used to avoid overlapping rectangles depth fighting
    depth_bias: f32,
}

impl Material2d for MouseMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/mouse_material.wgsl".into()
    }

    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }
}
