#import bevy_pbr::mesh_vertex_output        MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings   globals
#import bevy_render::view                   View

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    let resolution = view.viewport.zw;  // z = width, w = height
    var uv = in.uv.xy;
    uv.x *= resolution.x / resolution.y;

    let texture = textureSample(color_texture, color_sampler, in.uv);
    let blue_wave = (0.5 * sin(globals.time)) + 0.5;
    return texture * vec4(uv.x, uv.y, blue_wave, 1.0);
}