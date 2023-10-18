#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import "shaders/sd_shapes.wgsl"                sdCircle, sdRoundedBox, sdBox, sdOrientedBox, sdSegment, sdRhombus, sdTrapezoid, sdParallelogram, sdUnevenCapsule
#import bevy_render::view                       View

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;

    var d = sdCircle(uv - vec2(0.2), 0.1);
    d = min(d, sdBox(uv - vec2(0.5, 0.2), vec2(0.1, 0.08)));
    d = min(d, sdRoundedBox(uv - vec2(0.8, 0.2), vec2(0.1, 0.08), vec4(0.02)));
    d = min(d, sdOrientedBox(uv - vec2(0.2, 0.5), vec2(-0.05, -0.04), vec2(0.1, 0.08), 0.1));
    d = min(d, sdSegment(uv - vec2(0.5), vec2(-0.05, -0.04), vec2(0.1, 0.08)));
    d = min(d, sdRhombus(uv - vec2(0.8, 0.5), vec2(0.1, 0.08)));
    d = min(d, sdTrapezoid(uv - vec2(0.2, 0.8), 0.03, 0.08, 0.09));
    d = min(d, sdParallelogram(uv - vec2(0.5, 0.8), 0.08, 0.06, 0.05));
    d = min(d, sdUnevenCapsule(uv - vec2(0.8), 0.04, 0.07, 0.08));
    let color = vec3(smoothedge(d));

    let texture = textureSample(color_texture, color_sampler, in.uv);
    return texture * vec4(color, 1.0);
}

fn smoothedge(v: f32) -> f32 {
    return smoothstep(0.0, 1.0 / view.viewport.z, v);
}