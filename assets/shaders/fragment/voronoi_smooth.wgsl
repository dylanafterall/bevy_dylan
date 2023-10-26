#import bevy_pbr::mesh_vertex_output        MeshVertexOutput
#import "shaders/shader_utils.wgsl"         voronoi_smooth

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    let texture = textureSample(color_texture, color_sampler, in.uv);
    let uv = in.uv.xy;

    let v = voronoi_smooth(6.0 * uv, 0.15);

    // gamma
    var col = sqrt(v.yzw);

    col *= 1.0 - 0.8 * v.x * step(uv.y, 0.33);
    col *= mix(v.x, 1.0, step(uv.y, 0.66));


    col *= smoothstep(0.003, 0.005, abs(uv.y - 0.33));
    col *= smoothstep(0.003, 0.005, abs(uv.y - 0.66));
    col *= smoothstep(0.003, 0.005, abs(uv.x));

    return texture * vec4(col, 1.0);
}