use std::{f32::consts::TAU, time::Duration};

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use streamville::{bevy_example_animated_fox::FoxRenderTarget, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.with_transparent_window_at(IVec2::new(1800, 200)))
        .insert_resource(ClearColor(Color::NONE))
        .insert_resource(Time::<Fixed>::from_duration(Duration::from_millis(5)))
        .add_plugins((
            WorldAxesGizmoPlugin,
            BevyExampleAnimatedFoxPlugin {
                resolution: UVec2 { x: 2048, y: 2048 },
            },
            MaterialPlugin::<BulletMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (spiral_spawner, print_num_bullets))
        .add_systems(
            Update,
            (rotate_camera, move_bullets, fade, destroy.after(fade)),
        )
        .run();
}

#[derive(Debug, Component)]
struct MainCamara;

fn rotate_camera(time: Res<Time>, mut camera: Query<&mut Transform, With<MainCamara>>) {
    let mut cam_transform = camera.single_mut();

    cam_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_axis_angle(Vec3::Y, time.delta_seconds() * TAU / 15.),
    );
}

fn move_bullets(t: Res<Time>, mut bullets: Query<(&mut Transform, &Heading)>) {
    let speed = 0.8;

    for (mut bullet_transform, heading) in &mut bullets {
        bullet_transform.translation += **heading * t.delta_seconds() * speed;
    }
}

fn print_num_bullets(bullets: Query<Entity, With<Bullet>>) {
    debug!("Bullets: {}", bullets.iter().count());
}

/// Destroy when fully transparent
fn destroy(
    mut commands: Commands,

    bullets: Query<(Entity, &Handle<BulletMaterial>), With<Bullet>>,
    materials: Res<Assets<BulletMaterial>>,
) {
    for (entity, material_handle) in &bullets {
        let Some(material) = materials.get(material_handle) else {
            continue;
        };

        if material.alpha <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

/// Fade based on distance
fn fade(
    bullets: Query<(&Transform, &Handle<BulletMaterial>), With<Bullet>>,
    mut materials: ResMut<Assets<BulletMaterial>>,
) {
    for (Transform { translation, .. }, material_handle) in &bullets {
        let Some(material) = materials.get_mut(material_handle) else {
            continue;
        };

        let dist = (3.5 - translation.length()).min(1.);
        material.alpha = dist;
    }
}

/// It's a bullet!
#[derive(Debug, Clone, Copy, Component)]
struct Bullet;

/// Direction headed
#[derive(Debug, Clone, Copy, Deref, DerefMut, Component)]
struct Heading(Direction3d);

/// Spawns spheres (bullets) in a spiral fashion
fn spiral_spawner(
    fox_texture: Res<FoxRenderTarget>,
    mut counter: Local<usize>,
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BulletMaterial>>,
) {
    const BULLETS_PER_REVOLUTION: f32 = 16.0;
    let n = *counter as f32 / BULLETS_PER_REVOLUTION;

    let heading =
        Direction3d::new(Vec3::new(n.cos(), 0.0, n.sin())).expect("should be non-zero length 1");

    let material: BulletMaterial = BulletMaterial {
        color_texture: Some(fox_texture.clone_weak()),
        alpha: 1.0,
    };

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::default()),
            material: materials.add(material),
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::splat(0.1)),
            ..default()
        },
        Heading(heading),
        Bullet,
    ));

    *counter += 1;
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::LIME_GREEN),
        // Rotated into the XZ plane (default in XY)
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    //
    // Bevy is XYZ right handed
    // circular base is at origin
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 6.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamara,
    ));
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct BulletMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<Image>>,

    #[uniform(2)]
    alpha: f32,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl Material for BulletMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/bullet_material.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}
