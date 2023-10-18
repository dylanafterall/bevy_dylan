#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import "shaders/distance_field_shapes.wgsl"    smoothedge, circle, rect, roundRect, ring, hexagon, tri, ellipse, capsule, polygon

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;

    var d = circle(uv - vec2(0.2), 0.1);
    d = min(d, rect(uv - vec2(0.5, 0.2), vec2(0.1, 0.08)));
    d = min(d, roundRect(uv - vec2(0.8, 0.2), vec2(0.08, 0.06), 0.02));
    d = min(d, ring(uv - vec2(0.2, 0.5), 0.18, 0.02));
    d = min(d, hexagon(uv - vec2(0.5, 0.5), 0.1));
    d = min(d, tri(uv - vec2(0.8, 0.5), 0.1));
    d = min(d, ellipse(uv - vec2(0.2, 0.8), vec2(0.9, 1.2), 0.1));
    d = min(d, capsule(uv - vec2(0.5, 0.8), vec2(-0.05, -0.05), vec2(0.05, 0.05), 0.05));
    let color = vec3(smoothedge(d));

    let texture = textureSample(color_texture, color_sampler, in.uv);

    return texture * vec4(color, 1.0);
}