#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_sprite::mesh2d_view_bindings globals

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    let red_wave = (0.5 * sin(globals.time)) + 0.5;
    return textureSample(color_texture, color_sampler, in.uv) * vec4(red_wave, 0.0, 0.0, 1.0);
}