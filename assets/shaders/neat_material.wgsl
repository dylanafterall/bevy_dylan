// https://github.com/bevyengine/bevy/blob/30cb95d96e0b19ca44f92b30d638a9c1fc62a83f/assets/shaders/custom_material.wgsl
#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import "shaders/neat_material_import.wgsl" COLOR_MULTIPLIER

struct CustomMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0) var<uniform> material: CustomMaterial;
@group(1) @binding(1) var base_color_texture: texture_2d<f32>;
@group(1) @binding(2) var base_color_sampler: sampler;

@fragment
fn fragment(
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    return material.color * textureSample(base_color_texture, base_color_sampler, mesh.uv) * COLOR_MULTIPLIER;
}