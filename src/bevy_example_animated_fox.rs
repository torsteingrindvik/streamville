use std::f32::consts::PI;

use bevy::{
    pbr::CascadeShadowConfigBuilder,
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};

pub struct BevyExampleAnimatedFoxPlugin {
    pub resolution: UVec2,
}

#[derive(Debug, Resource, Deref, DerefMut)]
pub struct FoxRenderTarget(Handle<Image>);

#[derive(Debug, Resource, Deref, DerefMut)]
struct FoxRenderTargetSize(UVec2);

impl Plugin for BevyExampleAnimatedFoxPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoxRenderTargetSize(self.resolution))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (setup_scene_once_loaded, assign_render_layers_to_scene),
            );
    }
}

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Component)]
struct Fox;

const FOX_RENDER_LAYER: u8 = 10;

fn setup(
    mut commands: Commands,
    render_target_size: Res<FoxRenderTargetSize>,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    commands.insert_resource(Animations(vec![
        asset_server.load("models/animated/Fox.glb#Animation2"), // Running!
    ]));

    // TODO: Consolidate layers
    let fox_layer = RenderLayers::layer(FOX_RENDER_LAYER);

    let target = crate::render_util::make_image(**render_target_size, &mut images);

    // Camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                // Early render
                // TODO: Consolidate camera render orders
                order: -100000,
                target: RenderTarget::Image(target.clone_weak()),
                ..default()
            },
            transform: Transform::from_xyz(100.0, 100.0, 150.0)
                .looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
            ..default()
        },
        fox_layer,
    ));
    commands.insert_resource(FoxRenderTarget(target));

    commands.spawn((
        DirectionalLightBundle {
            transform: Transform::from_rotation(Quat::from_euler(
                EulerRot::ZYX,
                0.0,
                1.0,
                -PI / 4.,
            )),
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 200.0,
                maximum_distance: 400.0,
                ..default()
            }
            .into(),
            ..default()
        },
        fox_layer,
    ));

    // Fox
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/animated/Fox.glb#Scene0"),
            ..default()
        },
        Fox,
    ));
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in &mut players {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

// Why not just put render layers directly on the scene bundle?
// Because bug: https://github.com/bevyengine/bevy/issues/12461
// So we do it ourselves here
fn assign_render_layers_to_scene(
    mut finished: Local<bool>,
    mut commands: Commands,
    query: Query<Entity, (With<Fox>, With<Handle<Scene>>)>,
    children_query: Query<&Children>,
) {
    if *finished {
        return;
    };

    let Ok(scene_root) = query.get_single() else {
        return;
    };

    for descendant in children_query.iter_descendants(scene_root) {
        *finished = true;
        commands
            .entity(descendant)
            .insert(RenderLayers::layer(FOX_RENDER_LAYER));
    }
}
