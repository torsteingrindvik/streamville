#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var t: texture_2d<f32>;
@group(2) @binding(1) var s: sampler;
@group(2) @binding(2) var<uniform> alpha: f32;

@fragment
fn fragment(
    bullet: VertexOutput,
) -> @location(0) vec4<f32> {


    // Offset world pos to outside of circular base
    var uv = (vec2(5.) + vec2(bullet.world_position.x, bullet.world_position.z)) * 0.4;

    uv.x = uv.x % 1.0;
    uv.y = uv.y % 1.0;

    let sampled = textureSample(t, s, uv);

    return vec4<f32>(sampled.rgb, alpha);
}
