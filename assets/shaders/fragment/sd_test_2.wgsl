#import bevy_pbr::mesh_vertex_output            MeshVertexOutput
#import "shaders/sd_shapes.wgsl"                sdCircle, sdEquilateralTriangle, sdIsoscelesTriangle, sdTriangle, sdPentagon, sdHexagon, sdOctagon, sdHexagram, sdStar, sdPie
#import bevy_pbr::utils                         PI, HALF_PI
#import bevy_render::view                       View

@group(0) @binding(0) var<uniform> view: View;

@group(1) @binding(0) var color_texture: texture_2d<f32>;
@group(1) @binding(1) var color_sampler: sampler;

// -----------------------------------------------------------------------------
@fragment
fn fragment(in: MeshVertexOutput,) -> @location(0) vec4<f32> {
    var uv = in.uv.xy;

    var d = sdEquilateralTriangle(uv - vec2(0.2), 0.08);
    d = min(d, sdIsoscelesTriangle(uv - vec2(0.5, 0.2), vec2(0.05, 0.2)));
    d = min(d, sdTriangle(uv - vec2(0.8, 0.2), vec2(-0.03, 0.09), vec2(-0.1, -0.07), vec2(0.08, -0.0)));
    d = min(d, sdPentagon(uv - vec2(0.2, 0.5), 0.07));
    d = min(d, sdHexagon(uv - vec2(0.5), 0.07));
    d = min(d, sdOctagon(uv - vec2(0.8, 0.5), 0.07));
    d = min(d, sdHexagram(uv - vec2(0.2, 0.8), 0.06));
    d = min(d, sdStar(uv - vec2(0.5, 0.8), 0.1, u32(5), 3.0));
    d = min(d, sdPie(uv - vec2(0.8), vec2(sin(HALF_PI), cos(PI)), 0.09));
    let color = vec3(smoothedge(d));

    let texture = textureSample(color_texture, color_sampler, in.uv);
    return texture * vec4(color, 1.0);
}

fn smoothedge(v: f32) -> f32 {
    return smoothstep(0.0, 1.0 / view.viewport.z, v);
}