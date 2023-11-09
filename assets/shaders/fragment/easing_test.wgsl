#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/easings.wgsl"::cubicInOut

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
const SPEED = 0.25;

@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let CYAN = vec3(0.0, 1.0, 1.0);
    let YELLOW = vec3(1.0, 1.0, 0.0);

    let pct = cubicInOut(abs(fract(globals.time * SPEED) * 2.0 - 1.0));
    let color = vec3(mix(CYAN, YELLOW, pct));

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(color, 1.0);
}