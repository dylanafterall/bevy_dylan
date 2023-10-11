#import bevy_sprite::mesh2d_types as        MeshTypes
#import bevy_sprite::mesh2d_functions as    MeshFunctions

@group(1) @binding(0)
var<uniform> mesh: MeshTypes::Mesh2d;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: u32,
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) clip_position: vec4<f32>,
    // We pass the vertex color to the fragment shader in location 0
    @location(0) color: vec4<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    // Project the world position of the mesh into screen position
    out.clip_position = MeshFunctions::mesh2d_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    // Unpack the `u32` from the vertex buffer into the `vec4<f32>` used by the fragment shader
    out.color = vec4<f32>((vec4<u32>(vertex.color) >> vec4<u32>(0u, 8u, 16u, 24u)) & vec4<u32>(255u)) / 255.0;
    return out;
}