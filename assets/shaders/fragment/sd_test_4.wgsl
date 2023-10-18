#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import "shaders/sd_shapes.wgsl"                sdEllipse, sdQuadraticCircle, sdTunnel, sdCutDisk, sdBezier, sdBlobbyCross, sdStairs
#import bevy_render::view                       View

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;

    var d = sdEllipse(uv - vec2(0.2), vec2(0.1, 0.04));
    d = min(d, sdTunnel(uv - vec2(0.5, 0.2), vec2(0.09, 0.08)));
    d = min(d, sdCutDisk(uv - vec2(0.8, 0.2), 0.08, 0.01));
    d = min(d, sdBlobbyCross(uv - vec2(0.2, 0.5), 0.65));
    // d = min(d, sdQuadraticCircle(uv - vec2(0.5)));
    //d = min(d, sdBezier(uv - vec2(0.8, 0.5)));
    //d = min(d, sdStairs(uv - vec2(0.2, 0.8)));
    let color = vec3(smoothedge(d));

    let texture = textureSample(color_texture, color_sampler, in.uv);
    return texture * vec4(color, 1.0);
}

fn smoothedge(v: f32) -> f32 {
    return smoothstep(0.0, 1.0 / view.viewport.z, v);
}