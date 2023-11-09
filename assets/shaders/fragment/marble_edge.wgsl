#import bevy_pbr::forward_io::VertexOutput
#import bevy_sprite::mesh2d_view_bindings::globals
#import "shaders/shader_utils.wgsl"::fbm

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: VertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let uv = in.uv.xy;
    let t = (0.5 * sin(globals.time)) + 0.5;

    let v0 = edge(fbm(uv * 18.0), 0.5, 0.0, 0.2) + t;
    let v1 = smoothstep(0.5, 0.51, fbm(uv * 14.0)) + t;
    let v2 = edge(fbm(uv * 14.0), 0.5, 0.0, 0.05) + t;
    let v3 = edge(fbm(uv * 14.0), 0.5, 0.0, 0.25) + t;

    var col = vec3(1.0);
    col -= v0 * 0.75;
    col = mix(col, vec3(0.97), v1);
    col = mix(col, vec3(0.51), v2);
    col -= v3 * 0.2;

    return texture * vec4(col, 1.0);
}

fn edge(v: f32, center: f32, edge0: f32, edge1: f32) -> f32 {
    return 1.0 - smoothstep(edge0, edge1, abs(v - center));
}