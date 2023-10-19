#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings       globals
#import bevy_render::view                       View
#import "shaders/distance_field_shapes.wgsl"    smoothen

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;
    let t = globals.time * 0.75;

    let p0 = vec2(cos(t) * 0.3 + 0.5, 0.5);
    let p1 = vec2(-cos(t) * 0.3 + 0.5, 0.5);
    let d = smoothen(distance(uv, p0) * 5.0, distance(uv, p1) * 5.0);
    let ae = 5.0 / view.viewport.w;
    let color = vec3(smoothstep(0.8, 0.8 + ae, d));

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(color, 1.0);
}