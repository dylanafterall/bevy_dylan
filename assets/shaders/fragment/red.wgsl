#import bevy_pbr::mesh_vertex_output MeshVertexOutput

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
const COLOR_MULTIPLIER: vec4<f32> = vec4<f32>(1.0, 0.0, 0.0, 0.1);

@fragment
fn fragment(mesh: MeshVertexOutput,) -> @location(0) vec4<f32> {
    return textureSample(color_texture, color_sampler, mesh.uv) * COLOR_MULTIPLIER;
}